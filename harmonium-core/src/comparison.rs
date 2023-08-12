use crate::array::HArray;
use ndarray::Dimension;
use num_traits::{Float, FloatConst};

pub fn compare_harray<T, D>(lhs: HArray<T, D>, rhs: HArray<T, D>) -> bool
where
    T: Float + FloatConst,
    D: Dimension,
{
    if lhs.0.raw_dim() != rhs.0.raw_dim() {
        return false;
    }
    let mut result = true;
    let lhs_slice = lhs.as_slice().unwrap();
    let rhs_slice = rhs.as_slice().unwrap();

    for i in 0..lhs.len() {
        if (lhs_slice[i] - rhs_slice[i]).abs() >= T::from(1e-4).unwrap() {
            result = false;
        };
    }

    result
}
