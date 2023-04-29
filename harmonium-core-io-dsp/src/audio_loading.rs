#[cfg(feature = "arrow")]
mod decoded_audio {
    use crate::errors::{OwnError, OwnResult};
    use crate::stream::StreamStruct;
    use arrow2::array::{ListArray, Offset, PrimitiveArray};
    use arrow2::buffer::Buffer;
    use arrow2::datatypes::{DataType, Field};
    use arrow2::types::NativeType;
    use num_traits::Float;
    use rubato::{
        FftFixedIn, FftFixedInOut, FftFixedOut, InterpolationParameters, InterpolationType,
        Resampler, Sample, SincFixedIn, SincFixedOut, WindowFunction,
    };
    use std::fs::File;
    use std::marker::PhantomData;
    use std::path::Path;
    use symphonia::core::{
        audio::SampleBuffer, codecs::DecoderOptions, conv::ConvertibleSample,
        formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
    };

    #[derive(Debug)]
    pub struct DecodedAudio<O: Offset> {
        data: ListArray<O>,
        sr: u32,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum VerifyDecode {
        Passed,
        Failed,
        NotSupported,
    }

    impl<O> DecodedAudio<O>
    where
        O: Offset,
    {
        fn new(data: ListArray<O>, sr: u32) -> DecodedAudio<O> {
            DecodedAudio { data, sr }
        }

        // underlying data.
        pub fn data(&self) -> &ListArray<O> {
            &self.data
        }

        // sampling rate.
        pub fn sr(&self) -> u32 {
            self.sr
        }

        // number of channels.
        pub fn nchannels(&self) -> usize {
            self.data.len()
        }

        // samples per channel.
        pub fn nsamples(&self) -> usize {
            self.data.values().len() / self.data.len()
        }

        /// Get the audio duration from a DecodedAudio in seconds. \cr
        /// # Examples
        ///
        /// ```
        /// //let decoded_audio = DecodedAudio::new(array2![[1.,2.,3.], [4.,5.,6.]], 44100);
        /// //decoded_audio.duration()
        /// ```
        pub fn duration(&self) -> f64 {
            self.nsamples() as f64 / self.sr() as f64
        }
    }

    /// Load an audio file as DecodedAudio.
    /// The samples are normalized to fit in the range of \[-1.0, 1.0\].
    /// # Arguments
    /// `fname` - The input file.
    /// `offset` - Start reading the file after the offset, in seconds.
    /// `duration` - Duration to be loaded, in seconds, counting from the offset. Will load the file till the end if offset + duration >= file length.
    /// # Examples
    ///
    /// ```
    /// //let test = "../testfiles/test.wav";
    /// //load(test, false, 1_f64, 2_f64)
    /// ```
    pub fn load<O, T>(
        fpath: &str,
        offset: Option<f64>,
        duration: Option<f64>,
    ) -> OwnResult<DecodedAudio<O>>
    where
        O: Offset,
        T: Float + NativeType + ConvertibleSample,
    {
        let fpath = Path::new(fpath);
        let ext = Path::extension(fpath)
            .ok_or_else(|| OwnError::IoError("couldn't extract the file extension".into()))?
            .to_str()
            .ok_or_else(|| OwnError::IoError("cannot convert from &OsStr to &str".into()))?;
        // Create a media source. Note that the MediaSource trait is automatically implemented for File, among other types.
        let file = Box::new(File::open(fpath)?);
        // Create the media source stream using the boxed media source from above.
        let mss = MediaSourceStream::new(file, Default::default());
        // Create a hint to help the format registry guess what format reader is appropriate.
        let mut hint = Hint::new();
        hint.with_extension(ext);
        // Use the default options when reading and decoding.
        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = Default::default();
        // Probe the media source stream for a format.
        let probed =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
        // Get the format reader yielded by the probe operation.
        let mut reader = probed.format;
        // Get the default track.
        let track = reader
            .default_track()
            .ok_or_else(|| OwnError::DecodeError("no tracks were detected".into()))?;
        // Create a decoder for the track.
        let mut decoder =
            symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;
        let channels = decoder
            .codec_params()
            .channels
            .ok_or_else(|| OwnError::DecodeError("cannot retrieve the number of channels".into()))?
            .count();
        // Total number of frames. In PCM n_frames is the same as nsamples, but for each channel.
        let n_frames = decoder
            .codec_params()
            .n_frames
            .ok_or_else(|| OwnError::DecodeError("cannot retrieve the number of frames".into()))?;
        let sr = decoder
            .codec_params()
            .sample_rate
            .ok_or_else(|| OwnError::DecodeError("cannot retrieve the sample rate".into()))?;
        let file_duration = n_frames as f64 / sr as f64;
        let offset = offset.unwrap_or(0.);
        // Assures duration max = duration from file - offset
        let duration_to_decode = f64::min(
            duration.unwrap_or(file_duration - offset),
            file_duration - offset,
        );

        if duration_to_decode <= 0.0 {
            return Err(OwnError::DecodeError("duration must be positive".into()));
        }

        // Round to the lower bound integer by default when converting to integer. offset_samples is by channel.
        let mut offset_samples = (offset * (sr as f64)) as u64;

        if (offset_samples) >= n_frames {
            return Err(OwnError::DecodeError(
                "offset must be less than total audio duration".into(),
            ));
        }

        // Round to the lower bound integer by default when converting to integer.
        let mut frames_to_decode = (duration_to_decode * (sr as f64)) as usize;
        let track_id = track.id;
        let mut sample_buf: Option<SampleBuffer<T>> = None;
        let mut v: Vec<T> = vec![T::zero(); channels * frames_to_decode]; // implement uninit
        let mut idx = 0_usize;

        'outer: loop {
            // Get the next packet from the format reader.
            let packet = match reader.next_packet() {
                Ok(packet_ok) => packet_ok,
                Err(symphonia::core::errors::Error::IoError(ref packet_err))
                    if packet_err.kind() == std::io::ErrorKind::UnexpectedEof =>
                {
                    break;
                }
                Err(packet_err) => Err(packet_err)?,
            };

            // If the packet does not belong to the selected track, skip it.
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match decoder.decode(&packet) {
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
                    if sample_buf.is_none() {
                        // Get the audio buffer specification.
                        let spec = *audio_buf.spec();
                        // Get the capacity of the decoded buffer.
                        let cap = audio_buf.capacity() as u64;

                        // Create the sample buffer.
                        sample_buf = Some(SampleBuffer::<T>::new(cap, spec));
                    }

                    // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                    if let Some(buf) = &mut sample_buf {
                        let frames_in_buf = audio_buf.frames() as u64;

                        if offset_samples * channels as u64 >= frames_in_buf {
                            // deal with offset.
                            offset_samples -= frames_in_buf;
                            continue;
                        }

                        buf.copy_interleaved_ref(audio_buf);

                        // The samples may now be access via the `samples()` function.
                        let mut samples = buf.samples();
                        let mut ch: usize;

                        if offset_samples != 0 {
                            samples = &samples[(offset_samples as usize) * channels..];
                            offset_samples = 0;
                        }

                        // Helper to keep track of offset.
                        let mult = samples.len() / channels;

                        for (n, sample) in samples.iter().enumerate() {
                            ch = n % channels;
                            v[idx + ch * mult] = *sample;

                            if ch == channels - 1 {
                                idx += 1;
                                frames_to_decode -= 1;
                            }

                            if frames_to_decode == 0 {
                                // then skip the rest
                                break 'outer;
                            }
                        }
                    }
                }
                Err(symphonia::core::errors::Error::DecodeError(err_str)) => {
                    Err(symphonia::core::errors::Error::DecodeError(err_str))?
                }
                Err(_) => break,
            }
        }

        let array = PrimitiveArray::from_vec(v);
        let offsets: Buffer<O> = O::range(0, array.len() + 1)
            .expect("cannot create create iterator from usize")
            .step_by(array.len() / channels)
            .collect();
        let field = Box::new(Field::new("item", array.data_type().clone(), true));
        let list_array = match O::IS_LARGE {
            true => {
                ListArray::from_data(DataType::LargeList(field), offsets, Box::new(array), None)
            }
            false => ListArray::from_data(DataType::List(field), offsets, Box::new(array), None),
        };
        let decoded_audio = DecodedAudio::new(list_array, sr);

        Ok(decoded_audio)
    }

    /// stream an audio file as an iterator.
    /// The samples are normalized to fit in the range of \[-1.0, 1.0\].
    /// # Arguments
    /// `fname` - The input file.
    /// `offset` - Start reading the file after the offset, in seconds.
    /// `duration` - Duration to be loaded, in seconds, counting from the offset. Will load the file till the end if offset + duration >= file length.
    /// `frames` - Number of frames to decode per iteration.
    /// # Examples
    ///
    /// ```
    /// //let test = "../testfiles/test.wav";
    /// //stream(test, 1.0_f64, 2.0_f64, 1000)
    /// ```
    pub fn stream<O, T>(
        fpath: &str,
        offset: Option<f64>,
        duration: Option<f64>,
        frames: usize,
    ) -> OwnResult<StreamStruct<O, T>>
    where
        O: Offset,
        T: Float + NativeType + ConvertibleSample,
    {
        let fpath = Path::new(fpath);
        let ext = Path::extension(fpath)
            .ok_or_else(|| OwnError::IoError("couldn't extract the file extension".into()))?
            .to_str()
            .ok_or_else(|| OwnError::IoError("cannot convert from &OsStr to &str".into()))?;
        // Create a media source. Note that the MediaSource trait is automatically implemented for File, among other types.
        let file = Box::new(File::open(fpath)?);
        // Create the media source stream using the boxed media source from above.
        let mss = MediaSourceStream::new(file, Default::default());
        // Create a hint to help the format registry guess what format reader is appropriate.
        let mut hint = Hint::new();
        hint.with_extension(ext);
        // Use the default options when reading and decoding.
        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = DecoderOptions {
            verify: false,
            ..Default::default()
        };
        // Probe the media source stream for a format.
        let probed =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
        // Get the format reader yielded by the probe operation.
        let reader = probed.format;
        // Get the default track.
        let track = reader
            .default_track()
            .ok_or_else(|| OwnError::DecodeError("no tracks were detected".into()))?;
        // Create a decoder for the track.
        let decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;
        let channels = decoder
            .codec_params()
            .channels
            .ok_or_else(|| OwnError::DecodeError("cannot retrieve the number of channels".into()))?
            .count();
        // Total number of frames. In PCM n_frames is the same as nsamples, but for each channel.
        let n_frames = decoder
            .codec_params()
            .n_frames
            .ok_or_else(|| OwnError::DecodeError("cannot retrieve the number of frames".into()))?;
        let sr = decoder
            .codec_params()
            .sample_rate
            .ok_or_else(|| OwnError::DecodeError("cannot retrieve the sample rate".into()))?;
        let file_duration = n_frames as f64 / sr as f64;
        let offset = offset.unwrap_or(0.);
        // Assures duration max = duration from file - offset
        let duration_to_decode = f64::min(
            duration.unwrap_or(file_duration - offset),
            file_duration - offset,
        );

        if duration_to_decode <= 0.0 {
            return Err(OwnError::DecodeError("duration must be positive".into()));
        }

        // Rounds to the lower bound integer by default when converting to integer. offset_samples is by channel.
        let offset_samples = (offset * (sr as f64)) as u64;

        if (offset_samples) >= n_frames {
            return Err(OwnError::DecodeError(
                "offset must be less than total audio duration".into(),
            ));
        }

        // Total number of frames to decode considering duration and offset. Rounds to the lower bound integer by default when converting to integer.
        let frames_to_decode = (duration_to_decode * (sr as f64)) as usize;
        let track_id = track.id;
        let sample_buf: Option<SampleBuffer<T>> = None;
        let last_idx = 0;

        let stream_struct = StreamStruct::new(
            reader,
            decoder,
            track_id,
            sample_buf,
            frames_to_decode,
            offset_samples,
            channels,
            PhantomData,
            frames,
            last_idx,
        );

        Ok(stream_struct)
    }

    /// Verify an audio file, if supported by the decoder. The verification is done after the decoding process is finished.
    /// # Arguments
    /// `fname` - The input file.
    /// # Examples
    ///
    /// ```
    /// //verify("../testfiles/test.wav");
    /// ```
    pub fn verify(fpath: &str) -> OwnResult<VerifyDecode> {
        let fpath = Path::new(fpath);
        let ext = Path::extension(fpath)
            .ok_or_else(|| OwnError::IoError("couldn't extract the file extension".into()))?
            .to_str()
            .ok_or_else(|| OwnError::IoError("cannot convert from &OsStr to &str".into()))?;
        // Create a media source. Note that the MediaSource trait is automatically implemented for File, among other types.
        let file = Box::new(File::open(fpath)?);
        // Create the media source stream using the boxed media source from above.
        let mss = MediaSourceStream::new(file, Default::default());
        // Create a hint to help the format registry guess what format reader is appropriate.
        let mut hint = Hint::new();
        hint.with_extension(ext);
        // Use the default options when reading and decoding.
        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = DecoderOptions {
            verify: true,
            ..Default::default()
        };
        // Probe the media source stream for a format.
        let probed =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
        // Get the format reader yielded by the probe operation.
        let mut reader = probed.format;
        // Get the default track.
        let track = reader
            .default_track()
            .ok_or_else(|| OwnError::DecodeError("no tracks were detected".into()))?;
        // Create a decoder for the track.
        let mut decoder =
            symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;
        let track_id = track.id;

        loop {
            // Get the next packet from the format reader.
            let packet = match reader.next_packet() {
                Ok(packet_ok) => packet_ok,
                Err(symphonia::core::errors::Error::IoError(ref packet_err))
                    if packet_err.kind() == std::io::ErrorKind::UnexpectedEof =>
                {
                    break;
                }
                Err(packet_err) => Err(packet_err)?,
            };

            // If the packet does not belong to the selected track, skip it.
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match decoder.decode(&packet) {
                Ok(_) => continue,
                Err(symphonia::core::errors::Error::DecodeError(err_str)) => {
                    Err(symphonia::core::errors::Error::DecodeError(err_str))?
                }
                Err(_) => break,
            }
        }

        // check that it works. It doesn't work with the .wav example. Symphonia says it only works
        // with some type of exts.
        //finalize the decoder to get the verification result.
        let finalize_result = decoder.finalize();

        if let Some(verify_ok) = finalize_result.verify_ok {
            if verify_ok {
                Ok(VerifyDecode::Passed)
            } else {
                Ok(VerifyDecode::Failed)
            }
        } else {
            Ok(VerifyDecode::NotSupported)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_test() {
            let array = PrimitiveArray::from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]);
            let channels = 3_usize;
            let offsets: Buffer<i32> = (0..array.len() as i32 + 1)
                .step_by(array.len() / channels)
                .collect();
            let field = Box::new(Field::new("item", array.data_type().clone(), true));
            let list_array = ListArray::from_data(
                DataType::List(field.clone()),
                offsets.clone(),
                Box::new(array.clone()),
                None,
            );
            let decoded_audio = DecodedAudio::new(list_array, 44000);

            assert_eq!(
                *decoded_audio.data(),
                ListArray::from_data(DataType::List(field), offsets, Box::new(array), None)
            );
            assert_eq!(decoded_audio.sr(), 44000);
            assert_eq!(decoded_audio.nsamples(), 3);
            assert_eq!(decoded_audio.nchannels(), 3);
        }

        macro_rules! load_test {
            ($input: expr, $offset_slice: expr, $channel: expr, $results: expr, $(($type1: ty, $type2: ty)),+ $(,)*) => {
                let (fname, offset, duration) = $input;
                $(
                    let v: Vec<$type2> = (*$results.0)
                    .clone()
                    .into_iter()
                    .map(|x| <$type2>::from(x))
                    .collect();
                    let result = PrimitiveArray::from_vec(v).boxed();
                    let decoded_audio = load::<$type1, $type2>(
                        fname,
                        offset,
                        duration,
                    )
                    .unwrap();

                    assert_eq!(
                        decoded_audio.data().value($channel).slice($offset_slice, 5),
                        result
                    );
                    assert_eq!(decoded_audio.sr(), $results.1);
                    assert_eq!(decoded_audio.nchannels(), $results.2);
                    assert_eq!(decoded_audio.nsamples(), $results.3);
                    )+
            };
        }

        #[test]
        fn load_test() {
            let (fname, offset, duration) = ("../testfiles/gs-16b-2c-44100hz.wav", None, None);
            let channel = 0;
            let offset_slice = 0;
            let result = vec![
                0.0,
                3.0517578125e-5,
                -6.103515625e-5,
                6.103515625e-5,
                -6.103515625e-5,
            ];

            load_test!(
                (fname, offset, duration),
                offset_slice,
                channel,
                (&result, 44100, 2, 698194),
                (i32, f32),
                (i32, f64),
                (i64, f32),
                (i64, f64),
            );

            // test with offset
            let offset = Some(1.);
            let result = vec![
                -0.030242919921875,
                -0.0218505859375,
                -0.021697998046875,
                -0.01953125,
                -0.04119873046875,
            ];

            load_test!(
                (fname, offset, duration),
                offset_slice,
                channel,
                (&result, 44100, 2, 654094),
                (i32, f32),
                (i32, f64),
                (i64, f32),
                (i64, f64),
            );

            // test with offset and duration
            let duration = Some(0.5);
            let result = vec![
                0.09576416015625,
                0.100921630859375,
                0.10406494140625,
                0.110748291015625,
                0.10919189453125,
            ];
            let nsamples = 22050;
            let offset_slice = nsamples - 5;

            load_test!(
                (fname, offset, duration),
                offset_slice,
                channel,
                (&result, 44100, 2, 22050),
                (i32, f32),
                (i32, f64),
                (i64, f32),
                (i64, f64),
            );
        }

        macro_rules! stream_test {
            ($input: expr, $channel: expr, $dimensions: expr, $(($type1: ty, $type2: ty)),+ $(,)*) => {
                let (fname, offset, duration, frames) = $input;
                $(
                    let loaded_audio = load::<$type1, $type2>(fname, offset, duration).unwrap();

                    let mut stream_struct = stream::<$type1, $type2>(fname, offset, duration, frames).unwrap();

                    // test first iteration
                    let next_stream = stream_struct.next().unwrap();
                    assert_eq!((next_stream.len(), next_stream.value($channel).len()), ($dimensions.0, $dimensions.1));
                    assert_eq!(next_stream.value($channel), loaded_audio.data().value($channel).slice(0, frames));

                    // test second iteration
                    let next_stream = stream_struct.next().unwrap();
                    assert_eq!((next_stream.len(), next_stream.value($channel).len()), ($dimensions.0, $dimensions.1));
                    assert_eq!(next_stream.value($channel), loaded_audio.data().value($channel).slice(frames, frames));

                    // test last iteration
                    let last_stream = stream_struct.last().unwrap();
                    let nsamples_stream = last_stream.value($channel).len();
                    let nsamples_loaded = loaded_audio.data().value($channel).len();
                    assert_eq!(last_stream.value($channel).slice(nsamples_stream - 10, 10), loaded_audio.data().value($channel).slice(nsamples_loaded - 10, 10));
                )+
            };
        }

        #[test]
        fn stream_test() {
            let (fname, offset, duration, frames) =
                ("../testfiles/gs-16b-2c-44100hz.wav", None, None, 1000);
            let channel = 0;
            let (nchannels, nsamples) = (2, frames);

            stream_test!(
                (fname, offset, duration, frames),
                channel,
                (nchannels, nsamples),
                (i32, f32),
                (i32, f64),
                (i64, f32),
                (i64, f64),
            );
        }

        #[test]
        fn verify_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let verify_decode = verify(fname).unwrap();
            assert_eq!(verify_decode, VerifyDecode::NotSupported);
        }
    }
}
