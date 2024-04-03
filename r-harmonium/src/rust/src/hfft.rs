use crate::harray::HArray;
use savvy::savvy;

pub struct HFft;

#[savvy]
impl HFft {
    /// HFft
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
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// HArray$new_from_values(arr, dtype)
    /// HFft$fft(harray)
    /// ```
    ///
    /// _________
    ///
    fn fft(harray: &HArray) -> savvy::Result<HArray> {
        Ok(HArray(harray.0.fft()))
    }

    /// HFft
    /// ## fft_mut
    ///
    /// `fft_mut()` \
    ///
    /// Computes the fast fourier transform of the `HArray`. \
    /// The operation is done in-place. \
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
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HFft$fft_mut(harray)
    /// ```
    ///
    /// _________
    ///
    fn fft_mut(harray: &mut HArray) -> savvy::Result<()> {
        let inner_mut = harray.get_inner_mut();
        inner_mut.fft_mut();
        Ok(())
    }

    /// HFft
    /// ## fft_real_mut
    ///
    /// `fft_real_mut()` \
    ///
    /// Computes the fast fourier transform of a real-valued `HArray`. \
    /// The operation is not done in-place, although the same external pointer is used to store the new HArray. \
    /// The FFT of a real signal is Hermitian-symmetric, X[i] = conj(X[-i]) so the output contains only the positive frequencies
    /// below the Nyquist frequency. \
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
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HFft$fft_real_mut(harray)
    /// ```
    ///
    /// _________
    ///
    fn fft_real_mut(harray: &mut HArray) -> savvy::Result<()> {
        let inner_mut = harray.get_inner_mut();
        *harray = HArray(inner_mut.fft_real_mut());
        Ok(())
    }
}
