use crate::{
    array::HArray,
    errors::{HError, HResult},
};
use ndarray::{Dimension, IxDyn};
use num_complex::ComplexFloat;

pub trait IntoDynamic<T, D>
where
    T: ComplexFloat,
    D: Dimension,
{
    fn into_dynamic(self) -> HArray<T, IxDyn>;
}

impl<T, D> IntoDynamic<T, D> for HArray<T, D>
where
    T: ComplexFloat,
    D: Dimension,
{
    fn into_dynamic(self) -> HArray<T, IxDyn> {
        HArray(self.0.into_dyn())
    }
}

pub trait IntoStatic<T, D>
where
    T: ComplexFloat,
    D: Dimension,
{
    fn into_static(self) -> HResult<HArray<T, D>>;
}

impl<T, D> IntoStatic<T, D> for HArray<T, IxDyn>
where
    T: ComplexFloat,
    D: Dimension,
{
    fn into_static(self) -> HResult<HArray<T, D>> {
        Ok(HArray(self.0.into_dimensionality::<D>().map_err(|_| {
            HError::OutOfSpecError(
                "The dimensions don’t agree (the number of axes must match).".to_string(),
            )
        })?))
    }
}
