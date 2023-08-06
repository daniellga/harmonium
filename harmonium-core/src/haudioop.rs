use crate::{
    array::HArray,
    errors::{HError, HResult},
};
use ndarray::Axis;
use num_traits::{Float, FloatConst, FromPrimitive};

pub trait HAudioOp<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    fn nchannels(&self) -> usize;
    fn nframes(&self) -> usize;
    fn db_to_power(&mut self, reference: T);
    fn to_mono(&mut self) -> HResult<()>;
}

impl<T> HAudioOp<T> for HArray<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    /// The number of channels.
    fn nchannels(&self) -> usize {
        self.0.len_of(Axis(0))
    }

    /// The number of channels.
    fn nframes(&self) -> usize {
        self.0.len_of(Axis(1))
    }

    /// Converts from dB to power.
    /// $db_to_power(x) = reference * 10.0**(x * 0.1)$
    fn db_to_power(&mut self, reference: T) {
        let a = T::from(10).unwrap();
        let b = T::from(0.1).unwrap();

        self.0.mapv_inplace(|x| reference * a.powf(b * x));
    }

    fn to_mono(&mut self) -> HResult<()> {
        let harray = self
            .0
            .mean_axis(ndarray::Axis(0))
            .ok_or_else(|| HError::OutOfSpecError("The length of the axis is zero.".into()))?;
        self.0 = harray.into();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comparison::compare_harray;

    #[test]
    fn db_to_power_test() {
        let mut lhs =
            HArray::new_from_shape_vec(&[2, 4], vec![1., 2., 3., 4., 5., 6., 7., 8.]).unwrap();
        lhs.db_to_power(1.0);

        let rhs = HArray::new_from_shape_vec(
            &[2, 4],
            vec![
                1.258925, 1.584893, 1.995262, 2.511886, 3.162278, 3.981072, 5.011872, 6.309574,
            ],
        )
        .unwrap();

        assert!(compare_harray(lhs, rhs));
    }

    #[test]
    fn to_mono_test() {
        let mut lhs: HArray<f64> = HArray::new_from_shape_vec(
            &[3, 4],
            vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.],
        )
        .unwrap();
        lhs.to_mono().unwrap();

        let rhs: HArray<f64> = HArray::new_from_shape_vec(
            &[4],
            vec![
                (1. + 5. + 9.) / 3.,
                (2. + 6. + 10.) / 3.,
                (3. + 7. + 11.) / 3.,
                (4. + 8. + 12.) / 3.,
            ],
        )
        .unwrap();
        assert_eq!(lhs, rhs);
    }
}
