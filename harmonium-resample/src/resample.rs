use arrow2::types::NativeType;
use harmonium_core::{
    errors::{HError, HResult},
    structs::{HFloatAudio, HFloatMatrix},
};
use num_traits::Float;
use rubato::{
    FastFixedIn, FastFixedOut, FftFixedIn, FftFixedInOut, FftFixedOut, Resampler, Sample,
    SincFixedIn, SincFixedOut,
};

pub trait ProcessResampler<T>
where
    T: Float + NativeType + Sample,
{
    fn process_resampler(&mut self, haudio: &mut HFloatAudio<T>, sr_out: usize) -> HResult<()>;
}

macro_rules! impl_process_resampler_fixed_in {
    ($($t:ty),+) => {
        $(
            impl<T> ProcessResampler<T> for $t
            where
                T: Float + NativeType + Sample,
                {
                    fn process_resampler(&mut self, haudio: &mut HFloatAudio<T>, sr_out: usize) -> HResult<()> {
                        let nrows = haudio.inner.nrows();
                        let input_frames_next = self.input_frames_next();
                        let nchannels = self.nbr_channels();
                        let max_possible_frames_per_channel =
                            self.output_frames_max() * (nrows / self.output_frames_max() + 1);

                        // The `filled` argument determines if the vectors should be pre-filled with zeros or not.
                        // When false, the vectors are only allocated but returned empty.
                        let mut input_buffer = self.input_buffer_allocate(false);
                        let mut output_buffer = self.output_buffer_allocate(true);

                        let mut v_out: Vec<Vec<T>> = Vec::with_capacity(nchannels);

                        for _ in 0..nchannels {
                            v_out.push(Vec::<T>::with_capacity(max_possible_frames_per_channel));
                        }

                        let steps = nrows / input_frames_next;

                        let haudio_slice = haudio.inner.as_slice();

                        for n in 0..steps {
                            let col_chunks = haudio_slice.chunks_exact(nrows);
                            for (ib, cc) in input_buffer.iter_mut().zip(col_chunks) {
                                ib.extend_from_slice(&cc[input_frames_next*n..input_frames_next*(n+1)]);
                            }

                            let (_, nbr_out) = self.process_into_buffer(&input_buffer, &mut output_buffer, None)?;

                            for ((vo, ob), ib) in v_out.iter_mut().zip(output_buffer.iter()).zip(input_buffer.iter_mut()) {
                                vo.extend_from_slice(&ob[0..nbr_out]);
                                ib.clear();
                            }
                        }

                        let v = v_out.into_iter().flatten().collect();
                        let hmatrix = HFloatMatrix::<T>::new_from_vec(v, nchannels)?;

                        haudio.inner = hmatrix;
                        haudio.sr = sr_out
                            .try_into()
                            .map_err(|_| HError::OutOfSpecError("sr_out overflow".into()))?;

                        Ok(())
                    }
                }
        )+
    };
}

impl_process_resampler_fixed_in!(
    FftFixedIn<T>,
    FftFixedInOut<T>,
    SincFixedIn<T>,
    FastFixedIn<T>
);

macro_rules! impl_process_resampler_fixed_out {
    ($($t:ty),+) => {
        $(
            impl<T> ProcessResampler<T> for $t
            where
                T: Float + NativeType + Sample,
                {
                    fn process_resampler(&mut self, haudio: &mut HFloatAudio<T>, sr_out: usize) -> HResult<()> {
                        let nrows = haudio.inner.nrows();
                        let mut start_idx = 0;
                        let nchannels = self.nbr_channels();
                        let max_possible_frames_per_channel =
                            self.output_frames_max() * (nrows / self.output_frames_max() + 1);

                        // The `filled` argument determines if the vectors should be pre-filled with zeros or not.
                        // When false, the vectors are only allocated but returned empty.
                        let mut input_buffer = self.input_buffer_allocate(false);
                        let mut output_buffer = self.output_buffer_allocate(true);

                        let mut v_out: Vec<Vec<T>> = Vec::with_capacity(nchannels);

                        for _ in 0..nchannels {
                            v_out.push(Vec::<T>::with_capacity(max_possible_frames_per_channel));
                        }

                        let haudio_slice = haudio.inner.as_slice();

                        while nrows >= self.input_frames_next() + start_idx {
                            let col_chunks = haudio_slice.chunks_exact(nrows);
                            let end_idx = start_idx + self.input_frames_next();
                            for (ib, cc) in input_buffer.iter_mut().zip(col_chunks) {
                                ib.extend_from_slice(&cc[start_idx..end_idx]);
                            }

                            start_idx += end_idx;

                            let (_, _) = self.process_into_buffer(&input_buffer, &mut output_buffer, None)?;

                            for ((vo, ob), ib) in v_out.iter_mut().zip(output_buffer.iter()).zip(input_buffer.iter_mut()) {
                                vo.extend_from_slice(ob);
                                ib.clear();
                            }
                        }

                        let v = v_out.into_iter().flatten().collect();
                        let hmatrix = HFloatMatrix::<T>::new_from_vec(v, nchannels)?;

                        haudio.inner = hmatrix;
                        haudio.sr = sr_out
                            .try_into()
                            .map_err(|_| HError::OutOfSpecError("sr_out overflow".into()))?;

                        Ok(())
                    }
                }
        )+
    };
}

impl_process_resampler_fixed_out!(FftFixedOut<T>, SincFixedOut<T>, FastFixedOut<T>);

#[cfg(test)]
mod tests {
    use super::*;
    use rubato::{
        SincFixedIn, SincFixedOut, SincInterpolationParameters, SincInterpolationType,
        WindowFunction,
    };

    #[test]
    fn test_process_resampler_sinc_fixed_in() {
        // SincFixedIn.
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 1024;
        let chunk_size = 512;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut haudio = HFloatMatrix::new_from_vec(v, 2).unwrap().into_haudio(sr_in);
        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler =
            SincFixedIn::<f64>::new(sr_out as f64 / sr_in as f64, 2.0, params, chunk_size, 2)
                .unwrap();

        resampler.process_resampler(&mut haudio, sr_out).unwrap();

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler =
            SincFixedIn::<f64>::new(sr_out as f64 / sr_in as f64, 2.0, params, chunk_size, 2)
                .unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            haudio.inner.as_slice(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
        assert_eq!(haudio.sr(), sr_out as u32);
    }

    #[test]
    fn test_process_resampler_fast_fixed_in() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut haudio = HFloatMatrix::new_from_vec(v, 2).unwrap().into_haudio(sr_in);
        let mut resampler = FastFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            2,
        )
        .unwrap();

        resampler.process_resampler(&mut haudio, sr_out).unwrap();

        let mut resampler = FastFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            2,
        )
        .unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            haudio.inner.as_slice(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
        assert_eq!(haudio.sr(), sr_out as u32);
    }

    #[test]
    fn test_process_resampler_fft_fixed_in() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut haudio = HFloatMatrix::new_from_vec(v, 2).unwrap().into_haudio(sr_in);
        let mut resampler =
            FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 2).unwrap();

        resampler.process_resampler(&mut haudio, sr_out).unwrap();

        let mut resampler =
            FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 2).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            haudio.inner.as_slice(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
        assert_eq!(haudio.sr(), sr_out as u32);
    }

    #[test]
    fn test_process_resampler_fft_fixed_in_out() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 512;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut haudio = HFloatMatrix::new_from_vec(v, 2).unwrap().into_haudio(sr_in);
        let mut resampler =
            FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 2).unwrap();

        resampler.process_resampler(&mut haudio, sr_out).unwrap();

        let mut resampler =
            FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 2).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            haudio.inner.as_slice(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
        assert_eq!(haudio.sr(), sr_out as u32);
    }

    #[test]
    fn test_process_resampler_sinc_fixed_out() {
        let sr_in = 44100;
        let sr_out = 48000;
        let chunk_size = 512;
        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler =
            SincFixedOut::<f64>::new(sr_out as f64 / sr_in as f64, 2.0, params, chunk_size, 2)
                .unwrap();

        let length = resampler.input_frames_next() * 2;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut haudio = HFloatMatrix::new_from_vec(v, 2).unwrap().into_haudio(sr_in);

        resampler.process_resampler(&mut haudio, sr_out).unwrap();

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler =
            SincFixedOut::<f64>::new(sr_out as f64 / sr_in as f64, 2.0, params, chunk_size, 2)
                .unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            haudio.inner.as_slice(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
        assert_eq!(haudio.sr(), sr_out as u32);
    }

    #[test]
    fn test_process_resampler_fast_fixed_out() {
        let sr_in = 44100;
        let sr_out = 48000;
        let chunk_size = 512;
        let mut resampler = FastFixedOut::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            2,
        )
        .unwrap();

        let length = resampler.input_frames_next() * 2;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut haudio = HFloatMatrix::new_from_vec(v, 2).unwrap().into_haudio(sr_in);

        resampler.process_resampler(&mut haudio, sr_out).unwrap();

        let mut resampler = FastFixedOut::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            2,
        )
        .unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            haudio.inner.as_slice(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
        assert_eq!(haudio.sr(), sr_out as u32);
    }

    #[test]
    fn test_process_resampler_fft_fixed_out() {
        let sr_in = 44100;
        let sr_out = 48000;
        let chunk_size = 512;
        let mut resampler = FftFixedOut::<f64>::new(sr_in, sr_out, chunk_size, 2, 2).unwrap();

        let length = resampler.input_frames_next() * 2;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut haudio = HFloatMatrix::new_from_vec(v, 2)
            .unwrap()
            .into_haudio(sr_in as u32);

        resampler.process_resampler(&mut haudio, sr_out).unwrap();

        let mut resampler = FftFixedOut::<f64>::new(sr_in, sr_out, chunk_size, 2, 2).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            haudio.inner.as_slice(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
        assert_eq!(haudio.sr(), sr_out as u32);
    }
}

//pub trait Resampler {
//    fn resample_fftfixedin(
//        &mut self,
//        sr_out: usize,
//        chunk_size_in: usize,
//        sub_chunks: usize,
//    ) -> HResult<()>;
//    fn resample_fftfixedinout(&mut self, sr_out: usize, chunk_size_in: usize) -> HResult<()>;
//    fn resample_fftfixedout(
//        &mut self,
//        sr_out: usize,
//        chunk_size_out: usize,
//        sub_chunks: usize,
//    ) -> HResult<()>;
//    fn resample_sincfixedin(
//        &mut self,
//        sr_out: usize,
//        max_resample_ratio_relative: f64,
//        interpolation_params: InterpolationParameters,
//        chunk_size_in: usize,
//    ) -> HResult<()>;
//    fn resample_sincfixedout(
//        &mut self,
//        sr_out: usize,
//        max_resample_ratio_relative: f64,
//        interpolation_params: InterpolationParameters,
//        chunk_size_out: usize,
//    ) -> HResult<()>;
//}
//
//impl<T> Resampler for HFloatAudio<T>
//where
//    T: Float + NativeType + Sample,
//{
//    /// Resample the audio data from sr_in to sr_out.
//    /// fftfixedin: A synchronous resampler that needs a fixed number of audio frames for input and returns a variable number of frames. The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter before it’s inverse transformed to get the resampled waveforms. \cr
//    /// # Arguments
//    /// `sr_out` - Target sampling rate.
//    /// `chunk_size_in` - Size of input data in frames.
//    /// `sub_chunks` - Desired number of subchunks for processing, actual number may be different.
//    /// # Examples
//    ///
//    /// ```
//    /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
//    /// //let offset = None;
//    /// //let duration = None;
//    /// //let mut decoded_audio =
//    /// //    DecodedAudio::<f64, OwnedRepr<f64>>::load(fname, offset, duration, VerifyDecode::Verify).unwrap();
//    /// //decoded_audio.resample_fftfixedin(22000, 1024, 2).unwrap();
//    /// ```
//    fn resample_fftfixedin(
//        &mut self,
//        sr_out: usize,
//        chunk_size_in: usize,
//        sub_chunks: usize,
//    ) -> HResult<()> {
//        let mut resampler = FftFixedIn::<T>::new(
//            self.sr() as usize,
//            sr_out,
//            chunk_size_in,
//            sub_chunks,
//            self.nchannels(),
//        )?;
//
//        resampler.process_resampler(self, sr_out, FixedType::FixedIn)?;
//
//        Ok(())
//    }
//
//    /// Resample the audio data from sr_in to sr_out.
//    /// fftfixedinout: A synchronous resampler that accepts a fixed number of audio frames for input and returns a fixed number of frames. The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter before it’s inverse transformed to get the resampled waveforms.
//    /// Synchronous resampling: is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data. This type of resampler is considerably faster but doesn’t support changing the resampling ratio.
//    /// # Arguments
//    /// `sr_out` - Target sampling rate.
//    /// `chunk_size_in` - Size of input data in frames.
//    /// # Examples
//    ///
//    /// ```
//    /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
//    /// //let offset = None;
//    /// //let duration = None;
//    /// //let mut decoded_audio =
//    /// //    DecodedAudio::<f64, OwnedRepr<f64>>::load(fname, offset, duration, VerifyDecode::Verify).unwrap();
//    /// //decoded_audio.resample_fftfixedinout(22000, 1024).unwrap();
//    /// ```
//    fn resample_fftfixedinout(&mut self, sr_out: usize, chunk_size_in: usize) -> HResult<()>
//    where
//        T: Float + NativeType + Sample,
//    {
//        let mut resampler =
//            FftFixedInOut::<T>::new(self.sr() as usize, sr_out, chunk_size_in, self.nchannels())?;
//
//        resampler.process_resampler(self, sr_out, FixedType::FixedIn)?;
//
//        Ok(())
//    }
//
//    /// Resample the audio data from sr_in to sr_out.
//    /// fftfixedout: A synchronous resampler that needs a varying number of audio frames for input and returns a fixed number of frames. The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter before it’s inverse transformed to get the resampled waveforms.
//    /// Synchronous resampling: is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data. This type of resampler is considerably faster but doesn’t support changing the resampling ratio.
//    /// # Arguments
//    /// `sr_out` - Target sampling rate.
//    /// `chunk_size_out` - Size of output data in frames.
//    /// `sub_chunks` - Desired number of subchunks for processing, actual number may be different.
//    /// # Examples
//    ///
//    /// ```
//    /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
//    /// //let offset = None;
//    /// //let duration = None;
//    /// //let mut decoded_audio =
//    /// //    DecodedAudio::<f64, OwnedRepr<f64>>::load(fname, offset, duration, VerifyDecode::Verify).unwrap();
//    /// //decoded_audio.resample_fftfixedout(22000, 1024, 2).unwrap();
//    /// ```
//    fn resample_fftfixedout(
//        &mut self,
//        sr_out: usize,
//        chunk_size_out: usize,
//        sub_chunks: usize,
//    ) -> HResult<()>
//    where
//        T: Float + NativeType + Sample,
//    {
//        let mut resampler = FftFixedOut::<T>::new(
//            self.sr() as usize,
//            sr_out,
//            chunk_size_out,
//            sub_chunks,
//            self.nchannels(),
//        )?;
//
//        resampler.process_resampler(self, sr_out, FixedType::NotFixedIn)?;
//
//        Ok(())
//    }
//
//    /// Resample the audio data from sr_in to sr_out.
//    /// sincfixedin: An asynchronous resampler that accepts a fixed number of audio frames for input and returns a variable number of frames. The resampling is done by creating a number of intermediate points (defined by oversampling_factor) by sinc interpolation. The new samples are then calculated by interpolating between these points.
//    /// Asynchronous resampling: the resampling is based on band-limited interpolation using sinc interpolation filters. The sinc interpolation upsamples by an adjustable factor, and then the new sample points are calculated by interpolating between these points. The resampling ratio can be updated at any time.
//    /// # Arguments
//    /// `sr_out` - Target sampling rate.
//    /// `max_resample_ratio_relative` - Maximum ratio that can be set with Resampler::set_resample_ratio relative to resample_ratio, must be >= 1.0. The minimum relative ratio is the reciprocal of the maximum. For example, with max_resample_ratio_relative of 10.0, the ratio can be set between resample_ratio * 10.0 and resample_ratio / 10.0.
//    /// `interpolation_params` - An instance of `InterpolationParameters`. Contains the following
//    /// variables:
//    /// \itemize{
//    /// `sinc_len` - Length of the windowed sinc interpolation filter. Higher values can allow a higher cut-off frequency leading to less high frequency roll-off at the expense of higher cpu usage. 256 is a good starting point. The value will be rounded up to the nearest multiple of 8.
//    /// `f_cutoff` - Relative cutoff frequency of the sinc interpolation filter (relative to the lowest one of fs_in/2 or fs_out/2). Start at 0.95, and increase if needed.
//    /// `oversampling_factor` - The number of intermediate points to use for interpolation. Higher values use more memory for storing the sinc filters. Only the points actually needed are calculated during processing so a larger number does not directly lead to higher cpu usage. But keeping it down helps in keeping the sincs in the cpu cache. Starts at 128.
//    /// `interpolation` - Interpolation type. One of \["cubic", "linear", "nearest"\]. \cr
//    /// For asynchronous interpolation where the ratio between input and output sample rates can be any number, it’s not possible to pre-calculate all the needed interpolation filters. Instead they have to be computed as needed, which becomes impractical since the sincs are very expensive to generate in terms of cpu time. It’s more efficient to combine the sinc filters with some other interpolation technique. Then sinc filters are used to provide a fixed number of interpolated points between input samples, and then the new value is calculated by interpolation between those points. \cr
//    /// Variants:
//    /// \itemize{
//    /// \item "cubic": For cubic interpolation, the four nearest intermediate points are calculated using sinc interpolation. Then a cubic polynomial is fitted to these points, and is then used to calculate the new sample value. The computation time as about twice the one for linear interpolation, but it requires much fewer intermediate points for a good result.
//    /// \item "linear": With linear interpolation the new sample value is calculated by linear interpolation between the two nearest points. This requires two intermediate points to be calculated using sinc interpolation, and te output is a weighted average of these two. This is relatively fast, but needs a large number of intermediate points to push the resampling artefacts below the noise floor.
//    /// \item "nearest": The Nearest mode doesn’t do any interpolation, but simply picks the nearest intermediate point. This is useful when the nearest point is actually the correct one, for example when upsampling by a factor 2, like 48kHz->96kHz. Then setting the oversampling_factor to 2, and using Nearest mode, no unnecessary computations are performed and the result is the same as for synchronous resampling. This also works for other ratios that can be expressed by a fraction. For 44.1kHz -> 48 kHz, setting oversampling_factor to 160 gives the desired result (since 48kHz = 160/147 * 44.1kHz).
//    /// }
//    /// `window` - Window function to use. \cr
//    /// Variants:
//    /// \itemize{
//    /// \item "blackman": Intermediate rolloff and intermediate attenuation.
//    /// \item "blackman2": Slower rolloff but better attenuation than Blackman.
//    /// \item "blackmanharris": Slow rolloff but good attenuation.
//    /// \item "blackmanharris2": Slower rolloff but better attenuation than Blackman-Harris.
//    /// \item "hann": Fast rolloff but not very high attenuation.
//    /// \item "hann2": Slower rolloff and higher attenuation than simple Hann.
//    /// }
//    /// }
//    /// `chunk_size_in` - Size of input data in frames.
//    /// # Examples
//    ///
//    /// ```
//    /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
//    /// //let offset = None;
//    /// //let duration = None;
//    /// //decoded_audio
//    /// //    .resample_sincfixedin(
//    /// //        22000,
//    /// //        2.,
//    /// //        256,
//    /// //        0.95,
//    /// //        128,
//    /// //        InterpolationType::Linear,
//    /// //        WindowFunction::Blackman2,
//    /// //        1024,
//    /// //    )
//    /// //    .unwrap();
//    /// ```
//    fn resample_sincfixedin(
//        &mut self,
//        sr_out: usize,
//        max_resample_ratio_relative: f64,
//        interpolation_params: InterpolationParameters,
//        chunk_size_in: usize,
//    ) -> HResult<()>
//    where
//        T: Float + NativeType + Sample,
//    {
//        let f_ratio = sr_out as f64 / self.sr() as f64;
//
//        let mut resampler = SincFixedIn::<T>::new(
//            f_ratio,
//            max_resample_ratio_relative,
//            interpolation_params,
//            chunk_size_in,
//            self.nchannels(),
//        )?;
//
//        resampler.process_resampler(self, sr_out, FixedType::NotFixedIn)?;
//
//        Ok(())
//    }
//
//    /// Resample the audio data from sr_in to sr_out.
//    /// sincfixedout: An asynchronous resampler that return a fixed number of audio frames. The number of input frames required is given by the input_frames_next function. The resampling is done by creating a number of intermediate points (defined by oversampling_factor) by sinc interpolation. The new samples are then calculated by interpolating between these points.
//    /// Asynchronous resampling: the resampling is based on band-limited interpolation using sinc interpolation filters. The sinc interpolation upsamples by an adjustable factor, and then the new sample points are calculated by interpolating between these points. The resampling ratio can be updated at any time.
//    /// # Arguments
//    /// `sr_out` - Target sampling rate.
//    /// `max_resample_ratio_relative` - Maximum ratio that can be set with Resampler::set_resample_ratio relative to resample_ratio, must be >= 1.0. The minimum relative ratio is the reciprocal of the maximum. For example, with max_resample_ratio_relative of 10.0, the ratio can be set between resample_ratio * 10.0 and resample_ratio / 10.0.
//    /// `interpolation_params` - An instance of `InterpolationParameters`. Contains the following
//    /// variables:
//    /// \itemize{
//    /// `sinc_len` - Length of the windowed sinc interpolation filter. Higher values can allow a higher cut-off frequency leading to less high frequency roll-off at the expense of higher cpu usage. 256 is a good starting point. The value will be rounded up to the nearest multiple of 8.
//    /// `f_cutoff` - Relative cutoff frequency of the sinc interpolation filter (relative to the lowest one of fs_in/2 or fs_out/2). Start at 0.95, and increase if needed.
//    /// `oversampling_factor` - The number of intermediate points to use for interpolation. Higher values use more memory for storing the sinc filters. Only the points actually needed are calculated during processing so a larger number does not directly lead to higher cpu usage. But keeping it down helps in keeping the sincs in the cpu cache. Starts at 128.
//    /// `interpolation` - Interpolation type. One of \["cubic", "linear", "nearest"\]. \cr
//    /// For asynchronous interpolation where the ratio between input and output sample rates can be any number, it’s not possible to pre-calculate all the needed interpolation filters. Instead they have to be computed as needed, which becomes impractical since the sincs are very expensive to generate in terms of cpu time. It’s more efficient to combine the sinc filters with some other interpolation technique. Then sinc filters are used to provide a fixed number of interpolated points between input samples, and then the new value is calculated by interpolation between those points. \cr
//    /// Variants:
//    /// \itemize{
//    /// \item "cubic": For cubic interpolation, the four nearest intermediate points are calculated using sinc interpolation. Then a cubic polynomial is fitted to these points, and is then used to calculate the new sample value. The computation time as about twice the one for linear interpolation, but it requires much fewer intermediate points for a good result.
//    /// \item "linear": With linear interpolation the new sample value is calculated by linear interpolation between the two nearest points. This requires two intermediate points to be calculated using sinc interpolation, and te output is a weighted average of these two. This is relatively fast, but needs a large number of intermediate points to push the resampling artefacts below the noise floor.
//    /// \item "nearest": The Nearest mode doesn’t do any interpolation, but simply picks the nearest intermediate point. This is useful when the nearest point is actually the correct one, for example when upsampling by a factor 2, like 48kHz->96kHz. Then setting the oversampling_factor to 2, and using Nearest mode, no unnecessary computations are performed and the result is the same as for synchronous resampling. This also works for other ratios that can be expressed by a fraction. For 44.1kHz -> 48 kHz, setting oversampling_factor to 160 gives the desired result (since 48kHz = 160/147 * 44.1kHz).
//    /// }
//    /// `window` - Window function to use. \cr
//    /// Variants:
//    /// \itemize{
//    /// \item "blackman": Intermediate rolloff and intermediate attenuation.
//    /// \item "blackman2": Slower rolloff but better attenuation than Blackman.
//    /// \item "blackmanharris": Slow rolloff but good attenuation.
//    /// \item "blackmanharris2": Slower rolloff but better attenuation than Blackman-Harris.
//    /// \item "hann": Fast rolloff but not very high attenuation.
//    /// \item "hann2": Slower rolloff and higher attenuation than simple Hann.
//    /// }
//    /// }
//    /// `chunk_size_out` - Size of output data in frames.
//    /// # Examples
//    ///
//    /// ```
//    /// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
//    /// //let offset = None;
//    /// //let duration = None;
//    /// //decoded_audio
//    /// //    .resample_sincfixedout(
//    /// //        22000,
//    /// //        2.,
//    /// //        256,
//    /// //        0.95,
//    /// //        128,
//    /// //        InterpolationType::Linear,
//    /// //        WindowFunction::Blackman2,
//    /// //        1024,
//    /// //    )
//    /// //    .unwrap();
//    /// ```
//    fn resample_sincfixedout(
//        &mut self,
//        sr_out: usize,
//        max_resample_ratio_relative: f64,
//        interpolation_params: InterpolationParameters,
//        chunk_size_out: usize,
//    ) -> HResult<()>
//    where
//        T: Float + NativeType + Sample,
//    {
//        let f_ratio = sr_out as f64 / self.sr() as f64;
//        let mut resampler = SincFixedOut::<T>::new(
//            f_ratio,
//            max_resample_ratio_relative,
//            interpolation_params,
//            chunk_size_out,
//            self.nchannels(),
//        )?;
//
//        resampler.process_resampler(self, sr_out, FixedType::NotFixedIn)?;
//
//        Ok(())
//    }
//}
