use crate::{
    haudio::HAudio, hdatatype::HDataType, hpolynomialdegree::HPolynomialDegree,
    hresamplertype::HResamplerType, hsincinterpolationparams::HSincInterpolationParams,
};
use extendr_api::prelude::*;
use harmonium_core::structs::HFloatAudio;
use harmonium_resample::resample::ProcessResampler;
use rubato::{
    FastFixedIn, FastFixedOut, FftFixedIn, FftFixedInOut, FftFixedOut, SincFixedIn, SincFixedOut,
};
use std::any::Any;

pub trait HResamplerR: Send {
    fn as_any(&self) -> &dyn Any;
    fn process(&mut self, haudio: &mut HAudio, sr_out: i32);
    fn set_resample_ratio(&mut self, new_ratio: f64, ramp: bool);
    fn set_resample_ratio_relative(&mut self, rel_ratio: f64, ramp: bool);
    fn resampler_type(&self) -> HResamplerType;
    fn dtype(&self) -> HDataType;
    fn print(&self);
}

/// HResampler
/// A resampler. \
///
/// #### Asynchronous Resamplers
///
/// The resampling is based on band-limited interpolation using sinc interpolation filters. The sinc interpolation upsamples by an adjustable factor,
/// and then the new sample points are calculated by interpolating between these points. \
/// The resampling ratio can be updated at any time. \
///
/// * `SincFixedIn` \
///
/// * `SincFixedOut` \
///
/// * `FastFixedIn` \
///
/// * `FastFixedOut` \
///
/// #### Synchronous Resamplers
///
/// Is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data. \
/// This type of resampler is considerably faster but doesn’t support changing the resampling ratio. \
///
/// * `FftFixedIn` \
///
/// * `FftFixedInOut` \
///
/// * `FftFixedOut` \
///
/// # Methods
///
pub struct HResampler(pub Box<dyn HResamplerR>);

#[extendr]
impl HResampler {
    /// HResampler
    /// ## new_fft
    ///
    /// `new_fft(sr_in: integer, sr_out: integer, chunk_size: integer, sub_chunks: integer, nbr_channels: integer, resampler_type: HResamplerType, dtype: HDataType) -> HArray` \
    ///
    /// Creates a new `FFT` type HResampler. \
    /// Supportes any of  `[fft_fixed_in, fft_fixed_in_out, fft_fixed_out]` `HResamplerType`. \
    ///
    /// * `fft_fixed_in` \
    /// A synchronous resampler that needs a fixed number of audio frames for input and returns a variable number of frames. \
    /// The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing
    /// filter before it’s inverse transformed to get the resampled waveforms. \
    ///
    /// * `fft_fixed_in_out` \
    /// A synchronous resampler that accepts a fixed number of audio frames for input and returns a fixed number of frames. \
    /// The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter
    /// before it’s inverse transformed to get the resampled waveforms. \
    ///
    /// * `fft_fixed_out` \
    /// A synchronous resampler that needs a fixed number of audio frames for input and returns a variable number of frames.
    ///
    /// #### Arguments
    ///
    /// * `sr_in` \
    /// The input sampling rate in hz. \
    /// * `sr_out` \
    /// The output sampling rate in hz. \
    /// * `chunk_size` \
    /// Chunks size of input or output data in frames. \
    /// It can be used as input or output, depending on `HResamplerType`. \
    /// * `sub_chunks` \
    /// Desired number of subchunks for processing, actual number may be different. \
    /// * `nbr_channels` \
    /// Number of channels in input and output. \
    /// Must be the same number of channels as the `HAudio` that will be processed by the `HResampler`. \
    /// * `resampler_type` \
    /// An HResamplerType to indicate which type of `HResampler` to be created. \
    /// * `dtype` \
    /// A float `HDataType` to indicate the data type the `HResampler` will be working with. \
    /// Must be the same as the `HAudio`'s dtype that will be processed by the `HResampler`. \
    ///
    /// #### Returns
    ///
    /// A FFT type `HResampler`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// sr_in = 48000L
    /// sr_out = 44100L
    /// chunk_size = 1024L
    /// sub_chunks = 2L
    /// nbr_channels = 2L
    /// resampler_type = HResamplerType$fft_fixed_in
    /// dtype = HDataType$float32
    ///
    /// hresampler = HResampler$new_fft(sr_in, sr_out, chunk_size, sub_chunks, nbr_channels, resampler_type, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_fft(
        sr_in: i32,
        sr_out: i32,
        chunk_size: i32,
        sub_chunks: i32,
        nbr_channels: i32,
        resampler_type: &HResamplerType,
        dtype: &HDataType,
    ) -> HResampler {
        match (resampler_type, dtype) {
            (HResamplerType::FftFixedIn, HDataType::Float32) => {
                let resampler = FftFixedIn::<f32>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunk_size.try_into().unwrap(),
                    sub_chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::FftFixedIn, HDataType::Float64) => {
                let resampler = FftFixedIn::<f64>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunk_size.try_into().unwrap(),
                    sub_chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::FftFixedInOut, HDataType::Float32) => {
                let resampler = FftFixedInOut::<f32>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::FftFixedInOut, HDataType::Float64) => {
                let resampler = FftFixedInOut::<f64>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::FftFixedOut, HDataType::Float32) => {
                let resampler = FftFixedOut::<f32>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunk_size.try_into().unwrap(),
                    sub_chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::FftFixedOut, HDataType::Float64) => {
                let resampler = FftFixedOut::<f64>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunk_size.try_into().unwrap(),
                    sub_chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            _ => panic!("Invalid resampler or data type."),
        }
    }

    fn new_sinc(
        resample_ratio: f64,
        max_resample_ratio_relative: f64,
        parameters: &HSincInterpolationParams,
        chunk_size: i32,
        nbr_channels: i32,
        resampler_type: &HResamplerType,
        dtype: &HDataType,
    ) -> HResampler {
        match (resampler_type, dtype) {
            (HResamplerType::SincFixedIn, HDataType::Float32) => {
                let resampler = SincFixedIn::<f32>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    parameters.into(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::SincFixedIn, HDataType::Float64) => {
                let resampler = SincFixedIn::<f64>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    parameters.into(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::SincFixedOut, HDataType::Float32) => {
                let resampler = SincFixedOut::<f32>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    parameters.into(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::SincFixedOut, HDataType::Float64) => {
                let resampler = SincFixedOut::<f64>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    parameters.into(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            _ => panic!("Invalid resampler or data type."),
        }
    }

    fn new_fast(
        resample_ratio: f64,
        max_resample_ratio_relative: f64,
        pol_deg: &HPolynomialDegree,
        chunk_size: i32,
        nbr_channels: i32,
        resampler_type: &HResamplerType,
        dtype: &HDataType,
    ) -> HResampler {
        match (resampler_type, dtype) {
            (HResamplerType::FastFixedIn, HDataType::Float32) => {
                let resampler = FastFixedIn::<f32>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    pol_deg.into(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::FastFixedIn, HDataType::Float64) => {
                let resampler = FastFixedIn::<f64>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    pol_deg.into(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::FastFixedOut, HDataType::Float32) => {
                let resampler = FastFixedOut::<f32>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    pol_deg.into(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            (HResamplerType::FastFixedOut, HDataType::Float64) => {
                let resampler = FastFixedOut::<f64>::new(
                    resample_ratio,
                    max_resample_ratio_relative,
                    pol_deg.into(),
                    chunk_size.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResampler(Box::new(resampler))
            }
            _ => panic!("Invalid resampler or data type."),
        }
    }

    fn process(&mut self, haudio: &mut HAudio, sr_out: i32) {
        self.0.process(haudio, sr_out);
    }

    fn resampler_type(&self) -> HResamplerType {
        self.0.resampler_type()
    }

    fn dtype(&self) -> HDataType {
        self.0.dtype()
    }

    fn print(&self) {
        self.0.print();
    }
}

macro_rules! impl_hresamplerfftr {
    ($(($t1:ty, $t2:ty, $e1:expr, $e2: expr, $e3:expr)),+) => {
        $(
            impl HResamplerR for $t1 {
                fn as_any(&self) -> &dyn Any {
                    self
                }

                fn process(&mut self, haudio: &mut HAudio, sr_out: i32) {
                    let sr_out = sr_out.try_into().unwrap();
                    let haudio = haudio.get_inner_mut().as_any_mut().downcast_mut::<$t2>().unwrap();
                    self.process_resampler(haudio, sr_out).unwrap();
                }

                fn set_resample_ratio(&mut self, new_ratio: f64, ramp: bool) {
                    panic!("not available for fft resamplers");
                }

                fn set_resample_ratio_relative(&mut self, rel_ratio: f64, ramp: bool) {
                    panic!("not available for fft resamplers");
                }

                fn resampler_type(&self) -> HResamplerType {
                    $e1
                }

                fn dtype(&self) -> HDataType {
                    $e2
                }

                fn print(&self) {
                    rprintln!($e3);
                }
            }
        )+
    };
}

impl_hresamplerfftr!(
    (
        FftFixedIn<f32>,
        HFloatAudio<f32>,
        HResamplerType::FftFixedIn,
        HDataType::Float32,
        "FftFixedIn<f32>"
    ),
    (
        FftFixedIn<f64>,
        HFloatAudio<f64>,
        HResamplerType::FftFixedIn,
        HDataType::Float64,
        "FftFixedIn<f64>"
    ),
    (
        FftFixedInOut<f32>,
        HFloatAudio<f32>,
        HResamplerType::FftFixedInOut,
        HDataType::Float32,
        "FftFixedInOut<f32>"
    ),
    (
        FftFixedInOut<f64>,
        HFloatAudio<f64>,
        HResamplerType::FftFixedInOut,
        HDataType::Float64,
        "FftFixedInOut<f64>"
    ),
    (
        FftFixedOut<f32>,
        HFloatAudio<f32>,
        HResamplerType::FftFixedOut,
        HDataType::Float32,
        "FftFixedOut<f32>"
    ),
    (
        FftFixedOut<f64>,
        HFloatAudio<f64>,
        HResamplerType::FftFixedOut,
        HDataType::Float64,
        "FftFixedOut<f64>"
    )
);

macro_rules! impl_hresamplersincr {
    ($(($t1:ty, $t2:ty, $e1:expr, $e2:expr, $e3: expr)),+) => {
        $(
            impl HResamplerR for $t1 {
                fn as_any(&self) -> &dyn Any {
                    self
                }

                fn process(&mut self, haudio: &mut HAudio, sr_out: i32) {
                    let sr_out = sr_out.try_into().unwrap();
                    let haudio = haudio.get_inner_mut().as_any_mut().downcast_mut::<$t2>().unwrap();
                    self.process_resampler(haudio, sr_out).unwrap();
                }

                fn set_resample_ratio(&mut self, new_ratio: f64, ramp: bool) {
                    rubato::Resampler::set_resample_ratio(self, new_ratio, ramp).unwrap();
                }

                fn set_resample_ratio_relative(&mut self, rel_ratio: f64, ramp: bool) {
                    rubato::Resampler::set_resample_ratio_relative(self, rel_ratio, ramp).unwrap();
                }

                fn resampler_type(&self) -> HResamplerType {
                    $e1
                }

                fn dtype(&self) -> HDataType {
                    $e2
                }

                fn print(&self) {
                    rprintln!($e3);
                }
            }
        )+
    };
}

impl_hresamplersincr!(
    (
        SincFixedIn<f32>,
        HFloatAudio<f32>,
        HResamplerType::SincFixedIn,
        HDataType::Float32,
        "SincFixedIn<f32>"
    ),
    (
        SincFixedIn<f64>,
        HFloatAudio<f64>,
        HResamplerType::SincFixedIn,
        HDataType::Float64,
        "SincFixedIn<f64>"
    ),
    (
        SincFixedOut<f32>,
        HFloatAudio<f32>,
        HResamplerType::SincFixedOut,
        HDataType::Float32,
        "SincFixedOut<f32>"
    ),
    (
        SincFixedOut<f64>,
        HFloatAudio<f64>,
        HResamplerType::SincFixedOut,
        HDataType::Float64,
        "SincFixedOut<f64>"
    ),
    (
        FastFixedIn<f32>,
        HFloatAudio<f32>,
        HResamplerType::FastFixedIn,
        HDataType::Float32,
        "FastFixedIn<f32>"
    ),
    (
        FastFixedIn<f64>,
        HFloatAudio<f64>,
        HResamplerType::FastFixedIn,
        HDataType::Float64,
        "FastFixedIn<f64>"
    ),
    (
        FastFixedOut<f32>,
        HFloatAudio<f32>,
        HResamplerType::FastFixedOut,
        HDataType::Float32,
        "FastFixedOut<f32>"
    ),
    (
        FastFixedOut<f64>,
        HFloatAudio<f64>,
        HResamplerType::FastFixedOut,
        HDataType::Float64,
        "FastFixedOut<f64>"
    )
);

extendr_module! {
    mod hresampler;
    impl HResampler;
}
