use crate::{
    conversions::{try_from_usize_to_int_sexp, ToScalar},
    errors::HErrorR,
    harray::HArray,
};
use harmonium_core::audioop::AudioOp;
use ndarray::IxDyn;
use num_complex::Complex;
use savvy::{savvy, Sexp};

/// HArrayAudio
/// A collection of methods that can be applied to float 1D or 2D `HArray`s which represents audio data.
///
/// # Methods
///
#[savvy]
pub struct HArrayAudio;

#[savvy]
impl HArrayAudio {
    /// HArrayAudio
    /// ## nchannels
    ///
    /// `nchannels() -> integer`
    ///
    /// Returns the number of channels.
    ///
    /// This is the same as the number of rows of a 1D or 2D HArray.
    ///
    /// #### Returns
    ///
    /// An `integer`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HArrayAudio$nchannels(harray)
    /// ```
    ///
    /// _________
    ///
    fn nchannels(harray: &HArray) -> savvy::Result<Sexp> {
        let nchannels = harray.0.nchannels()?;
        let integer_sexp = try_from_usize_to_int_sexp(nchannels)?;
        integer_sexp.into()
    }

    /// HArrayAudio
    /// ## nframes
    ///
    /// `nframes() -> integer`
    ///
    /// Returns the number of frames.
    ///
    /// This is the same as the number of cols of a 1D or 2D HArray.
    ///
    /// The number of frames is equivalent to the number of samples divided by the number of channels.
    ///
    /// #### Returns
    ///
    /// An `integer`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HArrayAudio$nframes(harray)
    /// ```
    ///
    /// _________
    ///
    fn nframes(harray: &HArray) -> savvy::Result<Sexp> {
        let nframes = harray.0.nframes()?;
        let integer_sexp = try_from_usize_to_int_sexp(nframes)?;
        integer_sexp.into()
    }

    /// HArrayAudio
    /// ## db_to_amplitude
    ///
    /// `db_to_amplitude(harray: HArray, reference: double)`
    ///
    /// Converts the `HArray` input from dB to amplitude.
    ///
    /// $db_to_amplitude(x) = reference * (10.0**(x * 0.1))**power$
    ///
    /// The operation is done in-place.
    ///
    /// #### Arguments
    ///
    /// - `harray`
    ///
    /// A 1D or 2D float `HArray`.
    ///
    /// - `reference`
    ///
    /// A double that scales the output.
    ///
    /// - `power`
    ///
    /// A double. If 1.0, will compute DB to power. If 0.5, will compute DB to amplitude.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HArrayAudio$db_to_amplitude(harray, 2, 1)
    /// ```
    ///
    /// _________
    ///
    fn db_to_amplitude(harray: &mut HArray, reference: Sexp, power: Sexp) -> savvy::Result<()> {
        let inner_mut = harray.get_inner_mut();
        let reference: f64 = reference.to_scalar()?;
        let power: f64 = power.to_scalar()?;
        inner_mut.db_to_amplitude(reference, power)
    }

    /// HArrayAudio
    /// ## to_mono
    ///
    /// `to_mono(harray: HArray)`
    ///
    /// Convert to 1 channel by taking the average across channels.
    ///
    /// The operation is done in-place. A new inner array is created.
    ///
    /// #### Arguments
    ///
    /// - `harray`
    ///
    /// A 2D float `HArray`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// HArrayAudio$to_mono(harray)
    /// ```
    ///
    /// _________
    ///
    fn to_mono(harray: &mut HArray) -> savvy::Result<()> {
        let inner_mut = harray.get_inner_mut();
        inner_mut.to_mono()
    }
}

pub trait HAudioOp {
    fn nchannels(&self) -> savvy::Result<usize>;
    fn nframes(&self) -> savvy::Result<usize>;
    fn db_to_amplitude(&mut self, reference: f64, power: f64) -> savvy::Result<()>;
    fn to_mono(&mut self) -> savvy::Result<()>;
}

impl HAudioOp for harmonium_core::array::HArray<f32, IxDyn> {
    fn nchannels(&self) -> savvy::Result<usize> {
        Ok(AudioOp::nchannels(self))
    }

    fn nframes(&self) -> savvy::Result<usize> {
        Ok(AudioOp::nframes(self))
    }

    fn db_to_amplitude(&mut self, reference: f64, power: f64) -> savvy::Result<()> {
        AudioOp::db_to_amplitude(self, reference as f32, power as f32);
        Ok(())
    }

    fn to_mono(&mut self) -> savvy::Result<()> {
        *self = AudioOp::to_mono(self).map_err(HErrorR::from)?;
        Ok(())
    }
}

impl HAudioOp for harmonium_core::array::HArray<f64, IxDyn> {
    fn nchannels(&self) -> savvy::Result<usize> {
        Ok(AudioOp::nchannels(self))
    }

    fn nframes(&self) -> savvy::Result<usize> {
        Ok(AudioOp::nframes(self))
    }

    fn db_to_amplitude(&mut self, reference: f64, power: f64) -> savvy::Result<()> {
        AudioOp::db_to_amplitude(self, reference, power);
        Ok(())
    }

    fn to_mono(&mut self) -> savvy::Result<()> {
        *self = AudioOp::to_mono(self).map_err(HErrorR::from)?;
        Ok(())
    }
}

impl HAudioOp for harmonium_core::array::HArray<Complex<f32>, IxDyn> {
    fn nchannels(&self) -> savvy::Result<usize> {
        Err("Operation only allowed for float HArrays.".into())
    }

    fn nframes(&self) -> savvy::Result<usize> {
        Err("Operation only allowed for float HArrays.".into())
    }

    fn db_to_amplitude(&mut self, _: f64, _: f64) -> savvy::Result<()> {
        Err("Operation only allowed for float HArrays.".into())
    }

    fn to_mono(&mut self) -> savvy::Result<()> {
        Err("Operation only allowed for float HArrays.".into())
    }
}

impl HAudioOp for harmonium_core::array::HArray<Complex<f64>, IxDyn> {
    fn nchannels(&self) -> savvy::Result<usize> {
        Err("Operation only allowed for float HArrays.".into())
    }

    fn nframes(&self) -> savvy::Result<usize> {
        Err("Operation only allowed for float HArrays.".into())
    }

    fn db_to_amplitude(&mut self, _: f64, _: f64) -> savvy::Result<()> {
        Err("Operation only allowed for float HArrays.".into())
    }

    fn to_mono(&mut self) -> savvy::Result<()> {
        Err("Operation only allowed for float HArrays.".into())
    }
}
