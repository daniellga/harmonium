use cached::proc_macro::cached;
use harmonium_core::{array::HArray, errors::HError, errors::HResult};
use ndarray::{ArcArray1, ArcArray2, Axis, Dimension, Ix1, Ix2, IxDyn, Zip};
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};
use rustfft::{
    num_complex::Complex,
    num_traits::{Float, FloatConst},
    FftPlanner,
};
use std::sync::Arc;

pub trait Fft<T, D>
where
    T: Float + FloatConst,
    D: Dimension,
{
    fn fft(&self) -> HResult<HArray<Complex<T>, D>>;
    fn fft_mut(&mut self) -> HResult<()>;
    fn ifft(&self) -> HResult<HArray<Complex<T>, D>>;
    fn ifft_mut(&mut self) -> HResult<()>;
    fn rfft(&mut self) -> HResult<HArray<Complex<T>, D>>;
    fn irfft(&mut self, length: usize) -> HResult<HArray<T, D>>;
}

macro_rules! impl_fft {
    ($(($t: ty, $f1: ident, $f2: ident, $f3: ident, $f4: ident, $f5: ident, $f6: ident, $f7: ident, $f8: ident, $f9: ident, $f10: ident)),* $(,)?) => {
        $(
            impl Fft<$t, Ix1> for HArray<Complex<$t>, Ix1> {
                fn fft(&self) -> HResult<HArray<Complex<$t>, Ix1>> {
                    let length = self.len();
                    let (fft, mut ndarray) = $f1(length);
                    ndarray.assign(&self.0);
                    fft.process(ndarray.as_slice_mut().unwrap());
                    Ok(HArray(ndarray))
                }

                fn fft_mut(&mut self) -> HResult<()> {
                    let length = self.len();
                    let fft = $f3(length);
                    fft.process(self.as_slice_mut().unwrap());
                    Ok(())
                }

                fn ifft(&self) -> HResult<HArray<Complex<$t>, Ix1>> {
                    let length = self.len();
                    let (fft, mut ndarray) = $f6(length);
                    ndarray.assign(&self.0);
                    fft.process(ndarray.as_slice_mut().unwrap());
                    Ok(HArray(ndarray))
                }

                fn ifft_mut(&mut self) -> HResult<()> {
                    let length = self.len();
                    let fft = $f8(length);
                    fft.process(self.as_slice_mut().unwrap());
                    Ok(())
                }

                fn rfft(&mut self) -> HResult<HArray<Complex<$t>, Ix1>> {
                    Err(HError::OutOfSpecError("rfft must be called on a float HArray.".into()))
                }

                fn irfft(&mut self, length: usize) -> HResult<HArray<$t, Ix1>> {
                    let (c2r, mut ndarray, mut scratch_buffer) = $f9(length);
                    c2r.process_with_scratch(self.0.as_slice_mut().unwrap(), ndarray.as_slice_mut().unwrap(), &mut scratch_buffer).unwrap();
                    Ok(HArray(ndarray))
                }
            }

            impl Fft<$t, Ix2> for HArray<Complex<$t>, Ix2> {
                fn fft(&self) -> HResult<HArray<Complex<$t>, Ix2>> {
                    let nrows = self.0.nrows();
                    let ncols = self.0.ncols();
                    let (fft, mut ndarray) = $f2(nrows, ncols);
                    ndarray.assign(&self.0);

                    Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });

                    Ok(HArray(ndarray))
                }

                fn fft_mut(&mut self) -> HResult<()> {
                    let ncols = self.0.ncols();
                    let fft = $f3(ncols);

                    Zip::from(self.0.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });
                    Ok(())
                }

                fn ifft(&self) -> HResult<HArray<Complex<$t>, Ix2>> {
                    let nrows = self.0.nrows();
                    let ncols = self.0.ncols();
                    let (fft, mut ndarray) = $f7(nrows, ncols);
                    ndarray.assign(&self.0);

                    Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });

                    Ok(HArray(ndarray))
                }

                fn ifft_mut(&mut self) -> HResult<()> {
                    let ncols = self.0.ncols();
                    let fft = $f8(ncols);

                    Zip::from(self.0.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });
                    Ok(())
                }

                fn rfft(&mut self) -> HResult<HArray<Complex<$t>, Ix2>> {
                    Err(HError::OutOfSpecError("rfft must be called on a float HArray.".into()))
                }

                fn irfft(&mut self, length: usize) -> HResult<HArray<$t, Ix2>> {
                    let nrows = self.0.nrows();
                    let (c2r, mut ndarray, mut scratch_buffer) = $f10(nrows, length);

                    Zip::from(ndarray.lanes_mut(Axis(1)))
                        .and(self.0.lanes_mut(Axis(1)))
                        .for_each(|mut row_output, mut row_input| {
                            c2r
                                .process_with_scratch(row_input.as_slice_mut().unwrap(), row_output.as_slice_mut().unwrap(), &mut scratch_buffer)
                                .unwrap();
                            });

                    Ok(HArray(ndarray))
                }
            }

            impl Fft<$t, IxDyn> for HArray<Complex<$t>, IxDyn> {
                fn fft(&self) -> HResult<HArray<Complex<$t>, IxDyn>> {
                    match self.ndim() {
                        1 => {
                            let length = self.0.len();
                            let (fft, mut ndarray) = $f1(length);
                            ndarray.assign(&self.0);
                            fft.process(ndarray.as_slice_mut().unwrap());
                            Ok(HArray(ndarray.into_dyn()))
                         },
                        2 => {
                            let nrows = self.0.len_of(Axis(0));
                            let ncols = self.0.len_of(Axis(1));
                            let (fft, mut ndarray) = $f2(nrows, ncols);
                            ndarray.assign(&self.0);

                            Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                                fft.process(row.as_slice_mut().unwrap());
                            });

                            Ok(HArray(ndarray.into_dyn()))
                        },
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }

                fn fft_mut(&mut self) -> HResult<()> {
                    match self.ndim() {
                        1 => {
                            let length = self.0.len();
                            let fft = $f3(length);
                            fft.process(self.as_slice_mut().unwrap());
                            Ok(())
                        },
                        2 => {
                            let ncols = self.0.len_of(Axis(1));
                            let fft = $f3(ncols);

                            Zip::from(self.0.lanes_mut(Axis(1))).for_each(|mut row| {
                                fft.process(row.as_slice_mut().unwrap());
                            });
                            Ok(())
                        },
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }

                fn ifft(&self) -> HResult<HArray<Complex<$t>, IxDyn>> {
                    match self.ndim() {
                        1 => {
                            let length = self.0.len();
                            let (fft, mut ndarray) = $f6(length);
                            ndarray.assign(&self.0);
                            fft.process(ndarray.as_slice_mut().unwrap());
                            Ok(HArray(ndarray.into_dyn()))
                        },
                        2 => {
                            let nrows = self.0.len_of(Axis(0));
                            let ncols = self.0.len_of(Axis(1));
                            let (fft, mut ndarray) = $f7(nrows, ncols);
                            ndarray.assign(&self.0);

                            Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                                fft.process(row.as_slice_mut().unwrap());
                            });

                            Ok(HArray(ndarray.into_dyn()))
                        },
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }

                fn ifft_mut(&mut self) -> HResult<()> {
                    match self.ndim() {
                        1 => {
                            let length = self.0.len();
                            let fft = $f8(length);
                            fft.process(self.as_slice_mut().unwrap());
                            Ok(())
                        },
                        2 => {
                            let ncols = self.0.len_of(Axis(1));
                            let fft = $f8(ncols);

                            Zip::from(self.0.lanes_mut(Axis(1))).for_each(|mut row| {
                                fft.process(row.as_slice_mut().unwrap());
                            });
                            Ok(())
                        },
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }

                fn rfft(&mut self) -> HResult<HArray<Complex<$t>, IxDyn>> {
                    Err(HError::OutOfSpecError("rfft must be called on a float HArray.".into()))
                }

                fn irfft(&mut self, length: usize) -> HResult<HArray<$t, IxDyn>> {
                    match self.ndim() {
                        1 => {
                            let (c2r, mut ndarray, mut scratch_buffer) = $f9(length);
                            c2r.process_with_scratch(self.0.as_slice_mut().unwrap(), ndarray.as_slice_mut().unwrap(), &mut scratch_buffer).unwrap();
                            Ok(HArray(ndarray.into_dyn()))
                        },
                        2 => {
                            let nrows = self.0.len_of(Axis(0));
                            let (r2c, ndarray, mut scratch_buffer) = $f10(nrows, length);
                            let mut ndarray = ndarray.into_dyn();

                            Zip::from(ndarray.lanes_mut(Axis(1)))
                                .and(self.0.lanes_mut(Axis(1)))
                                .for_each(|mut row_output, mut row_input| {
                                    r2c
                                        .process_with_scratch(row_input.as_slice_mut().unwrap(), row_output.as_slice_mut().unwrap(), &mut scratch_buffer)
                                        .unwrap();
                                    });

                            Ok(HArray(ndarray.into_dyn()))
                        },
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }
            }

            impl Fft<$t, Ix1> for HArray<$t, Ix1> {
                fn fft(&self) -> HResult<HArray<Complex<$t>, Ix1>> {
                    let length = self.len();
                    let (fft, mut ndarray) = $f1(length);
                    ndarray.zip_mut_with(&self.0, |x, y| *x = Complex::new(*y, 0.));
                    fft.process(ndarray.as_slice_mut().unwrap());
                    Ok(HArray(ndarray))
                }

                fn fft_mut(&mut self) -> HResult<()> {
                    Err(HError::OutOfSpecError("fft_mut must be called on a complex HArray.".into()))
                }

                fn ifft(&self) -> HResult<HArray<Complex<$t>, Ix1>> {
                    Err(HError::OutOfSpecError("ifft must be called on a complex HArray.".into()))
                }

                fn ifft_mut(&mut self) -> HResult<()> {
                    Err(HError::OutOfSpecError("ifft_mut must be called on a complex HArray.".into()))
                }

                fn rfft(&mut self) -> HResult<HArray<Complex<$t>, Ix1>> {
                    let length = self.len();
                    let (r2c, mut ndarray, mut scratch_buffer) = $f4(length);
                    r2c.process_with_scratch(&mut self.0.as_slice_mut().unwrap(), ndarray.as_slice_mut().unwrap(), &mut scratch_buffer).unwrap();
                    Ok(HArray(ndarray))
                }

                fn irfft(&mut self, _length: usize) -> HResult<HArray<$t, Ix1>> {
                    Err(HError::OutOfSpecError("irfft must be called on a complex HArray.".into()))
                }


            }

            impl Fft<$t, Ix2> for HArray<$t, Ix2> {
                fn fft(&self) -> HResult<HArray<Complex<$t>, Ix2>> {
                    let nrows = self.0.nrows();
                    let ncols = self.0.ncols();
                    let (fft, mut ndarray) = $f2(nrows, ncols);

                    ndarray.zip_mut_with(&self.0, |x, y| *x = Complex::new(*y, 0.));

                    Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });

                    Ok(HArray(ndarray))
                }

                fn fft_mut(&mut self) -> HResult<()> {
                    Err(HError::OutOfSpecError("fft_mut must be called on a complex HArray.".into()))
                }

                fn ifft(&self) -> HResult<HArray<Complex<$t>, Ix2>> {
                    Err(HError::OutOfSpecError("ifft must be called on a complex HArray.".into()))
                }

                fn ifft_mut(&mut self) -> HResult<()> {
                    Err(HError::OutOfSpecError("ifft_mut must be called on a complex HArray.".into()))
                }

                fn rfft(&mut self) -> HResult<HArray<Complex<$t>, Ix2>> {
                    let nrows = self.0.nrows();
                    let ncols = self.0.ncols();
                    let (r2c, mut ndarray, mut scratch_buffer) = $f5(nrows, ncols);

                    Zip::from(ndarray.lanes_mut(Axis(1)))
                        .and(self.0.lanes_mut(Axis(1)))
                        .for_each(|mut row_output, mut row_input| {
                            r2c
                                .process_with_scratch(&mut row_input.as_slice_mut().unwrap(), row_output.as_slice_mut().unwrap(), &mut scratch_buffer)
                                .unwrap();
                            });

                    Ok(HArray(ndarray))
                }

                fn irfft(&mut self, _length: usize) -> HResult<HArray<$t, Ix2>> {
                    Err(HError::OutOfSpecError("irfft must be called on a complex HArray.".into()))
                }
            }

            impl Fft<$t, IxDyn> for HArray<$t, IxDyn> {
                fn fft(&self) -> HResult<HArray<Complex<$t>, IxDyn>> {
                    match self.ndim() {
                        1 => {
                            let length = self.len();
                            let (fft, mut ndarray) = $f1(length);
                            ndarray.zip_mut_with(&self.0, |x, y| *x = Complex::new(*y, 0.));
                            fft.process(ndarray.as_slice_mut().unwrap());
                            Ok(HArray(ndarray.into_dyn()))
                         },
                        2 => {
                            let nrows = self.0.len_of(Axis(0));
                            let ncols = self.0.len_of(Axis(1));
                            let (fft, mut ndarray) = $f2(nrows, ncols);

                            ndarray.zip_mut_with(&self.0, |x, y| *x = Complex::new(*y, 0.));

                            Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                                fft.process(row.as_slice_mut().unwrap());
                            });

                            Ok(HArray(ndarray.into_dyn()))
                        },
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }

                fn fft_mut(&mut self) -> HResult<()> {
                    Err(HError::OutOfSpecError("fft_mut must be called on a complex HArray.".into()))
                }

                fn ifft(&self) -> HResult<HArray<Complex<$t>, IxDyn>> {
                    Err(HError::OutOfSpecError("ifft must be called on a complex HArray.".into()))
                }

                fn ifft_mut(&mut self) -> HResult<()> {
                    Err(HError::OutOfSpecError("ifft_mut must be called on a complex HArray.".into()))
                }

                fn rfft(&mut self) -> HResult<HArray<Complex<$t>, IxDyn>> {
                    match self.ndim() {
                        1 => {
                            let length = self.len();
                            let (r2c, mut ndarray, mut scratch_buffer) = $f4(length);
                            r2c.process_with_scratch(&mut self.0.as_slice_mut().unwrap(), ndarray.as_slice_mut().unwrap(), &mut scratch_buffer).unwrap();
                            Ok(HArray(ndarray.into_dyn()))
                        },
                        2 => {
                            let nrows = self.0.len_of(Axis(0));
                            let ncols = self.0.len_of(Axis(1));
                            let (r2c, ndarray, mut scratch_buffer) = $f5(nrows, ncols);
                            let mut ndarray = ndarray.into_dyn();

                            Zip::from(ndarray.lanes_mut(Axis(1)))
                                .and(self.0.lanes_mut(Axis(1)))
                                .for_each(|mut row_output, mut row_input| {
                                    r2c
                                        .process_with_scratch(&mut row_input.as_slice_mut().unwrap(), row_output.as_slice_mut().unwrap(), &mut scratch_buffer)
                                        .unwrap();
                                    });

                            Ok(HArray(ndarray.into_dyn()))
                        }
                        _ => Err(HError::OutOfSpecError("The HArray's ndim should be 1 or 2.".into())),
                    }
                }

                fn irfft(&mut self, _length: usize) -> HResult<HArray<$t, IxDyn>> {
                    Err(HError::OutOfSpecError("irfft must be called on a complex HArray.".into()))
                }
            }

            #[cached]
            fn $f1(length: usize) -> (Arc<dyn rustfft::Fft<$t>>, ArcArray1<Complex<$t>>) {
                let mut planner = FftPlanner::<$t>::new();
                let ndarray = ArcArray1::zeros(length);
                (planner.plan_fft_forward(length), ndarray)
            }

            #[cached]
            fn $f2(
                nrows: usize,
                ncols: usize,
            ) -> (Arc<dyn rustfft::Fft<$t>>, ArcArray2<Complex<$t>>) {
                let mut planner = FftPlanner::<$t>::new();
                let ndarray = ArcArray2::zeros((nrows, ncols));
                (planner.plan_fft_forward(ncols), ndarray)
            }

            #[cached]
            fn $f3(length: usize) -> Arc<dyn rustfft::Fft<$t>> {
                let mut planner = FftPlanner::<$t>::new();
                planner.plan_fft_forward(length)
            }

            #[cached]
            pub fn $f4(length: usize) -> (Arc<dyn RealToComplex<$t>>, ArcArray1<Complex<$t>>, Vec<Complex<$t>>) {
                let mut real_planner = RealFftPlanner::<$t>::new();
                let r2c = real_planner.plan_fft_forward(length);
                let scratch_len = r2c.get_scratch_len();
                let scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                let ndarray = ArcArray1::from_elem(length / 2 + 1, Complex::new(0., 0.));
                (r2c, ndarray, scratch_buffer)
            }

            #[cached]
            pub fn $f5(nrows: usize, ncols: usize) -> (Arc<dyn RealToComplex<$t>>, ArcArray2<Complex<$t>>, Vec<Complex<$t>>) {
                let mut real_planner = RealFftPlanner::<$t>::new();
                let r2c = real_planner.plan_fft_forward(ncols);
                let scratch_len = r2c.get_scratch_len();
                let scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                let ndarray = ArcArray2::from_elem((nrows, ncols / 2 + 1), Complex::new(0., 0.));
                (r2c, ndarray, scratch_buffer)
            }

            #[cached]
            fn $f6(length: usize) -> (Arc<dyn rustfft::Fft<$t>>, ArcArray1<Complex<$t>>) {
                let mut planner = FftPlanner::<$t>::new();
                let ndarray = ArcArray1::zeros(length);
                (planner.plan_fft_inverse(length), ndarray)
            }

            #[cached]
            fn $f7(
                nrows: usize,
                ncols: usize,
            ) -> (Arc<dyn rustfft::Fft<$t>>, ArcArray2<Complex<$t>>) {
                let mut planner = FftPlanner::<$t>::new();
                let ndarray = ArcArray2::zeros((nrows, ncols));
                (planner.plan_fft_inverse(ncols), ndarray)
            }

            #[cached]
            fn $f8(length: usize) -> Arc<dyn rustfft::Fft<$t>> {
                let mut planner = FftPlanner::<$t>::new();
                planner.plan_fft_inverse(length)
            }

            #[cached]
            pub fn $f9(length: usize) -> (Arc<dyn ComplexToReal<$t>>, ArcArray1<$t>, Vec<Complex<$t>>) {
                let mut real_planner = RealFftPlanner::<$t>::new();
                let c2r = real_planner.plan_fft_inverse(length);
                let scratch_len = c2r.get_scratch_len();
                let scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                let ndarray = ArcArray1::from_elem(length, 0.);
                (c2r, ndarray, scratch_buffer)
            }

            #[cached]
            pub fn $f10(nrows: usize, length: usize) -> (Arc<dyn ComplexToReal<$t>>, ArcArray2<$t>, Vec<Complex<$t>>) {
                let mut real_planner = RealFftPlanner::<$t>::new();
                let c2r = real_planner.plan_fft_inverse(length);
                let scratch_len = c2r.get_scratch_len();
                let scratch_buffer = vec![Complex::new(0., 0.); scratch_len];
                let ndarray = ArcArray2::from_elem((nrows, length), 0.);
                (c2r, ndarray, scratch_buffer)
            }
        )+
    };
}

impl_fft!(
    (
        f32,
        create_planner_forward_1d_f32,
        create_planner_forward_2d_f32,
        creat_planner_mut_forward_f32,
        create_real_planner_forward_1d_f32,
        create_real_planner_forward_2d_f32,
        create_planner_inverse_1d_f32,
        create_planner_inverse_2d_f32,
        creat_planner_mut_inverse_f32,
        create_real_planner_inverse_1d_f32,
        create_real_planner_inverse_2d_f32
    ),
    (
        f64,
        create_planner_forward_1d_f64,
        create_planner_forward_2d_f64,
        creat_planner_mut_forward_f64,
        create_real_planner_forward_1d_f64,
        create_real_planner_forward_2d_f64,
        create_planner_inverse_1d_f64,
        create_planner_inverse_2d_f64,
        creat_planner_mut_inverse_f64,
        create_real_planner_inverse_1d_f64,
        create_real_planner_inverse_2d_f64
    ),
);

#[cfg(test)]
mod tests {
    use harmonium_core::{
        audioop::AudioOp,
        comparison::{compare_harray, compare_harray_complex},
        conversions::IntoDynamic,
    };

    use super::*;

    #[test]
    fn fft_complex_1d_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let harray = HArray::new_from_shape_vec(6, v.clone()).unwrap();
        let lhs = harray.fft().unwrap();
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

        let mut lhs = HArray::new_from_shape_vec(6, v.clone()).unwrap();
        lhs.fft_mut().unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));

        let lhs = HArray::new_from_shape_vec(6, v.clone()).unwrap();
        let lhs = lhs.ifft().unwrap();
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

        let mut lhs = HArray::new_from_shape_vec(6, v.clone()).unwrap();
        lhs.ifft_mut().unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));

        let mut harray = HArray::new_from_shape_vec(6, v.clone()).unwrap();
        assert_eq!(harray.rfft().is_err(), true);

        // Test irfft with length and length + 1.
        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec(6, v).unwrap();
        let length = harray.len();
        let mut spectrum = harray.rfft().unwrap();
        let lhs = spectrum.irfft(length).unwrap();
        let result = vec![6_f32, 12., 18., 24., 30., 36.];
        let rhs = HArray::new_from_shape_vec(length, result).unwrap();
        assert!(compare_harray(&lhs, &rhs));

        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec(6, v).unwrap();
        let length = harray.len() + 1;
        let mut spectrum = harray.rfft().unwrap();
        let lhs = spectrum.irfft(length).unwrap();
        let result = vec![
            3.000000000000007_f32,
            12.49771954121893,
            15.371269295763335,
            22.199291616805617,
            25.800708383194376,
            32.62873070423666,
            35.50228045878107,
        ];
        let rhs = HArray::new_from_shape_vec(length, result).unwrap();
        assert!(compare_harray(&lhs, &rhs));
    }

    #[test]
    fn fft_complex_1d_dyn_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let harray = HArray::new_from_shape_vec(6, v.clone())
            .unwrap()
            .into_dynamic();
        let lhs = harray.fft().unwrap();
        let result = vec![
            Complex::new(36.0, 42.0),
            Complex::new(-16.392305, 4.392305),
            Complex::new(-9.464102, -2.535898),
            Complex::new(-6.0, -6.0),
            Complex::new(-2.535898, -9.464102),
            Complex::new(4.392305, -16.392305),
        ];
        let rhs = HArray::new_from_shape_vec(6, result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray_complex(&lhs, &rhs));

        let mut lhs = HArray::new_from_shape_vec(6, v.clone())
            .unwrap()
            .into_dynamic();
        lhs.fft_mut().unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));

        let lhs = HArray::new_from_shape_vec(6, v.clone())
            .unwrap()
            .into_dynamic();
        let lhs = lhs.ifft().unwrap();
        let result = vec![
            Complex::new(36.0, 42.0),
            Complex::new(4.392305, -16.392305),
            Complex::new(-2.535898, -9.464102),
            Complex::new(-6.0, -6.0),
            Complex::new(-9.464102, -2.535898),
            Complex::new(-16.392305, 4.392305),
        ];
        let rhs = HArray::new_from_shape_vec(6, result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray_complex(&lhs, &rhs));

        let mut lhs = HArray::new_from_shape_vec(6, v.clone())
            .unwrap()
            .into_dynamic();
        lhs.ifft_mut().unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));

        let mut harray = HArray::new_from_shape_vec(6, v.clone())
            .unwrap()
            .into_dynamic();
        assert_eq!(harray.rfft().is_err(), true);

        println!("1111111");

        // Test irfft with length and length + 1.
        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec(6, v).unwrap();
        let length = harray.len();
        let mut spectrum = harray.rfft().unwrap().into_dynamic();
        println!("spectrum_len: {:?}", spectrum.len());
        println!("{:?}", spectrum);
        let lhs = spectrum.irfft(length).unwrap().into_dynamic();
        println!("lhs_len: {:?}", lhs.len());
        let result = vec![6_f32, 12., 18., 24., 30., 36.];
        let rhs = HArray::new_from_shape_vec(length, result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray(&lhs, &rhs));

        println!("22222222");

        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec(6, v).unwrap().into_dynamic();
        let length = harray.len() + 1;
        let mut spectrum = harray.rfft().unwrap().into_dynamic();
        let lhs = spectrum.irfft(length).unwrap().into_dynamic();
        let result = vec![
            3.000000000000007_f32,
            12.49771954121893,
            15.371269295763335,
            22.199291616805617,
            25.800708383194376,
            32.62873070423666,
            35.50228045878107,
        ];
        let rhs = HArray::new_from_shape_vec(length, result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray(&lhs, &rhs));
    }

    #[test]
    fn fft_complex_2d_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let harray = HArray::new_from_shape_vec((3, 2), v.clone()).unwrap();
        let lhs = harray.fft().unwrap();
        let result = vec![
            Complex::new(4_f32, 6_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(12_f32, 14_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(20_f32, 22_f32),
            Complex::new(-2_f32, -2_f32),
        ];
        let rhs = HArray::new_from_shape_vec((3, 2), result).unwrap();
        assert_eq!(lhs, rhs);

        let mut lhs = HArray::new_from_shape_vec((3, 2), v.clone()).unwrap();
        lhs.fft_mut().unwrap();
        assert_eq!(lhs, rhs);

        let lhs = HArray::new_from_shape_vec((3, 2), v.clone()).unwrap();
        let lhs = lhs.ifft().unwrap();
        let result = vec![
            Complex::new(4.0, 6.0),
            Complex::new(-2.0, -2.0),
            Complex::new(12.0, 14.0),
            Complex::new(-2.0, -2.0),
            Complex::new(20.0, 22.0),
            Complex::new(-2.0, -2.0),
        ];
        let rhs = HArray::new_from_shape_vec((3, 2), result).unwrap();
        assert_eq!(lhs, rhs);

        let mut lhs = HArray::new_from_shape_vec((3, 2), v.clone()).unwrap();
        lhs.ifft_mut().unwrap();
        assert_eq!(lhs, rhs);

        let mut harray = HArray::new_from_shape_vec((3, 2), v.clone()).unwrap();
        assert_eq!(harray.rfft().is_err(), true);

        // Test irfft with length and length + 1.
        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec((3, 2), v).unwrap();
        let length = harray.0.ncols();
        let mut spectrum = harray.rfft().unwrap();
        let lhs = spectrum.irfft(length).unwrap();
        let result = vec![2., 4., 6., 8., 10., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result).unwrap();
        assert_eq!(lhs, rhs);

        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec((3, 2), v).unwrap();
        let length = harray.0.ncols() + 1;
        let mut spectrum = harray.rfft().unwrap();
        let lhs = spectrum.irfft(length).unwrap();
        let result = vec![1., 4., 4., 5., 8., 8., 9., 12., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result).unwrap();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn fft_complex_2d_dyn_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let harray = HArray::new_from_shape_vec((3, 2), v.clone())
            .unwrap()
            .into_dynamic();
        let lhs = harray.fft().unwrap().into_dynamic();
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
        assert_eq!(lhs, rhs);

        let mut lhs = HArray::new_from_shape_vec((3, 2), v.clone())
            .unwrap()
            .into_dynamic();
        lhs.fft_mut().unwrap();
        assert_eq!(lhs, rhs);

        let lhs = HArray::new_from_shape_vec((3, 2), v.clone())
            .unwrap()
            .into_dynamic();
        let lhs = lhs.ifft().unwrap().into_dynamic();
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
        assert_eq!(lhs, rhs);

        let mut lhs = HArray::new_from_shape_vec((3, 2), v.clone())
            .unwrap()
            .into_dynamic();
        lhs.ifft_mut().unwrap();
        assert_eq!(lhs, rhs);

        let mut harray = HArray::new_from_shape_vec((3, 2), v.clone())
            .unwrap()
            .into_dynamic();
        assert_eq!(harray.rfft().is_err(), true);

        // Test irfft with length and length + 1.
        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec((3, 2), v)
            .unwrap()
            .into_dynamic();
        let length = harray.nframes();
        let mut spectrum = harray.rfft().unwrap().into_dynamic();
        let lhs = spectrum.irfft(length).unwrap().into_dynamic();
        let result = vec![2., 4., 6., 8., 10., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result)
            .unwrap()
            .into_dynamic();
        assert_eq!(lhs, rhs);

        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec((3, 2), v)
            .unwrap()
            .into_dynamic();
        let length = harray.nframes() + 1;
        let mut spectrum = harray.rfft().unwrap().into_dynamic();
        let lhs = spectrum.irfft(length).unwrap().into_dynamic();
        let result = vec![1., 4., 4., 5., 8., 8., 9., 12., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result)
            .unwrap()
            .into_dynamic();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn fft_float_1d_test() {
        let v = vec![1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32];
        let harray = HArray::new_from_shape_vec(6, v.clone()).unwrap();
        let lhs = harray.fft().unwrap();
        let result = vec![
            Complex::new(21.0, 0.0),
            Complex::new(-3.0, 5.196152),
            Complex::new(-3.0, 1.732051),
            Complex::new(-3.0, 0.0),
            Complex::new(-3.0, -1.732051),
            Complex::new(-3.0, -5.196152),
        ];
        let rhs = HArray::new_from_shape_vec(6, result).unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));

        let mut harray = HArray::new_from_shape_vec(6, v).unwrap();
        let lhs = harray.rfft().unwrap();
        let result = vec![
            Complex::new(21.0, 0.0),
            Complex::new(-3.0, 5.196152),
            Complex::new(-3.0, 1.732051),
            Complex::new(-3.0, 0.0),
        ];
        let rhs = HArray::new_from_shape_vec(4, result).unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));

        assert_eq!(harray.fft_mut().is_err(), true);
        assert_eq!(harray.ifft().is_err(), true);
        assert_eq!(harray.ifft_mut().is_err(), true);
        assert_eq!(harray.irfft(1000).is_err(), true);
    }

    #[test]
    fn fft_float_1d_dyn_test() {
        let v = vec![1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32];
        let harray = HArray::new_from_shape_vec(6, v.clone())
            .unwrap()
            .into_dynamic();
        let lhs = harray.fft().unwrap().into_dynamic();
        let result = vec![
            Complex::new(21.0, 0.0),
            Complex::new(-3.0, 5.196152),
            Complex::new(-3.0, 1.732051),
            Complex::new(-3.0, 0.0),
            Complex::new(-3.0, -1.732051),
            Complex::new(-3.0, -5.196152),
        ];
        let rhs = HArray::new_from_shape_vec(6, result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray_complex(&lhs, &rhs));

        let mut harray = HArray::new_from_shape_vec(6, v).unwrap().into_dynamic();
        let lhs = harray.rfft().unwrap().into_dynamic();
        let result = vec![
            Complex::new(21.0, 0.0),
            Complex::new(-3.0, 5.196152),
            Complex::new(-3.0, 1.732051),
            Complex::new(-3.0, 0.0),
        ];
        let rhs = HArray::new_from_shape_vec(4, result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray_complex(&lhs, &rhs));

        assert_eq!(harray.fft_mut().is_err(), true);
        assert_eq!(harray.ifft().is_err(), true);
        assert_eq!(harray.ifft_mut().is_err(), true);
        assert_eq!(harray.irfft(1000).is_err(), true);
    }

    #[test]
    fn fft_float_2d_test() {
        let v = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32, 11_f32, 12_f32,
        ];
        let harray = HArray::new_from_shape_vec((3, 4), v.clone()).unwrap();
        let lhs = harray.fft().unwrap();
        let result = vec![
            Complex::new(10_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(26_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(42_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(-2_f32, -2_f32),
        ];
        let rhs = HArray::new_from_shape_vec((3, 4), result).unwrap();
        assert!(compare_harray_complex(&lhs, &rhs));

        let mut harray = HArray::new_from_shape_vec((3, 4), v).unwrap();
        let lhs = harray.rfft().unwrap();
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

        assert_eq!(harray.fft_mut().is_err(), true);
        assert_eq!(harray.ifft().is_err(), true);
        assert_eq!(harray.ifft_mut().is_err(), true);
        assert_eq!(harray.irfft(1000).is_err(), true);
    }

    #[test]
    fn fft_float_2d_dyn_test() {
        let v = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32, 11_f32, 12_f32,
        ];
        let harray = HArray::new_from_shape_vec((3, 4), v.clone())
            .unwrap()
            .into_dynamic();
        let lhs = harray.fft().unwrap().into_dynamic();
        let result = vec![
            Complex::new(10_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(26_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(-2_f32, -2_f32),
            Complex::new(42_f32, 0_f32),
            Complex::new(-2_f32, 2_f32),
            Complex::new(-2_f32, 0_f32),
            Complex::new(-2_f32, -2_f32),
        ];
        let rhs = HArray::new_from_shape_vec((3, 4), result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray_complex(&lhs, &rhs));

        let mut harray = HArray::new_from_shape_vec((3, 4), v)
            .unwrap()
            .into_dynamic();
        let lhs = harray.rfft().unwrap().into_dynamic();
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

        assert_eq!(harray.fft_mut().is_err(), true);
        assert_eq!(harray.ifft().is_err(), true);
        assert_eq!(harray.ifft_mut().is_err(), true);
        assert_eq!(harray.irfft(1000).is_err(), true);
    }
}
