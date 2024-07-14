use crate::{
    array::HArray,
    errors::{HError, HResult},
};
use ndarray::{Axis, Dimension, Ix0, Ix1, Ix2, IxDyn};
use num_traits::{Float, FloatConst, FromPrimitive};

pub trait AudioOp<T, D>
where
    T: Float + FloatConst + FromPrimitive,
    D: Dimension,
{
    fn nchannels(&self) -> usize;
    fn nframes(&self) -> usize;
    fn db_to_amplitude(&mut self, reference: T, power: T);
    fn to_mono(&self) -> HResult<HArray<T, D::Smaller>>;
}

pub enum Audio<'a, T>
where
    T: Float + FloatConst,
{
    D1(&'a HArray<T, Ix1>),
    D2(&'a HArray<T, Ix2>),
    Dyn(&'a HArray<T, IxDyn>),
}

impl<T> AudioOp<T, Ix1> for HArray<T, Ix1>
where
    T: Float + FloatConst + FromPrimitive,
{
    /// The number of channels.
    fn nchannels(&self) -> usize {
        1
    }

    /// The number of frames.
    fn nframes(&self) -> usize {
        self.len()
    }

    /// Converts from dB to power.
    /// $db_to_amplitude(x) = reference * 10.0**(x * 0.1)$
    fn db_to_amplitude(&mut self, reference: T, power: T) {
        let a = T::from(10).unwrap();
        let b = T::from(0.1).unwrap();

        self.0
            .mapv_inplace(|x| reference * a.powf(b * x).powf(power));
    }

    fn to_mono(&self) -> HResult<HArray<T, Ix0>> {
        // To return an error is a design choice. This wasn't supposed to error.
        Err(HError::OutOfSpecError(
            "The length of the axis is zero.".into(),
        ))
    }
}

impl<T> AudioOp<T, Ix2> for HArray<T, Ix2>
where
    T: Float + FloatConst + FromPrimitive,
{
    /// The number of channels.
    fn nchannels(&self) -> usize {
        self.0.nrows()
    }

    /// The number of frames.
    fn nframes(&self) -> usize {
        self.0.ncols()
    }

    /// Converts from dB to power.
    /// $db_to_amplitude(x) = reference * 10.0**(x * 0.1)$
    fn db_to_amplitude(&mut self, reference: T, power: T) {
        let a = T::from(10).unwrap();
        let b = T::from(0.1).unwrap();

        self.0
            .mapv_inplace(|x| reference * a.powf(b * x).powf(power));
    }

    fn to_mono(&self) -> HResult<HArray<T, Ix1>> {
        // Ok to unwrap. This is infallible.
        let harray = unsafe { self.0.mean_axis(ndarray::Axis(0)).unwrap_unchecked() };
        Ok(HArray(harray.into()))
    }
}

impl<T> AudioOp<T, IxDyn> for HArray<T, IxDyn>
where
    T: Float + FloatConst + FromPrimitive,
{
    /// The number of channels.
    fn nchannels(&self) -> usize {
        self.0.len_of(Axis(0))
    }

    /// The number of frames.
    fn nframes(&self) -> usize {
        self.0.len_of(Axis(1))
    }

    /// Converts from dB to power.
    /// $db_to_amplitude(x) = reference * (10.0**(x * 0.1))**power$
    fn db_to_amplitude(&mut self, reference: T, power: T) {
        let a = T::from(10).unwrap();
        let b = T::from(0.1).unwrap();

        self.0
            .mapv_inplace(|x| reference * a.powf(b * x).powf(power));
    }

    /// Convert to 1 channel by taking the average across channels.
    /// A new inner array is created.
    fn to_mono(&self) -> HResult<HArray<T, IxDyn>> {
        let harray = self
            .0
            .mean_axis(ndarray::Axis(0))
            .ok_or_else(|| HError::OutOfSpecError("The length of the axis is zero.".into()))?;
        Ok(HArray(harray.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comparison::compare_harray;

    #[test]
    fn db_to_amplitude_test() {
        let mut lhs =
            HArray::new_from_shape_vec((2, 4), vec![1., 2., 3., 4., 5., 6., 7., 8.]).unwrap();
        lhs.db_to_amplitude(1.0, 1.0);

        let rhs = HArray::new_from_shape_vec(
            (2, 4),
            vec![
                1.258925, 1.584893, 1.995262, 2.511886, 3.162278, 3.981072, 5.011872, 6.309574,
            ],
        )
        .unwrap();

        assert!(compare_harray(&lhs, &rhs));
    }

    #[test]
    fn to_mono_test() {
        let lhs: HArray<f64, Ix2> = HArray::new_from_shape_vec(
            (3, 4),
            vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.],
        )
        .unwrap();
        let lhs = lhs.to_mono().unwrap();

        let rhs: HArray<f64, Ix1> = HArray::new_from_shape_vec(
            4,
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
