use harmonium_core::{array::HArray, audioop::AudioOp, errors::HResult};
use ndarray::{Dimension, Ix1, Ix2, IxDyn};
use num_traits::{Float, FloatConst};
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
                T: Float + FloatConst + Sample,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, Ix1>) -> HResult<()> {
                        let length = harray.len();
                        let input_frames_next = self.input_frames_next();
                        let max_possible_frames_per_channel = self.output_frames_max();

                        // The `filled` argument determines if the vectors should be pre-filled with zeros or not.
                        // When false, the vectors are only allocated but returned empty.
                        let mut input_buffer = self.input_buffer_allocate(false);
                        let mut output_buffer = self.output_buffer_allocate(true);

                        let mut v_out: Vec<Vec<T>> = vec![Vec::<T>::with_capacity(max_possible_frames_per_channel)];

                        let steps = length / input_frames_next;

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        for n in 0..steps {
                            let col_chunks = harray_slice.chunks_exact(length);
                            for (ib, cc) in input_buffer.iter_mut().zip(col_chunks) {
                                ib.extend_from_slice(&cc[input_frames_next*n..input_frames_next*(n+1)]);
                            }

                            let (_, nbr_out) = self.process_into_buffer(&input_buffer, &mut output_buffer, None)?;

                            for ((vo, ob), ib) in v_out.iter_mut().zip(output_buffer.iter()).zip(input_buffer.iter_mut()) {
                                vo.extend_from_slice(&ob[0..nbr_out]);
                                ib.clear();
                            }
                        }

                        let v: Vec<T> = v_out.into_iter().flatten().collect();
                        let length = v.len();

                        *harray = HArray::new_from_shape_vec(length, v)?;

                        Ok(())
                    }
                }

            impl<T> ProcessResampler<T, Ix2> for $t
            where
                T: Float + FloatConst + Sample,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, Ix2>) -> HResult<()> {
                        let nframes = harray.nframes();
                        let nchannels = harray.nchannels();
                        let input_frames_next = self.input_frames_next();
                        let max_possible_frames_per_channel = self.output_frames_max();

                        // The `filled` argument determines if the vectors should be pre-filled with zeros or not.
                        // When false, the vectors are only allocated but returned empty.
                        let mut input_buffer = self.input_buffer_allocate(false);
                        let mut output_buffer = self.output_buffer_allocate(true);

                        let mut v_out: Vec<Vec<T>> = Vec::with_capacity(nchannels);

                        for _ in 0..nchannels {
                            v_out.push(Vec::<T>::with_capacity(max_possible_frames_per_channel));
                        }

                        let steps = nframes / input_frames_next;

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        for n in 0..steps {
                            let col_chunks = harray_slice.chunks_exact(nframes);
                            for (ib, cc) in input_buffer.iter_mut().zip(col_chunks) {
                                ib.extend_from_slice(&cc[input_frames_next*n..input_frames_next*(n+1)]);
                            }

                            let (_, nbr_out) = self.process_into_buffer(&input_buffer, &mut output_buffer, None)?;

                            for ((vo, ob), ib) in v_out.iter_mut().zip(output_buffer.iter()).zip(input_buffer.iter_mut()) {
                                vo.extend_from_slice(&ob[0..nbr_out]);
                                ib.clear();
                            }
                        }

                        let v: Vec<T> = v_out.into_iter().flatten().collect();
                        let nframes = v.len() / nchannels;

                        *harray = HArray::new_from_shape_vec((nchannels, nframes), v)?;

                        Ok(())
                    }
                }

            impl<T> ProcessResampler<T, IxDyn> for $t
            where
                T: Float + FloatConst + Sample,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, IxDyn>) -> HResult<()> {
                        let nframes = harray.nframes();
                        let nchannels = harray.nchannels();
                        let input_frames_next = self.input_frames_next();
                        let max_possible_frames_per_channel = self.output_frames_max();

                        // The `filled` argument determines if the vectors should be pre-filled with zeros or not.
                        // When false, the vectors are only allocated but returned empty.
                        let mut input_buffer = self.input_buffer_allocate(false);
                        let mut output_buffer = self.output_buffer_allocate(true);

                        let mut v_out: Vec<Vec<T>> = Vec::with_capacity(nchannels);

                        for _ in 0..nchannels {
                            v_out.push(Vec::<T>::with_capacity(max_possible_frames_per_channel));
                        }

                        let steps = nframes / input_frames_next;

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        for n in 0..steps {
                            let col_chunks = harray_slice.chunks_exact(nframes);
                            for (ib, cc) in input_buffer.iter_mut().zip(col_chunks) {
                                ib.extend_from_slice(&cc[input_frames_next*n..input_frames_next*(n+1)]);
                            }

                            let (_, nbr_out) = self.process_into_buffer(&input_buffer, &mut output_buffer, None)?;

                            for ((vo, ob), ib) in v_out.iter_mut().zip(output_buffer.iter()).zip(input_buffer.iter_mut()) {
                                vo.extend_from_slice(&ob[0..nbr_out]);
                                ib.clear();
                            }
                        }

                        let v: Vec<T> = v_out.into_iter().flatten().collect();
                        let nframes = v.len() / nchannels;

                        *harray = HArray::new_from_shape_vec(IxDyn(&[nchannels, nframes]), v)?;

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
            impl<T> ProcessResampler<T, Ix1> for $t
            where
                T: Float + FloatConst + Sample,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, Ix1>) -> HResult<()> {
                        let length = harray.len();
                        let mut start_idx = 0;
                        let max_possible_frames_per_channel = self.output_frames_max();

                        // The `filled` argument determines if the vectors should be pre-filled with zeros or not.
                        // When false, the vectors are only allocated but returned empty.
                        let mut input_buffer = self.input_buffer_allocate(false);
                        let mut output_buffer = self.output_buffer_allocate(true);

                        let mut v_out: Vec<Vec<T>> = vec![Vec::<T>::with_capacity(max_possible_frames_per_channel)];

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        let mut input_frames_next = self.input_frames_next();

                        while length >= input_frames_next + start_idx {
                            let col_chunks = harray_slice.chunks_exact(length);
                            let end_idx = start_idx + input_frames_next;
                            for (ib, cc) in input_buffer.iter_mut().zip(col_chunks) {
                                ib.extend_from_slice(&cc[start_idx..end_idx]);
                            }

                            start_idx += end_idx;

                            let (_, _) = self.process_into_buffer(&input_buffer, &mut output_buffer, None)?;

                            input_frames_next = self.input_frames_next();

                            for ((vo, ob), ib) in v_out.iter_mut().zip(output_buffer.iter()).zip(input_buffer.iter_mut()) {
                                vo.extend_from_slice(ob);
                                ib.clear();
                            }
                        }

                        let v: Vec<T> = v_out.into_iter().flatten().collect();
                        let length = v.len();

                        *harray = HArray::new_from_shape_vec(length, v)?;

                        Ok(())
                    }
                }

            impl<T> ProcessResampler<T, Ix2> for $t
            where
                T: Float + FloatConst + Sample,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, Ix2>) -> HResult<()> {
                        let nframes = harray.nframes();
                        let nchannels = harray.nchannels();
                        let mut start_idx = 0;
                        let max_possible_frames_per_channel = self.output_frames_max();

                        // The `filled` argument determines if the vectors should be pre-filled with zeros or not.
                        // When false, the vectors are only allocated but returned empty.
                        let mut input_buffer = self.input_buffer_allocate(false);
                        let mut output_buffer = self.output_buffer_allocate(true);

                        let mut v_out: Vec<Vec<T>> = Vec::with_capacity(nchannels);

                        for _ in 0..nchannels {
                            v_out.push(Vec::<T>::with_capacity(max_possible_frames_per_channel));
                        }

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        let mut input_frames_next = self.input_frames_next();

                        while nframes >= input_frames_next + start_idx {
                            let col_chunks = harray_slice.chunks_exact(nframes);
                            let end_idx = start_idx + input_frames_next;
                            for (ib, cc) in input_buffer.iter_mut().zip(col_chunks) {
                                ib.extend_from_slice(&cc[start_idx..end_idx]);
                            }

                            start_idx += end_idx;

                            let (_, _) = self.process_into_buffer(&input_buffer, &mut output_buffer, None)?;

                            input_frames_next = self.input_frames_next();

                            for ((vo, ob), ib) in v_out.iter_mut().zip(output_buffer.iter()).zip(input_buffer.iter_mut()) {
                                vo.extend_from_slice(ob);
                                ib.clear();
                            }
                        }

                        let v: Vec<T> = v_out.into_iter().flatten().collect();
                        let nframes = v.len() / nchannels;

                        *harray = HArray::new_from_shape_vec((nchannels, nframes), v)?;

                        Ok(())
                    }
                }

            impl<T> ProcessResampler<T, IxDyn> for $t
            where
                T: Float + FloatConst + Sample,
                {
                    fn process_resampler(&mut self, harray: &mut HArray<T, IxDyn>) -> HResult<()> {
                        let nframes = harray.nframes();
                        let nchannels = harray.nchannels();
                        let mut start_idx = 0;
                        let max_possible_frames_per_channel = self.output_frames_max();

                        // The `filled` argument determines if the vectors should be pre-filled with zeros or not.
                        // When false, the vectors are only allocated but returned empty.
                        let mut input_buffer = self.input_buffer_allocate(false);
                        let mut output_buffer = self.output_buffer_allocate(true);

                        let mut v_out: Vec<Vec<T>> = Vec::with_capacity(nchannels);

                        for _ in 0..nchannels {
                            v_out.push(Vec::<T>::with_capacity(max_possible_frames_per_channel));
                        }

                        // Ok to unwrap.
                        let harray_slice = harray.as_slice().unwrap();

                        let mut input_frames_next = self.input_frames_next();

                        while nframes >= input_frames_next + start_idx {
                            let col_chunks = harray_slice.chunks_exact(nframes);
                            let end_idx = start_idx + input_frames_next;
                            for (ib, cc) in input_buffer.iter_mut().zip(col_chunks) {
                                ib.extend_from_slice(&cc[start_idx..end_idx]);
                            }

                            start_idx += end_idx;

                            let (_, _) = self.process_into_buffer(&input_buffer, &mut output_buffer, None)?;

                            input_frames_next = self.input_frames_next();

                            for ((vo, ob), ib) in v_out.iter_mut().zip(output_buffer.iter()).zip(input_buffer.iter_mut()) {
                                vo.extend_from_slice(ob);
                                ib.clear();
                            }
                        }

                        let v: Vec<T> = v_out.into_iter().flatten().collect();
                        let nframes = v.len() / nchannels;

                        *harray = HArray::new_from_shape_vec(IxDyn(&[nchannels, nframes]), v)?;

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

    // All tests below are for a chunk only. Changing chunks_size or length may yield failing results.
    #[test]
    fn process_resampler_sinc_fixed_in_test() {
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
        let mut resampler =
            SincFixedIn::<f64>::new(sr_out as f64 / sr_in as f64, 2.0, params, chunk_size, 2)
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
    fn process_resampler_fast_fixed_in_test() {
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
    fn process_resampler_fft_fixed_in_test() {
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
            FftFixedIn::<f64>::new(sr_in as usize, sr_out, chunk_size, 2, 2).unwrap();

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
    fn process_resampler_fft_fixed_in_out_test() {
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
            FftFixedInOut::<f64>::new(sr_in as usize, sr_out, chunk_size, 2).unwrap();

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
    fn process_resampler_sinc_fixed_out_test() {
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
        let mut resampler =
            SincFixedOut::<f64>::new(sr_out as f64 / sr_in as f64, 2.0, params, chunk_size, 2)
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
    fn process_resampler_fast_fixed_out_test() {
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
    fn process_resampler_fft_fixed_out_test() {
        let sr_in = 44100;
        let sr_out = 48000;
        let chunk_size = 512;
        let mut resampler = FftFixedOut::<f64>::new(sr_in, sr_out, chunk_size, 2, 2).unwrap();

        let length = resampler.input_frames_next() * 2;
        let v: Vec<f64> = (0..length).map(|x| x as f64).collect();
        let mut harray = HArray::new_from_shape_vec((2, length / 2), v).unwrap();

        resampler.process_resampler(&mut harray).unwrap();

        let mut resampler = FftFixedOut::<f64>::new(sr_in, sr_out, chunk_size, 2, 2).unwrap();

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
