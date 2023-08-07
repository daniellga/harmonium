use num_complex::ComplexFloat;

use crate::array::HArray;
use std::fmt;

impl<T> fmt::Display for HArray<T>
where
    T: ComplexFloat + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
