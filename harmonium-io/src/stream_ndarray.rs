use ndarray::{ArrayBase, DataMut, DataOwned, Ix2};
use num_traits::{Float, FromPrimitive};
use std::marker::PhantomData;
use symphonia::core::{
    audio::SampleBuffer, codecs::Decoder, conv::ConvertibleSample, formats::FormatReader,
};

pub struct StreamStruct<T, S>
where
    T: Float + ConvertibleSample,
    S: DataOwned<Elem = T> + DataMut<Elem = T>,
{
    reader: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    sample_buf: Option<SampleBuffer<T>>,
    frames_to_decode: usize,
    offset_samples: u64,
    channels: usize,
    array: PhantomData<*const S>,
}

impl<T, S> StreamStruct<T, S>
where
    T: Float + ConvertibleSample,
    S: DataOwned<Elem = T> + DataMut<Elem = T>,
{
    pub fn new(
        reader: Box<dyn FormatReader>,
        decoder: Box<dyn Decoder>,
        track_id: u32,
        sample_buf: Option<SampleBuffer<T>>,
        frames_to_decode: usize,
        offset_samples: u64,
        channels: usize,
        array: PhantomData<*const S>,
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
        }
    }
}

impl<T, S> Iterator for StreamStruct<T, S>
where
    T: Float + ConvertibleSample,
    S: DataOwned<Elem = T> + DataMut<Elem = T>,
{
    type Item = ArrayBase<S, Ix2>;

    fn next(&mut self) -> Option<Self::Item> {
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

                        if self.offset_samples * self.channels as u64 >= frames_in_buf {
                            // deal with offset.
                            self.offset_samples -= frames_in_buf;
                            continue;
                        }

                        buf.copy_interleaved_ref(audio_buf);

                        // The samples may now be access via the `samples()` function.
                        let mut samples = buf.samples();
                        let mut ch: usize;

                        if self.offset_samples != 0 {
                            samples = &samples[(self.offset_samples as usize) * self.channels..];
                            self.offset_samples = 0;
                        }

                        let n_samples_array = usize::min(self.frames_to_decode, samples.len() / 2);
                        let mut arr = ArrayBase::<S, Ix2>::zeros((self.channels, n_samples_array)); // try to cache arr
                        let mut idx = 0_usize;

                        for (n, sample) in samples.iter().enumerate() {
                            ch = n % self.channels;
                            arr[[ch, idx]] = *sample;

                            if ch == self.channels - 1 {
                                idx += 1;
                                self.frames_to_decode -= 1;
                            }

                            if self.frames_to_decode == 0 {
                                return Some(arr);
                            }
                        }
                        return Some(arr);
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
