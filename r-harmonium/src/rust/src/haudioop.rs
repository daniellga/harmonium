use crate::harray::HArray;
use savvy::{savvy, OwnedIntegerSexp, Sexp, TypedSexp};

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

#[savvy]
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
    fn nchannels(harray: &HArray) -> savvy::Result<Sexp> {
        let nchannels: i32 = harray.0.nchannels().try_into().unwrap();
        let integer_sexp: OwnedIntegerSexp = nchannels.try_into()?;
        integer_sexp.into()
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
    fn nframes(harray: &HArray) -> savvy::Result<Sexp> {
        let nframes: i32 = harray.0.nframes().try_into().unwrap();
        let integer_sexp: OwnedIntegerSexp = nframes.try_into()?;
        integer_sexp.into()
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
    fn db_to_amplitude(harray: &mut HArray, reference: Sexp, power: Sexp) -> savvy::Result<()> {
        let inner_mut = harray.get_inner_mut();

        let reference: f64 = match reference.into_typed() {
            TypedSexp::Real(real_sexp) if real_sexp.len() == 1 => real_sexp.as_slice()[0],
            _ => panic!("reference must be a double of length 1."),
        };

        let power: f64 = match power.into_typed() {
            TypedSexp::Real(real_sexp) if real_sexp.len() == 1 => real_sexp.as_slice()[0],
            _ => panic!("reference must be a double of length 1."),
        };

        inner_mut.db_to_amplitude(reference, power);

        Ok(())
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
    fn to_mono(harray: &mut HArray) -> savvy::Result<()> {
        let inner_mut = harray.get_inner_mut();
        inner_mut.to_mono();
        Ok(())
    }
}
