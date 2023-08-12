use ndarray::Dimension;
use num_complex::ComplexFloat;

use crate::array::HArray;
use std::fmt;

impl<T, D> fmt::Display for HArray<T, D>
where
    T: ComplexFloat + fmt::Display,
    D: Dimension,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
