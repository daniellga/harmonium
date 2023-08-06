use crate::errors::{HError, HResult};
use ndarray::{ArcArray, IxDyn};
use num_complex::ComplexFloat;

#[derive(Debug, PartialEq)]
pub struct HArray<T: ComplexFloat>(pub ArcArray<T, IxDyn>);

impl<T> HArray<T>
where
    T: ComplexFloat,
{
    pub fn new_from_shape_vec(sh: &[usize], v: Vec<T>) -> HResult<HArray<T>> {
        let ndarray =
            ArcArray::from_shape_vec(IxDyn(sh), v).map_err(|_| HError::OutOfSpecError("shape does not correspond to the number of elements in v or if the shape/strides would result in overflowing isize".to_string()))?;
        Ok(HArray(ndarray))
    }

    /// The length of this HArray.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return the shape of the array as a slice.
    pub fn shape(&self) -> &[usize] {
        self.0.shape()
    }

    /// The number of dimensions.
    pub fn ndim(&self) -> usize {
        self.0.ndim()
    }

    /// Returns true if the HArray is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets the underlying slice. Returns `None` if not contiguous.
    pub fn as_slice(&self) -> Option<&[T]> {
        self.0.as_slice()
    }

    /// Gets the underlying mutable slice. Returns `None` if not contiguous.
    pub fn as_slice_mut(&mut self) -> Option<&mut [T]> {
        self.0.as_slice_mut()
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;

    use super::*;

    #[test]
    fn new_from_shape_vec_test() {
        let nrows = 3_usize;
        let ncols = 4_usize;

        let harray = HArray::new_from_shape_vec(
            &[nrows, ncols],
            vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.],
        );

        let nrows = 2_usize;
        let ncols = 4_usize;
        let harray2 = HArray::new_from_shape_vec(
            &[nrows, ncols],
            vec![
                Complex::new(1., 2.),
                Complex::new(3., 4.),
                Complex::new(5., 6.),
                Complex::new(7., 8.),
                Complex::new(9., 10.),
                Complex::new(11., 12.),
                Complex::new(13., 14.),
                Complex::new(15., 16.),
            ],
        );

        assert!(harray.is_ok());
        assert!(harray2.is_ok());
    }

    #[test]
    fn slice_test() {
        let nrows = 3_usize;
        let ncols = 4_usize;

        let mut harray = HArray::new_from_shape_vec(
            &[nrows, ncols],
            vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.],
        )
        .unwrap();

        assert!(harray.as_slice().is_some());
        assert!(harray.as_slice_mut().is_some());
    }
}
