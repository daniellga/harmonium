use crate::{
    haudiodynamic::HAudio, hdatatype::HDataType, hinterpolationparams::HInterpolationParams,
    hresamplertype::HResamplerType,
};
use extendr_api::prelude::*;
use harmonium_core::structs::HFloatAudio;
use harmonium_resample::resample::ProcessResampler;
use rubato::{FftFixedIn, FftFixedInOut, FftFixedOut, SincFixedIn, SincFixedOut};
use std::any::Any;

pub trait HResamplerR: Send {
    fn as_any(&self) -> &dyn Any;
    fn process(&mut self, haudio: &mut HAudio, sr_out: i32);
    fn set_resample_ratio(&mut self, new_ratio: f64);
    fn set_resample_ratio_relative(&mut self, rel_ratio: f64);
    fn resampler_type(&self) -> HResamplerType;
    fn data_type(&self) -> HDataType;
    fn print(&self);
}

pub struct HResampler(pub Box<dyn HResamplerR>);

#[extendr]
impl HResampler {
    fn new_fft(
        sr_in: i32,
        sr_out: i32,
        chunk_size: i32,
        sub_chunks: i32,
        nbr_channels: i32,
        resampler_type: &HResamplerType,
        data_type: &HDataType,
    ) -> HResampler {
        match (resampler_type, data_type) {
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
        parameters: &HInterpolationParams,
        chunk_size: i32,
        nbr_channels: i32,
        resampler_type: &HResamplerType,
        data_type: &HDataType,
    ) -> HResampler {
        match (resampler_type, data_type) {
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

    fn process(&mut self, haudio: &mut HAudio, sr_out: i32) {
        assert!(self.data_type() == haudio.data_type());

        self.0.process(haudio, sr_out);
    }

    fn resampler_type(&self) -> HResamplerType {
        self.0.resampler_type()
    }

    fn data_type(&self) -> HDataType {
        self.0.data_type()
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

                fn set_resample_ratio(&mut self, _: f64) {
                    panic!("not available for fft resamplers");
                }

                fn set_resample_ratio_relative(&mut self, _: f64) {
                    panic!("not available for fft resamplers");
                }

                fn resampler_type(&self) -> HResamplerType {
                    $e1
                }

                fn data_type(&self) -> HDataType {
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

                fn set_resample_ratio(&mut self, new_ratio: f64) {
                    rubato::Resampler::set_resample_ratio(self, new_ratio).unwrap();
                }

                fn set_resample_ratio_relative(&mut self, rel_ratio: f64) {
                    rubato::Resampler::set_resample_ratio_relative(self, rel_ratio).unwrap();
                }

                fn resampler_type(&self) -> HResamplerType {
                    $e1
                }

                fn data_type(&self) -> HDataType {
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
    )
);

extendr_module! {
    mod hresampler;
    impl HResampler;
}
