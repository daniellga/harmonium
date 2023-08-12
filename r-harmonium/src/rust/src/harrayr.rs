use crate::hdatatype::HDataType;
use extendr_api::{prelude::*, wrapper, AsTypedSlice};
use harmonium_core::haudioop::HAudioOpDyn;
use harmonium_fft::fft::{FftComplex, FftFloat};
use ndarray::IxDyn;
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
    fn nchannels(&self) -> usize;
    fn nframes(&self) -> usize;
    fn db_to_power(&mut self, reference: Robj);
    fn to_mono(&mut self);
}

impl HArrayR for harmonium_core::array::HArray<f32, IxDyn> {
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
            .map(|z| Rint::from(*z as i32))
            .collect::<Integers>();

        let robj: Robj = self
            .0
            .iter()
            .map(|z| Rfloat::from(*z as f64))
            .collect::<Doubles>()
            .into();

        // Ok to unwrap.
        robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float32
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftFloat::<f32, IxDyn>::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        panic!();
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }

    fn nchannels(&self) -> usize {
        HAudioOpDyn::nchannels(self)
    }

    fn nframes(&self) -> usize {
        HAudioOpDyn::nframes(self)
    }

    fn db_to_power(&mut self, reference: Robj) {
        assert!(reference.len() == 1);
        let reference: &[f64] = reference.as_typed_slice().unwrap();
        let reference = reference[0];
        HAudioOpDyn::db_to_power(self, reference as f32);
    }

    fn to_mono(&mut self) {
        *self = HAudioOpDyn::to_mono(self).unwrap();
    }
}

impl HArrayR for harmonium_core::array::HArray<f64, IxDyn> {
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
            .map(|z| Rint::from(*z as i32))
            .collect::<Integers>();

        let robj: Robj = self
            .0
            .iter()
            .map(|z| Rfloat::from(*z))
            .collect::<Doubles>()
            .into();

        // Ok to unwrap.
        robj.set_attrib(wrapper::symbol::dim_symbol(), dim).unwrap()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float64
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftFloat::<f64, IxDyn>::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        panic!();
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }

    fn nchannels(&self) -> usize {
        HAudioOpDyn::nchannels(self)
    }

    fn nframes(&self) -> usize {
        HAudioOpDyn::nframes(self)
    }

    fn db_to_power(&mut self, reference: Robj) {
        assert!(reference.len() == 1);
        let reference: &[f64] = reference.as_typed_slice().unwrap();
        let reference = reference[0];
        HAudioOpDyn::db_to_power(self, reference);
    }

    fn to_mono(&mut self) {
        *self = HAudioOpDyn::to_mono(self).unwrap();
    }
}

impl HArrayR for harmonium_core::array::HArray<Complex<f32>, IxDyn> {
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
            .map(|z| Rint::from(*z as i32))
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

    fn nchannels(&self) -> usize {
        panic!("Operation only allowed for float HArrays");
    }

    fn nframes(&self) -> usize {
        panic!("Operation only allowed for float HArrays");
    }

    fn db_to_power(&mut self, _: Robj) {
        panic!("Operation only allowed for float HArrays");
    }

    fn to_mono(&mut self) {
        panic!("Operation only allowed for float HArrays");
    }
}

impl HArrayR for harmonium_core::array::HArray<Complex<f64>, IxDyn> {
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
            .map(|z| Rint::from(*z as i32))
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

    fn nchannels(&self) -> usize {
        panic!("Operation only allowed for float HArrays");
    }

    fn nframes(&self) -> usize {
        panic!("Operation only allowed for float HArrays");
    }

    fn db_to_power(&mut self, _: Robj) {
        panic!("Operation only allowed for float HArrays");
    }

    fn to_mono(&mut self) {
        panic!("Operation only allowed for float HArrays");
    }
}
