use crate::{haudioop::HAudioOp, hdatatype::HDataType};
use ndarray::{IxDyn, SliceInfo, SliceInfoElem};
use num_complex::Complex;
use savvy::{r_println, OwnedComplexSexp, OwnedIntegerSexp, OwnedLogicalSexp, OwnedRealSexp, Sexp};
use std::{any::Any, sync::Arc};

pub trait HArrayR: Send + Sync + HAudioOp {
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
    fn is_standard_layout(&self) -> savvy::Result<Sexp>;
    fn is_unique(&mut self) -> savvy::Result<Sexp>;
    fn clone_inner(&self) -> Arc<dyn HArrayR>;
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

    fn is_standard_layout(&self) -> savvy::Result<Sexp> {
        let is_standard_layout = self.0.is_standard_layout();
        let logical_sexp: OwnedLogicalSexp = is_standard_layout.try_into()?;
        logical_sexp.into()
    }

    fn is_unique(&mut self) -> savvy::Result<Sexp> {
        let is_unique = self.0.is_unique();
        let logical_sexp: OwnedLogicalSexp = is_unique.try_into()?;
        logical_sexp.into()
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
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

    fn is_standard_layout(&self) -> savvy::Result<Sexp> {
        let is_standard_layout = self.0.is_standard_layout();
        let logical_sexp: OwnedLogicalSexp = is_standard_layout.try_into()?;
        logical_sexp.into()
    }

    fn is_unique(&mut self) -> savvy::Result<Sexp> {
        let is_unique = self.0.is_unique();
        let logical_sexp: OwnedLogicalSexp = is_unique.try_into()?;
        logical_sexp.into()
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
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

    fn is_standard_layout(&self) -> savvy::Result<Sexp> {
        let is_standard_layout = self.0.is_standard_layout();
        let logical_sexp: OwnedLogicalSexp = is_standard_layout.try_into()?;
        logical_sexp.into()
    }

    fn is_unique(&mut self) -> savvy::Result<Sexp> {
        let is_unique = self.0.is_unique();
        let logical_sexp: OwnedLogicalSexp = is_unique.try_into()?;
        logical_sexp.into()
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
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

    fn is_standard_layout(&self) -> savvy::Result<Sexp> {
        let is_standard_layout = self.0.is_standard_layout();
        let logical_sexp: OwnedLogicalSexp = is_standard_layout.try_into()?;
        logical_sexp.into()
    }

    fn is_unique(&mut self) -> savvy::Result<Sexp> {
        let is_unique = self.0.is_unique();
        let logical_sexp: OwnedLogicalSexp = is_unique.try_into()?;
        logical_sexp.into()
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }
}
