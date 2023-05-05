use extendr_api::prelude::*;
use harmonium_core::structs::HFloatAudio;
use harmonium_resample::resample2::ProcessResampler;
use rubato::{FftFixedIn, FftFixedInOut, FftFixedOut, SincFixedIn};
use std::{any::Any, sync::Arc};

use crate::{
    haudiodynamic::{HAudio, HAudioR},
    hdatatype::HDataType,
    hresamplertype::HResamplerType,
};

pub trait HResamplerFftR: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn process(&mut self, haudio: &mut dyn HAudioR, sr_out: i32);
    fn resampler_type(&self) -> HResamplerType;
    fn data_type(&self) -> HDataType;
    fn print(&self);
    fn clone_inner(&self) -> Arc<dyn HResamplerFftR>;
}

pub trait HResamplerSincR: Send {
    fn as_any(&self) -> &dyn Any;
    fn process(&mut self, haudio: &mut dyn HAudioR, sr_out: i32);
    fn resampler_type(&self) -> HResamplerType;
    fn data_type(&self) -> HDataType;
    fn print(&self);
    fn clone_inner(&self) -> Arc<dyn HResamplerSincR>;
}

pub struct HResamplerFft(pub Arc<dyn HResamplerFftR>);
pub struct HResamplerSinc(pub Arc<dyn HResamplerSincR>);

#[extendr]
impl HResamplerFft {
    fn new(
        sr_in: i32,
        sr_out: i32,
        chunks: i32,
        sub_chunks: i32,
        nbr_channels: i32,
        resampler_type: &HResamplerType,
        data_type: &HDataType,
    ) -> HResamplerFft {
        match (resampler_type, data_type) {
            (HResamplerType::FftFixedIn, HDataType::Float32) => {
                let fft_fixed_in = FftFixedIn::<f32>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunks.try_into().unwrap(),
                    sub_chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResamplerFft(Arc::new(fft_fixed_in))
            }
            (HResamplerType::FftFixedIn, HDataType::Float64) => {
                let fft_fixed_in = FftFixedIn::<f64>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunks.try_into().unwrap(),
                    sub_chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResamplerFft(Arc::new(fft_fixed_in))
            }
            (HResamplerType::FftFixedInOut, HDataType::Float32) => {
                let fft_fixed_in_out = FftFixedInOut::<f32>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResamplerFft(Arc::new(fft_fixed_in_out))
            }
            (HResamplerType::FftFixedInOut, HDataType::Float64) => {
                let fft_fixed_in_out = FftFixedInOut::<f64>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResamplerFft(Arc::new(fft_fixed_in_out))
            }
            (HResamplerType::FftFixedOut, HDataType::Float32) => {
                let fft_fixed_out = FftFixedOut::<f32>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunks.try_into().unwrap(),
                    sub_chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResamplerFft(Arc::new(fft_fixed_out))
            }
            (HResamplerType::FftFixedOut, HDataType::Float64) => {
                let fft_fixed_out = FftFixedOut::<f64>::new(
                    sr_in.try_into().unwrap(),
                    sr_out.try_into().unwrap(),
                    chunks.try_into().unwrap(),
                    sub_chunks.try_into().unwrap(),
                    nbr_channels.try_into().unwrap(),
                )
                .unwrap();
                HResamplerFft(Arc::new(fft_fixed_out))
            }
            _ => panic!("The resampler type doesn't match."),
        }
    }

    fn process(&mut self, haudio: &mut HAudio, sr_out: i32) {
        assert!(self.data_type() == haudio.data_type());
        let inner_mut = self.get_inner_mut();
        let haudio_inner_mut = haudio.get_inner_mut();
        inner_mut.process(haudio_inner_mut, sr_out);
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

impl HResamplerFft {
    #[doc(hidden)]
    pub fn get_inner_mut(&mut self) -> &mut dyn HResamplerFftR {
        if Arc::weak_count(&self.0) + Arc::strong_count(&self.0) != 1 {
            self.0 = self.0.clone_inner();
        }
        Arc::get_mut(&mut self.0).expect("implementation error")
    }
}

impl HResamplerFftR for FftFixedIn<f32> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn process(&mut self, haudio: &mut dyn HAudioR, sr_out: i32) {
        let sr_out = sr_out.try_into().unwrap();
        let haudio = haudio.as_any().downcast_mut::<HFloatAudio<f32>>().unwrap();
        self.process_resampler(haudio, sr_out);
    }

    fn resampler_type(&self) -> HResamplerType {
        HResamplerType::FftFixedIn
    }

    fn data_type(&self) -> HDataType {
        HDataType::Float32
    }

    fn print(&self) {
        rprintln!("FftFixedIn<f32>");
    }

    fn clone_inner(&self) -> Arc<dyn HResamplerFftR> {
        Arc::new(self.clone())
    }
}
macro_rules! impl_hresamplerfftr {
    ($(($t1:ty, $t2:ty, $t3:ty, $id:expr)),+) => {
        $(
        )+
    };
}

impl_hresamplerfftr!((
    FftFixedIn<f32>,
    HFloatAudio<f32>,
    HDataType::Float32,
    "FftFixedIn<f32>"
));

impl HResamplerFftR for FftFixedIn<f64> {}
impl HResamplerFftR for FftFixedInOut<f32> {}
impl HResamplerFftR for FftFixedInOut<f64> {}
impl HResamplerFftR for FftFixedOut<f32> {}
impl HResamplerFftR for FftFixedOut<f64> {}
impl HResamplerSincR for SincFixedIn<f32> {}
impl HResamplerSincR for SincFixedIn<f64> {}

extendr_module! {
    mod hresampler;
    impl HResamplerFft;
}
