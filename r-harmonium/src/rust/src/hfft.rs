use crate::harray::HArray;
use extendr_api::prelude::*;

pub struct HFft;

#[extendr]
impl HFft {
    /// HArray
    /// ## fft
    ///
    /// `fft() -> HArray` \
    ///
    /// Computes the fast fourier transform of the `HArray`. \
    /// FFT (Fast Fourier Transform) refers to a way the discrete Fourier Transform (DFT) can be calculated efficiently,
    /// by using symmetries in the calculated terms. The symmetry is highest when n is a power of 2, and the transform
    /// is therefore most efficient for these sizes. \
    ///
    /// The function does not normalize outputs. Callers must manually normalize the results by scaling each element by
    /// `1/sqrt(n)`. Multiple normalization steps can be merged into one via pairwise multiplication, so when doing
    /// a forward FFT followed by an inverse callers can normalize once by scaling each element by `1/n`. \
    ///
    /// Elements in the output are ordered by ascending frequency, with the first element corresponding to frequency 0. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray = HArray$new_from_values(c(1,2,3,4), HDataType$float64)
    /// harray$fft()
    /// ```
    ///
    /// _________
    ///
    fn fft(harray: &HArray) -> HArray {
        HArray(harray.0.fft())
    }

    fn fft_mut(harray: &mut HArray) {
        let inner_mut = harray.get_inner_mut();
        inner_mut.fft_mut()
    }
}

extendr_module! {
    mod hfft;
    impl HFft;
}
