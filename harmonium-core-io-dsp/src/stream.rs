use std::marker::PhantomData;

use arrow2::{
    array::{ListArray, PrimitiveArray},
    buffer::Buffer,
    datatypes::{DataType, Field},
    types::{NativeType, Offset},
};
use num_traits::Float;
use symphonia::core::{
    audio::SampleBuffer, codecs::Decoder, conv::ConvertibleSample, formats::FormatReader,
};

pub struct StreamStruct<O, T>
where
    O: Offset,
    T: Float + NativeType + ConvertibleSample,
{
    reader: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    // Buffer to allocate the samples read in a packet.
    sample_buf: Option<SampleBuffer<T>>,
    // Number of frames to decode, considering offset and duration.
    pub frames_to_decode: usize,
    // Number of samples to offset.
    pub offset_samples: u64,
    pub channels: usize,
    array: PhantomData<*const O>,
    // Number of frames to be returned in each iteration.
    pub frames: usize,
    // last index read in the packet. Needed to keep reading from the following sample when in a new iteration.
    pub last_idx: usize,
}

impl<O, T> StreamStruct<O, T>
where
    O: Offset,
    T: Float + NativeType + ConvertibleSample,
{
    pub fn new(
        reader: Box<dyn FormatReader>,
        decoder: Box<dyn Decoder>,
        track_id: u32,
        sample_buf: Option<SampleBuffer<T>>,
        frames_to_decode: usize,
        offset_samples: u64,
        channels: usize,
        array: PhantomData<*const O>,
        frames: usize,
        last_idx: usize,
    ) -> Self {
        StreamStruct {
            reader,
            decoder,
            track_id,
            sample_buf,
            frames_to_decode,
            offset_samples,
            channels,
            array,
            frames,
            last_idx,
        }
    }
}

impl<O, T> Iterator for StreamStruct<O, T>
where
    O: Offset,
    T: Float + NativeType + ConvertibleSample,
{
    type Item = ListArray<O>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frames_to_decode == 0 {
            return None;
        }

        // Vec to store the number of frames inputed in stream call.
        let mut vec_buf =
            vec![T::zero(); usize::min(self.frames, self.frames_to_decode) * self.channels];
        let mut idx = 0_usize;

        // If samples reminiscent from last read packet.
        if self.last_idx != 0 {
            if let Some(audio_buf) = &mut self.sample_buf {
                // The samples may now be access via the `samples()` function.
                let mut samples = audio_buf.samples();
                let mut ch: usize;

                samples = &samples[(self.last_idx * self.channels)..];

                // Helper to keep track of indices.
                let mult = vec_buf.len() / self.channels;

                for (n, sample) in samples.iter().enumerate() {
                    ch = n % self.channels;
                    vec_buf[idx + ch * mult] = *sample;

                    if ch == self.channels - 1 {
                        idx += 1;
                        self.frames_to_decode -= 1;

                        // Vec is filled.
                        if idx * self.channels == vec_buf.len() {
                            self.last_idx += idx;
                            let list_array = list_array_from_vec(vec_buf, self.channels);
                            return Some(list_array);
                        }

                        // Number of frames to decode reached.
                        if self.frames_to_decode == 0 {
                            let list_array = list_array_from_vec(vec_buf, self.channels);
                            return Some(list_array);
                        }
                    }
                }
            };
        }

        loop {
            // Get the next packet from the format reader.
            let packet = match self.reader.next_packet() {
                Ok(packet_ok) => packet_ok,
                Err(symphonia::core::errors::Error::IoError(ref packet_err))
                    if packet_err.kind() == std::io::ErrorKind::UnexpectedEof =>
                {
                    return None;
                }
                Err(_) => return None,
            };

            // If the packet does not belong to the selected track, skip it.
            if packet.track_id() != self.track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match self.decoder.decode(&packet) {
                Ok(audio_buf) => {
                    // The decoded audio samples may now be accessed via the audio buffer if per-channel
                    // slices of samples in their native decoded format is desired. Use-cases where
                    // the samples need to be accessed in an interleaved order or converted into
                    // another sample format, or a byte buffer is required, are covered by copying the
                    // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                    // example below, we will copy the audio buffer into a sample buffer in an
                    // interleaved order.

                    // If this is the *first* decoded packet, create a sample buffer matching the
                    // decoded audio buffer format.
                    if (self.sample_buf).is_none() {
                        // Get the audio buffer specification.
                        let spec = *audio_buf.spec();
                        // Get the capacity of the decoded buffer.
                        let cap = audio_buf.capacity() as u64;

                        // Create the sample buffer.
                        self.sample_buf = Some(SampleBuffer::<T>::new(cap, spec));
                    }

                    // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                    if let Some(buf) = &mut self.sample_buf {
                        let frames_in_buf = audio_buf.frames() as u64;

                        // Deal with offset.
                        if self.offset_samples * self.channels as u64 >= frames_in_buf {
                            self.offset_samples -= frames_in_buf;
                            continue;
                        }

                        buf.copy_interleaved_ref(audio_buf);

                        // The samples may now be access via the `samples()` function.
                        let mut samples = buf.samples();
                        let mut ch: usize;
                        self.last_idx = 0;

                        if self.offset_samples != 0 {
                            samples = &samples[(self.offset_samples as usize) * self.channels..];
                            self.offset_samples = 0;
                            self.last_idx += self.offset_samples as usize;
                        }

                        // Helper to keep track of indices.
                        let mult = vec_buf.len() / self.channels;

                        for (n, sample) in samples.iter().enumerate() {
                            ch = n % self.channels;
                            vec_buf[idx + ch * mult] = *sample;

                            if ch == self.channels - 1 {
                                idx += 1;
                                self.last_idx += 1;
                                self.frames_to_decode -= 1;

                                // Vec is filled.
                                if idx * self.channels == vec_buf.len() {
                                    let list_array = list_array_from_vec(vec_buf, self.channels);
                                    return Some(list_array);
                                }

                                // Number of frames to decode reached.
                                if self.frames_to_decode == 0 {
                                    let list_array = list_array_from_vec(vec_buf, self.channels);
                                    return Some(list_array);
                                }
                            }
                        }
                    }
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => {
                    return None;
                }
                Err(_) => return None,
            }
        }
    }
}

fn list_array_from_vec<O, T>(v: Vec<T>, channels: usize) -> ListArray<O>
where
    O: Offset,
    T: Float + NativeType + ConvertibleSample,
{
    let array = PrimitiveArray::from_vec(v);
    let offsets: Buffer<O> = O::range(0, array.len() + 1)
        .expect("cannot create create iterator from usize")
        .step_by(array.len() / channels)
        .collect();
    let field = Box::new(Field::new("item", array.data_type().clone(), true));

    match O::IS_LARGE {
        true => ListArray::from_data(DataType::LargeList(field), offsets, Box::new(array), None),
        false => ListArray::from_data(DataType::List(field), offsets, Box::new(array), None),
    }
}
