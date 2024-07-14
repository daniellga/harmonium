use std::sync::Arc;

use harmonium_core::{array::HArray, errors::HError, errors::HResult};
use ndarray::{ArcArray1, ArcArray2, Axis, Dimension, Ix1, Ix2, IxDyn, Zip};
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};
use rustfft::{
    num_complex::Complex,
    num_traits::{Float, FloatConst},
    FftNum, FftPlanner,
};

#[derive(Clone)]
pub struct Fft<T> {
    pub fft: Arc<dyn rustfft::Fft<T>>,
    pub scratch_buffer: Arc<[Complex<T>]>,
}

#[derive(Clone)]
pub struct RealFftForward<T> {
    fft: Arc<dyn RealToComplex<T>>,
    scratch_buffer: Arc<[Complex<T>]>,
}

#[derive(Clone)]
pub struct RealFftInverse<T> {
    fft: Arc<dyn ComplexToReal<T>>,
    scratch_buffer: Arc<[Complex<T>]>,
}

impl<T: FftNum + Float + FloatConst> Fft<T> {
    pub fn new_fft_forward(length: usize) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(length);
        let scratch_len = fft.get_inplace_scratch_len();
        let zero = T::zero();
        let scratch_buffer = vec![Complex::new(zero, zero); scratch_len];
        let scratch_buffer: Arc<[Complex<T>]> = Arc::from(scratch_buffer);

        Self {
            fft,
            scratch_buffer,
        }
    }

    pub fn new_fft_inverse(length: usize) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_inverse(length);
        let scratch_len = fft.get_inplace_scratch_len();
        let zero = T::zero();
        let scratch_buffer = vec![Complex::new(zero, zero); scratch_len];
        let scratch_buffer: Arc<[Complex<T>]> = Arc::from(scratch_buffer);

        Self {
            fft,
            scratch_buffer,
        }
    }
}

impl<T: FftNum + Float + FloatConst> RealFftForward<T> {
    pub fn new_real_fft_forward(length: usize) -> Self {
        let mut planner = RealFftPlanner::new();
        let fft = planner.plan_fft_forward(length);
        let zero = T::zero();
        let scratch_len = fft.get_scratch_len();
        let scratch_buffer = vec![Complex::new(zero, zero); scratch_len];
        let scratch_buffer: Arc<[Complex<T>]> = Arc::from(scratch_buffer);

        Self {
            fft,
            scratch_buffer,
        }
    }
}

impl<T: FftNum + Float + FloatConst> RealFftInverse<T> {
    pub fn new_real_fft_inverse(length: usize) -> Self {
        let mut planner = RealFftPlanner::new();
        let fft = planner.plan_fft_inverse(length);
        let zero = T::zero();
        let scratch_len = fft.get_scratch_len();
        let scratch_buffer = vec![Complex::new(zero, zero); scratch_len];
        let scratch_buffer: Arc<[Complex<T>]> = Arc::from(scratch_buffer);

        Self {
            fft,
            scratch_buffer,
        }
    }
}

pub trait ProcessFft<T, D>
where
    T: FftNum + Float + FloatConst,
    D: Dimension,
{
    fn process(&mut self, harray: &mut HArray<Complex<T>, D>) -> HResult<()>;
}

pub trait ProcessRealFftForward<T, D>
where
    T: FftNum + Float + FloatConst,
    D: Dimension,
{
    fn process(&mut self, harray: &mut HArray<T, D>) -> HResult<HArray<Complex<T>, D>>;
}

pub trait ProcessRealFftInverse<T, D>
where
    T: FftNum + Float + FloatConst,
    D: Dimension,
{
    fn process(&mut self, harray: &mut HArray<Complex<T>, D>) -> HResult<HArray<T, D>>;
}

impl<T> ProcessFft<T, Ix1> for Fft<T>
where
    T: FftNum + Float + FloatConst,
{
    fn process(&mut self, harray: &mut HArray<Complex<T>, Ix1>) -> HResult<()> {
        let scratch_buffer = make_mut_slice(&mut self.scratch_buffer);
        self.fft
            .process_with_scratch(harray.as_slice_mut().unwrap(), scratch_buffer);
        Ok(())
    }
}

impl<T> ProcessRealFftForward<T, Ix1> for RealFftForward<T>
where
    T: FftNum + Float + FloatConst,
{
    fn process(&mut self, harray: &mut HArray<T, Ix1>) -> HResult<HArray<Complex<T>, Ix1>> {
        let zero = T::zero();
        let length = harray.len();
        let mut ndarray = ArcArray1::from_elem(length / 2 + 1, Complex::new(zero, zero));
        let scratch_buffer = make_mut_slice(&mut self.scratch_buffer);
        self.fft
            .process_with_scratch(
                harray.as_slice_mut().unwrap(),
                ndarray.as_slice_mut().unwrap(),
                scratch_buffer,
            )
            .unwrap();
        Ok(HArray(ndarray))
    }
}

impl<T> ProcessRealFftInverse<T, Ix1> for RealFftInverse<T>
where
    T: FftNum + Float + FloatConst,
{
    fn process(&mut self, harray: &mut HArray<Complex<T>, Ix1>) -> HResult<HArray<T, Ix1>> {
        let zero = T::zero();
        let length = self.fft.len();
        let scratch_buffer = make_mut_slice(&mut self.scratch_buffer);
        let mut ndarray = ArcArray1::from_elem(length, zero);
        self.fft
            .process_with_scratch(
                harray.as_slice_mut().unwrap(),
                ndarray.as_slice_mut().unwrap(),
                scratch_buffer,
            )
            .unwrap();
        Ok(HArray(ndarray))
    }
}

impl<T> ProcessFft<T, Ix2> for Fft<T>
where
    T: FftNum + Float + FloatConst,
{
    fn process(&mut self, harray: &mut HArray<Complex<T>, Ix2>) -> HResult<()> {
        let scratch_buffer = make_mut_slice(&mut self.scratch_buffer);

        Zip::from(harray.0.lanes_mut(Axis(1))).for_each(|mut row| {
            self.fft
                .process_with_scratch(row.as_slice_mut().unwrap(), scratch_buffer);
        });
        Ok(())
    }
}

impl<T> ProcessRealFftForward<T, Ix2> for RealFftForward<T>
where
    T: FftNum + Float + FloatConst,
{
    fn process(&mut self, harray: &mut HArray<T, Ix2>) -> HResult<HArray<Complex<T>, Ix2>> {
        let zero = T::zero();
        let nrows = harray.0.nrows();
        let ncols = harray.0.ncols();
        let scratch_buffer = make_mut_slice(&mut self.scratch_buffer);
        let mut ndarray = ArcArray2::from_elem((nrows, ncols / 2 + 1), Complex::new(zero, zero));

        Zip::from(ndarray.lanes_mut(Axis(1)))
            .and(harray.0.lanes_mut(Axis(1)))
            .for_each(|mut row_output, mut row_input| {
                self.fft
                    .process_with_scratch(
                        row_input.as_slice_mut().unwrap(),
                        row_output.as_slice_mut().unwrap(),
                        scratch_buffer,
                    )
                    .unwrap();
            });

        Ok(HArray(ndarray))
    }
}

impl<T> ProcessRealFftInverse<T, Ix2> for RealFftInverse<T>
where
    T: FftNum + Float + FloatConst,
{
    fn process(&mut self, harray: &mut HArray<Complex<T>, Ix2>) -> HResult<HArray<T, Ix2>> {
        let zero = T::zero();
        let length = self.fft.len();
        let nrows = harray.0.nrows();
        let scratch_buffer = make_mut_slice(&mut self.scratch_buffer);
        let mut ndarray = ArcArray2::from_elem((nrows, length), zero);

        Zip::from(ndarray.lanes_mut(Axis(1)))
            .and(harray.0.lanes_mut(Axis(1)))
            .for_each(|mut row_output, mut row_input| {
                self.fft
                    .process_with_scratch(
                        row_input.as_slice_mut().unwrap(),
                        row_output.as_slice_mut().unwrap(),
                        scratch_buffer,
                    )
                    .unwrap();
            });

        Ok(HArray(ndarray))
    }
}

impl<T> ProcessFft<T, IxDyn> for Fft<T>
where
    T: FftNum + Float + FloatConst,
{
    fn process(&mut self, harray: &mut HArray<Complex<T>, IxDyn>) -> HResult<()> {
        let scratch_buffer = make_mut_slice(&mut self.scratch_buffer);
        match harray.ndim() {
            1 => {
                self.fft
                    .process_with_scratch(harray.as_slice_mut().unwrap(), scratch_buffer);
                Ok(())
            }
            2 => {
                Zip::from(harray.0.lanes_mut(Axis(1))).for_each(|mut row| {
                    self.fft
                        .process_with_scratch(row.as_slice_mut().unwrap(), scratch_buffer);
                });
                Ok(())
            }
            _ => Err(HError::OutOfSpecError(
                "The HArray's ndim should be 1 or 2.".into(),
            )),
        }
    }
}

impl<T> ProcessRealFftForward<T, IxDyn> for RealFftForward<T>
where
    T: FftNum + Float + FloatConst,
{
    fn process(&mut self, harray: &mut HArray<T, IxDyn>) -> HResult<HArray<Complex<T>, IxDyn>> {
        let zero = T::zero();
        let scratch_buffer = make_mut_slice(&mut self.scratch_buffer);
        match harray.ndim() {
            1 => {
                let length = harray.len();
                let mut ndarray = ArcArray1::from_elem(length / 2 + 1, Complex::new(zero, zero));
                self.fft
                    .process_with_scratch(
                        harray.as_slice_mut().unwrap(),
                        ndarray.as_slice_mut().unwrap(),
                        scratch_buffer,
                    )
                    .unwrap();
                Ok(HArray(ndarray.into_dyn()))
            }
            2 => {
                let nrows = harray.0.len_of(Axis(0));
                let ncols = harray.0.len_of(Axis(1));
                let mut ndarray =
                    ArcArray2::from_elem((nrows, ncols / 2 + 1), Complex::new(zero, zero))
                        .into_dyn();

                Zip::from(ndarray.lanes_mut(Axis(1)))
                    .and(harray.0.lanes_mut(Axis(1)))
                    .for_each(|mut row_output, mut row_input| {
                        self.fft
                            .process_with_scratch(
                                row_input.as_slice_mut().unwrap(),
                                row_output.as_slice_mut().unwrap(),
                                scratch_buffer,
                            )
                            .unwrap();
                    });

                Ok(HArray(ndarray))
            }
            _ => Err(HError::OutOfSpecError(
                "The HArray's ndim should be 1 or 2.".into(),
            )),
        }
    }
}

impl<T> ProcessRealFftInverse<T, IxDyn> for RealFftInverse<T>
where
    T: FftNum + Float + FloatConst,
{
    fn process(&mut self, harray: &mut HArray<Complex<T>, IxDyn>) -> HResult<HArray<T, IxDyn>> {
        let zero = T::zero();
        let length = self.fft.len();
        let scratch_buffer = make_mut_slice(&mut self.scratch_buffer);
        match harray.ndim() {
            1 => {
                let mut ndarray = ArcArray1::from_elem(length, zero);
                self.fft
                    .process_with_scratch(
                        harray.as_slice_mut().unwrap(),
                        ndarray.as_slice_mut().unwrap(),
                        scratch_buffer,
                    )
                    .unwrap();
                Ok(HArray(ndarray.into_dyn()))
            }
            2 => {
                let nrows = harray.0.len_of(Axis(0));
                let mut ndarray = ArcArray2::from_elem((nrows, length), zero).into_dyn();

                Zip::from(ndarray.lanes_mut(Axis(1)))
                    .and(harray.0.lanes_mut(Axis(1)))
                    .for_each(|mut row_output, mut row_input| {
                        self.fft
                            .process_with_scratch(
                                row_input.as_slice_mut().unwrap(),
                                row_output.as_slice_mut().unwrap(),
                                scratch_buffer,
                            )
                            .unwrap();
                    });

                Ok(HArray(ndarray))
            }
            _ => Err(HError::OutOfSpecError(
                "The HArray's ndim should be 1 or 2.".into(),
            )),
        }
    }
}

// replace this function by make_mut when in stable (it is currently, but doesn't work for slices.)
fn make_mut_slice<T: Clone>(arc: &mut Arc<[T]>) -> &mut [T] {
    if Arc::get_mut(arc).is_none() {
        *arc = Arc::from(&arc[..]);
    }
    // Replace by get_mut_unchecked when available in stable. This can't failed since get_mut was
    // checked above.
    unsafe { Arc::get_mut(arc).unwrap_unchecked() }
}

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
        let length = lhs.len();
        let mut fft = Fft::new_fft_forward(length);
        fft.process(&mut lhs).unwrap();
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
        let ncols = lhs.0.ncols();
        let mut fft = Fft::new_fft_forward(ncols);
        fft.process(&mut lhs).unwrap();
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
        let ncols = lhs.0.len_of(Axis(1));
        let mut fft = Fft::new_fft_forward(ncols);
        fft.process(&mut lhs).unwrap();
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
        let length = lhs.len();
        let mut fft = Fft::new_fft_inverse(length);
        fft.process(&mut lhs).unwrap();
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
        let ncols = lhs.0.ncols();
        let mut fft = Fft::new_fft_forward(ncols);
        fft.process(&mut lhs).unwrap();
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
        let ncols = lhs.0.len_of(Axis(1));
        let mut fft = Fft::new_fft_forward(ncols);
        fft.process(&mut lhs).unwrap();
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
        let length = harray.len();
        let mut rfft = RealFftForward::new_real_fft_forward(length);
        let lhs = rfft.process(&mut harray).unwrap();
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
        let ncols = harray.0.ncols();
        let mut rfft = RealFftForward::new_real_fft_forward(ncols);
        let lhs = rfft.process(&mut harray).unwrap();
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
        let ncols = harray.0.len_of(Axis(1));
        let mut rfft = RealFftForward::new_real_fft_forward(ncols);
        let lhs = rfft.process(&mut harray).unwrap();
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
        let mut rfft = RealFftForward::new_real_fft_forward(length);
        let mut spectrum = rfft.process(&mut harray).unwrap();
        let mut irfft = RealFftInverse::new_real_fft_inverse(length);
        let lhs = irfft.process(&mut spectrum).unwrap();
        let result = vec![6_f32, 12., 18., 24., 30., 36.];
        let rhs = HArray::new_from_shape_vec(length, result).unwrap();
        assert!(compare_harray(&lhs, &rhs));

        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec(6, v).unwrap();
        let length = harray.len() + 1;
        let mut rfft = RealFftForward::new_real_fft_forward(length - 1);
        let mut spectrum = rfft.process(&mut harray).unwrap();
        let mut irfft = RealFftInverse::new_real_fft_inverse(length);
        let lhs = irfft.process(&mut spectrum).unwrap();
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
        let mut rfft = RealFftForward::new_real_fft_forward(length);
        let mut spectrum = rfft.process(&mut harray).unwrap();
        let mut irfft = RealFftInverse::new_real_fft_inverse(length);
        let lhs = irfft.process(&mut spectrum).unwrap();
        let result = vec![2., 4., 6., 8., 10., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result).unwrap();
        assert!(compare_harray(&lhs, &rhs));

        let v = vec![1_f32, 2., 3., 4., 5., 6.];
        let mut harray = HArray::new_from_shape_vec((3, 2), v).unwrap();
        let length = harray.0.ncols() + 1;
        let mut rfft = RealFftForward::new_real_fft_forward(length - 1);
        let mut spectrum = rfft.process(&mut harray).unwrap();
        let mut irfft = RealFftInverse::new_real_fft_inverse(length);
        let lhs = irfft.process(&mut spectrum).unwrap();
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
        let mut rfft = RealFftForward::new_real_fft_forward(length);
        let mut spectrum = rfft.process(&mut harray).unwrap();
        let mut irfft = RealFftInverse::new_real_fft_inverse(length);
        let lhs = irfft.process(&mut spectrum).unwrap();
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
        let mut rfft = RealFftForward::new_real_fft_forward(length - 1);
        let mut spectrum = rfft.process(&mut harray).unwrap();
        let mut irfft = RealFftInverse::new_real_fft_inverse(length);
        let lhs = irfft.process(&mut spectrum).unwrap();
        let result = vec![1., 4., 4., 5., 8., 8., 9., 12., 12.];
        let rhs = HArray::new_from_shape_vec((3, length), result)
            .unwrap()
            .into_dynamic();
        assert!(compare_harray(&lhs, &rhs));
    }
}
