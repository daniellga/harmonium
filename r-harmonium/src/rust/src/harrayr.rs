use crate::{errors::HErrorR, hdatatype::HDataType};
use harmonium_core::audioop::AudioOp;
use ndarray::{IxDyn, SliceInfo, SliceInfoElem};
use num_complex::Complex;
use savvy::{r_println, OwnedComplexSexp, OwnedIntegerSexp, OwnedRealSexp, Sexp};
use std::{any::Any, sync::Arc};

pub trait HArrayR: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn len(&self) -> usize;
    fn shape(&self) -> &[usize];
    fn ndim(&self) -> usize;
    fn slice(&self, range: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn>) -> Arc<dyn HArrayR>;
    fn print(&self);
    fn collect(&self) -> savvy::Result<Sexp>;
    fn dtype(&self) -> HDataType;
    fn mem_adress(&self) -> String;
    fn clone_inner(&self) -> Arc<dyn HArrayR>;
    fn nchannels(&self) -> savvy::Result<usize>;
    fn nframes(&self) -> savvy::Result<usize>;
    fn db_to_amplitude(&mut self, reference: f64, power: f64) -> savvy::Result<()>;
    fn to_mono(&mut self) -> savvy::Result<()>;
}

impl HArrayR for harmonium_core::array::HArray<f32, IxDyn> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn shape(&self) -> &[usize] {
        self.shape()
    }

    fn ndim(&self) -> usize {
        self.ndim()
    }

    fn slice(&self, slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn>) -> Arc<dyn HArrayR> {
        let ndarray = self.0.clone().slice_move(slice_info);
        Arc::new(harmonium_core::array::HArray(ndarray))
    }

    fn print(&self) {
        r_println!("{}", self);
    }

    fn collect(&self) -> savvy::Result<Sexp> {
        let mut dim = unsafe { OwnedIntegerSexp::new_without_init(self.shape().len())? };
        self.shape()
            .iter()
            .rev()
            .map(|z| *z as i32)
            .zip(dim.as_mut_slice().iter_mut())
            .for_each(|(sh, int_sxp)| *int_sxp = sh);

        let mut real_sexp = unsafe { OwnedRealSexp::new_without_init(self.0.len())? };
        self.0
            .iter()
            .map(|z| *z as f64)
            .zip(real_sexp.as_mut_slice().iter_mut())
            .for_each(|(k, real_sxp)| *real_sxp = k);

        real_sexp.set_attrib("dim", dim.into())?;

        real_sexp.into()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float32
    }

    fn mem_adress(&self) -> String {
        let s = format!("{:p}", self.0.as_ptr());
        s.to_string()
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }

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

impl HArrayR for harmonium_core::array::HArray<f64, IxDyn> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn shape(&self) -> &[usize] {
        self.shape()
    }

    fn ndim(&self) -> usize {
        self.ndim()
    }

    fn slice(&self, slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn>) -> Arc<dyn HArrayR> {
        let ndarray = self.0.clone().slice_move(slice_info);
        Arc::new(harmonium_core::array::HArray(ndarray))
    }

    fn print(&self) {
        r_println!("{}", self);
    }

    fn collect(&self) -> savvy::Result<Sexp> {
        let mut dim = unsafe { OwnedIntegerSexp::new_without_init(self.shape().len())? };
        self.shape()
            .iter()
            .rev()
            .map(|z| *z as i32)
            .zip(dim.as_mut_slice().iter_mut())
            .for_each(|(sh, int_sxp)| *int_sxp = sh);

        let mut real_sexp = unsafe { OwnedRealSexp::new_without_init(self.0.len())? };
        self.0
            .iter()
            .zip(real_sexp.as_mut_slice().iter_mut())
            .for_each(|(k, real_sxp)| *real_sxp = *k);

        real_sexp.set_attrib("dim", dim.into())?;

        real_sexp.into()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float64
    }

    fn mem_adress(&self) -> String {
        let s = format!("{:p}", self.0.as_ptr());
        s.to_string()
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }

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

impl HArrayR for harmonium_core::array::HArray<Complex<f32>, IxDyn> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn shape(&self) -> &[usize] {
        self.shape()
    }

    fn ndim(&self) -> usize {
        self.ndim()
    }

    fn slice(&self, slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn>) -> Arc<dyn HArrayR> {
        let ndarray = self.0.clone().slice_move(slice_info);
        Arc::new(harmonium_core::array::HArray(ndarray))
    }

    fn print(&self) {
        r_println!("{}", self);
    }

    fn collect(&self) -> savvy::Result<Sexp> {
        let mut dim = unsafe { OwnedIntegerSexp::new_without_init(self.shape().len())? };
        self.shape()
            .iter()
            .rev()
            .map(|z| *z as i32)
            .zip(dim.as_mut_slice().iter_mut())
            .for_each(|(sh, int_sxp)| *int_sxp = sh);

        let mut complex_sexp = unsafe { OwnedComplexSexp::new_without_init(self.0.len())? };
        self.0
            .iter()
            .map(|z| Complex::<f64>::new(z.re as f64, z.im as f64))
            .zip(complex_sexp.as_mut_slice().iter_mut())
            .for_each(|(k, complex_sxp)| *complex_sxp = k);

        complex_sexp.set_attrib("dim", dim.into())?;
        complex_sexp.into()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Complex32
    }

    fn mem_adress(&self) -> String {
        let s = format!("{:p}", self.0.as_ptr());
        s.to_string()
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }

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

impl HArrayR for harmonium_core::array::HArray<Complex<f64>, IxDyn> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn shape(&self) -> &[usize] {
        self.shape()
    }

    fn ndim(&self) -> usize {
        self.ndim()
    }

    fn slice(&self, slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn>) -> Arc<dyn HArrayR> {
        let ndarray = self.0.clone().slice_move(slice_info);
        Arc::new(harmonium_core::array::HArray(ndarray))
    }

    fn print(&self) {
        r_println!("{}", self);
    }

    fn collect(&self) -> savvy::Result<Sexp> {
        let mut dim = unsafe { OwnedIntegerSexp::new_without_init(self.shape().len())? };
        self.shape()
            .iter()
            .rev()
            .map(|z| *z as i32)
            .zip(dim.as_mut_slice().iter_mut())
            .for_each(|(sh, int_sxp)| *int_sxp = sh);

        let mut complex_sexp = unsafe { OwnedComplexSexp::new_without_init(self.0.len())? };
        self.0
            .iter()
            .zip(complex_sexp.as_mut_slice().iter_mut())
            .for_each(|(k, complex_sxp)| *complex_sxp = *k);

        complex_sexp.set_attrib("dim", dim.into())?;

        complex_sexp.into()
    }

    fn dtype(&self) -> HDataType {
        HDataType::Complex64
    }

    fn mem_adress(&self) -> String {
        let s = format!("{:p}", self.0.as_ptr());
        s.to_string()
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }

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
