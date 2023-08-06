use crate::array::HArray;
use num_traits::{Float, FloatConst};

pub fn compare_harray<T>(lhs: HArray<T>, rhs: HArray<T>) -> bool
where
    T: Float + FloatConst,
{
    if lhs.shape() != rhs.shape() {
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
