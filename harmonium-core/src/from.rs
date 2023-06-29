use crate::structs::{HComplexArray, HComplexMatrix, HFloatArray, HFloatMatrix};
use arrow2::types::NativeType;
use bytemuck::{cast_slice, cast_slice_mut, cast_vec};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2, ArrayViewMut2};
use num_complex::Complex;
use num_traits::Float;

#[derive(PartialEq, Debug)]
pub enum CowArray2<'a, T> {
    Mut(ArrayViewMut2<'a, T>),
    Owned(Array2<T>),
}

impl<'a, T> From<&'a HFloatArray<T>> for ArrayView1<'a, T>
where
    T: Float + NativeType,
{
    fn from(value: &'a HFloatArray<T>) -> Self {
        let slice = value.as_slice();

        ArrayView1::from(slice)
    }
}

impl<T> From<Array1<T>> for HFloatArray<T>
where
    T: Float + NativeType,
{
    fn from(value: Array1<T>) -> Self {
        let v = value.into_raw_vec();

        HFloatArray::new_from_vec(v)
    }
}

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

impl<T> From<HFloatMatrix<T>> for Array2<T>
where
    T: Float + NativeType,
{
    fn from(value: HFloatMatrix<T>) -> Self {
        let nrows = value.nrows();
        let ncols = value.ncols();
        let v = value.as_slice().to_vec();

        // Ok to unwrap. Not supposed to error.
        Array2::from_shape_vec((ncols, nrows), v).unwrap()
    }
}

impl<'a, T> From<&'a mut HFloatMatrix<T>> for CowArray2<'a, T>
where
    T: Float + NativeType,
{
    fn from(value: &'a mut HFloatMatrix<T>) -> CowArray2<'a, T> {
        let nrows = value.nrows();
        let ncols = value.ncols();

        // Unsafe needed here to perform lifetime extension. https://docs.rs/polonius-the-crab/latest/polonius_the_crab/#the-arcanemagic
        let reborrow: &mut HFloatMatrix<T> = unsafe { &mut *(value as *mut _) };
        if let Some(slice) = reborrow.get_mut_values() {
            CowArray2::Mut(ArrayViewMut2::from_shape((ncols, nrows), slice).unwrap())
        } else {
            let arr = Array2::from_shape_vec((ncols, nrows), value.as_slice().to_vec()).unwrap();
            CowArray2::Owned(arr)
        }
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

impl<'a, T> From<&'a HComplexMatrix<T>> for ArrayView2<'a, Complex<T>>
where
    T: Float + NativeType,
{
    fn from(value: &'a HComplexMatrix<T>) -> Self {
        let nrows = value.nrows();
        let ncols = value.ncols();
        let slice = value.as_slice();
        let slice = cast_slice::<T, Complex<T>>(slice);

        // Ok to unwrap. Not supposed to error.
        // Swap ncols and nrows because ndarray uses row major order.
        ArrayView2::<Complex<T>>::from_shape((ncols, nrows), slice).unwrap()
    }
}

impl<'a, T> From<&'a mut HComplexMatrix<T>> for CowArray2<'a, Complex<T>>
where
    T: Float + NativeType,
{
    fn from(value: &'a mut HComplexMatrix<T>) -> CowArray2<'a, Complex<T>> {
        let nrows = value.nrows();
        let ncols = value.ncols();

        // Unsafe needed here to perform lifetime extension. https://docs.rs/polonius-the-crab/latest/polonius_the_crab/#the-arcanemagic
        let reborrow: &mut HComplexMatrix<T> = unsafe { &mut *(value as *mut _) };
        if let Some(slice) = reborrow.get_mut_values() {
            let slice = cast_slice_mut::<T, Complex<T>>(slice);
            CowArray2::Mut(ArrayViewMut2::from_shape((ncols, nrows), slice).unwrap())
        } else {
            let slc = value.as_slice();
            let slc = cast_slice::<T, Complex<T>>(slc);
            let v = slc.to_vec();
            let arr = Array2::from_shape_vec((ncols, nrows), v).unwrap();
            CowArray2::Owned(arr)
        }
    }
}

impl<T> From<Array2<Complex<T>>> for HComplexMatrix<T>
where
    T: Float + NativeType,
{
    fn from(value: Array2<Complex<T>>) -> Self {
        // Use nrows because ndarray uses row major order.
        let nrows = value.nrows();
        let v = value.into_raw_vec();
        let v = cast_vec::<Complex<T>, T>(v);

        // Ok to unwrap. Not supposed to error.
        HComplexMatrix::new_from_vec(v, nrows).unwrap()
    }
}

impl<T> From<Array1<Complex<T>>> for HComplexMatrix<T>
where
    T: Float + NativeType,
{
    fn from(value: Array1<Complex<T>>) -> Self {
        let v = value.into_raw_vec();
        let v = cast_vec::<Complex<T>, T>(v);

        // Ok to unwrap. Not supposed to error.
        HComplexMatrix::new_from_vec(v, 1).unwrap()
    }
}

impl<'a, T> From<&'a HComplexArray<T>> for ArrayView1<'a, Complex<T>>
where
    T: Float + NativeType,
{
    fn from(value: &'a HComplexArray<T>) -> Self {
        let slice = value.as_slice();
        let slice = cast_slice::<T, Complex<T>>(slice);

        ArrayView1::from(slice)
    }
}

impl<T> From<Array1<Complex<T>>> for HComplexArray<T>
where
    T: Float + NativeType,
{
    fn from(value: Array1<Complex<T>>) -> Self {
        let v = value.into_raw_vec();
        let v = cast_vec::<Complex<T>, T>(v);

        HComplexArray::new_from_vec(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harray_to_arrayview1_test() {
        let v = vec![1., 2., 3., 4., 5., 6., 7., 8.];
        let harray = HFloatArray::new_from_vec(v.clone());
        let arrayview1 = ArrayView1::from(&harray);
        let arrayview1_result = ArrayView1::from(v.as_slice());
        assert_eq!(arrayview1, arrayview1_result);

        let harray = HComplexArray::new_from_vec(v.clone());
        let arrayview1 = ArrayView1::from(&harray);
        let v = vec![
            Complex::new(1., 2.),
            Complex::new(3., 4.),
            Complex::new(5., 6.),
            Complex::new(7., 8.),
        ];
        let arrayview1_result = ArrayView1::from(v.as_slice());
        assert_eq!(arrayview1, arrayview1_result);
    }

    #[test]
    fn array1_to_harray_test() {
        let v = vec![1., 2., 3., 4., 5., 6., 7., 8.];
        let array1 = Array1::from_vec(v.clone());
        let harray = HFloatArray::from(array1);
        let harray_result = HFloatArray::new_from_vec(v.clone());
        assert_eq!(harray, harray_result);

        let v_complex = vec![
            Complex::new(1., 2.),
            Complex::new(3., 4.),
            Complex::new(5., 6.),
            Complex::new(7., 8.),
        ];
        let array1 = Array1::from_vec(v_complex);
        let harray = HComplexArray::from(array1);
        let harray_result = HComplexArray::new_from_vec(v);
        assert_eq!(harray, harray_result);
    }

    #[test]
    fn hmatrix_to_arrayview2_test() {
        let ncols = 2;
        let nrows = 4;
        let v = vec![1., 2., 3., 4., 5., 6., 7., 8.];
        let hmatrix = HFloatMatrix::new_from_vec(v.clone(), ncols).unwrap();
        let arrayview2 = ArrayView2::from(&hmatrix);
        let arrayview2_result = ArrayView2::from_shape((ncols, nrows), v.as_slice()).unwrap();
        assert_eq!(arrayview2, arrayview2_result);

        let nrows_complex = 2;
        let v_complex = vec![
            Complex::new(1., 2.),
            Complex::new(3., 4.),
            Complex::new(5., 6.),
            Complex::new(7., 8.),
        ];
        let hmatrix = HComplexMatrix::new_from_vec(v, ncols).unwrap();
        let arrayview2 = ArrayView2::from(&hmatrix);
        let arrayview2_result =
            ArrayView2::from_shape((ncols, nrows_complex), v_complex.as_slice()).unwrap();
        assert_eq!(arrayview2, arrayview2_result);
    }

    #[test]
    fn hmatrix_to_arrayviewmut2_test() {
        let ncols = 2;
        let nrows = 4;
        let mut v = vec![1., 2., 3., 4., 5., 6., 7., 8.];
        let mut hmatrix = HFloatMatrix::new_from_vec(v.clone(), ncols).unwrap();
        let arrayviewmut2 = CowArray2::from(&mut hmatrix);
        let arrayviewmut2_result =
            CowArray2::Mut(ArrayViewMut2::from_shape((ncols, nrows), v.as_mut_slice()).unwrap());
        assert_eq!(arrayviewmut2, arrayviewmut2_result);

        let mut hmatrix = HFloatMatrix::new_from_vec(v.clone(), ncols).unwrap();
        let _hmatrix2 = hmatrix.clone();
        let arrayviewmut2 = CowArray2::from(&mut hmatrix);
        let arrayviewmut2_result =
            CowArray2::Owned(Array2::from_shape_vec((ncols, nrows), v.clone()).unwrap());
        assert_eq!(arrayviewmut2, arrayviewmut2_result);

        let mut v_complex = vec![
            Complex::new(1., 2.),
            Complex::new(3., 4.),
            Complex::new(5., 6.),
            Complex::new(7., 8.),
        ];
        let nrows_complex = 2;
        let mut hmatrix = HComplexMatrix::new_from_vec(v.clone(), ncols).unwrap();
        let arrayviewmut2 = CowArray2::from(&mut hmatrix);
        let arrayviewmut2_result = CowArray2::Mut(
            ArrayViewMut2::from_shape((ncols, nrows_complex), v_complex.as_mut_slice()).unwrap(),
        );
        assert_eq!(arrayviewmut2, arrayviewmut2_result);

        let mut hmatrix = HComplexMatrix::new_from_vec(v.clone(), ncols).unwrap();
        let _hmatrix2 = hmatrix.clone();
        let arrayviewmut2 = CowArray2::from(&mut hmatrix);
        let arrayviewmut2_result =
            CowArray2::Owned(Array2::from_shape_vec((ncols, nrows_complex), v_complex).unwrap());
        assert_eq!(arrayviewmut2, arrayviewmut2_result);
    }

    #[test]
    fn array2_to_hmatrix_test() {
        let ncols = 2;
        let nrows = 4;
        let v = vec![1., 2., 3., 4., 5., 6., 7., 8.];
        let array2 = Array2::from_shape_vec((ncols, nrows), v.clone()).unwrap();
        let hmatrix = HFloatMatrix::from(array2);
        let hmatrix_result = HFloatMatrix::new_from_vec(v.clone(), 2).unwrap();
        assert_eq!(hmatrix, hmatrix_result);

        let v_complex = vec![
            Complex::new(1., 2.),
            Complex::new(3., 4.),
            Complex::new(5., 6.),
            Complex::new(7., 8.),
        ];
        let nrows_complex = 2;
        let array2 = Array2::from_shape_vec((ncols, nrows_complex), v_complex).unwrap();
        let hmatrix = HComplexMatrix::from(array2);
        let hmatrix_result = HComplexMatrix::new_from_vec(v, ncols).unwrap();
        assert_eq!(hmatrix, hmatrix_result);
    }
}
