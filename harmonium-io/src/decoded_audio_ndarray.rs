#[cfg(feature = "ndarray")]
mod decoded_audio_ndarray {
    use crate::errors::{OwnError, OwnResult};
    use crate::stream::StreamStruct;
    use ndarray::{s, ArrayBase, Axis, Data, DataMut, DataOwned, Ix2, OwnedRepr, RawDataClone};
    use num_traits::{Float, FromPrimitive};
    use rubato::{
        FftFixedIn, FftFixedInOut, FftFixedOut, InterpolationParameters, InterpolationType,
        Resampler, Sample, SincFixedIn, SincFixedOut, WindowFunction,
    };
    use std::path::Path;
    use std::{fs::File, marker::PhantomData};
    use symphonia::core::{
        audio::SampleBuffer, codecs::DecoderOptions, conv::ConvertibleSample,
        formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
    };

    #[derive(Clone, Debug)]
    struct DecodedAudio<T, S>
    where
        T: Float,
        S: Data<Elem = T> + RawDataClone<Elem = T>,
    {
        data: ArrayBase<S, Ix2>,
        sr: u32,
    }

    enum VerifyDecode {
        Verify,
        NotVerify,
    }

    impl<T, S> DecodedAudio<T, S>
    where
        T: Float,
        S: Data<Elem = T> + RawDataClone<Elem = T>,
    {
        fn new(data: ArrayBase<S, Ix2>, sr: u32) -> DecodedAudio<T, S> {
            DecodedAudio { data, sr }
        }

        // underlying data.
        pub fn data(&self) -> &ArrayBase<S, Ix2> {
            &self.data
        }

        // sampling rate.
        pub fn sr(&self) -> u32 {
            self.sr
        }

        // number of channels.
        pub fn nchannels(&self) -> u8 {
            self.data.nrows() as u8
        }

        // samples per channel.
        pub fn nsamples(&self) -> u64 {
            self.data.ncols() as u64
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

        /// Load an audio file as DecodedAudio.
        /// The samples are normalized to fit in the range of \[-1.0, 1.0\].
        /// # Arguments
        /// `fname` - The input file.
        /// `offset` - Start reading the file after the offset, in seconds.
        /// `duration` - Duration to be loaded, in seconds, counting from the offset. Will load the file till the end if offset + duration >= file length.
        /// `verify_decode` - If verification is enabled and supported by the decoder, provides the verification result.
        /// # Examples
        ///
        /// ```
        /// //let test = "../testfiles/test.wav";
        /// //load(test, false, 1_f64, 2_f64)
        /// ```
        pub fn load(
            fpath: &str,
            offset: Option<f64>,
            duration: Option<f64>,
            verify_decode: VerifyDecode,
        ) -> OwnResult<DecodedAudio<T, S>>
        where
            T: ConvertibleSample,
            S: DataOwned<Elem = T> + DataMut<Elem = T>,
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
            let decoder_opts: DecoderOptions = match verify_decode {
                VerifyDecode::Verify => DecoderOptions {
                    verify: true,
                    ..Default::default()
                },
                VerifyDecode::NotVerify => DecoderOptions {
                    verify: false,
                    ..Default::default()
                },
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
            let channels = decoder
                .codec_params()
                .channels
                .ok_or_else(|| {
                    OwnError::DecodeError("cannot retrieve the number of channels".into())
                })?
                .count();
            let n_frames = decoder // in PCM n_frames is the same as nsamples, but for each channel.
                .codec_params()
                .n_frames
                .ok_or_else(|| {
                    OwnError::DecodeError("cannot retrieve the number of frames".into())
                })?;
            let sr = decoder
                .codec_params()
                .sample_rate
                .ok_or_else(|| OwnError::DecodeError("cannot retrieve the sample rate".into()))?;
            let file_duration = n_frames as f64 / sr as f64;
            let offset = offset.unwrap_or(0.);
            // assures duration max = duration from file - offset
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
            let mut arr = ArrayBase::<S, Ix2>::zeros((channels, frames_to_decode)); // implement uninit
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

                            for (n, sample) in samples.iter().enumerate() {
                                ch = n % channels;
                                arr[[ch, idx]] = *sample;

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

            // check that it works. It doesn't work with the .wav example. Symphonia says it only works
            // with some type of exts.
            match verify_decode {
                VerifyDecode::Verify => {
                    //finalize the decoder to get the verification result.
                    let finalize_result = decoder.finalize();

                    if let Some(verify_ok) = finalize_result.verify_ok {
                        if verify_ok {
                            println!("verification passed");
                        } else {
                            println!("verification failed");
                        }
                    } else {
                        println!("verification not supported");
                    }
                }
                VerifyDecode::NotVerify => (),
            }

            let decoded_audio = DecodedAudio::new(arr, sr);

            Ok(decoded_audio)
        }

        /// stream an audio file as an iterator with ndarray chunks.
        /// The number of decoded samples per packet depends on the codec and is quite variable.
        /// The samples are normalized to fit in the range of \[-1.0, 1.0\].
        /// # Arguments
        /// `fname` - The input file.
        /// `offset` - Start reading the file after the offset, in seconds.
        /// `duration` - Duration to be loaded, in seconds, counting from the offset. Will load the file till the end if offset + duration >= file length.
        /// # Examples
        ///
        /// ```
        /// //let test = "../testfiles/test.wav";
        /// //stream(test, 1_f64, 2_f64)
        /// ```
        pub fn stream(
            fpath: &str,
            offset: Option<f64>,
            duration: Option<f64>,
        ) -> OwnResult<StreamStruct<T, S>>
        where
            T: ConvertibleSample,
            S: DataOwned<Elem = T> + DataMut<Elem = T>,
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
            let decoder =
                symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;
            let channels = decoder
                .codec_params()
                .channels
                .ok_or_else(|| {
                    OwnError::DecodeError("cannot retrieve the number of channels".into())
                })?
                .count();
            let n_frames = decoder // in PCM n_frames is the same as nsamples, but for each channel.
                .codec_params()
                .n_frames
                .ok_or_else(|| {
                    OwnError::DecodeError("cannot retrieve the number of frames".into())
                })?;
            let sr = decoder
                .codec_params()
                .sample_rate
                .ok_or_else(|| OwnError::DecodeError("cannot retrieve the sample rate".into()))?;
            let file_duration = n_frames as f64 / sr as f64;
            let offset = offset.unwrap_or(0.);
            // assures duration max = duration from file - offset
            let duration_to_decode = f64::min(
                duration.unwrap_or(file_duration - offset),
                file_duration - offset,
            );

            if duration_to_decode <= 0.0 {
                return Err(OwnError::DecodeError("duration must be positive".into()));
            }

            // Round to the lower bound integer by default when converting to integer. offset_samples is by channel.
            let offset_samples = (offset * (sr as f64)) as u64;

            if (offset_samples) >= n_frames {
                return Err(OwnError::DecodeError(
                    "offset must be less than total audio duration".into(),
                ));
            }

            // Round to the lower bound integer by default when converting to integer.
            let frames_to_decode = (duration_to_decode * (sr as f64)) as usize;
            let track_id = track.id;
            let sample_buf: Option<SampleBuffer<T>> = None;

            let stream_struct = StreamStruct::new(
                reader,
                decoder,
                track_id,
                sample_buf,
                frames_to_decode,
                offset_samples,
                channels,
                PhantomData,
            );

            Ok(stream_struct)
        }

        /// Verify an audio file if supported by the decoder. The verification is done after the decoding process is finished.
        /// # Arguments
        /// `fname` - The input file.
        /// # Examples
        ///
        /// ```
        /// //verify("../testfiles/test.wav");
        /// ```
        pub fn verify(fpath: &str) -> OwnResult<()>
        where
            T: ConvertibleSample,
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
                    println!("verification passed");
                } else {
                    println!("verification failed");
                }
            }

            Ok(())
        }

        /// Resample the audio data from sr_in to sr_out.
        /// fftfixedin: A synchronous resampler that needs a fixed number of audio frames for input and returns a variable number of frames. The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter before it’s inverse transformed to get the resampled waveforms. \cr
        /// Synchronous resampling: is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data. This type of resampler is considerably faster but doesn’t support changing the resampling ratio.
        /// # Arguments
        /// `sr_out` - Target sampling rate.
        /// `chunk_size_in` - Size of input data in frames.
        /// `sub_chunks` - Desired number of subchunks for processing, actual number may be different.
        /// # Examples
        ///
        /// ```
        /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
        /// //let offset = None;
        /// //let duration = None;
        /// //let mut decoded_audio =
        /// //    DecodedAudio::<f64, OwnedRepr<f64>>::load(fname, offset, duration, VerifyDecode::Verify).unwrap();
        /// //decoded_audio.resample_fftfixedin(22000, 1024, 2).unwrap();
        /// ```
        pub fn resample_fftfixedin(
            &mut self,
            sr_out: usize,
            chunk_size_in: usize,
            sub_chunks: usize,
        ) -> OwnResult<()>
        where
            T: Sample,
            S: DataOwned<Elem = T>,
        {
            let nchannels = self.nchannels() as usize;
            let nsamples = self.nsamples() as usize;
            let mut resampler = FftFixedIn::<T>::new(
                self.sr() as usize,
                sr_out,
                chunk_size_in,
                sub_chunks,
                nchannels,
            )?;

            resampler.process_resampler(
                self,
                nchannels,
                nsamples,
                sr_out,
                ResamplerType::FixedIn,
            )?;

            Ok(())
        }

        /// Resample the audio data from sr_in to sr_out.
        /// fftfixedinout: A synchronous resampler that accepts a fixed number of audio frames for input and returns a fixed number of frames. The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter before it’s inverse transformed to get the resampled waveforms.
        /// Synchronous resampling: is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data. This type of resampler is considerably faster but doesn’t support changing the resampling ratio.
        /// # Arguments
        /// `sr_out` - Target sampling rate.
        /// `chunk_size_in` - Size of input data in frames.
        /// # Examples
        ///
        /// ```
        /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
        /// //let offset = None;
        /// //let duration = None;
        /// //let mut decoded_audio =
        /// //    DecodedAudio::<f64, OwnedRepr<f64>>::load(fname, offset, duration, VerifyDecode::Verify).unwrap();
        /// //decoded_audio.resample_fftfixedinout(22000, 1024).unwrap();
        /// ```
        pub fn resample_fftfixedinout(
            &mut self,
            sr_out: usize,
            chunk_size_in: usize,
        ) -> OwnResult<()>
        where
            T: Sample,
            S: DataOwned<Elem = T>,
        {
            let nchannels = self.nchannels() as usize;
            let nsamples = self.nsamples() as usize;
            let mut resampler =
                FftFixedInOut::<T>::new(self.sr() as usize, sr_out, chunk_size_in, nchannels)?;

            resampler.process_resampler(
                self,
                nchannels,
                nsamples,
                sr_out,
                ResamplerType::FixedIn,
            )?;

            Ok(())
        }

        /// Resample the audio data from sr_in to sr_out.
        /// fftfixedout: A synchronous resampler that needs a varying number of audio frames for input and returns a fixed number of frames. The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter before it’s inverse transformed to get the resampled waveforms.
        /// Synchronous resampling: is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data. This type of resampler is considerably faster but doesn’t support changing the resampling ratio.
        /// # Arguments
        /// `sr_out` - Target sampling rate.
        /// `chunk_size_out` - Size of output data in frames.
        /// `sub_chunks` - Desired number of subchunks for processing, actual number may be different.
        /// # Examples
        ///
        /// ```
        /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
        /// //let offset = None;
        /// //let duration = None;
        /// //let mut decoded_audio =
        /// //    DecodedAudio::<f64, OwnedRepr<f64>>::load(fname, offset, duration, VerifyDecode::Verify).unwrap();
        /// //decoded_audio.resample_fftfixedout(22000, 1024, 2).unwrap();
        /// ```
        pub fn resample_fftfixedout(
            &mut self,
            sr_out: usize,
            chunk_size_out: usize,
            sub_chunks: usize,
        ) -> OwnResult<()>
        where
            T: Sample,
            S: DataOwned<Elem = T>,
        {
            let nchannels = self.nchannels() as usize;
            let nsamples = self.nsamples() as usize;
            let mut resampler = FftFixedOut::<T>::new(
                self.sr() as usize,
                sr_out,
                chunk_size_out,
                sub_chunks,
                nchannels,
            )?;

            resampler.process_resampler(
                self,
                nchannels,
                nsamples,
                sr_out,
                ResamplerType::NotFixedIn,
            )?;

            Ok(())
        }

        /// Resample the audio data from sr_in to sr_out.
        /// sincfixedin: An asynchronous resampler that accepts a fixed number of audio frames for input and returns a variable number of frames. The resampling is done by creating a number of intermediate points (defined by oversampling_factor) by sinc interpolation. The new samples are then calculated by interpolating between these points.
        /// Asynchronous resampling: the resampling is based on band-limited interpolation using sinc interpolation filters. The sinc interpolation upsamples by an adjustable factor, and then the new sample points are calculated by interpolating between these points. The resampling ratio can be updated at any time.
        /// # Arguments
        /// `sr_out` - Target sampling rate.
        /// `max_resample_ratio_relative` - Maximum ratio that can be set with Resampler::set_resample_ratio relative to resample_ratio, must be >= 1.0. The minimum relative ratio is the reciprocal of the maximum. For example, with max_resample_ratio_relative of 10.0, the ratio can be set between resample_ratio * 10.0 and resample_ratio / 10.0.
        /// `sinc_len` - Length of the windowed sinc interpolation filter. Higher values can allow a higher cut-off frequency leading to less high frequency roll-off at the expense of higher cpu usage. 256 is a good starting point. The value will be rounded up to the nearest multiple of 8.
        /// `f_cutoff` - Relative cutoff frequency of the sinc interpolation filter (relative to the lowest one of fs_in/2 or fs_out/2). Start at 0.95, and increase if needed.
        /// `oversampling_factor` - The number of intermediate points to use for interpolation. Higher values use more memory for storing the sinc filters. Only the points actually needed are calculated during processing so a larger number does not directly lead to higher cpu usage. But keeping it down helps in keeping the sincs in the cpu cache. Starts at 128.
        /// `interpolation` - Interpolation type. One of \["cubic", "linear", "nearest"\]. \cr
        /// For asynchronous interpolation where the ratio between input and output sample rates can be any number, it’s not possible to pre-calculate all the needed interpolation filters. Instead they have to be computed as needed, which becomes impractical since the sincs are very expensive to generate in terms of cpu time. It’s more efficient to combine the sinc filters with some other interpolation technique. Then sinc filters are used to provide a fixed number of interpolated points between input samples, and then the new value is calculated by interpolation between those points. \cr
        /// Variants:
        /// \itemize{
        /// \item "cubic": For cubic interpolation, the four nearest intermediate points are calculated using sinc interpolation. Then a cubic polynomial is fitted to these points, and is then used to calculate the new sample value. The computation time as about twice the one for linear interpolation, but it requires much fewer intermediate points for a good result.
        /// \item "linear": With linear interpolation the new sample value is calculated by linear interpolation between the two nearest points. This requires two intermediate points to be calculated using sinc interpolation, and te output is a weighted average of these two. This is relatively fast, but needs a large number of intermediate points to push the resampling artefacts below the noise floor.
        /// \item "nearest": The Nearest mode doesn’t do any interpolation, but simply picks the nearest intermediate point. This is useful when the nearest point is actually the correct one, for example when upsampling by a factor 2, like 48kHz->96kHz. Then setting the oversampling_factor to 2, and using Nearest mode, no unnecessary computations are performed and the result is the same as for synchronous resampling. This also works for other ratios that can be expressed by a fraction. For 44.1kHz -> 48 kHz, setting oversampling_factor to 160 gives the desired result (since 48kHz = 160/147 * 44.1kHz).
        /// }
        /// `window` - Window function to use. \cr
        /// Variants:
        /// \itemize{
        /// \item "blackman": Intermediate rolloff and intermediate attenuation.
        /// \item "blackman2": Slower rolloff but better attenuation than Blackman.
        /// \item "blackmanharris": Slow rolloff but good attenuation.
        /// \item "blackmanharris2": Slower rolloff but better attenuation than Blackman-Harris.
        /// \item "hann": Fast rolloff but not very high attenuation.
        /// \item "hann2": Slower rolloff and higher attenuation than simple Hann.
        /// }
        /// `chunk_size_in` - Size of input data in frames.
        /// # Examples
        ///
        /// ```
        /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
        /// //let offset = None;
        /// //let duration = None;
        /// //decoded_audio
        /// //    .resample_sincfixedin(
        /// //        22000,
        /// //        2.,
        /// //        256,
        /// //        0.95,
        /// //        128,
        /// //        InterpolationType::Linear,
        /// //        WindowFunction::Blackman2,
        /// //        1024,
        /// //    )
        /// //    .unwrap();
        /// ```
        pub fn resample_sincfixedin(
            &mut self,
            sr_out: usize,
            max_resample_ratio_relative: f64,
            sinc_len: usize,
            f_cutoff: f32,
            oversampling_factor: usize,
            interpolation: InterpolationType,
            window: WindowFunction,
            chunk_size_in: usize,
        ) -> OwnResult<()>
        where
            T: Sample,
            S: DataOwned<Elem = T>,
        {
            let nchannels = self.nchannels() as usize;
            let nsamples = self.nsamples() as usize;
            let f_ratio = sr_out as f64 / self.sr() as f64;
            let params = InterpolationParameters {
                sinc_len,
                f_cutoff,
                interpolation,
                oversampling_factor,
                window,
            };
            let mut resampler = SincFixedIn::<T>::new(
                f_ratio,
                max_resample_ratio_relative,
                params,
                chunk_size_in,
                self.nchannels() as usize,
            )?;

            resampler.process_resampler(
                self,
                nchannels,
                nsamples,
                sr_out,
                ResamplerType::NotFixedIn,
            )?;

            Ok(())
        }

        /// Resample the audio data from sr_in to sr_out.
        /// sincfixedout: An asynchronous resampler that return a fixed number of audio frames. The number of input frames required is given by the input_frames_next function. The resampling is done by creating a number of intermediate points (defined by oversampling_factor) by sinc interpolation. The new samples are then calculated by interpolating between these points.
        /// Asynchronous resampling: the resampling is based on band-limited interpolation using sinc interpolation filters. The sinc interpolation upsamples by an adjustable factor, and then the new sample points are calculated by interpolating between these points. The resampling ratio can be updated at any time.
        /// # Arguments
        /// `sr_out` - Target sampling rate.
        /// `max_resample_ratio_relative` - Maximum ratio that can be set with Resampler::set_resample_ratio relative to resample_ratio, must be >= 1.0. The minimum relative ratio is the reciprocal of the maximum. For example, with max_resample_ratio_relative of 10.0, the ratio can be set between resample_ratio * 10.0 and resample_ratio / 10.0.
        /// `sinc_len` - Length of the windowed sinc interpolation filter. Higher values can allow a higher cut-off frequency leading to less high frequency roll-off at the expense of higher cpu usage. 256 is a good starting point. The value will be rounded up to the nearest multiple of 8.
        /// `f_cutoff` - Relative cutoff frequency of the sinc interpolation filter (relative to the lowest one of fs_in/2 or fs_out/2). Start at 0.95, and increase if needed.
        /// `oversampling_factor` - The number of intermediate points to use for interpolation. Higher values use more memory for storing the sinc filters. Only the points actually needed are calculated during processing so a larger number does not directly lead to higher cpu usage. But keeping it down helps in keeping the sincs in the cpu cache. Starts at 128.
        /// `interpolation` - Interpolation type. One of \["cubic", "linear", "nearest"\]. \cr
        /// For asynchronous interpolation where the ratio between input and output sample rates can be any number, it’s not possible to pre-calculate all the needed interpolation filters. Instead they have to be computed as needed, which becomes impractical since the sincs are very expensive to generate in terms of cpu time. It’s more efficient to combine the sinc filters with some other interpolation technique. Then sinc filters are used to provide a fixed number of interpolated points between input samples, and then the new value is calculated by interpolation between those points. \cr
        /// Variants:
        /// \itemize{
        /// \item "cubic": For cubic interpolation, the four nearest intermediate points are calculated using sinc interpolation. Then a cubic polynomial is fitted to these points, and is then used to calculate the new sample value. The computation time as about twice the one for linear interpolation, but it requires much fewer intermediate points for a good result.
        /// \item "linear": With linear interpolation the new sample value is calculated by linear interpolation between the two nearest points. This requires two intermediate points to be calculated using sinc interpolation, and te output is a weighted average of these two. This is relatively fast, but needs a large number of intermediate points to push the resampling artefacts below the noise floor.
        /// \item "nearest": The Nearest mode doesn’t do any interpolation, but simply picks the nearest intermediate point. This is useful when the nearest point is actually the correct one, for example when upsampling by a factor 2, like 48kHz->96kHz. Then setting the oversampling_factor to 2, and using Nearest mode, no unnecessary computations are performed and the result is the same as for synchronous resampling. This also works for other ratios that can be expressed by a fraction. For 44.1kHz -> 48 kHz, setting oversampling_factor to 160 gives the desired result (since 48kHz = 160/147 * 44.1kHz).
        /// }
        /// `window` - Window function to use. \cr
        /// Variants:
        /// \itemize{
        /// \item "blackman": Intermediate rolloff and intermediate attenuation.
        /// \item "blackman2": Slower rolloff but better attenuation than Blackman.
        /// \item "blackmanharris": Slow rolloff but good attenuation.
        /// \item "blackmanharris2": Slower rolloff but better attenuation than Blackman-Harris.
        /// \item "hann": Fast rolloff but not very high attenuation.
        /// \item "hann2": Slower rolloff and higher attenuation than simple Hann.
        /// }
        /// `chunk_size_out` - Size of output data in frames.
        /// # Examples
        ///
        /// ```
        /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
        /// //let offset = None;
        /// //let duration = None;
        /// //decoded_audio
        /// //    .resample_sincfixedout(
        /// //        22000,
        /// //        2.,
        /// //        256,
        /// //        0.95,
        /// //        128,
        /// //        InterpolationType::Linear,
        /// //        WindowFunction::Blackman2,
        /// //        1024,
        /// //    )
        /// //    .unwrap();
        /// ```
        pub fn resample_sincfixedout(
            &mut self,
            sr_out: usize,
            max_resample_ratio_relative: f64,
            sinc_len: usize,
            f_cutoff: f32,
            oversampling_factor: usize,
            interpolation: InterpolationType,
            window: WindowFunction,
            chunk_size_out: usize,
        ) -> OwnResult<()>
        where
            T: Sample,
            S: DataOwned<Elem = T>,
        {
            let nchannels = self.nchannels() as usize;
            let nsamples = self.nsamples() as usize;
            let f_ratio = sr_out as f64 / self.sr() as f64;
            let params = InterpolationParameters {
                sinc_len,
                f_cutoff,
                interpolation,
                oversampling_factor,
                window,
            };
            let mut resampler = SincFixedOut::<T>::new(
                f_ratio,
                max_resample_ratio_relative,
                params,
                chunk_size_out,
                self.nchannels() as usize,
            )?;

            resampler.process_resampler(
                self,
                nchannels,
                nsamples,
                sr_out,
                ResamplerType::NotFixedIn,
            )?;

            Ok(())
        }
    }

    impl<T> DecodedAudio<T, OwnedRepr<T>>
    where
        T: Float + FromPrimitive,
    {
        /// Convert to 1 channel taking the average across channels.
        /// # Examples
        ///
        /// ```
        /// //let decoded_audio = DecodedAudio::new(array2![[1.,2.,3.], [4.,5.,6.]], 44100);
        /// //decoded_audio.as_mono()
        /// ```
        pub fn as_mono(&mut self) {
            let data = self
                .data
                .mean_axis(Axis(0))
                .expect("length of the axis cannot be zero")
                .insert_axis(Axis(0));
            self.data = data;
        }
    }

    /// Extract metadata from a file to a Vec of Strings.
    /// Tags that are part of the container format are preferentially extracted. Additional tags that were found while probing will not be extracted.
    /// The following metadata tagging formats are supported.
    ///
    /// * ID3v1
    /// * ID3v2
    /// * ISO/MP4
    /// * RIFF
    /// * Vorbis Comment (in OGG & FLAC)
    ///
    /// # Examples
    ///
    /// ```
    /// //metadata_file(../testfiles/gs-16b-1c-44100hz.mp3).unwrap();
    /// ```
    pub fn metadata_from_file(fpath: &str) -> OwnResult<Vec<String>> {
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
        // Probe the media source stream for a format.
        let mut probed =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

        let mut metadata_string: Vec<String> = Vec::new();

        if let Some(metadata_rev) = probed.format.metadata().current() {
            let tags = metadata_rev.tags();
            for (idx, tag) in tags.iter().enumerate() {
                if tag.to_string().len() > 400 {
                    continue;
                }
                metadata_string.push(format!("{idx}: {:?}", tag));
            }
        } else if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current())
        {
            let tags = metadata_rev.tags();
            for (idx, tag) in tags.iter().enumerate() {
                if tag.to_string().len() > 400 {
                    continue;
                }
                metadata_string.push(format!("{idx}: {:?}", tag));
            }
        }

        Ok(metadata_string)
    }

    /// Get the audio duration from a file in seconds. \cr
    /// # Arguments
    /// `fname` - The path to the input file. Note that this avoids loading the contents into memory, and is therefore useful for querying the duration of long files.
    /// # Examples
    ///
    /// ```
    /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
    /// //duration_file(fname)
    /// ```
    pub fn duration_from_file(fpath: &str) -> OwnResult<f64> {
        let fpath = Path::new(fpath);
        let ext = Path::extension(fpath)
            .ok_or_else(|| OwnError::IoError("couldn't extract the file extension".into()))?
            .to_str()
            .ok_or_else(|| OwnError::IoError("cannot convert from &OsStr to &str".into()))?;
        let file = Box::new(File::open(fpath)?);
        let mss = MediaSourceStream::new(file, Default::default());
        let mut hint = Hint::new();
        hint.with_extension(ext);
        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let probed =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
        let reader = probed.format;
        let track = reader
            .default_track()
            .ok_or_else(|| OwnError::DecodeError("no tracks were detected".into()))?;
        let sr = track
            .codec_params
            .sample_rate
            .ok_or_else(|| OwnError::DecodeError("cannot retrieve the sample rate".into()))?;
        let n_frames = track
            .codec_params
            .n_frames
            .ok_or_else(|| OwnError::DecodeError("cannot retrieve the number of frames".into()))?;

        Ok(n_frames as f64 / (sr as f64)) // fix n_frames conversion
    }

    /// Get the audio sampling rate.
    /// # Arguments
    /// `fname` - The path to the input file.
    /// @examples
    ///
    /// ```
    /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
    /// //sr_file(fname)
    /// ```
    pub fn sr_from_file(fpath: &str) -> OwnResult<u32> {
        let fpath = Path::new(fpath);
        let ext = Path::extension(fpath)
            .ok_or_else(|| OwnError::IoError("couldn't extract the file extension".into()))?
            .to_str()
            .ok_or_else(|| OwnError::IoError("cannot convert from &OsStr to &str".into()))?;
        let file = Box::new(File::open(fpath)?);
        let mss = MediaSourceStream::new(file, Default::default());
        let mut hint = Hint::new();
        hint.with_extension(ext);
        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let probed =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
        let reader = probed.format;
        let track = reader
            .default_track()
            .ok_or_else(|| OwnError::DecodeError("no tracks were detected".into()))?;
        let sr = track
            .codec_params
            .sample_rate
            .ok_or_else(|| OwnError::DecodeError("cannot retrieve the sample rate".into()))?;

        Ok(sr)
    }

    enum ResamplerType {
        FixedIn,
        NotFixedIn,
    }

    trait ProcessResampler<T, S>
    where
        T: Float + Sample,
        S: DataOwned<Elem = T> + RawDataClone<Elem = T>,
    {
        fn process_resampler(
            &mut self,
            decoded_audio: &mut DecodedAudio<T, S>,
            nchannels: usize,
            nsamples: usize,
            sr_out: usize,
            resampler_type: ResamplerType,
        ) -> OwnResult<()>;
    }

    impl<T, S, R> ProcessResampler<T, S> for R
    where
        T: Float + Sample,
        S: DataOwned<Elem = T> + RawDataClone<Elem = T>,
        R: Resampler<T>,
    {
        fn process_resampler(
            &mut self,
            decoded_audio: &mut DecodedAudio<T, S>,
            nchannels: usize,
            nsamples: usize,
            sr_out: usize,
            resampler_type: ResamplerType,
        ) -> OwnResult<()> {
            let data = decoded_audio.data();
            let mut nbr_frames_next = self.input_frames_next();
            let mut v_out: Vec<Vec<T>> = Vec::with_capacity(nchannels); // try to replace vec<vec> by a flat vec or ndarray
            let max_possible_resamples_per_channel =
                self.output_frames_max() * (nsamples / self.output_frames_max() + 1);
            for _ in 0..nchannels {
                v_out.push(Vec::<T>::with_capacity(max_possible_resamples_per_channel));
                // try to make it a single vec with length ch*max_possible_resamples_per_channel
            }

            let mut idx: usize = 0;
            let mut input_buffer = self.input_buffer_allocate();
            let mut output_buffer = self.output_buffer_allocate();

            loop {
                (0..nchannels).for_each(|ch| {
                    let sli = data.slice(s![ch, idx..(idx + nbr_frames_next)]);
                    input_buffer[ch].extend_from_slice(
                        sli.as_slice().expect("cannot convert ArrayBase to slice"),
                    );
                });
                // the input and output buffers are noninterleaved
                self.process_into_buffer(&input_buffer, &mut output_buffer, None)?;

                for ch in 0..nchannels {
                    v_out[ch].append(&mut output_buffer[ch]);
                    input_buffer[ch].clear();
                }

                idx += nbr_frames_next;

                match resampler_type {
                    ResamplerType::FixedIn => (),
                    ResamplerType::NotFixedIn => nbr_frames_next = self.input_frames_next(),
                }

                if idx + nbr_frames_next > nsamples {
                    break;
                }
            }

            let samples_per_channel_out: usize = v_out[0].len();
            let data = ArrayBase::<S, Ix2>::from_shape_vec(
                (nchannels, samples_per_channel_out),
                v_out.into_iter().flatten().collect(),
            )
            .expect("erro");

            decoded_audio.data = data;
            decoded_audio.sr = sr_out as u32;

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ndarray::{arr2, s, OwnedRepr};

        #[test]
        fn new_test() {
            let decoded_audio = DecodedAudio::new(arr2(&[[1., 2., 3.], [4., 5., 6.]]), 44000);

            assert_eq!(decoded_audio.sr(), 44000);
            assert_eq!(decoded_audio.data(), arr2(&[[1., 2., 3.], [4., 5., 6.]]));
            assert_eq!(decoded_audio.nsamples(), 3);
            assert_eq!(decoded_audio.nchannels(), 2);
        }

        #[test]
        fn load_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = None;
            let duration = None;
            let decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();

            assert_eq!(decoded_audio.sr(), 44100);
            assert_eq!(
                decoded_audio
                    .data()
                    .slice(s![0, 0..5])
                    .to_owned()
                    .as_slice()
                    .unwrap(),
                &[
                    0.0,
                    3.0517578125e-5,
                    -6.103515625e-5,
                    6.103515625e-5,
                    -6.103515625e-5
                ]
            );
            assert_eq!(decoded_audio.data().dim(), (2, 698194));

            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = Some(1.);
            let duration = None;
            let decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();

            assert_eq!(decoded_audio.sr(), 44100);
            assert_eq!(
                decoded_audio
                    .data()
                    .slice(s![0, 0..5])
                    .to_owned()
                    .as_slice()
                    .unwrap(),
                &[
                    -0.030242919921875,
                    -0.0218505859375,
                    -0.021697998046875,
                    -0.01953125,
                    -0.04119873046875,
                ]
            );
            assert_eq!(decoded_audio.data().dim(), (2, 654094));

            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = Some(1.);
            let duration = Some(0.5);
            let decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();
            let data = decoded_audio.data();
            let nsamples = data.ncols();
            assert_eq!(decoded_audio.sr(), 44100);
            assert_eq!(
                data.slice(s![0, nsamples - 5..nsamples])
                    .to_owned()
                    .as_slice()
                    .unwrap(),
                &[
                    0.09576416015625,
                    0.100921630859375,
                    0.10406494140625,
                    0.110748291015625,
                    0.10919189453125,
                ]
            );
            assert_eq!(decoded_audio.data().dim(), (2, 22050));
        }

        #[test]
        fn stream_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let duration = None;
            let offset = None;
            let decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();
            let mut stream_struct =
                DecodedAudio::<f64, OwnedRepr<f64>>::stream(fname, offset, duration).unwrap();

            let s1 = stream_struct.next().unwrap();
            assert_eq!(s1.dim(), (2, 1152));
            assert_eq!(
                s1.slice(s![0, ..]).to_owned().as_slice().unwrap(),
                decoded_audio
                    .data()
                    .slice(s![0, 0..s1.dim().1])
                    .to_owned()
                    .as_slice()
                    .unwrap(),
            );

            // test one more iteration
            let s2 = stream_struct.next().unwrap();
            assert_eq!(s2.dim(), (2, 1152));
            assert_eq!(
                s2.slice(s![0, ..]).to_owned().as_slice().unwrap(),
                decoded_audio
                    .data()
                    .slice(s![0, s2.dim().1..2 * s2.dim().1])
                    .to_owned()
                    .as_slice()
                    .unwrap(),
            );

            // test until the end
        }

        #[test]
        fn verify_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            DecodedAudio::<f64, OwnedRepr<f64>>::verify(fname).unwrap();
        }
        #[test]
        fn metadata_file_test() {
            let fname = "../testfiles/gs-16b-1c-44100hz.mp3";
            metadata_file(fname).unwrap();
        }

        #[test]
        fn as_mono_test() {
            let mut decoded_audio = DecodedAudio::new(arr2(&[[1., 2., 3.], [4., 5., 6.]]), 44100);
            decoded_audio.as_mono();

            assert_eq!(decoded_audio.data(), arr2(&[[2.5, 3.5, 4.5]]));
            assert_eq!(decoded_audio.data().ndim(), 2);
            assert_eq!(decoded_audio.data().dim(), (1, 3));
            assert_eq!(decoded_audio.nchannels(), 1);
        }

        #[test]
        fn duration_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = None;
            let duration = None;
            let decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();

            assert_eq!(decoded_audio.duration(), duration_file(fname).unwrap());
        }

        #[test]
        fn sr_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = None;
            let duration = None;
            let decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();

            assert_eq!(decoded_audio.sr(), sr_file(fname).unwrap());
        }

        #[test]
        fn resample_fftfixedin_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = None;
            let duration = None;
            let mut decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();

            decoded_audio.resample_fftfixedin(22000, 1024, 2).unwrap();
            assert_eq!(decoded_audio.sr(), 22000);
            assert_eq!(decoded_audio.nchannels(), 2);
            assert_eq!(decoded_audio.nsamples(), 347600);
        }

        #[test]
        fn resample_fftfixedinout_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = None;
            let duration = None;
            let mut decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();

            decoded_audio.resample_fftfixedinout(22000, 1024).unwrap();
            assert_eq!(decoded_audio.sr(), 22000);
            assert_eq!(decoded_audio.nchannels(), 2);
            assert_eq!(decoded_audio.nsamples(), 347600);
        }

        #[test]
        fn resample_fftfixedout_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = None;
            let duration = None;
            let mut decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();

            decoded_audio.resample_fftfixedout(22000, 1024, 2).unwrap();
            assert_eq!(decoded_audio.sr(), 22000);
            assert_eq!(decoded_audio.nchannels(), 2);
            assert_eq!(decoded_audio.nsamples(), 347136);
        }

        #[test]
        fn resample_sincfixedin_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = None;
            let duration = None;
            let mut decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();

            decoded_audio
                .resample_sincfixedin(
                    22000,
                    2.,
                    256,
                    0.95,
                    128,
                    InterpolationType::Linear,
                    WindowFunction::Blackman2,
                    1024,
                )
                .unwrap();
            assert_eq!(decoded_audio.sr(), 22000);
            assert_eq!(decoded_audio.nchannels(), 2);
            assert_eq!(decoded_audio.nsamples(), 347816);
        }

        #[test]
        fn resample_sincfixedout_test() {
            let fname = "../testfiles/gs-16b-2c-44100hz.wav";
            let offset = None;
            let duration = None;
            let mut decoded_audio = DecodedAudio::<f64, OwnedRepr<f64>>::load(
                fname,
                offset,
                duration,
                VerifyDecode::NotVerify,
            )
            .unwrap();

            decoded_audio
                .resample_sincfixedout(
                    22000,
                    2.,
                    256,
                    0.95,
                    128,
                    InterpolationType::Linear,
                    WindowFunction::Blackman2,
                    1024,
                )
                .unwrap();
            assert_eq!(decoded_audio.sr(), 22000);
            assert_eq!(decoded_audio.nchannels(), 2);
            assert_eq!(decoded_audio.nsamples(), 348160);
        }
    }
}
