use crate::hdatatype::HDataType;
use extendr_api::{prelude::*, wrapper};
use harmonium_fft::fft::{FftComplex, FftFloat};
use num_complex::Complex;
use std::{any::Any, sync::Arc};

pub trait HArrayR: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn len(&self) -> i32;
    fn print(&self);
    fn collect(&self) -> Robj;
    fn dtype(&self) -> HDataType;
    fn fft(&self) -> Arc<dyn HArrayR>;
    fn fft_mut(&mut self);
    fn clone_inner(&self) -> Arc<dyn HArrayR>;
}

impl HArrayR for harmonium_core::array::HArray<f32> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn collect(&self) -> Robj {
        let dim = self
            .shape()
            .iter()
            .rev()
            .map(|z| Rint(*z as i32))
            .collect::<Integers>();

        let robj: Robj = self
            .0
            .iter()
            .map(|z| Rfloat(*z as f64))
            .collect::<Doubles>()
            .into();

        // Ok to unwrap.
        robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float32
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftFloat::<f32>::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        panic!();
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }
}

impl HArrayR for harmonium_core::array::HArray<f64> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn collect(&self) -> Robj {
        let dim = self
            .shape()
            .iter()
            .rev()
            .map(|z| Rint(*z as i32))
            .collect::<Integers>();

        let robj: Robj = self
            .0
            .iter()
            .map(|z| Rfloat(*z))
            .collect::<Doubles>()
            .into();

        // Ok to unwrap.
        robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float64
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftFloat::<f64>::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        panic!();
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }
}

impl HArrayR for harmonium_core::array::HArray<Complex<f32>> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn collect(&self) -> Robj {
        let dim = self
            .shape()
            .iter()
            .rev()
            .map(|z| Rint(*z as i32))
            .collect::<Integers>();

        let robj: Robj = self
            .0
            .iter()
            .map(|z| Rcplx::new(z.re as f64, z.im as f64))
            .collect::<Complexes>()
            .into();

        // Ok to unwrap.
        robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Complex32
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftComplex::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        FftComplex::fft_mut(self);
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }
}

impl HArrayR for harmonium_core::array::HArray<Complex<f64>> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn collect(&self) -> Robj {
        let dim = self
            .shape()
            .iter()
            .rev()
            .map(|z| Rint(*z as i32))
            .collect::<Integers>();

        let robj: Robj = self
            .0
            .iter()
            .map(|z| Rcplx::new(z.re, z.im))
            .collect::<Complexes>()
            .into();

        // Ok to unwrap.
        robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Complex64
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftComplex::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        FftComplex::fft_mut(self);
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }
}
