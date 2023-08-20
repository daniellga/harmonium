use crate::harray::HArray;
use extendr_api::prelude::*;

/// HAudioOp
///
/// _________
///
/// HAudioOp
/// A collection of methods that can be applied to float 1D or 2D HArray which represents audio data. \
///
/// # Methods
///
pub struct HAudioOp;

#[extendr]
impl HAudioOp {
    /// HAudioOp
    /// ## nchannels
    ///
    /// `nchannels() -> integer` \
    ///
    /// Returns the number of channels. \
    /// This is the same as the number of rows of a 1D or 2D HArray. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$nchannels()
    /// ```
    ///
    /// _________
    ///
    fn nchannels(harray: &HArray) -> Robj {
        let nchannels = harray.0.nchannels();
        let rint = Rint::from(i32::try_from(nchannels).unwrap());
        rint.into()
    }

    /// HAudioOp
    /// ## nframes
    ///
    /// `nframes() -> integer` \
    ///
    /// Returns the number of frames. \
    /// This is the same as the number of cols of a 1D or 2D HArray. \
    /// The number of frames is equivalent to the number of samples divided by the number of channels. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$nframes()
    /// ```
    ///
    /// _________
    ///
    fn nframes(harray: &HArray) -> Robj {
        let nframes = harray.0.nframes();
        let rint = Rint::from(i32::try_from(nframes).unwrap());
        rint.into()
    }

    /// HAudioOp
    /// ## db_to_amplitude
    ///
    /// `db_to_amplitude(harray: HArray, reference: double)` \
    ///
    /// Converts the `HArray` input from dB to amplitude. \
    /// $db_to_amplitude(x) = reference * (10.0**(x * 0.1))**power$ \
    /// The operation is done in-place. \
    ///
    /// #### Arguments
    ///
    /// * `harray` \
    /// A 1D or 2D float `HArray`. \
    /// * `reference` \
    /// A double that scales the output. \
    /// * `power` \
    /// A double. If 1.0, will compute DB to power. If 0.5, will compute DB to amplitude. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HAudioOp$db_to_amplitude(harray, 2, 1)
    /// ```
    ///
    /// _________
    ///
    fn db_to_amplitude(harray: &mut HArray, reference: Robj, power: Robj) {
        let inner_mut = harray.get_inner_mut();
        inner_mut.db_to_amplitude(&reference, &power);
    }

    /// HAudioOp
    /// ## to_mono
    ///
    /// `to_mono(harray: HArray)` \
    ///
    /// Convert to 1 channel by taking the average across channels. \
    /// The operation is done in-place. A new inner array is created. \
    ///
    /// #### Arguments
    ///
    /// * `harray` \
    /// A 2D float `HArray`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HAudioOp$to_mono(harray)
    /// ```
    ///
    /// _________
    ///
    fn to_mono(harray: &mut HArray) {
        let inner_mut = harray.get_inner_mut();
        inner_mut.to_mono();
    }
}

extendr_module! {
    mod haudioop;
    impl HAudioOp;
}
