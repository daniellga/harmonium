use crate::errors::{HError, HResult};
use arrow2::{
    array::{Array, PrimitiveArray},
    datatypes::Field,
    ffi,
    types::NativeType,
};
use ndarray::ArrayView2;
use num_traits::{Float, FromPrimitive};

#[derive(Debug, PartialEq, Clone)]
pub struct HFloatArray<T: NativeType + Float> {
    pub inner: PrimitiveArray<T>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HComplexArray<T: NativeType + Float> {
    pub inner: PrimitiveArray<T>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HFloatMatrix<T: NativeType + Float> {
    pub inner: HFloatArray<T>,
    pub ncols: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HComplexMatrix<T: NativeType + Float> {
    pub inner: HComplexArray<T>,
    pub ncols: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HFloatAudio<T: NativeType + Float> {
    pub inner: HFloatMatrix<T>,
    pub sr: u32,
}

impl<T> HFloatArray<T>
where
    T: NativeType + Float,
{
    pub fn new(inner: PrimitiveArray<T>) -> HFloatArray<T> {
        HFloatArray { inner }
    }

    pub fn new_from_vec(v: Vec<T>) -> HFloatArray<T> {
        let inner = PrimitiveArray::from_vec(v);
        HFloatArray { inner }
    }

    pub fn inner(&self) -> &PrimitiveArray<T> {
        &self.inner
    }

    /// Convert to HMatrix.
    pub fn into_hmatrix(self, ncols: usize) -> HResult<HFloatMatrix<T>> {
        Ok(HFloatMatrix::new(self, ncols)?)
    }

    /// Returns the length of this HArray.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the HArray is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Slices in-place by an offset and a length.
    /// This operation is O(1).
    pub fn slice(&mut self, offset: usize, length: usize) {
        self.inner.slice(offset, length);
    }

    /// Gets the underlying slice.
    pub fn as_slice(&self) -> &[T] {
        self.inner.values().as_slice()
    }

    /// Returns an option of a mutable reference to the values
    pub fn get_mut_values(&mut self) -> Option<&mut [T]> {
        self.inner.get_mut_values()
    }

    /// Export the underlying array to Arrow C interface.
    pub fn export_c_arrow(&self) -> (ffi::ArrowArray, ffi::ArrowSchema) {
        let array = self.inner.clone().boxed();

        // importing an array requires an associated field so that the consumer knows its datatype.
        // Thus, we need to export both
        let field = Field::new("a", array.data_type().clone(), true);
        (
            ffi::export_array_to_c(array),
            ffi::export_field_to_c(&field),
        )
    }
}

impl<T> HComplexArray<T>
where
    T: NativeType + Float,
{
    pub fn new(inner: PrimitiveArray<T>) -> HComplexArray<T> {
        HComplexArray { inner }
    }

    pub fn new_from_vec(v: Vec<T>) -> HComplexArray<T> {
        let inner = PrimitiveArray::from_vec(v);
        HComplexArray { inner }
    }

    pub fn inner(&self) -> &PrimitiveArray<T> {
        &self.inner
    }

    /// Convert to HMatrix.
    pub fn into_hmatrix(self, ncols: usize) -> HResult<HComplexMatrix<T>> {
        Ok(HComplexMatrix::new(self, ncols)?)
    }

    /// Returns the length of this Harray.
    pub fn len(&self) -> usize {
        self.inner.len() / 2
    }

    /// Returns true if the HArray is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns a clone of this HArray sliced by an offset and length.
    /// This operation is O(1).
    pub fn slice(&mut self, offset: usize, length: usize) {
        self.inner.slice(offset * 2, length * 2);
    }

    /// Gets the underlying slice.
    pub fn as_slice(&self) -> &[T] {
        self.inner.values().as_slice()
    }

    /// Returns an option of a mutable reference to the values
    pub fn get_mut_values(&mut self) -> Option<&mut [T]> {
        self.inner.get_mut_values()
    }

    /// Export the underlying array to Arrow C interface.
    pub fn export_c_arrow(&self) -> (ffi::ArrowArray, ffi::ArrowSchema) {
        let array = self.inner.clone().boxed();

        // importing an array requires an associated field so that the consumer knows its datatype.
        // Thus, we need to export both
        let field = Field::new("a", array.data_type().clone(), true);
        (
            ffi::export_array_to_c(array),
            ffi::export_field_to_c(&field),
        )
    }
}

impl<T: NativeType + Float> HFloatMatrix<T> {
    pub fn new(inner: HFloatArray<T>, ncols: usize) -> HResult<HFloatMatrix<T>> {
        if inner.len() % ncols != 0 {
            return Err(HError::OutOfSpecError(
                "The HFloatArray's length must be a multiple of ncols.".into(),
            ));
        }
        Ok(HFloatMatrix { inner, ncols })
    }

    pub fn new_from_vec(v: Vec<T>, ncols: usize) -> HResult<HFloatMatrix<T>> {
        let inner = HFloatArray::new_from_vec(v);
        Ok(HFloatMatrix::new(inner, ncols)?)
    }

    /// Gets a reference to the inner HArray.
    pub fn inner(&self) -> &HFloatArray<T> {
        &self.inner
    }

    /// Number of columns.
    pub fn ncols(&self) -> usize {
        self.ncols
    }

    /// Number of rows.
    pub fn nrows(&self) -> usize {
        self.len() / self.ncols
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the HMatrix is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns a clone of this HMatrix sliced by an offset and length in the columns dimension.
    /// This operation is O(1).
    pub fn slice(&mut self, offset: usize, length: usize) {
        let nrows = self.nrows();
        self.inner.slice(offset * nrows, length * nrows);
        self.ncols = length;
    }

    /// Gets the underlying slice.
    pub fn as_slice(&self) -> &[T] {
        self.inner.inner.values().as_slice()
    }

    /// Returns an option of a mutable reference to the values
    pub fn get_mut_values(&mut self) -> Option<&mut [T]> {
        self.inner.get_mut_values()
    }

    /// Convert to HFloatArray.
    pub fn into_harray(self) -> HFloatArray<T> {
        self.inner
    }

    /// Convert to HFloatAudio.
    pub fn into_haudio(self, sr: u32) -> HFloatAudio<T> {
        HFloatAudio::new(self, sr)
    }

    /// Take the average across columns. A new inner array is created.
    pub fn mean_cols(&self) -> HResult<HFloatMatrix<T>>
    where
        T: FromPrimitive,
    {
        let arrayview2 = ArrayView2::from(self);

        let array1 = arrayview2
            .mean_axis(ndarray::Axis(0))
            .ok_or_else(|| HError::OutOfSpecError("The length of the axis is zero.".into()))?;

        Ok(array1.into())
    }

    /// Export the underlying array to Arrow C interface.
    pub fn export_c_arrow(&self) -> (ffi::ArrowArray, ffi::ArrowSchema) {
        let array = self.inner.inner.clone().boxed();

        // importing an array requires an associated field so that the consumer knows its datatype.
        // Thus, we need to export both
        let field = Field::new("a", array.data_type().clone(), true);
        (
            ffi::export_array_to_c(array),
            ffi::export_field_to_c(&field),
        )
    }
}

impl<T: NativeType + Float> HComplexMatrix<T> {
    pub fn new(inner: HComplexArray<T>, ncols: usize) -> HResult<HComplexMatrix<T>> {
        if inner.len() % ncols != 0 {
            return Err(HError::OutOfSpecError(
                "The HComplexArray's length must be a multiple of ncols.".into(),
            ));
        }
        Ok(HComplexMatrix { inner, ncols })
    }

    pub fn new_from_vec(v: Vec<T>, ncols: usize) -> HResult<HComplexMatrix<T>> {
        let inner = HComplexArray::new_from_vec(v);
        Ok(HComplexMatrix::new(inner, ncols)?)
    }

    /// Gets a reference to the inner HArray.
    pub fn inner(&self) -> &HComplexArray<T> {
        &self.inner
    }

    /// Number of columns.
    pub fn ncols(&self) -> usize {
        self.ncols
    }

    /// Number of rows.
    pub fn nrows(&self) -> usize {
        self.len() / self.ncols
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the HMatrix is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns a clone of this HMatrix sliced by an offset and length in the columns dimension.
    /// This operation is O(1).
    pub fn slice(&mut self, offset: usize, length: usize) {
        let nrows = self.nrows();
        self.inner.slice(offset * nrows, length * nrows);
        self.ncols = length;
    }

    /// Gets the underlying slice.
    pub fn as_slice(&self) -> &[T] {
        self.inner.inner.values().as_slice()
    }

    /// Returns an option of a mutable reference to the values
    pub fn get_mut_values(&mut self) -> Option<&mut [T]> {
        self.inner.get_mut_values()
    }

    /// Convert to HFloatArray.
    pub fn into_harray(self) -> HComplexArray<T> {
        self.inner
    }

    /// Take the average across columns. A new inner array is created.
    pub fn mean_cols(&self) -> HResult<HComplexMatrix<T>>
    where
        T: FromPrimitive,
    {
        let arrayview2 = ArrayView2::from(self);

        let array1 = arrayview2
            .mean_axis(ndarray::Axis(0))
            .ok_or_else(|| HError::OutOfSpecError("The length of the axis is zero.".into()))?;

        Ok(array1.into())
    }

    /// Export the underlying array to Arrow C interface.
    pub fn export_c_arrow(&self) -> (ffi::ArrowArray, ffi::ArrowSchema) {
        let array = self.inner.inner.clone().boxed();

        // importing an array requires an associated field so that the consumer knows its datatype.
        // Thus, we need to export both
        let field = Field::new("a", array.data_type().clone(), true);
        (
            ffi::export_array_to_c(array),
            ffi::export_field_to_c(&field),
        )
    }
}

impl<T: NativeType + Float> HFloatAudio<T> {
    /// Create a new instance.
    /// # Arguments
    ///
    /// * `inner` - An HMatrix representing the decoded audio.
    /// * `sr` - The sampling rate.
    pub fn new(inner: HFloatMatrix<T>, sr: u32) -> HFloatAudio<T> {
        HFloatAudio { inner, sr }
    }

    /// Acess the underlying data.
    pub fn inner(&self) -> &HFloatMatrix<T> {
        &self.inner
    }

    /// Get the sampling rate.
    pub fn sr(&self) -> u32 {
        self.sr
    }

    /// Get the number of channels.
    pub fn nchannels(&self) -> usize {
        self.inner.ncols()
    }

    /// Get the number of frames (number of samples per channel).
    pub fn nframes(&self) -> usize {
        self.inner.nrows()
    }

    /// Get the number of samples.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the HMatrix is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get the audio duration from a HFloatAudio in seconds.
    pub fn duration(&self) -> f64 {
        self.nframes() as f64 / self.sr() as f64
    }

    /// Convert to 1 channel taking the average across channels. A new inner array is created.
    pub fn as_mono(&mut self) -> HResult<()>
    where
        T: FromPrimitive,
    {
        self.inner = self.inner.mean_cols()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let harray: HFloatArray<f64> =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let channels = 3_usize;
        let hmatrix = harray.into_hmatrix(channels).unwrap();
        let haudio = HFloatAudio::new(hmatrix, 44000);

        let harray =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let rhs = HFloatMatrix::new(harray, channels).unwrap();

        assert_eq!(*haudio.inner(), rhs);
        assert_eq!(haudio.sr(), 44000);
        assert_eq!(haudio.nframes(), 4);
        assert_eq!(haudio.nchannels(), 3);
    }

    #[test]
    fn eq_test() {
        let channels = 3_usize;

        let harray: HFloatArray<f64> =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let harray2: HFloatArray<f64> =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        assert_eq!(harray, harray2);

        let hmatrix = harray.into_hmatrix(channels).unwrap();
        let hmatrix2 = harray2.into_hmatrix(channels).unwrap();
        assert_eq!(hmatrix, hmatrix2);

        let haudio = HFloatAudio::new(hmatrix, 44000);
        let haudio2 = HFloatAudio::new(hmatrix2, 44000);
        assert_eq!(haudio, haudio2);
    }

    #[test]
    fn slice_test() {
        let mut harray: HFloatArray<f32> =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let result = HFloatArray::new_from_vec(vec![4., 5.]);
        harray.slice(3, 2);
        assert_eq!(harray, result);

        let mut harray: HComplexArray<f32> =
            HComplexArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let result = HComplexArray::new_from_vec(vec![7., 8., 9., 10.]);
        harray.slice(3, 2);
        assert_eq!(harray, result);

        let harray: HFloatArray<f32> =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let ncols = 3_usize;
        let mut hmatrix = harray.into_hmatrix(ncols).unwrap();
        let harray: HFloatArray<f32> = HFloatArray::new_from_vec(vec![5., 6., 7., 8.]);
        let ncols = 1_usize;
        let result = harray.into_hmatrix(ncols).unwrap();
        hmatrix.slice(1, 1);
        assert_eq!(hmatrix, result);

        let harray: HComplexArray<f32> =
            HComplexArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let ncols = 3_usize;
        let mut hmatrix = harray.into_hmatrix(ncols).unwrap();
        let harray: HComplexArray<f32> = HComplexArray::new_from_vec(vec![5., 6., 7., 8.]);
        let ncols = 1_usize;
        let result = harray.into_hmatrix(ncols).unwrap();
        hmatrix.slice(1, 1);
        assert_eq!(hmatrix, result);
    }

    #[test]
    fn mean_cols_test() {
        let harray: HFloatArray<f64> =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let ncols = 3_usize;
        let hmatrix = harray.into_hmatrix(ncols).unwrap();
        let hmatrix = hmatrix.mean_cols().unwrap();

        let harray_result: HFloatArray<f64> = HFloatArray::new_from_vec(vec![
            (1. + 5. + 9.) / 3.,
            (2. + 6. + 10.) / 3.,
            (3. + 7. + 11.) / 3.,
            (4. + 8. + 12.) / 3.,
        ]);
        let hmatrix_result = harray_result.into_hmatrix(1).unwrap();
        assert_eq!(hmatrix, hmatrix_result);

        let harray: HComplexArray<f64> =
            HComplexArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let ncols = 2_usize;
        let hmatrix = harray.into_hmatrix(ncols).unwrap();
        let hmatrix = hmatrix.mean_cols().unwrap();

        let harray_result: HComplexArray<f64> = HComplexArray::new_from_vec(vec![
            (1. + 7.) / 2.,
            (2. + 8.) / 2.,
            (3. + 9.) / 2.,
            (4. + 10.) / 2.,
            (5. + 11.) / 2.,
            (6. + 12.) / 2.,
        ]);
        let hmatrix_result = harray_result.into_hmatrix(1).unwrap();
        assert_eq!(hmatrix, hmatrix_result)
    }

    #[test]
    fn as_mono_test() {
        let harray: HFloatArray<f32> =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let ncols = 3_usize;
        let hmatrix = harray.into_hmatrix(ncols).unwrap();
        let mut haudio = HFloatAudio::new(hmatrix, 44100);
        haudio.as_mono().unwrap();

        let harray_result: HFloatArray<f32> = HFloatArray::new_from_vec(vec![
            (1. + 5. + 9.) / 3.,
            (2. + 6. + 10.) / 3.,
            (3. + 7. + 11.) / 3.,
            (4. + 8. + 12.) / 3.,
        ]);
        let hmatrix_result = harray_result.into_hmatrix(1).unwrap();
        let haudio_result = HFloatAudio::new(hmatrix_result, 44100);
        assert_eq!(haudio, haudio_result);

        let harray: HFloatArray<f64> =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let ncols = 3_usize;
        let hmatrix = harray.into_hmatrix(ncols).unwrap();
        let mut haudio = HFloatAudio::new(hmatrix, 44100);
        haudio.as_mono().unwrap();

        let harray_result: HFloatArray<f64> = HFloatArray::new_from_vec(vec![
            (1. + 5. + 9.) / 3.,
            (2. + 6. + 10.) / 3.,
            (3. + 7. + 11.) / 3.,
            (4. + 8. + 12.) / 3.,
        ]);
        let hmatrix_result = harray_result.into_hmatrix(1).unwrap();
        let haudio_result = HFloatAudio::new(hmatrix_result, 44100);
        assert_eq!(haudio, haudio_result);
    }

    #[test]
    fn get_inner_mut_test() {
        let mut harray: HFloatArray<f64> =
            HFloatArray::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
        let harray_mut = harray.get_mut_values().unwrap();
        harray_mut[0] = 100.;
        assert_eq!(harray.as_slice()[0], 100.);

        let mut hmatrix = HFloatMatrix::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.], 3).unwrap();
        let hmatrix_mut = hmatrix.get_mut_values().unwrap();
        hmatrix_mut[0] = 100.;
        assert_eq!(hmatrix.as_slice()[0], 100.);
    }
}
