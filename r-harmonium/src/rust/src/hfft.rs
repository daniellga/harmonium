use crate::{conversions::Conversions, harray::HArray};
use savvy::{savvy, Sexp};

#[savvy]
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
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HFft$fft(harray)
    /// ```
    ///
    /// _________
    ///
    fn fft(harray: &HArray) -> savvy::Result<HArray> {
        Ok(HArray(harray.0.fft()?))
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
    /// library(harmonium)
    /// r = as.double(sample(100, 30, replace = TRUE))
    /// i = as.double(sample(100, 30, replace = TRUE))
    /// arr = array(complex(real=r, imaginary=i), c(6,5))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HFft$fft_mut(harray)
    /// ```
    ///
    /// _________
    ///
    fn fft_mut(harray: &mut HArray) -> savvy::Result<()> {
        let inner_mut = harray.get_inner_mut();
        inner_mut.fft_mut()
    }

    /// HFft
    /// ## ifft
    ///
    /// `ifft() -> HArray` \
    ///
    /// Computes the inverse fast fourier transform of a complex `HArray`. \
    /// FFT (Fast Fourier Transform) refers to a way the discrete Fourier Transform (DFT) can be calculated efficiently,
    /// by using symmetries in the calculated terms. The symmetry is highest when n is a power of 2, and the transform
    /// is therefore most efficient for these sizes. \
    ///
    /// The function does not normalize outputs. Callers must manually normalize the results by scaling each element by
    /// `1/sqrt(n)`. Multiple normalization steps can be merged into one via pairwise multiplication, so when doing
    /// a forward FFT followed by an inverse callers can normalize once by scaling each element by `1/n`. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    /// Will return an error if the `HArray` is of a float type. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i), c(3,2))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HFft$ifft(harray)
    /// ```
    ///
    /// _________
    ///
    fn ifft(harray: &HArray) -> savvy::Result<HArray> {
        Ok(HArray(harray.0.ifft()?))
    }

    /// HFft
    /// ## ifft_mut
    ///
    /// `ifft_mut() -> HArray` \
    ///
    /// Computes the inverse fast fourier transform of a complex `HArray`. \
    /// The operation is done in-place. \
    /// FFT (Fast Fourier Transform) refers to a way the discrete Fourier Transform (DFT) can be calculated efficiently,
    /// by using symmetries in the calculated terms. The symmetry is highest when n is a power of 2, and the transform
    /// is therefore most efficient for these sizes. \
    ///
    /// The function does not normalize outputs. Callers must manually normalize the results by scaling each element by
    /// `1/sqrt(n)`. Multiple normalization steps can be merged into one via pairwise multiplication, so when doing
    /// a forward FFT followed by an inverse callers can normalize once by scaling each element by `1/n`. \
    ///
    /// #### Returns
    ///
    /// Will return an error if the `HArray` is not of complex type. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i), c(3,2))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HFft$ifft_mut(harray)
    /// ```
    ///
    /// _________
    ///
    fn ifft_mut(harray: &mut HArray) -> savvy::Result<()> {
        let inner_mut = harray.get_inner_mut();
        inner_mut.ifft_mut()
    }

    /// HFft
    /// ## rfft_mut
    ///
    /// `rfft_mut()` \
    ///
    /// Computes the fast fourier transform of a float `HArray`. Transforms a real signal of length `N` to a complex-valued spectrum of length `N/2+1` (with `N/2` rounded down). \
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
    /// #### Returns
    ///
    /// Will return an error if the `HArray` is not of float type. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HFft$rfft_mut(harray)
    /// ```
    ///
    /// _________
    ///
    fn rfft_mut(harray: &mut HArray) -> savvy::Result<()> {
        let inner_mut = harray.get_inner_mut();
        *harray = HArray(inner_mut.rfft_mut()?);
        Ok(())
    }

    /// HFft
    /// ## irfft_mut
    ///
    /// `irfft_mut(length: integer)` \
    ///
    /// Computes the inverse fast fourier transform of a complex `HArray`. Transforms a complex spectrum of length `N/2+1` (with `N/2` rounded down) to a real-valued signal of length `N`. \
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
    /// #### Returns
    ///
    /// Will return an error if: \
    /// * The `HArray` is not of float type. \
    /// * The `length` argument is not compatible with the spectrum length. \
    ///
    /// #### Arguments
    ///
    /// * `length` \
    /// An integer. The output length of the signal. Since the spectrum is `N/2+1`, the length can be `N` and `N+1`, if `N` is even, or can be `N` and `N-1` if `N` is odd.  \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// r = as.double(sample(100, 4, replace = TRUE))
    /// i = as.double(sample(100, 3, replace = TRUE))
    /// arr = array(complex(real=r, imaginary=c(0,i)), c(4,1))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HFft$irfft_mut(harray, 7L)
    /// ```
    ///
    /// _________
    ///
    fn irfft_mut(harray: &mut HArray, length: Sexp) -> savvy::Result<()> {
        let length: i32 = length.to_scalar()?;
        let length = usize::try_from(length)
            .map_err(|_| savvy::Error::new("Cannot convert i32 to usize."))?;
        let inner_mut = harray.get_inner_mut();
        *harray = HArray(inner_mut.irfft_mut(length)?);
        Ok(())
    }
}
