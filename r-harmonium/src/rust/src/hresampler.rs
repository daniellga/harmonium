use crate::{
    conversions::{try_from_i32_to_usize, try_from_usize_to_int_sexp, ToScalar},
    errors::{to_savvy_error, HErrorR},
    harray::HArray,
    hdatatype::HDataType,
    hpolynomialdegree::HPolynomialDegree,
    hresamplertype::HResamplerType,
    hsincinterpolationparameters::HSincInterpolationParameters,
};
use harmonium_core::errors::HError;
use harmonium_resample::resample::ProcessResampler;
use ndarray::IxDyn;
use rubato::{
    FastFixedIn, FastFixedOut, FftFixedIn, FftFixedInOut, FftFixedOut, SincFixedIn, SincFixedOut,
};
use savvy::{r_println, savvy, OwnedIntegerSexp, Sexp};

pub trait HResamplerR: Send {
    fn process(&mut self, harray: &mut HArray) -> savvy::Result<()>;
    fn set_resample_ratio(&mut self, new_ratio: f64, ramp: bool) -> savvy::Result<()>;
    fn set_resample_ratio_relative(&mut self, rel_ratio: f64, ramp: bool) -> savvy::Result<()>;
    fn nbr_channels(&self) -> savvy::Result<OwnedIntegerSexp>;
    fn set_chunk_size(&mut self, chunk_size: usize) -> savvy::Result<()>;
    fn reset(&mut self) -> savvy::Result<()>;
    fn res_type(&self) -> savvy::Result<HResamplerType>;
    fn dtype(&self) -> savvy::Result<HDataType>;
    fn print(&self) -> savvy::Result<()>;
}

/// HResampler
/// A resampler.
///
/// #### Asynchronous Resamplers
///
/// The resampling is based on band-limited interpolation using sinc interpolation filters. The sinc interpolation upsamples by an adjustable factor,
/// and then the new sample points are calculated by interpolating between these points.
///
/// The resampling ratio can be updated at any time.
///
/// - `SincFixedIn`
///
/// - `SincFixedOut`
///
/// - `FastFixedIn`
///
/// - `FastFixedOut`
///
/// #### Synchronous Resamplers
///
/// Is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data.
///
/// This type of resampler is considerably faster but doesn’t support changing the resampling ratio.
///
/// - `FftFixedIn`
///
/// - `FftFixedInOut`
///
/// - `FftFixedOut`
///
/// # Methods
///
#[savvy]
pub struct HResampler(pub Box<dyn HResamplerR>);

#[savvy]
impl HResampler {
    /// HResampler
    /// ## new_fft
    ///
    /// `new_fft(sr_in: integer, sr_out: integer, chunk_size: integer, sub_chunks: integer, nbr_channels: integer, res_type: HResamplerType, dtype: HDataType) -> HResampler`
    ///
    /// Creates a new FFT type HResampler.
    ///
    /// Supports any of  `[FftFixedIn, FftFixedInOut, FftFixedOut]` `HResamplerType`.
    ///
    /// The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing
    /// filter before it’s inverse transformed to get the resampled waveforms.
    ///
    /// - `FftFixedIn`
    ///
    /// A synchronous resampler that needs a fixed number of audio frames for input and returns a variable number of frames.
    ///
    /// - `FftFixedInOut`
    ///
    /// A synchronous resampler that accepts a fixed number of audio frames for input and returns a fixed number of frames.
    ///
    /// - `FftFixedOut`
    ///
    /// A synchronous resampler that needs a fixed number of audio frames for input and returns a variable number of frames.
    ///
    /// #### Arguments
    ///
    /// - `sr_in`
    ///
    /// The input sampling rate in hz.
    ///
    /// - `sr_out`
    ///
    /// The output sampling rate in hz.
    ///
    /// - `chunk_size`
    ///
    /// Chunks size of input or output data in frames.
    ///
    /// It can be used as input or output, depending on `HResamplerType`.
    ///
    /// - `sub_chunks`
    ///
    /// Desired number of subchunks for processing, actual number may be different.
    ///
    /// - `nbr_channels`
    ///
    /// Number of channels in input and output.
    ///
    /// Must be the same number of channels as the `HAudio` that will be processed by the `HResampler`.
    ///
    /// - `res_type`
    ///
    /// An `HResamplerType` to indicate which type of `HResampler` to be created.
    ///
    /// - `dtype`
    ///
    /// A float `HDataType` to indicate the dtype that the `HResampler` will be working with.
    ///
    /// Must be the same as the `HAudio`'s dtype that will be processed by the `HResampler`.
    ///
    /// #### Returns
    ///
    /// A FFT type `HResampler`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// sr_in = 48000L
    /// sr_out = 44100L
    /// chunk_size = 1024L
    /// sub_chunks = 2L
    /// nbr_channels = 2L
    /// res_type = HResamplerType$FftFixedIn
    /// dtype = HDataType$Float32
    ///
    /// hresampler = HResampler$new_fft(sr_in, sr_out, chunk_size, sub_chunks, nbr_channels, res_type, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_fft(
        sr_in: Sexp,
        sr_out: Sexp,
        chunk_size: Sexp,
        sub_chunks: Sexp,
        nchannels: Sexp,
        res_type: &HResamplerType,
        dtype: &HDataType,
    ) -> savvy::Result<HResampler> {
        let sr_in: i32 = sr_in.to_scalar()?;
        let sr_in = try_from_i32_to_usize(sr_in)?;
        let sr_out: i32 = sr_out.to_scalar()?;
        let sr_out = try_from_i32_to_usize(sr_out)?;
        let chunk_size: i32 = chunk_size.to_scalar()?;
        let chunk_size = try_from_i32_to_usize(chunk_size)?;
        let nchannels: i32 = nchannels.to_scalar()?;
        let nchannels = try_from_i32_to_usize(nchannels)?;

        match (res_type, dtype) {
            (HResamplerType::FftFixedIn, HDataType::Float32) => {
                let sub_chunks: i32 = sub_chunks.to_scalar()?;
                let sub_chunks = try_from_i32_to_usize(sub_chunks)?;
                let resampler =
                    FftFixedIn::<f32>::new(sr_in, sr_out, chunk_size, sub_chunks, nchannels)
                        .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::FftFixedIn, HDataType::Float64) => {
                let sub_chunks: i32 = sub_chunks.to_scalar()?;
                let sub_chunks = try_from_i32_to_usize(sub_chunks)?;
                let resampler =
                    FftFixedIn::<f64>::new(sr_in, sr_out, chunk_size, sub_chunks, nchannels)
                        .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::FftFixedInOut, HDataType::Float32) => {
                let resampler = FftFixedInOut::<f32>::new(sr_in, sr_out, chunk_size, nchannels)
                    .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::FftFixedInOut, HDataType::Float64) => {
                let resampler = FftFixedInOut::<f64>::new(sr_in, sr_out, chunk_size, nchannels)
                    .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::FftFixedOut, HDataType::Float32) => {
                let sub_chunks: i32 = sub_chunks.to_scalar()?;
                let sub_chunks = try_from_i32_to_usize(sub_chunks)?;
                let resampler =
                    FftFixedOut::<f32>::new(sr_in, sr_out, chunk_size, sub_chunks, nchannels)
                        .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::FftFixedOut, HDataType::Float64) => {
                let sub_chunks: i32 = sub_chunks.to_scalar()?;
                let sub_chunks = try_from_i32_to_usize(sub_chunks)?;
                let resampler =
                    FftFixedOut::<f64>::new(sr_in, sr_out, chunk_size, sub_chunks, nchannels)
                        .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            _ => Err("Invalid HResamplerType or dtype.".into()),
        }
    }

    /// HResampler
    /// ## new_sinc
    ///
    /// `new_sinc(resample_ratio: double, max_resample_ratio_relative: double, parameters: HSincInterpolationParameters, chunk_size: integer, nchannels: integer, res_type: HResamplerType, dtype: HDataType) -> HResampler`
    ///
    /// Creates a new Sinc type HResampler.
    ///
    /// Supports any of  `[SincFixedIn, SincFixedOut]` `HResamplerType`.
    ///
    /// The resampling is done by creating a number of intermediate points (defined by oversampling_factor) by sinc interpolation.
    /// The new samples are then calculated by interpolating between these points.
    ///
    /// - `SincFixedIn`
    ///
    /// An asynchronous resampler that accepts a fixed number of audio frames for input and returns a variable number of frames.
    ///
    /// - `SincFixedOut`
    ///
    /// An asynchronous resampler that accepts a variable number of audio frames for input nad returns a fixed number of frames.
    ///
    /// #### Arguments
    ///
    /// - `resample_ratio`
    ///
    /// The output's sampling rate divided by the input's sampling rate.
    ///
    /// - `max_resample_ratio_relative`
    ///
    /// Maximum ratio that can be set with `set_resample_ratio` relative to `resample_ratio`, must be >= 1.0. The minimum relative
    /// ratio is the reciprocal of the maximum. For example, with `max_resample_ratio_relative` of 10.0, the ratio can be set between
    /// `resample_ratio * 10.0` and `resample_ratio / 10.0`.
    ///
    /// - `parameters`
    ///
    /// An `HSincInterpolationParameters`. Parameters for interpolation.
    ///
    /// - `chunk_size`
    ///
    /// Chunks size of input or output data in frames.
    ///
    /// - `nchannels`
    ///
    /// Number of channels in input and output.
    ///
    /// Must be the same number of channels as the `HAudio` that will be processed by the `HResampler`.
    ///
    /// - `res_type`
    ///
    /// An `HResamplerType`. Indicates which type of `HResampler` to be created.
    ///
    /// - `dtype`
    ///
    /// A float `HDataType` to indicate the dtype that the `HResampler` will be working with.
    ///
    /// Must be the same as the `HAudio`'s dtype that will be processed by the `HResampler`.
    ///
    /// #### Returns
    ///
    /// A Sinc type `HResampler`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// sr_in = 44100L
    /// sr_out = 48000L
    /// resample_ratio = sr_out / sr_in
    /// max_resample_ratio_relative = 2
    /// hparams = HSincInterpolationParameters$new(256, 0.95, 256, "linear", "blackmanharris2")
    /// chunk_size = 512L
    /// nchannels = 2L
    /// res_type = HResamplerType$SincFixedOut
    /// dtype = HDataType$Float32
    ///
    /// res = HResampler$new_sinc(resample_ratio, max_resample_ratio_relative, hparams, chunk_size, nchannels, res_type, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_sinc(
        resample_ratio: Sexp,
        max_resample_ratio_relative: Sexp,
        parameters: &HSincInterpolationParameters,
        chunk_size: Sexp,
        nchannels: Sexp,
        res_type: &HResamplerType,
        dtype: &HDataType,
    ) -> savvy::Result<HResampler> {
        let resample_ratio: f64 = resample_ratio.to_scalar()?;
        let max_resample_ratio_relative: f64 = max_resample_ratio_relative.to_scalar()?;
        let parameters = parameters.try_into()?;
        let chunk_size: i32 = chunk_size.to_scalar()?;
        let chunk_size = try_from_i32_to_usize(chunk_size)?;
        let nchannels: i32 = nchannels.to_scalar()?;
        let nchannels = try_from_i32_to_usize(nchannels)?;

        match (res_type, dtype) {
            (HResamplerType::SincFixedIn, HDataType::Float32) => {
                let resampler = SincFixedIn::<f32>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    parameters,
                    chunk_size,
                    nchannels,
                )
                .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::SincFixedIn, HDataType::Float64) => {
                let resampler = SincFixedIn::<f64>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    parameters,
                    chunk_size,
                    nchannels,
                )
                .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::SincFixedOut, HDataType::Float32) => {
                let resampler = SincFixedOut::<f32>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    parameters,
                    chunk_size,
                    nchannels,
                )
                .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::SincFixedOut, HDataType::Float64) => {
                let resampler = SincFixedOut::<f64>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    parameters,
                    chunk_size,
                    nchannels,
                )
                .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            _ => Err("Invalid HResamplerType or dtype.".into()),
        }
    }

    /// HResampler
    /// ## new_fast
    ///
    /// `new_sinc(resample_ratio: double, max_resample_ratio_relative: double, pol_deg: HPolynomialDegree, chunk_size: integer, nchannels: integer, res_type: HResamplerType, dtype: HDataType) -> HResampler`
    ///
    /// Creates a new Fast type HResampler.
    ///
    /// Supports any of  `[FastFixedIn, FastFixedOut]` `HResamplerType`.
    ///
    /// The resampling is done by interpolating between the input samples by fitting polynomials.
    ///
    /// Note that no anti-aliasing filter is used. This makes it run considerably faster than the corresponding `SincFixedIn`, which performs anti-aliasing filtering. The price is that the resampling creates some artefacts
    /// in the output, mainly at higher frequencies. Use `SincFixedIn` if this can not be tolerated.
    ///
    /// - `FastFixedIn`
    ///
    /// An asynchronous resampler that accepts a fixed number of audio frames for input and returns a variable number of frames.
    ///
    /// - `FastFixedOut`
    ///
    /// An asynchronous resampler that accepts a variable number of audio frames for input nad returns a fixed number of frames.
    ///
    /// #### Arguments
    ///
    /// - `resample_ratio`
    ///
    /// The output's sampling rate divided by the input's sampling rate.
    ///
    /// - `max_resample_ratio_relative`
    ///
    /// Maximum ratio that can be set with `set_resample_ratio` relative to `resample_ratio`, must be >= 1.0. The minimum relative
    /// ratio is the reciprocal of the maximum. For example, with `max_resample_ratio_relative` of 10.0, the ratio can be set between
    /// `resample_ratio * 10.0` and `resample_ratio / 10.0`.
    ///
    /// - `pol_deg`
    ///
    /// An `HPolynomialDegree`. Used to select the polynomial degree for interpolation.
    ///
    /// - `chunk_size`
    ///
    /// Chunks size of input or output data in frames.
    ///
    /// - `nchannels`
    ///
    /// Number of channels in input and output.
    ///
    /// Must be the same number of channels as the `HAudio` that will be processed by the `HResampler`.
    ///
    /// - `res_type`
    ///
    /// An `HResamplerType`. Indicates which type of `HResampler` to be created.
    ///
    /// - `dtype`
    ///
    /// A float `HDataType` to indicate the dtype that the `HResampler` will be working with.
    ///
    /// Must be the same as the `HAudio`'s dtype that will be processed by the `HResampler`.
    ///
    /// #### Returns
    ///
    /// A Fast type `HResampler`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// sr_in = 44100L
    /// sr_out = 48000L
    /// resample_ratio = sr_out / sr_in
    /// max_resample_ratio_relative = 2
    /// pol_deg = HPolynomialDegree$linear
    /// chunk_size = 512L
    /// nchannels = 2L
    /// res_type = HResamplerType$FastFixedOut
    /// dtype = HDataType$Float32
    ///
    /// res = HResampler$new_fast(resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nchannels, res_type, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_fast(
        resample_ratio: Sexp,
        max_resample_ratio_relative: Sexp,
        pol_deg: &HPolynomialDegree,
        chunk_size: Sexp,
        nchannels: Sexp,
        res_type: &HResamplerType,
        dtype: &HDataType,
    ) -> savvy::Result<HResampler> {
        let resample_ratio: f64 = resample_ratio.to_scalar()?;
        let max_resample_ratio_relative: f64 = max_resample_ratio_relative.to_scalar()?;
        let chunk_size: i32 = chunk_size.to_scalar()?;
        let chunk_size = try_from_i32_to_usize(chunk_size)?;
        let nchannels: i32 = nchannels.to_scalar()?;
        let nchannels = try_from_i32_to_usize(nchannels)?;

        match (res_type, dtype) {
            (HResamplerType::FastFixedIn, HDataType::Float32) => {
                let resampler = FastFixedIn::<f32>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    pol_deg.into(),
                    chunk_size,
                    nchannels,
                )
                .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::FastFixedIn, HDataType::Float64) => {
                let resampler = FastFixedIn::<f64>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    pol_deg.into(),
                    chunk_size,
                    nchannels,
                )
                .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::FastFixedOut, HDataType::Float32) => {
                let resampler = FastFixedOut::<f32>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    pol_deg.into(),
                    chunk_size,
                    nchannels,
                )
                .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            (HResamplerType::FastFixedOut, HDataType::Float64) => {
                let resampler = FastFixedOut::<f64>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    pol_deg.into(),
                    chunk_size,
                    nchannels,
                )
                .map_err(|err| HErrorR::from(HError::from(err)))?;
                Ok(HResampler(Box::new(resampler)))
            }
            _ => Err("Invalid HResamplerType or dtype.".into()),
        }
    }

    /// HResampler
    /// ## process
    ///
    /// `process(harray: HArray)`
    ///
    /// Process the resampler, changing the `HArray`'s sampling rate.
    ///
    /// #### Arguments
    ///
    /// - `harray`
    ///
    /// An `HArray` that will have it's sampling rate converted.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = matrix(0, nrow = 512, ncol = 2)
    /// harray = HArray$new_from_values(arr, dtype = HDataType$Float64)
    /// hparams = HSincInterpolationParameters$new(256L, 0.95, 256L, "linear", "blackmanharris2")
    /// res = HResampler$new_sinc(48000L / 44100L, 2, hparams, 512L, 2L, HResamplerType$SincFixedIn, HDataType$Float64)
    /// res$process(harray)
    /// ```
    ///
    /// _________
    ///
    fn process(&mut self, harray: &mut HArray) -> savvy::Result<()> {
        self.0.process(harray)
    }

    /// HResampler
    /// ## set_resample_ratio
    ///
    /// `set_resample_ratio(new_ratio: double, ramp: bool)`
    ///
    /// Update the resample ratio.
    ///
    /// For asynchronous resamplers, the ratio must be within `original / maximum` to `original * maximum`, where `original` and `maximum` are the resampling ratios that were provided to the constructor.
    /// Trying to set the ratio outside these bounds will return an error.
    ///
    /// For synchronous resamplers, this will always return an error.
    ///
    /// #### Arguments
    ///
    /// - `new_ratio`
    ///
    /// The new `resample_ratio` to be set.
    ///
    /// - `ramp`
    ///
    /// If `TRUE`, the ratio will be ramped from the old to the new value during processing of the next chunk. This allows smooth transitions from one ratio to another.
    /// If `FALSE`, the new ratio will be applied from the start of the next chunk.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// data = matrix(0, nrow = 512, ncol = 2)
    /// haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$Float64)
    /// hparams = HSincInterpolationParameters$new(256L, 0.95, 256L, "linear", "blackmanharris2")
    /// res = HResampler$new_sinc(48000L / 44100L, 2, hparams, 512L, 2L, HResamplerType$SincFixedIn, HDataType$Float64)
    /// res$set_resample_ratio(1, FALSE)
    /// ```
    ///
    /// _________
    ///
    fn set_resample_ratio(&mut self, new_ratio: Sexp, ramp: Sexp) -> savvy::Result<()> {
        let new_ratio: f64 = new_ratio.to_scalar()?;
        let ramp: bool = ramp.to_scalar()?;
        self.0.set_resample_ratio(new_ratio, ramp)
    }

    /// HResampler
    /// ## set_resample_ratio_relative
    ///
    /// `set_resample_ratio_relative(rel_ratio: double, ramp: bool)`
    ///
    /// Update the resample ratio as a factor relative to the original one.
    ///
    /// For asynchronous resamplers, the relative ratio must be within `1 / maximum` to `maximum`, where `maximum` is the maximum resampling ratio that
    /// was provided to the constructor. Trying to set the ratio outside these bounds will return an error.
    /// Higher ratios above `1.0` slow down the output and lower the pitch. Lower ratios below `1.0` speed up the output and raise the pitch.
    ///
    /// For synchronous resamplers, this will always return an error.
    ///
    /// #### Arguments
    ///
    /// - `rel_ratio`: A factor to update the resample_ratio relative to the original one.
    ///
    /// - `ramp`: If `TRUE`, the ratio will be ramped from the old to the new value during processing of the next chunk. This allows smooth transitions
    /// from one ratio to another. If ramp is false, the new ratio will be applied from the start of the next chunk.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// data = matrix(0, nrow = 512, ncol = 2)
    /// haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$Float64)
    /// hparams = HSincInterpolationParameters$new(256L, 0.95, 256L, "linear", "blackmanharris2")
    /// res = HResampler$new_sinc(48000L / 44100L, 2, hparams, 512L, 2L, HResamplerType$SincFixedIn, HDataType$Float64)
    /// res$set_resample_ratio_relative(0.5, FALSE)
    /// ```
    ///
    /// _________
    ///
    fn set_resample_ratio_relative(&mut self, rel_ratio: Sexp, ramp: Sexp) -> savvy::Result<()> {
        let rel_ratio: f64 = rel_ratio.to_scalar()?;
        let ramp: bool = ramp.to_scalar()?;
        self.0.set_resample_ratio_relative(rel_ratio, ramp)
    }

    /// HResampler
    /// ## nbr_channels
    ///
    /// Get the maximum number of channels this Resampler is configured for.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// hparams = HSincInterpolationParameters$new(256L, 0.95, 256L, "linear", "blackmanharris2")
    /// res = HResampler$new_sinc(48000L / 44100L, 2, hparams, 512L, 2L, HResamplerType$SincFixedIn, HDataType$Float64)
    /// res$nbr_channels()
    /// ```
    ///
    /// _________
    ///
    //fn nbr_channels(&self) -> savvy::Result<OwnedIntegerSexp> {
    //    self.0.nbr_channels()
    //}

    /// HResampler
    /// ## set_chunk_size
    ///
    /// `set_chunk_size(chunk_size: integer)`
    ///
    /// Change the chunk size for the resampler. This is not supported by all resampler types. The value must be equal to or smaller than
    /// the chunk size the value that the resampler was created with. ResampleError::InvalidChunkSize is returned if the value is zero or too large.
    ///
    /// The meaning of chunk size depends on the resampler, it refers to the input size for FixedIn, and output size for FixedOut types.
    ///
    /// Types that do not support changing the chunk size return ResampleError::ChunkSizeNotAdjustable.
    ///
    /// #### Arguments
    ///
    /// - `chunk_size`: The new `chunk_size` to be set.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// hparams = HSincInterpolationParameters$new(256L, 0.95, 256L, "linear", "blackmanharris2")
    /// res = HResampler$new_sinc(48000L / 44100L, 2, hparams, 512L, 2L, HResamplerType$SincFixedIn, HDataType$Float64)
    /// res$set_chunk_size(1024L)
    /// ```
    ///
    /// _________
    ///
    //fn set_chunk_size(&mut self, chunk_size: Sexp) -> savvy::Result<()> {
    //    let chunk_size: i32 = chunk_size.to_scalar()?;
    //    let chunk_size = try_from_i32_to_usize(chunk_size)?;
    //    self.0.set_chunk_size(chunk_size)
    //}

    /// HResampler
    /// ## reset
    ///
    /// `reset()`
    ///
    /// Reset the resampler state and clear all internal buffers.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// sr_in = 44100L
    /// sr_out = 48000L
    /// resample_ratio = sr_out / sr_in
    /// max_resample_ratio_relative = 2
    /// pol_deg = HPolynomialDegree$linear
    /// chunk_size = 512L
    /// nchannels = 2L
    /// res_type = HResamplerType$FastFixedOut
    /// dtype = HDataType$Float32
    ///
    /// res = HResampler$new_fast(resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nchannels, res_type, dtype)
    /// res$reset()
    /// ```
    ///
    /// _________
    ///
    fn reset(&mut self) -> savvy::Result<()> {
        self.0.reset()
    }

    /// HResampler
    /// ## res_type
    ///
    /// `res_type() -> HResamplerType`
    ///
    /// Gets the `HResampler`'s type.
    ///
    /// #### Returns
    ///
    /// An `HResamplerType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// sr_in = 44100L
    /// sr_out = 48000L
    /// resample_ratio = sr_out / sr_in
    /// max_resample_ratio_relative = 2
    /// pol_deg = HPolynomialDegree$linear
    /// chunk_size = 512L
    /// nchannels = 2L
    /// res_type = HResamplerType$FastFixedOut
    /// dtype = HDataType$Float32
    ///
    /// res = HResampler$new_fast(resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nchannels, res_type, dtype)
    /// res$res_type()
    /// ```
    ///
    /// _________
    ///
    fn res_type(&self) -> savvy::Result<HResamplerType> {
        self.0.res_type()
    }

    /// HResampler
    /// ## dtype
    ///
    /// `dtype() -> HDataType`
    ///
    /// Gets the `HResampler`'s dtype.
    ///
    /// #### Returns
    ///
    /// An `HDataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// sr_in = 44100L
    /// sr_out = 48000L
    /// resample_ratio = sr_out / sr_in
    /// max_resample_ratio_relative = 2
    /// pol_deg = HPolynomialDegree$linear
    /// chunk_size = 512L
    /// nchannels = 2L
    /// res_type = HResamplerType$FastFixedOut
    /// dtype = HDataType$Float32
    ///
    /// res = HResampler$new_fast(resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nchannels, res_type, dtype)
    /// res$dtype()
    /// ```
    ///
    /// _________
    ///
    fn dtype(&self) -> savvy::Result<HDataType> {
        self.0.dtype()
    }

    /// HResampler
    /// ## print
    ///
    /// `print()`
    ///
    /// Prints the `HResampler`.
    ///
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// sr_in = 44100L
    /// sr_out = 48000L
    /// resample_ratio = sr_out / sr_in
    /// max_resample_ratio_relative = 2
    /// pol_deg = HPolynomialDegree$linear
    /// chunk_size = 512L
    /// nchannels = 2L
    /// res_type = HResamplerType$FastFixedOut
    /// dtype = HDataType$Float32
    ///
    /// res = HResampler$new_fast(resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nchannels, res_type, dtype)
    /// res$print()
    ///
    /// # or similarly:
    /// print(res)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        self.0.print()
    }
}

macro_rules! impl_hresampler {
    ($(($t1:ty, $t2:ty, $e1:expr, $e2: expr, $e3:expr)),+) => {
        $(
            impl HResamplerR for $t1 {
                fn process(&mut self, harray: &mut HArray) -> savvy::Result<()> {
                    let harray = harray.get_inner_mut().as_any_mut().downcast_mut::<$t2>().ok_or_else(|| savvy::Error::new("HAudio and HResampler must have the same HDataType."))?;
                    self.process_resampler(harray).map_err(|err| savvy::Error::from(HErrorR::from(err)))
                }

                fn set_resample_ratio(&mut self, new_ratio: f64, ramp: bool) -> savvy::Result<()> {
                    rubato::Resampler::set_resample_ratio(self, new_ratio, ramp).map_err(to_savvy_error)
                }

                fn set_resample_ratio_relative(&mut self, rel_ratio: f64, ramp: bool) -> savvy::Result<()> {
                    rubato::Resampler::set_resample_ratio_relative(self, rel_ratio, ramp).map_err(to_savvy_error)
                }

                fn nbr_channels(&self) -> savvy::Result<OwnedIntegerSexp> {
                    let nbr_channels = rubato::Resampler::nbr_channels(self);
                    try_from_usize_to_int_sexp(nbr_channels)
                }

                fn set_chunk_size(&mut self, chunk_size: usize) -> savvy::Result<()> {
                    rubato::Resampler::set_chunk_size(self, chunk_size).map_err(to_savvy_error)
                }

                fn res_type(&self) -> savvy::Result<HResamplerType> {
                    Ok($e1)
                }

                fn reset(&mut self) -> savvy::Result<()> {
                    rubato::Resampler::reset(self);
                    Ok(())
                }

                fn dtype(&self) -> savvy::Result<HDataType> {
                    Ok($e2)
                }

                fn print(&self) -> savvy::Result<()> {
                    r_println!($e3);
                    Ok(())
                }
            }
        )+
    };
}

impl_hresampler!(
    (
        FftFixedIn<f32>,
        harmonium_core::array::HArray<f32, IxDyn>,
        HResamplerType::FftFixedIn,
        HDataType::Float32,
        "FftFixedIn<f32>"
    ),
    (
        FftFixedIn<f64>,
        harmonium_core::array::HArray<f64, IxDyn>,
        HResamplerType::FftFixedIn,
        HDataType::Float64,
        "FftFixedIn<f64>"
    ),
    (
        FftFixedInOut<f32>,
        harmonium_core::array::HArray<f32, IxDyn>,
        HResamplerType::FftFixedInOut,
        HDataType::Float32,
        "FftFixedInOut<f32>"
    ),
    (
        FftFixedInOut<f64>,
        harmonium_core::array::HArray<f64, IxDyn>,
        HResamplerType::FftFixedInOut,
        HDataType::Float64,
        "FftFixedInOut<f64>"
    ),
    (
        FftFixedOut<f32>,
        harmonium_core::array::HArray<f32, IxDyn>,
        HResamplerType::FftFixedOut,
        HDataType::Float32,
        "FftFixedOut<f32>"
    ),
    (
        FftFixedOut<f64>,
        harmonium_core::array::HArray<f64, IxDyn>,
        HResamplerType::FftFixedOut,
        HDataType::Float64,
        "FftFixedOut<f64>"
    ),
    (
        SincFixedIn<f32>,
        harmonium_core::array::HArray<f32, IxDyn>,
        HResamplerType::SincFixedIn,
        HDataType::Float32,
        "SincFixedIn<f32>"
    ),
    (
        SincFixedIn<f64>,
        harmonium_core::array::HArray<f64, IxDyn>,
        HResamplerType::SincFixedIn,
        HDataType::Float64,
        "SincFixedIn<f64>"
    ),
    (
        SincFixedOut<f32>,
        harmonium_core::array::HArray<f32, IxDyn>,
        HResamplerType::SincFixedOut,
        HDataType::Float32,
        "SincFixedOut<f32>"
    ),
    (
        SincFixedOut<f64>,
        harmonium_core::array::HArray<f64, IxDyn>,
        HResamplerType::SincFixedOut,
        HDataType::Float64,
        "SincFixedOut<f64>"
    ),
    (
        FastFixedIn<f32>,
        harmonium_core::array::HArray<f32, IxDyn>,
        HResamplerType::FastFixedIn,
        HDataType::Float32,
        "FastFixedIn<f32>"
    ),
    (
        FastFixedIn<f64>,
        harmonium_core::array::HArray<f64, IxDyn>,
        HResamplerType::FastFixedIn,
        HDataType::Float64,
        "FastFixedIn<f64>"
    ),
    (
        FastFixedOut<f32>,
        harmonium_core::array::HArray<f32, IxDyn>,
        HResamplerType::FastFixedOut,
        HDataType::Float32,
        "FastFixedOut<f32>"
    ),
    (
        FastFixedOut<f64>,
        harmonium_core::array::HArray<f64, IxDyn>,
        HResamplerType::FastFixedOut,
        HDataType::Float64,
        "FastFixedOut<f64>"
    )
);
