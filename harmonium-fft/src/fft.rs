use cached::proc_macro::cached;
use harmonium_core::array::HArray;
use ndarray::{ArcArray1, ArcArray2, Axis, Dimension, Ix1, Ix2, IxDyn, Zip};
use rustfft::{
    num_complex::Complex,
    num_traits::{Float, FloatConst},
    Fft, FftPlanner,
};
use std::sync::Arc;

pub trait FftComplex {
    fn fft(&self) -> Self;
    fn fft_mut(&mut self);
}

pub trait FftFloat<T, D>
where
    T: Float + FloatConst,
    D: Dimension,
{
    fn fft(&self) -> HArray<Complex<T>, D>;
}

macro_rules! impl_fft {
    ($(($t: ty, $f1: ident, $f2: ident, $f3: ident)),* $(,)?) => {
        $(
            impl FftComplex for HArray<Complex<$t>, Ix1> {
                fn fft(&self) -> Self {
                    let length = self.len();
                    let (fft, mut ndarray) = $f1(length);
                    ndarray.assign(&self.0);

                    fft.process(ndarray.as_slice_mut().unwrap());

                    HArray(ndarray)
                }

                fn fft_mut(&mut self) {
                    let length = self.len();
                    let fft = $f3(length);

                    fft.process(self.as_slice_mut().unwrap());
                }
            }

            impl FftComplex for HArray<Complex<$t>, Ix2> {
                fn fft(&self) -> Self {
                    let nrows = self.0.nrows();
                    let ncols = self.0.ncols();
                    let (fft, mut ndarray) = $f2(nrows, ncols);
                    ndarray.assign(&self.0);

                    Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });

                    HArray(ndarray)
                }

                fn fft_mut(&mut self) {
                    let ncols = self.0.ncols();
                    let fft = $f3(ncols);

                    Zip::from(self.0.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });
                }
            }

            impl FftComplex for HArray<Complex<$t>, IxDyn> {
                fn fft(&self) -> Self {
                    assert!(self.ndim() <= 2);
                    let nrows = self.0.len_of(Axis(0));
                    let ncols = self.0.len_of(Axis(1));
                    let (fft, mut ndarray) = $f2(nrows, ncols);
                    ndarray.assign(&self.0);

                    Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });

                    HArray(ndarray.into_dyn())
                }

                fn fft_mut(&mut self) {
                    let ncols = self.0.len_of(Axis(1));
                    let fft = $f3(ncols);

                    Zip::from(self.0.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });
                }
            }

            impl FftFloat<$t, Ix1> for HArray<$t, Ix1> {
                fn fft(&self) -> HArray<Complex<$t>, Ix1> {
                    let length = self.len();
                    let (fft, mut ndarray) = $f1(length);

                    ndarray.zip_mut_with(&self.0, |x, y| *x = Complex::new(*y, 0.));

                    fft.process(ndarray.as_slice_mut().unwrap());

                    HArray(ndarray)
                }
            }

            impl FftFloat<$t, Ix2> for HArray<$t, Ix2> {
                fn fft(&self) -> HArray<Complex<$t>, Ix2> {
                    let nrows = self.0.nrows();
                    let ncols = self.0.ncols();
                    let (fft, mut ndarray) = $f2(nrows, ncols);

                    ndarray.zip_mut_with(&self.0, |x, y| *x = Complex::new(*y, 0.));

                    Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });

                    HArray(ndarray)
                }
            }

            impl FftFloat<$t, IxDyn> for HArray<$t, IxDyn> {
                fn fft(&self) -> HArray<Complex<$t>, IxDyn> {
                    let nrows = self.0.len_of(Axis(0));
                    let ncols = self.0.len_of(Axis(1));
                    let (fft, mut ndarray) = $f2(nrows, ncols);

                    ndarray.zip_mut_with(&self.0, |x, y| *x = Complex::new(*y, 0.));

                    Zip::from(ndarray.lanes_mut(Axis(1))).for_each(|mut row| {
                        fft.process(row.as_slice_mut().unwrap());
                    });

                    HArray(ndarray.into_dyn())
                }
            }

            #[cached]
            fn $f1(length: usize) -> (Arc<dyn Fft<$t>>, ArcArray1<Complex<$t>>) {
                let mut planner = FftPlanner::<$t>::new();
                let ndarray = ArcArray1::zeros(length);
                (planner.plan_fft_forward(length), ndarray)
            }

            #[cached]
            fn $f2(
                nchannels: usize,
                nframes: usize,
            ) -> (Arc<dyn Fft<$t>>, ArcArray2<Complex<$t>>) {
                let mut planner = FftPlanner::<$t>::new();
                let ndarray = ArcArray2::zeros((nchannels, nframes));
                (planner.plan_fft_forward(nframes), ndarray)
            }

            #[cached]
            fn $f3(length: usize) -> Arc<dyn Fft<$t>> {
                let mut planner = FftPlanner::<$t>::new();
                planner.plan_fft_forward(length)
            }
            )+
    };
}

impl_fft!(
    (
        f32,
        create_planner_1d_f32,
        create_planner_12_f32,
        creat_planner_mut_f32
    ),
    (
        f64,
        create_planner_1d_f64,
        create_planner_12_f64,
        creat_planner_mut_f64
    ),
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fft_complex_test() {
        let v = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let harray = HArray::new_from_shape_vec((3, 2), v.clone()).unwrap();
        let lhs = harray.fft();
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

        let mut lhs = HArray::new_from_shape_vec((3, 2), v).unwrap();
        lhs.fft_mut();

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn fft_float_test() {
        let v = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32, 11_f32, 12_f32,
        ];
        let harray = HArray::new_from_shape_vec((3, 4), v).unwrap();
        let lhs = harray.fft();
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
        assert_eq!(lhs, rhs);
    }
}
