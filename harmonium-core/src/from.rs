use crate::structs::{HComplexMatrix, HFloatMatrix};
use arrow2::types::NativeType;
use ndarray::{Array1, Array2, ArrayView2};
use num_traits::Float;

impl<'a, T> From<&'a HFloatMatrix<T>> for ArrayView2<'a, T>
where
    T: Float + NativeType,
{
    fn from(value: &'a HFloatMatrix<T>) -> Self {
        let nrows = value.nrows();
        let ncols = value.ncols();
        let slice = value.as_slice();

        // Ok to unwrap. Not supposed to error.
        ArrayView2::from_shape((ncols, nrows), slice).unwrap()
    }
}

impl<'a, T> From<&'a HComplexMatrix<T>> for ArrayView2<'a, T>
where
    T: Float + NativeType,
{
    fn from(value: &'a HComplexMatrix<T>) -> Self {
        let nrows = value.nrows() * 2;
        let ncols = value.ncols();
        let slice = value.as_slice();

        // Ok to unwrap. Not supposed to error.
        // Swap ncols and nrows because ndarray uses row major order.
        ArrayView2::from_shape((ncols, nrows), slice).unwrap()
    }
}

impl<T> From<Array2<T>> for HFloatMatrix<T>
where
    T: Float + NativeType,
{
    fn from(value: Array2<T>) -> Self {
        // Use nrows because ndarray uses row major order.
        let nrows = value.nrows();
        let v = value.into_raw_vec();

        // Ok to unwrap. Not supposed to error.
        HFloatMatrix::new_from_vec(v, nrows).unwrap()
    }
}

impl<T> From<Array2<T>> for HComplexMatrix<T>
where
    T: Float + NativeType,
{
    fn from(value: Array2<T>) -> Self {
        // Use nrows because ndarray uses row major order.
        let nrows = value.nrows();
        let v = value.into_raw_vec();

        // Ok to unwrap. Not supposed to error.
        HComplexMatrix::new_from_vec(v, nrows).unwrap()
    }
}

impl<T> From<Array1<T>> for HFloatMatrix<T>
where
    T: Float + NativeType,
{
    fn from(value: Array1<T>) -> Self {
        let v = value.into_raw_vec();

        // Ok to unwrap. Not supposed to error.
        HFloatMatrix::new_from_vec(v, 1).unwrap()
    }
}

impl<T> From<Array1<T>> for HComplexMatrix<T>
where
    T: Float + NativeType,
{
    fn from(value: Array1<T>) -> Self {
        let v = value.into_raw_vec();

        // Ok to unwrap. Not supposed to error.
        HComplexMatrix::new_from_vec(v, 1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hmatrix_to_arrayview2_test() {
        let ncols = 2;
        let nrows = 4;
        let v = vec![1., 2., 3., 4., 5., 6., 7., 8.];
        let hmatrix = HFloatMatrix::new_from_vec(v.clone(), ncols).unwrap();
        let arrayview2 = ArrayView2::from(&hmatrix);
        let arrayview2_result = ArrayView2::from_shape((ncols, nrows), v.as_slice()).unwrap();
        assert_eq!(arrayview2, arrayview2_result);

        let hmatrix = HComplexMatrix::new_from_vec(v.clone(), ncols).unwrap();
        let arrayview2 = ArrayView2::from(&hmatrix);
        let arrayview2_result = ArrayView2::from_shape((ncols, nrows), v.as_slice()).unwrap();
        assert_eq!(arrayview2, arrayview2_result);
    }

    #[test]
    fn array2_to_hmatrix_test() {
        let v = vec![1., 2., 3., 4., 5., 6., 7., 8.];
        let array2 = Array2::from_shape_vec((2, 4), v.clone()).unwrap();
        let hmatrix = HFloatMatrix::from(array2);
        let hmatrix_result = HFloatMatrix::new_from_vec(v.clone(), 2).unwrap();
        assert_eq!(hmatrix, hmatrix_result);

        let array2 = Array2::from_shape_vec((2, 4), v.clone()).unwrap();
        let hmatrix = HComplexMatrix::from(array2);
        let hmatrix_result = HComplexMatrix::new_from_vec(v.clone(), 2).unwrap();
        assert_eq!(hmatrix, hmatrix_result);
    }
}
