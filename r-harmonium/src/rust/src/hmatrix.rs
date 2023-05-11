use crate::{
    harray::{HArray, HArrayR},
    haudio::{HAudio, HAudioR},
    hdatatype::HDataType,
};
use arrow2::{
    array::{FixedSizeListArray, PrimitiveArray},
    ffi::{import_array_from_c, import_field_from_c, ArrowArray, ArrowSchema},
};
use extendr_api::prelude::*;
use harmonium_core::structs::{HComplexArray, HComplexMatrix, HFloatArray, HFloatMatrix};
use harmonium_fft::fft::fft_arrow::{FftComplexMatrix, FftFloatMatrix};
use std::{any::Any, sync::Arc};

pub trait HMatrixR: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn len(&self) -> i32;
    fn slice(&mut self, offset: i32, length: i32);
    fn ncols(&self) -> i32;
    fn nrows(&self) -> i32;
    fn print(&self);
    fn as_harray(&self) -> Arc<dyn HArrayR>;
    fn as_haudio(&self, sr: i32) -> Arc<dyn HAudioR>;
    fn collect(&self) -> Robj;
    fn mem_adress(&self) -> String;
    fn dtype(&self) -> HDataType;
    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema);
    fn fft(&self) -> Arc<dyn HMatrixR>;
    fn mean_cols(&mut self);
    fn clone_inner(&self) -> Arc<dyn HMatrixR>;
}

#[derive(Clone)]
pub struct HMatrix(pub Arc<dyn HMatrixR>);

#[extendr(use_try_from = true)]
impl HMatrix {
    pub fn new_from_values(robj: Robj, dtype: &HDataType) -> HMatrix {
        match (robj.rtype(), dtype) {
            (Rtype::Doubles, HDataType::Float32) => {
                let rmatrix: RMatrix<f64> = robj.try_into().expect("not valid input types");
                let ncols = rmatrix.ncols();
                let v: Vec<f32> = rmatrix.data().iter().map(|z| *z as f32).collect();
                let hfloatmatrix = HFloatArray::<f32>::new_from_vec(v)
                    .into_hmatrix(ncols)
                    .unwrap();
                let inner = Arc::new(hfloatmatrix);
                HMatrix(inner)
            }
            (Rtype::Doubles, HDataType::Float64) => {
                let rmatrix: RMatrix<f64> = robj.try_into().expect("not valid input types");
                let ncols = rmatrix.ncols();
                let slice = rmatrix.data();
                let array = PrimitiveArray::from_slice(slice);
                let hfloatmatrix = HFloatArray::<f64>::new(array).into_hmatrix(ncols).unwrap();
                let inner = Arc::new(hfloatmatrix);
                HMatrix(inner)
            }
            (Rtype::Complexes, HDataType::Complex32) => {
                let rmatrix: RMatrix<Rcplx> = robj.try_into().expect("not valid input types");
                let slice = rmatrix.data();
                let length = slice.len() * 2;
                let ncols = rmatrix.ncols();
                let mut v: Vec<f32> = Vec::with_capacity(length);
                slice.iter().for_each(|x| {
                    v.push(x.re().0 as f32);
                    v.push(x.im().0 as f32);
                });
                let hcomplexmatrix = HComplexArray::<f32>::new_from_vec(v)
                    .into_hmatrix(ncols)
                    .unwrap();
                let inner = Arc::new(hcomplexmatrix);
                HMatrix(inner)
            }
            (Rtype::Complexes, HDataType::Complex64) => {
                let rmatrix: RMatrix<Rcplx> = robj.try_into().expect("not valid input types");
                let slice = rmatrix.data();
                let length = slice.len() * 2;
                let ncols = rmatrix.ncols();
                let mut v: Vec<f64> = Vec::with_capacity(length);
                slice.iter().for_each(|x| {
                    v.push(x.re().0);
                    v.push(x.im().0);
                });
                let hcomplexmatrix = HComplexArray::<f64>::new_from_vec(v)
                    .into_hmatrix(ncols)
                    .unwrap();
                let inner = Arc::new(hcomplexmatrix);
                HMatrix(inner)
            }
            _ => panic!("not valid input types"),
        }
    }

    pub fn new_from_arrow(robj: Robj, dtype: &HDataType) -> HMatrix {
        if !robj.class().unwrap().any(|x| x == "Array") {
            panic!("wrong type");
        }

        let array = ArrowArray::empty();
        let schema = ArrowSchema::empty();

        let array_ptr = (&array as *const ArrowArray) as usize;
        let schema_ptr = (&schema as *const ArrowSchema) as usize;

        robj.dollar("export_to_c")
            .unwrap()
            .call(pairlist!(array_ptr, schema_ptr))
            .unwrap();

        let field = unsafe { import_field_from_c(&schema).unwrap() };
        let arr = unsafe { import_array_from_c(array, field.data_type).unwrap() };

        match (dtype, arr.data_type().to_physical_type()) {
            (HDataType::Float32, arrow2::datatypes::PhysicalType::FixedSizeList) => {
                let arr = arr.as_any().downcast_ref::<FixedSizeListArray>().unwrap();
                let hmatrix = HFloatMatrix::<f32>::new(arr.clone());
                HMatrix(Arc::new(hmatrix))
            }
            (HDataType::Float64, arrow2::datatypes::PhysicalType::FixedSizeList) => {
                let arr = arr.as_any().downcast_ref::<FixedSizeListArray>().unwrap();
                let hmatrix = HFloatMatrix::<f64>::new(arr.clone());
                HMatrix(Arc::new(hmatrix))
            }
            (HDataType::Complex32, arrow2::datatypes::PhysicalType::FixedSizeList) => {
                let arr = arr.as_any().downcast_ref::<FixedSizeListArray>().unwrap();
                let hmatrix = HComplexMatrix::<f32>::new(arr.clone());
                HMatrix(Arc::new(hmatrix))
            }
            (HDataType::Complex64, arrow2::datatypes::PhysicalType::FixedSizeList) => {
                let arr = arr.as_any().downcast_ref::<FixedSizeListArray>().unwrap();
                let hmatrix = HComplexMatrix::<f64>::new(arr.clone());
                HMatrix(Arc::new(hmatrix))
            }
            _ => panic!("not valid input"),
        }
    }

    pub fn len(&self) -> i32 {
        self.0.len()
    }

    fn slice(&mut self, offset: i32, length: i32) {
        let inner_mut = self.get_inner_mut();
        inner_mut.slice(offset, length);
    }

    pub fn ncols(&self) -> i32 {
        self.0.ncols()
    }

    pub fn nrows(&self) -> i32 {
        self.0.nrows()
    }

    pub fn print(&self) {
        self.0.print();
    }

    pub fn eq(&self, other: &HMatrix) -> bool {
        self.0.eq(&other.0)
    }

    pub fn ne(&self, other: &HMatrix) -> bool {
        self.0.ne(&other.0)
    }

    /// Compares inner array equality with an HMatrix.
    pub fn eq_inner(&self, other: &HMatrix) -> bool {
        self.0.eq_inner(&*other.0)
    }

    /// Compares inner array equality with an HArray.
    pub fn eq_inner_harray(&self, other: &HArray) -> bool {
        self.0.eq_inner(&*other.0)
    }

    /// Compares inner array equality with an HAudio.
    pub fn eq_inner_haudio(&self, other: &HAudio) -> bool {
        self.0.eq_inner(&*other.0)
    }

    /// Creates a new HMatrix, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> HMatrix {
        std::clone::Clone::clone(self)
    }

    /// Converts to HArray. The new HArray Uses the same underlying data as the HMatrix.
    pub fn as_harray(&self) -> HArray {
        HArray(self.0.as_harray())
    }

    /// Converts to HAudio. The new HAudio Uses the same underlying data as the HMatrix.
    pub fn as_haudio(&self, sr: i32) -> HAudio {
        HAudio(self.0.as_haudio(sr))
    }

    pub fn collect(&self) -> Robj {
        self.0.collect()
    }

    pub fn mem_adress(&self) -> String {
        self.0.mem_adress()
    }

    pub fn dtype(&self) -> HDataType {
        self.0.dtype()
    }

    /// Returns true if the inner Arc is shared.
    pub fn is_shared(&self) -> bool {
        Arc::weak_count(&self.0) + Arc::strong_count(&self.0) != 1
    }

    /// Export the underlying array to Arrow C interface.
    pub fn to_c_arrow(&self, array_ptr: &str, schema_ptr: &str) {
        let (mut array_ffi, mut schema_ffi) = self.0.export_c_arrow();

        let array_out_ptr_addr: usize = array_ptr.parse().unwrap();
        let array_out_ptr = array_out_ptr_addr as *mut arrow2::ffi::ArrowArray;

        let schema_out_ptr_addr: usize = schema_ptr.parse().unwrap();
        let schema_out_ptr = schema_out_ptr_addr as *mut arrow2::ffi::ArrowSchema;

        unsafe {
            std::ptr::swap_nonoverlapping(
                array_out_ptr,
                &mut array_ffi as *mut arrow2::ffi::ArrowArray,
                1,
            );
            std::ptr::swap_nonoverlapping(
                schema_out_ptr,
                &mut schema_ffi as *mut arrow2::ffi::ArrowSchema,
                1,
            );
        }
    }

    pub fn fft(&self) -> HMatrix {
        HMatrix(self.0.fft())
    }

    /// Take the average across columns. A new inner array is created.
    /// The operation is done in-place.
    pub fn mean_cols(&mut self) {
        let inner_mut = self.get_inner_mut();
        inner_mut.mean_cols();
    }
}

impl HMatrix {
    #[doc(hidden)]
    pub fn get_inner_mut(&mut self) -> &mut dyn HMatrixR {
        if Arc::weak_count(&self.0) + Arc::strong_count(&self.0) != 1 {
            self.0 = self.0.clone_inner();
        }
        Arc::get_mut(&mut self.0).expect("implementation error")
    }
}

impl HMatrixR for HFloatMatrix<f32> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        self.len().try_into().unwrap()
    }

    fn slice(&mut self, offset: i32, length: i32) {
        HFloatMatrix::slice(self, offset.try_into().unwrap(), length.try_into().unwrap());
    }

    fn ncols(&self) -> i32 {
        self.ncols().try_into().unwrap()
    }

    fn nrows(&self) -> i32 {
        self.nrows().try_into().unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_harray(&self) -> Arc<dyn HArrayR> {
        let hmatrix = self.clone().into_harray();
        Arc::new(hmatrix)
    }

    fn as_haudio(&self, sr: i32) -> Arc<dyn HAudioR> {
        let haudio = self.clone().into_haudio(sr.try_into().unwrap());
        Arc::new(haudio)
    }

    fn collect(&self) -> Robj {
        let list_array = self.inner();
        let ncols = list_array.len();
        let nrows = list_array.size();
        list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values()
            .iter()
            .map(|x| *x as f64)
            .collect_rarray([nrows, ncols])
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values()
            .as_slice();
        format!("{:p}", p)
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float32
    }

    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema) {
        HFloatMatrix::export_c_arrow(self)
    }

    fn fft(&self) -> Arc<dyn HMatrixR> {
        let hmatrix = FftFloatMatrix::<f32>::fft(self).unwrap();
        Arc::new(hmatrix)
    }

    fn mean_cols(&mut self) {
        HFloatMatrix::<f32>::mean_cols(self).unwrap();
    }

    fn clone_inner(&self) -> Arc<dyn HMatrixR> {
        Arc::new(self.clone())
    }
}

impl HMatrixR for HFloatMatrix<f64> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        self.len().try_into().unwrap()
    }

    fn slice(&mut self, offset: i32, length: i32) {
        HFloatMatrix::slice(self, offset.try_into().unwrap(), length.try_into().unwrap());
    }

    fn ncols(&self) -> i32 {
        self.ncols().try_into().unwrap()
    }

    fn nrows(&self) -> i32 {
        self.nrows().try_into().unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_harray(&self) -> Arc<dyn HArrayR> {
        let hmatrix = self.clone().into_harray();
        Arc::new(hmatrix)
    }

    fn as_haudio(&self, sr: i32) -> Arc<dyn HAudioR> {
        let haudio = self.clone().into_haudio(sr.try_into().unwrap());
        Arc::new(haudio)
    }

    fn collect(&self) -> Robj {
        let list_array = self.inner();
        let ncols = list_array.len();
        let nrows = list_array.size();
        list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values()
            .iter()
            .copied()
            .collect_rarray([nrows, ncols])
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values()
            .as_slice();
        format!("{:p}", p)
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float64
    }

    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema) {
        HFloatMatrix::export_c_arrow(self)
    }

    fn fft(&self) -> Arc<dyn HMatrixR> {
        let hmatrix = FftFloatMatrix::<f64>::fft(self).unwrap();
        Arc::new(hmatrix)
    }

    fn mean_cols(&mut self) {
        HFloatMatrix::<f64>::mean_cols(self).unwrap();
    }

    fn clone_inner(&self) -> Arc<dyn HMatrixR> {
        Arc::new(self.clone())
    }
}

impl HMatrixR for HComplexMatrix<f32> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        self.len().try_into().unwrap()
    }

    fn slice(&mut self, offset: i32, length: i32) {
        HComplexMatrix::slice(self, offset.try_into().unwrap(), length.try_into().unwrap());
    }

    fn ncols(&self) -> i32 {
        self.ncols().try_into().unwrap()
    }

    fn nrows(&self) -> i32 {
        self.nrows().try_into().unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_harray(&self) -> Arc<dyn HArrayR> {
        let hmatrix = self.clone().into_harray();
        Arc::new(hmatrix)
    }

    fn as_haudio(&self, _sr: i32) -> Arc<dyn HAudioR> {
        panic!("cannot represent audio with complex data");
    }

    fn collect(&self) -> Robj {
        let ncols = self.ncols();
        let nrows = self.nrows();
        self.inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values()
            .chunks_exact(2)
            .map(|x| Rcplx::new(x[0] as f64, x[1] as f64))
            .collect_rarray([nrows, ncols])
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values()
            .as_slice();
        format!("{:p}", p)
    }

    fn dtype(&self) -> HDataType {
        HDataType::Complex32
    }

    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema) {
        HComplexMatrix::export_c_arrow(self)
    }

    fn fft(&self) -> Arc<dyn HMatrixR> {
        let harray = FftComplexMatrix::<f32>::fft(self).unwrap();
        Arc::new(harray)
    }

    fn mean_cols(&mut self) {
        HComplexMatrix::<f32>::mean_cols(self).unwrap();
    }

    fn clone_inner(&self) -> Arc<dyn HMatrixR> {
        Arc::new(self.clone())
    }
}

impl HMatrixR for HComplexMatrix<f64> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        self.len().try_into().unwrap()
    }

    fn slice(&mut self, offset: i32, length: i32) {
        HComplexMatrix::slice(self, offset.try_into().unwrap(), length.try_into().unwrap());
    }

    fn ncols(&self) -> i32 {
        self.ncols().try_into().unwrap()
    }

    fn nrows(&self) -> i32 {
        self.nrows().try_into().unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_harray(&self) -> Arc<dyn HArrayR> {
        let hmatrix = self.clone().into_harray();
        Arc::new(hmatrix)
    }

    fn as_haudio(&self, _sr: i32) -> Arc<dyn HAudioR> {
        panic!("cannot represent audio with complex data");
    }

    fn collect(&self) -> Robj {
        let ncols = self.ncols();
        let nrows = self.nrows();
        self.inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values()
            .chunks_exact(2)
            .map(|x| Rcplx::new(x[0], x[1]))
            .collect_rarray([nrows, ncols])
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values()
            .as_slice();
        format!("{:p}", p)
    }

    fn dtype(&self) -> HDataType {
        HDataType::Complex64
    }

    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema) {
        HComplexMatrix::export_c_arrow(self)
    }

    fn fft(&self) -> Arc<dyn HMatrixR> {
        let harray = FftComplexMatrix::<f64>::fft(self).unwrap();
        Arc::new(harray)
    }

    fn mean_cols(&mut self) {
        HComplexMatrix::<f64>::mean_cols(self).unwrap();
    }

    fn clone_inner(&self) -> Arc<dyn HMatrixR> {
        Arc::new(self.clone())
    }
}

extendr_module! {
    mod hmatrix;
    impl HMatrix;
}
