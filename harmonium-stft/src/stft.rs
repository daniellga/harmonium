use harmonium_core::array::HArray;
use ndarray::{ArcArray2, Dimension, Ix1, Ix2, IxDyn};
use rustfft::{
    num_complex::{Complex, ComplexFloat},
    num_traits::Float,
    Fft, FftNum, FftPlanner,
};
use std::sync::Arc;

struct StftPlanner<T>
where
    T: FftNum,
{
    fft: Arc<dyn Fft<T>>,
    buffer: Vec<T>,
}

impl<T> StftPlanner<T>
where
    T: FftNum + ComplexFloat,
{
    fn new(fft_length: usize) -> Self {
        let mut fft_planner = FftPlanner::new();
        let fft = fft_planner.plan_fft_forward(fft_length);
        let buffer = vec![T::zero(); fft_length];
        StftPlanner { fft, buffer }
    }

    fn len(&self) -> usize {
        self.fft.len()
    }
}

// The input buffer is used as scratch space, so the contents of input should be considered garbage after calling.
pub trait Stft<T, D>
where
    T: FftNum + ComplexFloat,
    D: Dimension,
{
    fn process(
        &mut self,
        harray: HArray<T, D>,
        hop_length: usize,
        window_length: usize,
        window: Option<&[T]>,
    ) -> HArray<T, D::Larger>;
}

impl<T> Stft<T, Ix1> for StftPlanner<T>
where
    T: FftNum + ComplexFloat,
{
    fn process(
        &mut self,
        harray: HArray<T, Ix1>,
        hop_length: usize,
        window_length: usize,
        window: Option<&[T]>,
    ) -> HArray<T, Ix2> {
        assert!(hop_length > 0);

        let fft_length = self.len();
        let n_fft = (harray.len() - window_length) / hop_length + 1;
        //let stft_ndarray = ArcArray2::zeros((fft_length, n_fft));
        todo!()
    }
}

trait ApplyWindow<T> {
    fn apply_window(&mut self, window: &[T]);
}

impl<T> ApplyWindow<T> for [Complex<T>]
where
    T: Float,
{
    fn apply_window(&mut self, window: &[T]) {
        for (i, w) in self.iter_mut().zip(window.iter()) {
            *i = (*i).scale(*w);
        }
    }
}

impl<T> ApplyWindow<T> for [T]
where
    T: Float,
{
    fn apply_window(&mut self, window: &[T]) {
        for (i, w) in self.iter_mut().zip(window.iter()) {
            *i = *i * *w;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
