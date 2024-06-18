use harmonium_core::{array::HArray, errors::HError, errors::HResult};
use ndarray::{ArcArray1, ArcArray2, Axis, Dimension, Ix1, Ix2, IxDyn, Zip};
use realfft::RealFftPlanner;
use rustfft::{
    num_complex::Complex,
    num_traits::{Float, FloatConst},
    FftPlanner,
};

pub trait ProcessFft<T, D>
where
    T: Float + FloatConst,
    D: Dimension,
{
    fn fft(&mut self, harray: &mut HArray<Complex<T>, D>) -> HResult<()>;
    fn ifft(&mut self, harray: &mut HArray<Complex<T>, D>) -> HResult<()>;
}

pub trait ProcessRealFft<T, D>
where
    T: Float + FloatConst,
    D: Dimension,
{
    fn rfft(&mut self, harray: &mut HArray<T, D>) -> HResult<HArray<Complex<T>, D>>;
    fn irfft(&mut self, harray: &mut HArray<Complex<T>, D>, length: usize)
        -> HResult<HArray<T, D>>;
}

macro_rules! impl_fft {
    ($($t: ty),+) => {
        $(
            impl ProcessFft<$t, Ix1> for FftPlanner<$t> {
                fn fft(&mut self, harray: &mut HArray<Complex<$t>, Ix1>) -> HResult<()> {
                    let length = harray.len();
                    let fft = self.plan_fft_forward(length);
                    let scratch_len = fft.get_inplace_scratch_len();
                    let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                    fft.process_with_scratch(harray.as_slice_mut().unwrap(), &mut scratch_buffer);
                    Ok(())
                }

                fn ifft(&mut self, harray: &mut HArray<Complex<$t>, Ix1>) -> HResult<()> {
                    let length = harray.len();
                    let fft = self.plan_fft_inverse(length);
                    let scratch_len = fft.get_inplace_scratch_len();
                    let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                    fft.process_with_scratch(harray.as_slice_mut().unwrap(), &mut scratch_buffer);
                    Ok(())
                }
            }

            impl ProcessRealFft<$t, Ix1> for RealFftPlanner<$t> {
                fn rfft(&mut self, harray: &mut HArray<$t, Ix1>) -> HResult<HArray<Complex<$t>, Ix1>> {
                    let length = harray.len();
                    let r2c = self.plan_fft_forward(length);
                    let scratch_len = r2c.get_scratch_len();
                    let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                    let mut ndarray = ArcArray1::from_elem(length / 2 + 1, Complex::new(0., 0.));
                    r2c.process_with_scratch(harray.as_slice_mut().unwrap(), ndarray.as_slice_mut().unwrap(), &mut scratch_buffer).unwrap();
                    Ok(HArray(ndarray))
                }

                fn irfft(&mut self, harray: &mut HArray<Complex<$t>, Ix1>, length: usize) -> HResult<HArray<$t, Ix1>> {
                    let c2r = self.plan_fft_inverse(length);
                    let scratch_len = c2r.get_scratch_len();
                    let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                    let mut ndarray = ArcArray1::from_elem(length, 0.);
                    c2r.process_with_scratch(harray.as_slice_mut().unwrap(), ndarray.as_slice_mut().unwrap(), &mut scratch_buffer).unwrap();
                    Ok(HArray(ndarray))
                }
            }

            impl ProcessFft<$t, Ix2> for FftPlanner<$t> {
                fn fft(&mut self, harray: &mut HArray<Complex<$t>, Ix2>) -> HResult<()> {
                    let ncols = harray.0.ncols();
                    let fft = self.plan_fft_forward(ncols);
                    let scratch_len = fft.get_inplace_scratch_len();
                    let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];

                    Zip::from(harray.0.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process_with_scratch(row.as_slice_mut().unwrap(), &mut scratch_buffer);
                    });
                    Ok(())
                }

                fn ifft(&mut self, harray: &mut HArray<Complex<$t>, Ix2>) -> HResult<()> {
                    let ncols = harray.0.ncols();
                    let fft = self.plan_fft_inverse(ncols);
                    let scratch_len = fft.get_inplace_scratch_len();
                    let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];

                    Zip::from(harray.0.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process_with_scratch(row.as_slice_mut().unwrap(), &mut scratch_buffer);
                    });
                    Ok(())
                }
            }

            impl ProcessRealFft<$t, Ix2> for RealFftPlanner<$t> {
                fn rfft(&mut self, harray: &mut HArray<$t, Ix2>) -> HResult<HArray<Complex<$t>, Ix2>> {
                    let nrows = harray.0.nrows();
                    let ncols = harray.0.ncols();
                    let r2c = self.plan_fft_forward(ncols);
                    let scratch_len = r2c.get_scratch_len();
                    let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                    let mut ndarray = ArcArray2::from_elem((nrows, ncols / 2 + 1), Complex::new(0., 0.));

                    Zip::from(ndarray.lanes_mut(Axis(1)))
                        .and(harray.0.lanes_mut(Axis(1)))
                        .for_each(|mut row_output, mut row_input| {
                            r2c
                                .process_with_scratch(row_input.as_slice_mut().unwrap(), row_output.as_slice_mut().unwrap(), &mut scratch_buffer)
                                .unwrap();
                            });

                    Ok(HArray(ndarray))
                }

                fn irfft(&mut self, harray: &mut HArray<Complex<$t>, Ix2>, length: usize) -> HResult<HArray<$t, Ix2>> {
                    let nrows = harray.0.nrows();
                    let c2r = self.plan_fft_inverse(length);
                    let scratch_len = c2r.get_scratch_len();
                    let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                    let mut ndarray = ArcArray2::from_elem((nrows, length), 0.);

                    Zip::from(ndarray.lanes_mut(Axis(1)))
                        .and(harray.0.lanes_mut(Axis(1)))
                        .for_each(|mut row_output, mut row_input| {
                            c2r
                                .process_with_scratch(row_input.as_slice_mut().unwrap(), row_output.as_slice_mut().unwrap(), &mut scratch_buffer)
                                .unwrap();
                            });

                    Ok(HArray(ndarray))
                }
            }

            impl ProcessFft<$t, IxDyn> for FftPlanner<$t> {
                fn fft(&mut self, harray: &mut HArray<Complex<$t>, IxDyn>) -> HResult<()> {
                    match harray.ndim() {
                        1 => {
                            let length = harray.len();
                            let fft = self.plan_fft_forward(length);
                            let scratch_len = fft.get_inplace_scratch_len();
                            let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                            fft.process_with_scratch(harray.as_slice_mut().unwrap(), &mut scratch_buffer);
                            Ok(())
                        },
                        2 => {
                            let ncols = harray.0.len_of(Axis(1));
                            let fft = self.plan_fft_forward(ncols);
                            let scratch_len = fft.get_inplace_scratch_len();
                            let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];

                            Zip::from(harray.0.lanes_mut(Axis(1))).for_each(|mut row| {
                                fft.process_with_scratch(row.as_slice_mut().unwrap(), &mut scratch_buffer);
                            });
                            Ok(())
                        },
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }

                fn ifft(&mut self, harray: &mut HArray<Complex<$t>, IxDyn>) -> HResult<()> {
                    match harray.ndim() {
                        1 => {
                            let length = harray.len();
                            let fft = self.plan_fft_inverse(length);
                            let scratch_len = fft.get_inplace_scratch_len();
                            let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                            fft.process_with_scratch(harray.as_slice_mut().unwrap(), &mut scratch_buffer);
                            Ok(())
                        },
                        2 => {
                            let ncols = harray.0.len_of(Axis(1));
                            let fft = self.plan_fft_inverse(ncols);
                            let scratch_len = fft.get_inplace_scratch_len();
                            let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];

                            Zip::from(harray.0.lanes_mut(Axis(1))).for_each(|mut row| {
                                fft.process_with_scratch(row.as_slice_mut().unwrap(), &mut scratch_buffer);
                            });
                            Ok(())
                        },
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }
            }

            impl ProcessRealFft<$t, IxDyn> for RealFftPlanner<$t> {
                fn rfft(&mut self, harray: &mut HArray<$t, IxDyn>) -> HResult<HArray<Complex<$t>, IxDyn>> {
                    match harray.ndim() {
                        1 => {
                            let length = harray.len();
                            let r2c = self.plan_fft_forward(length);
                            let scratch_len = r2c.get_scratch_len();
                            let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                            let mut ndarray = ArcArray1::from_elem(length / 2 + 1, Complex::new(0., 0.));
                            r2c.process_with_scratch(harray.as_slice_mut().unwrap(), ndarray.as_slice_mut().unwrap(), &mut scratch_buffer).unwrap();
                            Ok(HArray(ndarray.into_dyn()))
                        },
                        2 => {
                            let nrows = harray.0.len_of(Axis(0));
                            let ncols = harray.0.len_of(Axis(1));
                            let r2c = self.plan_fft_forward(ncols);
                            let scratch_len = r2c.get_scratch_len();
                            let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                            let mut ndarray = ArcArray2::from_elem((nrows, ncols / 2 + 1), Complex::new(0., 0.)).into_dyn();

                            Zip::from(ndarray.lanes_mut(Axis(1)))
                                .and(harray.0.lanes_mut(Axis(1)))
                                .for_each(|mut row_output, mut row_input| {
                                    r2c
                                        .process_with_scratch(row_input.as_slice_mut().unwrap(), row_output.as_slice_mut().unwrap(), &mut scratch_buffer)
                                        .unwrap();
                                    });

                            Ok(HArray(ndarray))
                        }
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }

                fn irfft(&mut self, harray: &mut HArray<Complex<$t>, IxDyn>, length: usize) -> HResult<HArray<$t, IxDyn>> {
                    match harray.ndim() {
                        1 => {
                            let c2r = self.plan_fft_inverse(length);
                            let scratch_len = c2r.get_scratch_len();
                            let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                            let mut ndarray = ArcArray1::from_elem(length, 0.);
                            c2r.process_with_scratch(harray.as_slice_mut().unwrap(), ndarray.as_slice_mut().unwrap(), &mut scratch_buffer).unwrap();
                            Ok(HArray(ndarray.into_dyn()))
                        },
                        2 => {
                            let nrows = harray.0.len_of(Axis(0));
                            let c2r = self.plan_fft_inverse(length);
                            let scratch_len = c2r.get_scratch_len();
                            let mut scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                            let mut ndarray = ArcArray2::from_elem((nrows, length), 0.).into_dyn();

                            Zip::from(ndarray.lanes_mut(Axis(1)))
                                .and(harray.0.lanes_mut(Axis(1)))
                                .for_each(|mut row_output, mut row_input| {
                                    c2r
                                        .process_with_scratch(row_input.as_slice_mut().unwrap(), row_output.as_slice_mut().unwrap(), &mut scratch_buffer)
                                        .unwrap();
                                    });

                            Ok(HArray(ndarray))
                        },
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }
            }
        )+
    };
}

impl_fft!(f32, f64);

#[cfg(test)]
mod tests {
    use harmonium_core::{
        comparison::{compare_harray, compare_harray_complex},
        conversions::IntoDynamic,
    };

    use super::*;

    #[test]
    fn fft_1d_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let mut lhs = HArray::new_from_shape_vec(6, v).unwrap();
        let mut planner = FftPlanner::new();
        planner.fft(&mut lhs).unwrap();
        let result = vec![
            Complex::new(36.0, 42.0),
            Complex::new(-16.392305, 4.392305),
            Complex::new(-9.464102, -2.535898),
            Complex::new(-6.0, -6.0),
            Complex::new(-2.535898, -9.464102),
            Complex::new(4.392305, -16.392305),
        ];
        let rhs = HArray::new_from_shape_vec(6, result).unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));
    }
    #[test]
    fn fft_2d_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let mut lhs = HArray::new_from_shape_vec((3, 2), v).unwrap();
        let mut planner = FftPlanner::new();
        planner.fft(&mut lhs).unwrap();
        let result = vec![
            Complex::new(4_f32, 6_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(12_f32, 14_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(20_f32, 22_f32),
            Complex::new(-2_f32, -2_f32),
        ];
        let rhs = HArray::new_from_shape_vec((3, 2), result).unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));
    }
    #[test]
    fn fft_dyn_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let mut lhs = HArray::new_from_shape_vec((3, 2), v)
            .unwrap()
            .into_dynamic();
        let mut planner = FftPlanner::new();
        planner.fft(&mut lhs).unwrap();
        let result = vec![
            Complex::new(4_f32, 6_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(12_f32, 14_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(20_f32, 22_f32),
            Complex::new(-2_f32, -2_f32),
        ];
        let rhs = HArray::new_from_shape_vec((3, 2), result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray_complex(&lhs, &rhs));
    }

    #[test]
    fn ifft_1d_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let mut lhs = HArray::new_from_shape_vec(6, v.clone()).unwrap();
        let mut planner = FftPlanner::new();
        planner.ifft(&mut lhs).unwrap();
        let result = vec![
            Complex::new(36.0, 42.0),
            Complex::new(4.392305, -16.392305),
            Complex::new(-2.535898, -9.464102),
            Complex::new(-6.0, -6.0),
            Complex::new(-9.464102, -2.535898),
            Complex::new(-16.392305, 4.392305),
        ];
        let rhs = HArray::new_from_shape_vec(6, result).unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));
    }

    #[test]
    fn ifft_2d_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let mut lhs = HArray::new_from_shape_vec((3, 2), v).unwrap();
        let mut planner = FftPlanner::new();
        planner.ifft(&mut lhs).unwrap();
        let result = vec![
            Complex::new(4.0, 6.0),
            Complex::new(-2.0, -2.0),
            Complex::new(12.0, 14.0),
            Complex::new(-2.0, -2.0),
            Complex::new(20.0, 22.0),
            Complex::new(-2.0, -2.0),
        ];
        let rhs = HArray::new_from_shape_vec((3, 2), result).unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));
    }

    #[test]
    fn ifft_dyn_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let mut lhs = HArray::new_from_shape_vec((3, 2), v)
            .unwrap()
            .into_dynamic();
        let mut planner = FftPlanner::new();
        planner.ifft(&mut lhs).unwrap();
        let result = vec![
            Complex::new(4.0, 6.0),
            Complex::new(-2.0, -2.0),
            Complex::new(12.0, 14.0),
            Complex::new(-2.0, -2.0),
            Complex::new(20.0, 22.0),
            Complex::new(-2.0, -2.0),
        ];
        let rhs = HArray::new_from_shape_vec((3, 2), result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray_complex(&lhs, &rhs));
    }

    #[test]
    fn rfft_1d_test() {
        let v = vec![1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32];
        let mut harray = HArray::new_from_shape_vec(6, v).unwrap();
        let mut planner = RealFftPlanner::new();
        let lhs = planner.rfft(&mut harray).unwrap();
        let result = vec![
            Complex::new(21.0, 0.0),
            Complex::new(-3.0, 5.196152),
            Complex::new(-3.0, 1.732051),
            Complex::new(-3.0, 0.0),
        ];
        let rhs = HArray::new_from_shape_vec(4, result).unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));
    }

    #[test]
    fn rfft_2d_test() {
        let v = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32, 11_f32, 12_f32,
        ];
        let mut harray = HArray::new_from_shape_vec((3, 4), v).unwrap();
        let mut planner = RealFftPlanner::new();
        let lhs = planner.rfft(&mut harray).unwrap();
        let result = vec![
            Complex::new(10_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(26_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(42_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
        ];
        let rhs = HArray::new_from_shape_vec((3, 3), result).unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));
    }

    #[test]
    fn rfft_dyn_test() {
        let v = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32, 11_f32, 12_f32,
        ];
        let mut harray = HArray::new_from_shape_vec((3, 4), v)
            .unwrap()
            .into_dynamic();
        let mut planner = RealFftPlanner::new();
        let lhs = planner.rfft(&mut harray).unwrap();
        let result = vec![
            Complex::new(10_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(26_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(42_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
        ];
        let rhs = HArray::new_from_shape_vec((3, 3), result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray_complex(&lhs, &rhs));
    }

    #[test]
    fn irfft_1d_test() {
        // Test irfft with length and length + 1.
        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec(6, v).unwrap();
        let length = harray.len();
        let mut planner = RealFftPlanner::new();
        let mut spectrum = planner.rfft(&mut harray).unwrap();
        let lhs = planner.irfft(&mut spectrum, length).unwrap();
        let result = vec![6_f32, 12., 18., 24., 30., 36.];
        let rhs = HArray::new_from_shape_vec(length, result).unwrap();
        assert!(compare_harray(&lhs, &rhs));

        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec(6, v).unwrap();
        let length = harray.len() + 1;
        let mut planner = RealFftPlanner::new();
        let mut spectrum = planner.rfft(&mut harray).unwrap();
        let lhs = planner.irfft(&mut spectrum, length).unwrap();
        let result = vec![
            3.000000000000007_f32,
            12.497_72,
            15.371_269,
            22.199_291,
            25.800_709,
            32.628_73,
            35.502_28,
        ];
        let rhs = HArray::new_from_shape_vec(length, result).unwrap();
        assert!(compare_harray(&lhs, &rhs));
    }

    #[test]
    fn irfft_2d_test() {
        // Test irfft with length and length + 1.
        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec((3, 2), v).unwrap();
        let length = harray.0.ncols();
        let mut planner = RealFftPlanner::new();
        let mut spectrum = planner.rfft(&mut harray).unwrap();
        let lhs = planner.irfft(&mut spectrum, length).unwrap();
        let result = vec![2., 4., 6., 8., 10., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result).unwrap();
        assert!(compare_harray(&lhs, &rhs));

        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec((3, 2), v).unwrap();
        let length = harray.0.ncols() + 1;
        let mut planner = RealFftPlanner::new();
        let mut spectrum = planner.rfft(&mut harray).unwrap();
        let lhs = planner.irfft(&mut spectrum, length).unwrap();
        let result = vec![1., 4., 4., 5., 8., 8., 9., 12., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result).unwrap();
        assert!(compare_harray(&lhs, &rhs));
    }

    #[test]
    fn irfft_dyn_test() {
        // Test irfft with length and length + 1.
        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec((3, 2), v)
            .unwrap()
            .into_dynamic();
        let length = harray.0.len_of(Axis(1));
        let mut planner = RealFftPlanner::new();
        let mut spectrum = planner.rfft(&mut harray).unwrap();
        let lhs = planner.irfft(&mut spectrum, length).unwrap();
        let result = vec![2., 4., 6., 8., 10., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray(&lhs, &rhs));

        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec((3, 2), v)
            .unwrap()
            .into_dynamic();
        let length = harray.0.len_of(Axis(1)) + 1;
        let mut planner = RealFftPlanner::new();
        let mut spectrum = planner.rfft(&mut harray).unwrap();
        let lhs = planner.irfft(&mut spectrum, length).unwrap();
        let result = vec![1., 4., 4., 5., 8., 8., 9., 12., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray(&lhs, &rhs));
    }
}
