use harmonium_core::{
    array::HArray,
    audioop::AudioOp,
    errors::{HError, HResult},
};
use ndarray::{Dimension, Ix1, Ix2, IxDyn};
use num_traits::{ConstZero, Float, FloatConst};
use rubato::{
    FastFixedIn, FastFixedOut, FftFixedIn, FftFixedInOut, FftFixedOut, Resampler, Sample,
    SincFixedIn, SincFixedOut,
};

pub trait ProcessResampler<T, D>
where
    T: Float + FloatConst + Sample,
    D: Dimension,
{
    fn process_resampler(&mut self, harray: &mut HArray<T, D>) -> HResult<()>;
}

macro_rules! impl_process_resampler_fixed_in {
    ($($t:ty),+) => {
        $(
            impl<T> ProcessResampler<T, Ix1> for $t
            where
                T: Float + FloatConst + Sample + ConstZero,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, Ix1>) -> HResult<()> {
                        if self.nbr_channels() != 1 {
                            return Err(HError::OutOfSpecError("The resampler's nchannels must be 1.".into()))
                        }
                        let length = harray.len();
                        let input_frames_next = self.input_frames_next();
                        let max_possible_frames_per_channel = self.output_frames_max();
                        let mut v_out: Vec<T> = vec![T::ZERO; max_possible_frames_per_channel];

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        let mut idx_input = 0;
                        let mut idx_output = 0;

                        while idx_input + input_frames_next <= length {
                            // Safety: input_frames_next*(n+1) == nframes for n = steps - 1, since steps = nframes / input_frames_next.
                            let input_buffer = unsafe { harray_slice.get_unchecked(idx_input..idx_input + input_frames_next) };
                            let output_frames_next = self.output_frames_next();
                            let output_buffer = &mut v_out[idx_output..idx_output + output_frames_next];
                            let (_, nbr_out) = self.process_into_buffer(&[input_buffer], &mut [output_buffer], None)?;
                            idx_input += input_frames_next;
                            idx_output += nbr_out;
                        }

                        v_out.drain(idx_output..max_possible_frames_per_channel);
                        v_out.shrink_to(idx_output);
                        let length = v_out.len();

                        *harray = HArray::new_from_shape_vec(length, v_out)?;

                        Ok(())
                    }
                }

            impl<T> ProcessResampler<T, Ix2> for $t
            where
                T: Float + FloatConst + Sample + ConstZero,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, Ix2>) -> HResult<()> {
                        if self.nbr_channels() != 2 {
                            return Err(HError::OutOfSpecError("The resampler's nchannels must be 2.".into()))
                        }
                        let nframes = harray.nframes();
                        let nchannels = 2;
                        let input_frames_next = self.input_frames_next();
                        let max_possible_frames_per_channel = self.output_frames_max();
                        let mut v_out: Vec<T> = vec![T::ZERO; max_possible_frames_per_channel + max_possible_frames_per_channel];

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        let mut idx_input = 0;
                        let mut idx_output = 0;

                        // Safety: max_possible_frames_per_channel <= v_out.len().
                        let (output_buffer1, output_buffer2) = unsafe { v_out.split_at_mut_unchecked(max_possible_frames_per_channel) };
                        // Safety: nframes <= harray.len().
                        let (harray_slice1, harray_slice2) = unsafe { harray_slice.split_at_unchecked(nframes) };

                        while idx_input + input_frames_next <= nframes {
                            // Safety: input_frames_next*(n+1) == nframes for n = steps - 1, since steps = nframes / input_frames_next.
                            let input_buffer1 = unsafe { harray_slice1.get_unchecked(idx_input..idx_input + input_frames_next) };
                            let input_buffer2 = unsafe { harray_slice2.get_unchecked(idx_input..idx_input + input_frames_next) };
                            let output_frames_next = self.output_frames_next();
                            let output_buffer1 = &mut output_buffer1[idx_output..idx_output + output_frames_next];
                            let output_buffer2 = &mut output_buffer2[idx_output..idx_output + output_frames_next];
                            let (_, nbr_out) = self.process_into_buffer(&[input_buffer1, input_buffer2], &mut [output_buffer1, output_buffer2], None)?;
                            idx_input += input_frames_next;
                            idx_output += nbr_out;
                        }

                        v_out.drain(idx_output + max_possible_frames_per_channel..v_out.len());
                        v_out.drain(idx_output..max_possible_frames_per_channel);
                        v_out.shrink_to(idx_output + idx_output);
                        let nframes = idx_output;

                        *harray = HArray::new_from_shape_vec((nchannels, nframes), v_out)?;

                        Ok(())
                    }
                }

            impl<T> ProcessResampler<T, IxDyn> for $t
            where
                T: Float + FloatConst + Sample + ConstZero,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, IxDyn>) -> HResult<()> {
                        match harray.ndim() {
                            1 => {
                                if self.nbr_channels() != 1 {
                                    return Err(HError::OutOfSpecError("The resampler's nchannels must be 1.".into()))
                                }
                                let length = harray.len();
                                let input_frames_next = self.input_frames_next();
                                let max_possible_frames_per_channel = self.output_frames_max();
                                let mut v_out: Vec<T> = vec![T::ZERO; max_possible_frames_per_channel];

                                // Ok to unwrap.
                                let harray_slice = harray.as_slice().unwrap();

                                let mut idx_input = 0;
                                let mut idx_output = 0;

                                while idx_input + input_frames_next <= length {
                                    // Safety: input_frames_next*(n+1) == nframes for n = steps - 1, since steps = nframes / input_frames_next.
                                    let input_buffer = unsafe { harray_slice.get_unchecked(idx_input..idx_input + input_frames_next) };
                                    let output_frames_next = self.output_frames_next();
                                    let output_buffer = &mut v_out[idx_output..idx_output + output_frames_next];
                                    let (_, nbr_out) = self.process_into_buffer(&[input_buffer], &mut [output_buffer], None)?;
                                    idx_input += input_frames_next;
                                    idx_output += nbr_out;
                                }

                                v_out.drain(idx_output..max_possible_frames_per_channel);
                                v_out.shrink_to(idx_output);
                                let length = idx_output;

                                *harray = HArray::new_from_shape_vec(IxDyn(&[length]), v_out)?;

                                Ok(())
                            },

                            2 => {
                                if self.nbr_channels() != 2 {
                                    return Err(HError::OutOfSpecError("The resampler's nchannels must be 2.".into()))
                                }
                                let nframes = harray.nframes();
                                let nchannels = 2;
                                let input_frames_next = self.input_frames_next();
                                let max_possible_frames_per_channel = self.output_frames_max();
                                let mut v_out: Vec<T> = vec![T::ZERO; max_possible_frames_per_channel + max_possible_frames_per_channel];

                                // Ok to unwrap.
                                let harray_slice = harray.as_slice().unwrap();

                                let mut idx_input = 0;
                                let mut idx_output = 0;

                                // Safety: max_possible_frames_per_channel <= v_out.len().
                                let (output_buffer1, output_buffer2) = unsafe { v_out.split_at_mut_unchecked(max_possible_frames_per_channel) };
                                // Safety: nframes <= harray.len().
                                let (harray_slice1, harray_slice2) = unsafe { harray_slice.split_at_unchecked(nframes) };

                                while idx_input + input_frames_next <= nframes {
                                    // Safety: input_frames_next*(n+1) == nframes for n = steps - 1, since steps = nframes / input_frames_next.
                                    let input_buffer1 = unsafe { harray_slice1.get_unchecked(idx_input..idx_input + input_frames_next) };
                                    let input_buffer2 = unsafe { harray_slice2.get_unchecked(idx_input .. idx_input + input_frames_next) };
                                    let output_frames_next = self.output_frames_next();
                                    let output_buffer1 = &mut output_buffer1[idx_output..idx_output + output_frames_next];
                                    let output_buffer2 = &mut output_buffer2[idx_output..idx_output + output_frames_next];
                                    let (_, nbr_out) = self.process_into_buffer(&[input_buffer1, input_buffer2], &mut [output_buffer1, output_buffer2], None)?;
                                    idx_input += input_frames_next;
                                    idx_output += nbr_out;
                                }

                                v_out.drain(idx_output + max_possible_frames_per_channel..v_out.len());
                                v_out.drain(idx_output..max_possible_frames_per_channel);
                                v_out.shrink_to(idx_output + idx_output);
                                let nframes = idx_output;

                                *harray = HArray::new_from_shape_vec(IxDyn(&[nchannels, nframes]), v_out)?;

                                Ok(())
                            },
                            _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                        }
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
            impl<T> ProcessResampler<T, Ix1> for $t
            where
                T: Float + FloatConst + Sample + ConstZero,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, Ix1>) -> HResult<()> {
                        if self.nbr_channels() != 1 {
                            return Err(HError::OutOfSpecError("The resampler's nchannels must be 1.".into()))
                        }
                        let length = harray.len();
                        let mut idx_input = 0;
                        let mut idx_output = 0;
                        let max_possible_frames_per_channel = self.output_frames_max();
                        let mut input_frames_next = self.input_frames_next();
                        let output_frames_next = self.output_frames_next();

                        let mut v_out: Vec<T> = vec![T::ZERO; max_possible_frames_per_channel];

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        while length >= input_frames_next + idx_input {
                            // Safety: input_frames_next +  idx <= length as checked in the while loop.
                            let input_buffer = unsafe { harray_slice.get_unchecked(idx_input..idx_input + input_frames_next) };
                            let output_buffer = &mut v_out[idx_output..idx_output + output_frames_next];
                            idx_input += input_frames_next;
                            idx_output += output_frames_next;
                            let (_, _) = self.process_into_buffer(&[input_buffer], &mut [output_buffer], None)?;
                            input_frames_next = self.input_frames_next();
                        }

                        v_out.drain(idx_output..max_possible_frames_per_channel);
                        v_out.shrink_to(idx_output);
                        let length = idx_output;

                        *harray = HArray::new_from_shape_vec(length, v_out)?;

                        Ok(())
                    }
                }

            impl<T> ProcessResampler<T, Ix2> for $t
            where
                T: Float + FloatConst + Sample + ConstZero,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, Ix2>) -> HResult<()> {
                        if self.nbr_channels() != 2 {
                            return Err(HError::OutOfSpecError("The resampler's nchannels must be 2.".into()))
                        }
                        let nframes = harray.nframes();
                        let nchannels = 2;
                        let mut idx_input = 0;
                        let mut idx_output = 0;
                        let max_possible_frames_per_channel = self.output_frames_max();
                        let mut input_frames_next = self.input_frames_next();
                        let output_frames_next = self.output_frames_next();

                        let mut v_out: Vec<T> = vec![T::ZERO; max_possible_frames_per_channel + max_possible_frames_per_channel];

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        // Safety: max_possible_frames_per_channel <= v_out.len().
                        let (output_buffer1, output_buffer2) = unsafe { v_out.split_at_mut_unchecked(max_possible_frames_per_channel) };
                        // Safety: nframes <= harray.len().
                        let (harray_slice1, harray_slice2) = unsafe { harray_slice.split_at_unchecked(nframes) };

                        while nframes >= input_frames_next + idx_input {
                            // Safety: input_frames_next + idx_input <= nframes as checked in the while loop.
                            let input_buffer1 = unsafe { harray_slice1.get_unchecked(idx_input..idx_input + input_frames_next) };
                            let input_buffer2 = unsafe { harray_slice2.get_unchecked(idx_input..idx_input + input_frames_next) };
                            let output_buffer1 = &mut output_buffer1[idx_output..idx_output + output_frames_next];
                            let output_buffer2 = &mut output_buffer2[idx_output..idx_output + output_frames_next];
                            idx_input += input_frames_next;
                            idx_output += output_frames_next;
                            let (_, _) = self.process_into_buffer(&[input_buffer1, input_buffer2], &mut [output_buffer1, output_buffer2], None)?;
                            input_frames_next = self.input_frames_next();
                        }

                        v_out.drain(idx_output + max_possible_frames_per_channel..v_out.len());
                        v_out.drain(idx_output..max_possible_frames_per_channel);
                        v_out.shrink_to(idx_output + idx_output);
                        let nframes = idx_output;

                        *harray = HArray::new_from_shape_vec((nchannels, nframes), v_out)?;

                        Ok(())
                    }
                }

            impl<T> ProcessResampler<T, IxDyn> for $t
            where
                T: Float + FloatConst + Sample + ConstZero,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, IxDyn>) -> HResult<()> {
                        match harray.ndim() {
                            1 => {
                                if self.nbr_channels() != 1 {
                                    return Err(HError::OutOfSpecError("The resampler's nchannels must be 1.".into()))
                                }

                                let length = harray.len();
                                let mut idx_input = 0;
                                let mut idx_output = 0;
                                let max_possible_frames_per_channel = self.output_frames_max();
                                let mut input_frames_next = self.input_frames_next();
                                let output_frames_next = self.output_frames_next();

                                let mut v_out: Vec<T> = vec![T::ZERO; max_possible_frames_per_channel];

                                // Ok to unwrap.
                                let harray_slice = harray.as_slice().unwrap();

                                while length >= input_frames_next + idx_input {
                                    // Safety: input_frames_next +  idx <= length as checked in the while loop.
                                    let input_buffer = unsafe { harray_slice.get_unchecked(idx_input..idx_input + input_frames_next) };
                                    let output_buffer = &mut v_out[idx_output..idx_output + output_frames_next];
                                    idx_input += input_frames_next;
                                    idx_output += output_frames_next;
                                    let (_, _) = self.process_into_buffer(&[input_buffer], &mut [output_buffer], None)?;
                                    input_frames_next = self.input_frames_next();
                                }

                                v_out.drain(idx_output..max_possible_frames_per_channel);
                                v_out.shrink_to(idx_output);
                                let length = idx_output;

                                *harray = HArray::new_from_shape_vec(IxDyn(&[length]), v_out)?;

                                Ok(())
                            }
                            2 => {
                                if self.nbr_channels() != 2 {
                                    return Err(HError::OutOfSpecError("The resampler's nchannels must be 2.".into()))
                                }

                                if self.nbr_channels() != 2 {
                                    return Err(HError::OutOfSpecError("The resampler's nchannels must be 2.".into()))
                                }
                                let nframes = harray.nframes();
                                let nchannels = 2;
                                let mut idx_input = 0;
                                let mut idx_output = 0;
                                let max_possible_frames_per_channel = self.output_frames_max();
                                let mut input_frames_next = self.input_frames_next();
                                let output_frames_next = self.output_frames_next();

                                let mut v_out: Vec<T> = vec![T::ZERO; max_possible_frames_per_channel + max_possible_frames_per_channel];

                                // Ok to unwrap.
                                let harray_slice = harray.as_slice().unwrap();

                                // Safety: max_possible_frames_per_channel <= v_out.len().
                                let (output_buffer1, output_buffer2) = unsafe { v_out.split_at_mut_unchecked(max_possible_frames_per_channel) };
                                // Safety: nframes <= harray.len().
                                let (harray_slice1, harray_slice2) = unsafe { harray_slice.split_at_unchecked(nframes) };

                                while nframes >= input_frames_next + idx_input {
                                    // Safety: input_frames_next + idx_input <= nframes as checked in the while loop.
                                    let input_buffer1 = unsafe { harray_slice1.get_unchecked(idx_input..idx_input + input_frames_next) };
                                    let input_buffer2 = unsafe { harray_slice2.get_unchecked(idx_input..idx_input + input_frames_next) };
                                    let output_buffer1 = &mut output_buffer1[idx_output..idx_output + output_frames_next];
                                    let output_buffer2 = &mut output_buffer2[idx_output..idx_output + output_frames_next];
                                    idx_input += input_frames_next;
                                    idx_output += output_frames_next;
                                    let (_, _) = self.process_into_buffer(&[input_buffer1, input_buffer2], &mut [output_buffer1, output_buffer2], None)?;
                                    input_frames_next = self.input_frames_next();
                                }

                                v_out.drain(idx_output + max_possible_frames_per_channel..v_out.len());
                                v_out.drain(idx_output..max_possible_frames_per_channel);
                                v_out.shrink_to(idx_output + idx_output);
                                let nframes = idx_output;

                                *harray = HArray::new_from_shape_vec(IxDyn(&[nchannels, nframes]), v_out)?;

                                Ok(())
                            }
                            _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),

                        }
                    }
                }
            )+
    };
}

impl_process_resampler_fixed_out!(FftFixedOut<T>, SincFixedOut<T>, FastFixedOut<T>);

#[cfg(test)]
mod tests {
    use harmonium_core::conversions::IntoDynamic;
    use rubato::{SincInterpolationParameters, SincInterpolationType, WindowFunction};

    use super::*;

    // All tests below are for a chunk only. Changing chunks_size or length may yield failing results.
    #[test]
    fn process_resampler_sinc_fixed_in_test_1d() {
        // SincFixedIn.
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 1024;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec(length, v).unwrap();

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler =
            SincFixedIn::<f64>::new(sr_out as f64 / sr_in as f64, 2.0, params, chunk_size, 1)
                .unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler = rubato::SincFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            params,
            chunk_size,
            1,
        )
        .unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let waves_in = vec![v];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_sinc_fixed_in_test_1d_dyn() {
        // SincFixedIn.
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 1024;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec(length, v)
            .unwrap()
            .into_dynamic();

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler =
            SincFixedIn::<f64>::new(sr_out as f64 / sr_in as f64, 2.0, params, chunk_size, 1)
                .unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler = rubato::SincFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            params,
            chunk_size,
            1,
        )
        .unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let waves_in = vec![v];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_sinc_fixed_in_test_2d() {
        // SincFixedIn.
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 1024;
        let chunk_size = 512;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v).unwrap();
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

        resampler.process_resampler(&mut harray).unwrap();

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler = rubato::SincFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            params,
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
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_sinc_fixed_in_test_2d_dyn() {
        // SincFixedIn.
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 1024;
        let chunk_size = 512;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v)
            .unwrap()
            .into_dynamic();
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

        resampler.process_resampler(&mut harray).unwrap();

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler = rubato::SincFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            params,
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
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fast_fixed_in_test_1d() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 2048;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec(length, v).unwrap();
        let mut resampler = FastFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            1,
        )
        .unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler = rubato::FastFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            1,
        )
        .unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let waves_in = vec![v];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fast_fixed_in_test_1d_dyn() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 2048;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec(length, v)
            .unwrap()
            .into_dynamic();
        let mut resampler = FastFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            1,
        )
        .unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler = rubato::FastFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            1,
        )
        .unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let waves_in = vec![v];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fast_fixed_in_test_2d() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v).unwrap();
        let mut resampler = FastFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            2,
        )
        .unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler = rubato::FastFixedIn::<f64>::new(
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
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fast_fixed_in_test_2d_dyn() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v)
            .unwrap()
            .into_dynamic();
        let mut resampler = FastFixedIn::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            rubato::PolynomialDegree::Linear,
            chunk_size,
            2,
        )
        .unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler = rubato::FastFixedIn::<f64>::new(
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
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fft_fixed_in_test_1d() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 2048;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec(length, v).unwrap();
        let mut resampler =
            FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 1).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler =
            rubato::FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 1).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let waves_in = vec![v];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fft_fixed_in_test_1d_dyn() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 2048;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec(length, v)
            .unwrap()
            .into_dynamic();
        let mut resampler =
            FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 1).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler =
            rubato::FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 1).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let waves_in = vec![v];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fft_fixed_in_test_2d() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v).unwrap();
        let mut resampler =
            FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 2).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler =
            rubato::FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 2).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fft_fixed_in_test_2d_dyn() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v)
            .unwrap()
            .into_dynamic();
        let mut resampler =
            FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 2).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler =
            rubato::FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 2).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fft_fixed_in_out_test_1d() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec(length, v).unwrap();
        let mut resampler =
            FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 1).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler =
            rubato::FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 1).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let waves_in = vec![v];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fft_fixed_in_out_test_1d_dyn() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 1024;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec(length, v)
            .unwrap()
            .into_dynamic();
        let mut resampler =
            FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 1).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler =
            rubato::FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 1).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let waves_in = vec![v];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fft_fixed_in_out_test_2d() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 512;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v).unwrap();
        let mut resampler =
            FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 2).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler =
            rubato::FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 2).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fft_fixed_in_out_test_2d_dyn() {
        let sr_in = 44100;
        let sr_out = 48000;
        let length = 2048;
        let chunk_size = 512;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v)
            .unwrap()
            .into_dynamic();
        let mut resampler =
            FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 2).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler =
            rubato::FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 2).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_sinc_fixed_out_test_2d() {
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
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler = rubato::SincFixedOut::<f64>::new(
            sr_out as f64 / sr_in as f64,
            2.0,
            params,
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
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fast_fixed_out_test_2d() {
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
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler = rubato::FastFixedOut::<f64>::new(
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
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }

    #[test]
    fn process_resampler_fft_fixed_out_test_2d() {
        let sr_in = 44100;
        let sr_out = 48000;
        let chunk_size = 512;
        let mut resampler = FftFixedOut::<f64>::new(sr_in, sr_out, chunk_size, 2, 2).unwrap();

        let length = resampler.input_frames_next() * 2;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler =
            rubato::FftFixedOut::<f64>::new(sr_in, sr_out, chunk_size, 2, 2).unwrap();

        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let v1 = v[..(length / 2)].to_vec();
        let v2 = v[(length / 2)..].to_vec();
        let waves_in = vec![v1, v2];
        let waves_out = resampler.process(&waves_in, None).unwrap();

        assert_eq!(
            harray.as_slice().unwrap(),
            waves_out
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f64>>()
                .as_slice()
        );
    }
}
