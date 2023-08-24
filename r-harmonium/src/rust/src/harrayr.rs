use crate::{conversions::RobjConversions, hdatatype::HDataType};
use extendr_api::{prelude::*, wrapper};
use harmonium_core::haudioop::HAudioOpDyn;
use harmonium_fft::fft::{FftComplex, FftFloat};
use ndarray::{IxDyn, SliceInfo, SliceInfoElem};
use num_complex::Complex;
use std::{any::Any, sync::Arc};

pub trait HArrayR: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn len(&self) -> usize;
    fn shape(&self) -> &[usize];
    fn ndim(&self) -> usize;
    fn slice(&self, range: &Robj) -> Arc<dyn HArrayR>;
    fn print(&self);
    fn collect(&self) -> Robj;
    fn dtype(&self) -> HDataType;
    fn mem_adress(&self) -> String;
    fn fft(&self) -> Arc<dyn HArrayR>;
    fn fft_mut(&mut self);
    fn fft_real_mut(&mut self) -> Arc<dyn HArrayR>;
    fn clone_inner(&self) -> Arc<dyn HArrayR>;
    fn nchannels(&self) -> usize;
    fn nframes(&self) -> usize;
    fn db_to_amplitude(&mut self, reference: &Robj, power: &Robj);
    fn to_mono(&mut self);
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

    fn slice(&self, range: &Robj) -> Arc<dyn HArrayR> {
        // ndarray already panics if an index is out of bounds or step size is zero. Also panics if D is IxDyn and info does not match the number of array axes.
        let range = List::try_from(range).unwrap();
        let list_len = range.len();

        let mut vec_ranges: Vec<SliceInfoElem> = Vec::with_capacity(list_len);
        for robj in range.values() {
            assert!(
                robj.is_integer() && robj.len() == 3,
                "Each vector in the list must be of integers and have a length of 3."
            );
            let slice: &[i32] = robj.robj_to_slice();

            // Ok to unsafe because we checked robj.len() == 3.
            let slice_info_elem = SliceInfoElem::Slice {
                start: unsafe { *slice.get_unchecked(0) as isize },
                end: Some(unsafe { *slice.get_unchecked(1) as isize }),
                step: unsafe { *slice.get_unchecked(2) as isize },
            };
            vec_ranges.push(slice_info_elem);
        }

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn> =
            vec_ranges.try_into().unwrap();

        let ndarray = self.0.clone().slice_move(slice_info);

        Arc::new(harmonium_core::array::HArray(ndarray))
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

    fn mem_adress(&self) -> String {
        let s = format!("{:p}", self.0.as_ptr());
        s.to_string()
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftFloat::<f32, IxDyn>::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        panic!("Operation only allowed for complex HArrays");
    }

    fn fft_real_mut(&mut self) -> Arc<dyn HArrayR> {
        Arc::new(FftFloat::<f32, IxDyn>::fft_real(self))
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

    fn db_to_amplitude(&mut self, reference: &Robj, power: &Robj) {
        let reference: f64 = reference.robj_to_scalar();
        let power: f64 = power.robj_to_scalar();
        HAudioOpDyn::db_to_amplitude(self, reference as f32, power as f32);
    }

    fn to_mono(&mut self) {
        *self = HAudioOpDyn::to_mono(self).unwrap();
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

    fn slice(&self, range: &Robj) -> Arc<dyn HArrayR> {
        let ndim = self.ndim();
        let range = List::try_from(range).unwrap();
        let list_len = range.len();

        assert_eq!(
            list_len, ndim,
            "The list must have the same length as the number of dimensions."
        );

        let mut vec_ranges: Vec<SliceInfoElem> = Vec::with_capacity(list_len);
        for robj in range.values() {
            assert!(
                robj.is_integer() && robj.len() == 3,
                "Each vector in the list must be of integers and have a length of 3."
            );
            let slice: &[i32] = robj.robj_to_slice();
            let slice_info_elem = SliceInfoElem::Slice {
                start: slice[0] as isize,
                end: Some(slice[1] as isize),
                step: slice[2] as isize,
            };
            vec_ranges.push(slice_info_elem);
        }

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn> =
            vec_ranges.try_into().unwrap();

        let ndarray = self.0.clone().slice_move(slice_info);

        Arc::new(harmonium_core::array::HArray(ndarray))
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

    fn mem_adress(&self) -> String {
        let s = format!("{:p}", self.0.as_ptr());
        s.to_string()
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftFloat::<f64, IxDyn>::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        panic!("Operation only allowed for complex HArrays");
    }

    fn fft_real_mut(&mut self) -> Arc<dyn HArrayR> {
        Arc::new(FftFloat::<f64, IxDyn>::fft_real(self))
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

    fn db_to_amplitude(&mut self, reference: &Robj, power: &Robj) {
        let reference: f64 = reference.robj_to_scalar();
        let power: f64 = power.robj_to_scalar();
        HAudioOpDyn::db_to_amplitude(self, reference, power);
    }

    fn to_mono(&mut self) {
        *self = HAudioOpDyn::to_mono(self).unwrap();
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

    fn slice(&self, range: &Robj) -> Arc<dyn HArrayR> {
        let ndim = self.ndim();
        let range = List::try_from(range).unwrap();
        let list_len = range.len();

        assert_eq!(
            list_len, ndim,
            "The list must have the same length as the number of dimensions."
        );

        let mut vec_ranges: Vec<SliceInfoElem> = Vec::with_capacity(list_len);
        for robj in range.values() {
            assert!(
                robj.is_integer() && robj.len() == 3,
                "Each vector in the list must be of integers and have a length of 3."
            );
            let slice: &[i32] = robj.robj_to_slice();
            let slice_info_elem = SliceInfoElem::Slice {
                start: slice[0] as isize,
                end: Some(slice[1] as isize),
                step: slice[2] as isize,
            };
            vec_ranges.push(slice_info_elem);
        }

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn> =
            vec_ranges.try_into().unwrap();

        let ndarray = self.0.clone().slice_move(slice_info);

        Arc::new(harmonium_core::array::HArray(ndarray))
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

    fn mem_adress(&self) -> String {
        let s = format!("{:p}", self.0.as_ptr());
        s.to_string()
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftComplex::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        FftComplex::fft_mut(self);
    }

    fn fft_real_mut(&mut self) -> Arc<dyn HArrayR> {
        panic!("Operation only allowed for float HArrays");
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

    fn db_to_amplitude(&mut self, _: &Robj, _: &Robj) {
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

    fn slice(&self, range: &Robj) -> Arc<dyn HArrayR> {
        let ndim = self.ndim();
        let range = List::try_from(range).unwrap();
        let list_len = range.len();

        assert_eq!(
            list_len, ndim,
            "The list must have the same length as the number of dimensions."
        );

        let mut vec_ranges: Vec<SliceInfoElem> = Vec::with_capacity(list_len);
        for robj in range.values() {
            assert!(
                robj.is_integer() && robj.len() == 3,
                "Each vector in the list must be of integers and have a length of 3."
            );
            let slice: &[i32] = robj.robj_to_slice();
            let slice_info_elem = SliceInfoElem::Slice {
                start: slice[0] as isize,
                end: Some(slice[1] as isize),
                step: slice[2] as isize,
            };
            vec_ranges.push(slice_info_elem);
        }

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn> =
            vec_ranges.try_into().unwrap();

        let ndarray = self.0.clone().slice_move(slice_info);

        Arc::new(harmonium_core::array::HArray(ndarray))
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

    fn mem_adress(&self) -> String {
        let s = format!("{:p}", self.0.as_ptr());
        s.to_string()
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftComplex::fft(self);
        Arc::new(harray)
    }

    fn fft_mut(&mut self) {
        FftComplex::fft_mut(self);
    }

    fn fft_real_mut(&mut self) -> Arc<dyn HArrayR> {
        panic!("Operation only allowed for float HArrays");
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

    fn db_to_amplitude(&mut self, _: &Robj, _: &Robj) {
        panic!("Operation only allowed for float HArrays");
    }

    fn to_mono(&mut self) {
        panic!("Operation only allowed for float HArrays");
    }
}
