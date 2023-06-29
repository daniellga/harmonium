#[cfg(feature = "arrow")]
pub mod fft_arrow {
    use arrow2::{array::PrimitiveArray, types::NativeType};
    use cached::proc_macro::cached;
    use harmonium_core::errors::HResult;
    use harmonium_core::structs::{HComplexArray, HComplexMatrix, HFloatArray, HFloatMatrix};
    use rustfft::{num_complex::Complex, num_traits::Float, Fft, FftPlanner};
    use std::sync::Arc;

    pub trait FftComplexArray {
        fn fft(&self) -> Self;
    }

    pub trait FftFloatArray<T: NativeType + Float> {
        fn fft(&self) -> HComplexArray<T>;
    }

    pub trait FftComplexMatrix<T: NativeType + Float> {
        fn fft(&self) -> HResult<HComplexMatrix<T>>;
    }

    pub trait FftFloatMatrix<T: NativeType + Float> {
        fn fft(&self) -> HResult<HComplexMatrix<T>>;
    }

    macro_rules! impl_fft_complex_array {
        ($(($t: ty, $f: ident)),+) => {
            $(
                impl FftComplexArray for HComplexArray<$t> {
                    fn fft(&self) -> Self {
                        let length = self.inner().len();
                        let (fft, mut new_vec) = $f(length);
                        let mut vec: Vec<Complex<$t>> = self
                            .inner()
                            .values()
                            .chunks_exact(2)
                            .map(|x| Complex::<$t>::new(x[0], x[1]))
                            .collect();

                        fft.process(&mut vec);

                        for i in vec.iter() {
                            new_vec.push(i.re);
                            new_vec.push(i.im);
                        }
                        let new_arr = PrimitiveArray::<$t>::from_vec(new_vec);
                        HComplexArray::<$t>::new(new_arr)
                    }
                }

                #[cached]
                pub fn $f(length: usize) -> (Arc<dyn Fft<$t>>, Vec<$t>) {
                    let mut planner = FftPlanner::<$t>::new();
                    let new_vec = Vec::with_capacity(length);
                    (planner.plan_fft_forward(length / 2), new_vec)
                }
            )+
        };
    }

    impl_fft_complex_array!(
        (f32, create_complex32_array_planner),
        (f64, create_complex64_array_planner)
    );

    macro_rules! impl_fft_float_array {
        ($(($t: ty, $f: ident)),+) => {
            $(
                impl FftFloatArray<$t> for HFloatArray<$t> {
                    fn fft(&self) -> HComplexArray<$t> {
                        let length = self.inner().len();
                        let (fft, mut new_vec) = $f(length);
                        let mut vec: Vec<Complex<$t>> = self
                            .inner()
                            .values()
                            .iter()
                            .map(|x| Complex::<$t>::new(*x, 0.))
                            .collect();

                        fft.process(&mut vec);

                        for i in vec.iter() {
                            new_vec.push(i.re);
                            new_vec.push(i.im);
                        }
                        let new_arr = PrimitiveArray::<$t>::from_vec(new_vec);
                        HComplexArray::<$t>::new(new_arr)
                    }
                }

                #[cached]
                pub fn $f(length: usize) -> (Arc<dyn Fft<$t>>, Vec<$t>) {
                    let mut planner = FftPlanner::<$t>::new();
                    let new_vec = Vec::with_capacity(2 * length);
                    (planner.plan_fft_forward(length), new_vec)
                }
            )+
        };
    }

    impl_fft_float_array!(
        (f32, create_float32_array_planner),
        (f64, create_float64_array_planner)
    );

    macro_rules! impl_fft_complex_matrix {
        ($(($t: ty, $f: ident)),+) => {
            $(
                impl FftComplexMatrix<$t> for HComplexMatrix<$t> {
                    fn fft(&self) -> HResult<Self> {
                        let length = self.len();
                        let nrows = self.nrows();
                        let child_len = nrows * 2;
                        let (fft, mut new_vec) = $f(length, child_len);
                        let mut vec: Vec<Complex<$t>> = self
                            .inner
                            .inner
                            .values()
                            .chunks_exact(2)
                            .map(|x| Complex::<$t>::new(x[0], x[1]))
                            .collect();

                        for v in vec.chunks_exact_mut(nrows) {
                            fft.process(v);
                        }

                        for i in vec.iter() {
                            new_vec.push(i.re);
                            new_vec.push(i.im);
                        }

                        let hmatrix = HComplexMatrix::<$t>::new_from_vec(new_vec, self.ncols())?;
                        Ok(hmatrix)
                    }
                }

            #[cached]
            pub fn $f(length: usize, child_len: usize) -> (Arc<dyn Fft<$t>>, Vec<$t>) {
                let mut planner = FftPlanner::<$t>::new();
                let new_vec = Vec::with_capacity(length);
                (planner.plan_fft_forward(child_len / 2), new_vec)
            }
            )+
        };
    }

    impl_fft_complex_matrix!(
        (f32, create_complex32_matrix_planner),
        (f64, create_complex64_matrix_planner)
    );

    macro_rules! impl_fft_float_matrix {
        ($(($t: ty, $f: ident)),+) => {
            $(
                impl FftFloatMatrix<$t> for HFloatMatrix<$t> {
                    fn fft(&self) -> HResult<HComplexMatrix<$t>> {
                        let length = self.len();
                        let child_len = self.nrows();
                        let (fft, mut new_vec) = $f(length, child_len);
                        let mut vec: Vec<Complex<$t>> = self
                            .inner
                            .inner
                            .values()
                            .iter()
                            .map(|x| Complex::<$t>::new(*x, 0.))
                            .collect();

                        for v in vec.chunks_exact_mut(child_len) {
                            fft.process(v);
                        }

                        for i in vec.iter() {
                            new_vec.push(i.re);
                            new_vec.push(i.im);
                        }

                        let hmatrix = HComplexMatrix::<$t>::new_from_vec(new_vec, self.ncols())?;
                        Ok(hmatrix)
                    }
                }

            #[cached]
            pub fn $f(length: usize, child_len: usize) -> (Arc<dyn Fft<$t>>, Vec<$t>) {
                let mut planner = FftPlanner::<$t>::new();
                let new_vec = Vec::with_capacity(length * 2);
                (planner.plan_fft_forward(child_len), new_vec)
            }
            )+
        };
    }

    impl_fft_float_matrix!(
        (f32, create_float32_matrix_planner),
        (f64, create_float64_matrix_planner)
    );

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fft_complex_array_test() {
            let v = vec![1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32];
            let lhs = HComplexArray::new_from_vec(v).fft();
            let v = vec![16_f32, 20_f32, -8_f32, 0_f32, -4_f32, -4_f32, 0_f32, -8.];
            let rhs = HComplexArray::new_from_vec(v);
            assert_eq!(lhs, rhs);

            let v = vec![1., 2., 3., 4., 5., 6., 7., 8.];
            let lhs = HComplexArray::new_from_vec(v).fft();
            let v = vec![16., 20., -8., 0., -4., -4., 0., -8.];
            let rhs = HComplexArray::new_from_vec(v);
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn fft_float_array_test() {
            let v = vec![1_f32, 2_f32, 3_f32, 4_f32];
            let lhs = HFloatArray::new_from_vec(v).fft();
            let v = vec![10_f32, 0_f32, -2_f32, 2_f32, -2_f32, 0_f32, -2_f32, -2.];
            let rhs = HComplexArray::new_from_vec(v);
            assert_eq!(lhs, rhs);

            let v = vec![1., 2., 3., 4.];
            let lhs = HFloatArray::new_from_vec(v).fft();
            let v = vec![10., 0., -2., 2., -2., 0., -2., -2.];
            let rhs = HComplexArray::new_from_vec(v);
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn fft_complex_matrix_test() {
            let v = vec![
                1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32, 11_f32,
                12_f32,
            ];
            let hmatrix = HComplexMatrix::new_from_vec(v, 3).unwrap();
            let lhs = hmatrix.fft().unwrap();
            let v = vec![
                4_f32, 6_f32, -2_f32, -2_f32, 12_f32, 14_f32, -2_f32, -2_f32, 20_f32, 22_f32,
                -2_f32, -2_f32,
            ];
            let rhs = HComplexMatrix::new_from_vec(v, 3).unwrap();
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn fft_float_matrix_test() {
            let v = vec![
                1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32, 11_f32,
                12_f32,
            ];
            let hmatrix = HFloatMatrix::new_from_vec(v, 3).unwrap();
            let lhs = hmatrix.fft().unwrap();
            let v = vec![
                10_f32, 0_f32, -2_f32, 2_f32, -2_f32, 0_f32, -2_f32, -2_f32, 26_f32, 0_f32, -2_f32,
                2_f32, -2_f32, 0_f32, -2_f32, -2_f32, 42_f32, 0_f32, -2_f32, 2_f32, -2_f32, 0_f32,
                -2_f32, -2_f32,
            ];
            let rhs = HComplexMatrix::new_from_vec(v, 3).unwrap();
            assert_eq!(lhs, rhs);
        }
    }
}
