use arrow2::types::NativeType;
use num_traits::Float;

use crate::structs::{HComplexArray, HComplexMatrix, HFloatArray, HFloatMatrix};

pub fn compare_hfarray<T>(lhs: HFloatArray<T>, rhs: HFloatArray<T>) -> bool
where
    T: NativeType + Float,
{
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut result = true;
    for i in 0..lhs.len() {
        if (lhs.as_slice()[i] - rhs.as_slice()[i]).abs() >= T::from(1e-4).unwrap() {
            result = false;
        };
    }

    result
}

pub fn compare_hcarray<T>(lhs: HComplexArray<T>, rhs: HComplexArray<T>) -> bool
where
    T: NativeType + Float,
{
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut result = true;
    for i in 0..lhs.len() {
        if (lhs.as_slice()[i] - rhs.as_slice()[i]).abs() >= T::from(1e-4).unwrap() {
            result = false;
        };
    }

    result
}

pub fn compare_hfmatrix<T>(lhs: HFloatMatrix<T>, rhs: HFloatMatrix<T>) -> bool
where
    T: NativeType + Float,
{
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut result = true;
    for i in 0..lhs.len() {
        if (lhs.as_slice()[i] - rhs.as_slice()[i]).abs() >= T::from(1e-4).unwrap() {
            result = false;
        };
    }

    result
}

pub fn compare_hcmatrix<T>(lhs: HComplexMatrix<T>, rhs: HComplexMatrix<T>) -> bool
where
    T: NativeType + Float,
{
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut result = true;
    for i in 0..lhs.len() {
        if (lhs.as_slice()[i] - rhs.as_slice()[i]).abs() >= T::from(1e-4).unwrap() {
            result = false;
        };
    }

    result
}
