use crate::harray::HArray;
use extendr_api::{prelude::*, AsTypedSlice};

pub struct HAudioOp;

#[extendr]
impl HAudioOp {
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
    fn nchannels(harray: &HArray) -> Robj {
        let nchannels = harray.0.nchannels();
        let rint = Rint::from(i32::try_from(nchannels).unwrap());
        rint.into()
    }

    fn nframes(harray: &HArray) -> Robj {
        let nframes = harray.0.nframes();
        let rint = Rint::from(i32::try_from(nframes).unwrap());
        rint.into()
    }

    fn db_to_power(harray: &mut HArray, reference: Robj) {
        let inner_mut = harray.get_inner_mut();
        inner_mut.db_to_power(reference);
    }

    fn to_mono(harray: &mut HArray) {
        let inner_mut = harray.get_inner_mut();
        inner_mut.to_mono();
    }
}

extendr_module! {
    mod haudioop;
    impl HAudioOp;
}
