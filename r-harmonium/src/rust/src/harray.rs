use crate::{
    haudio::HAudio,
    hdatatype::HDataType,
    hmatrix::{HMatrix, HMatrixR},
    partialeq::PartialEqInner,
};
use arrow2::{
    array::PrimitiveArray,
    datatypes::PhysicalType,
    ffi::{import_array_from_c, import_field_from_c, ArrowArray, ArrowSchema},
    types::PrimitiveType,
};
use extendr_api::{prelude::*, AsTypedSlice};
use harmonium_core::structs::{HComplexArray, HFloatArray};
use harmonium_fft::fft::fft_arrow::{FftComplexArray, FftFloatArray};
use std::{any::Any, sync::Arc};

pub trait HArrayR: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn len(&self) -> i32;
    fn slice(&mut self, offset: i32, length: i32);
    fn print(&self);
    fn as_hmatrix(&self, ncols: i32) -> Arc<dyn HMatrixR>;
    fn collect(&self) -> Robj;
    fn mem_adress(&self) -> String;
    fn data_type(&self) -> HDataType;
    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema);
    fn fft(&self) -> Arc<dyn HArrayR>;
    fn clone_inner(&self) -> Arc<dyn HArrayR>;
}

/// HArray
/// An array representation.
/// Supports Float32, Float64, Complex32 and Complex64 types.
#[derive(Clone)]
pub struct HArray(pub Arc<dyn HArrayR>);

#[extendr]
impl HArray {
    /// HArray
    /// `new_from_values(robj: atomicvector, dtype: HDataType)`
    /// Creates a new `HArray` from an R atomic vector.
    ///
    /// # Arguments
    ///
    /// * `robj` - A double or complex atomic vector.
    /// * `dtype` - An `HDataType` to indicate which type of `HArray` to be created.
    ///             For float dtypes, the atomic vector must be a double. For complex dtypes, a
    ///             complex atomic vector.
    ///
    /// # Returns
    ///
    /// An `HArray` external pointer
    ///
    /// # Examples
    ///
    /// ```
    /// robj = c(1,2,3,4,5,6,7,8,9,10,11,12)
    /// dtype = HDataType$float32
    /// HArray$new_from_values(robj, dtype)
    /// ```
    ///
    pub fn new_from_values(robj: Robj, dtype: &HDataType) -> HArray {
        match (robj.rtype(), dtype) {
            (Rtype::Doubles, HDataType::Float32) => {
                let slice = robj.as_real_slice().unwrap();
                let v = slice.iter().map(|x| *x as f32).collect();
                let hfloatarray = HFloatArray::<f32>::new_from_vec(v);
                let data = Arc::new(hfloatarray);
                HArray(data)
            }
            (Rtype::Doubles, HDataType::Float64) => {
                let v = robj.as_real_vector().unwrap();
                let hfloatarray = HFloatArray::<f64>::new_from_vec(v);
                let data = Arc::new(hfloatarray);
                HArray(data)
            }
            (Rtype::Complexes, HDataType::Complex32) => {
                let slice: &[Rcplx] = robj.as_typed_slice().unwrap();
                let length = slice.len() * 2;
                let mut v: Vec<f32> = Vec::with_capacity(length);
                slice.iter().for_each(|x| {
                    v.push(x.re().0 as f32);
                    v.push(x.im().0 as f32);
                });
                let hcomplexarray = HComplexArray::<f32>::new_from_vec(v);
                let data = Arc::new(hcomplexarray);
                HArray(data)
            }
            (Rtype::Complexes, HDataType::Complex64) => {
                let slice: &[Rcplx] = robj.as_typed_slice().unwrap();
                let length = slice.len() * 2;
                let mut v: Vec<f64> = Vec::with_capacity(length);
                slice.iter().for_each(|x| {
                    v.push(x.re().0);
                    v.push(x.im().0);
                });
                let hcomplexarray = HComplexArray::<f64>::new_from_vec(v);
                let data = Arc::new(hcomplexarray);
                HArray(data)
            }
            _ => panic!("not valid input types"),
        }
    }

    /// HArray
    /// `new_from_values(robj: Array, dtype: HDataType)`
    /// Creates a new `HArray` from an R's arrow `Array`.
    /// The conversion is zero copy.
    ///
    /// # Arguments
    ///
    /// * `robj` - A float32 or float64 arrow `Array`.
    /// * `dtype` - An `HDataType` to indicate which type of `HArray` to be created.
    ///
    /// # Returns
    ///
    /// An `HArray` external pointer
    ///
    /// # Examples
    ///
    /// ```
    /// robj = arrow::Array$create(1:10, type = float32())
    /// dtype = HDataType$complex32
    /// HArray$new_from_arrow(robj, dtype)
    /// ```
    ///
    pub fn new_from_arrow(robj: Robj, dtype: &HDataType) -> HArray {
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
            (HDataType::Float32, PhysicalType::Primitive(PrimitiveType::Float32)) => {
                let arr = arr.as_any().downcast_ref::<PrimitiveArray<f32>>().unwrap();
                let harray = HFloatArray::new(arr.clone());
                HArray(Arc::new(harray))
            }
            (HDataType::Float64, PhysicalType::Primitive(PrimitiveType::Float64)) => {
                let arr = arr.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
                let harray = HFloatArray::new(arr.clone());
                HArray(Arc::new(harray))
            }
            (HDataType::Complex32, PhysicalType::Primitive(PrimitiveType::Float32)) => {
                let arr = arr.as_any().downcast_ref::<PrimitiveArray<f32>>().unwrap();
                let harray = HComplexArray::new(arr.clone());
                HArray(Arc::new(harray))
            }
            (HDataType::Complex64, PhysicalType::Primitive(PrimitiveType::Float64)) => {
                let arr = arr.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
                let harray = HComplexArray::new(arr.clone());
                HArray(Arc::new(harray))
            }
            _ => panic!("not valid input"),
        }
    }

    /// HArray
    /// `len() -> integer`
    /// Returns the length of this Harray.
    ///
    /// # Returns
    ///
    /// An integer.
    ///
    /// # Examples
    ///
    /// ```
    /// harray$len()
    /// ```
    ///
    pub fn len(&self) -> i32 {
        self.0.len()
    }

    /// HArray
    /// `slice(offset: integer, length: integer)`
    /// Slice the HArray by an offset and length.
    /// This operation is O(1).
    /// The function will modify in place the current HArray. If a clone of the HArray has been
    /// made, it will create a new one sliced one.
    ///
    /// # Arguments
    ///
    /// * `offset` - An integer representing the offset starting from 0.
    /// * `length` - An integer representing the desired length.
    ///
    /// # Examples
    ///
    /// ```
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray$slice(2, 3)
    /// print(harray)
    /// ```
    ///
    pub fn slice(&mut self, offset: i32, length: i32) {
        let inner_mut = self.get_inner_mut();
        inner_mut.slice(offset, length);
    }

    /// HArray
    /// `print()`
    /// Print the HArray.
    ///
    /// # Examples
    ///
    /// ```
    /// harray$print()
    ///
    /// # or similarly:
    /// print(harray)
    /// ```
    ///
    pub fn print(&self) {
        self.0.print();
    }

    /// HArray
    /// `eq(other: HArray) -> logical`
    /// Equality with another HArray.
    /// The comparison only checks if the dtype and the values are the same. To compare if the
    /// underlying data is the same in memory, check `eq_inner`.
    ///
    /// # Returns
    ///
    /// A logical.
    ///
    /// # Examples
    ///
    /// ```
    /// harray1 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray2 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray1$eq(harray2) # TRUE
    ///
    /// # or similarly:
    /// harray1 == harray2
    /// ```
    ///
    pub fn eq(&self, other: &HArray) -> bool {
        self.0.eq(&other.0)
    }

    /// HArray
    /// `ne(other: HArray) -> logical`
    /// Difference with another HArray.
    /// The comparison only checks if the dtype and the values are the same. To compare if the
    /// underlying data is the same in memory, check `eq_inner`.
    ///
    /// # Returns
    ///
    /// A logical.
    ///
    /// # Examples
    ///
    /// ```
    /// harray1 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray2 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray1$ne(harray2) # FALSE
    ///
    /// # or similarly:
    /// harray1 != harray2
    /// ```
    ///
    pub fn ne(&self, other: &HArray) -> bool {
        self.0.ne(&other.0)
    }

    /// HArray
    /// `eq_inner(other: HArray) -> logical`
    /// Inner equality with another HArray.
    /// The comparison checks if the underlying data is the same in memory. To compare if the
    /// HArrays are the same regarding values and dtype, check `eq` and `ne`.
    ///
    /// # Returns
    ///
    /// A logical.
    ///
    /// # Examples
    ///
    /// ```
    /// harray1 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray2 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray1$eq_inner(harray2) # FALSE
    ///
    /// harray1 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray2 = harray1$clone()
    /// harray1$eq_inner(harray2) # TRUE
    /// ```
    ///
    pub fn eq_inner(&self, other: &HArray) -> bool {
        self.0.eq_inner(&*other.0)
    }

    /// HArray
    /// `eq_inner_hmatrix(other: HMatrix) -> logical`
    /// Inner equality with an HMatrix.
    /// The comparison checks if the underlying data is the same in memory.
    ///
    /// # Returns
    ///
    /// A logical.
    ///
    /// # Examples
    ///
    /// ```
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4,5,6,7,8), 2, 4), HDataType$float32)
    /// harray$eq_inner_hmatrix(hmatrix) # FALSE
    ///
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// hmatrix = harray$as_hmatrix(ncols = 4)
    /// harray$eq_inner_hmatrix(hmatrix) # TRUE
    /// ```
    ///
    pub fn eq_inner_hmatrix(&self, other: &HMatrix) -> bool {
        self.0.eq_inner(&*other.0)
    }

    /// HArray
    /// `eq_inner_haudio(other: HAudio) -> logical`
    /// Inner equality with an HAudio.
    /// The comparison checks if the underlying data is the same in memory.
    ///
    /// # Returns
    ///
    /// A logical.
    ///
    /// # Examples
    ///
    /// ```
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// haudio = HMatrix$new_from_values(matrix(c(1,2,3,4,5,6,7,8), 2, 4), HDataType$float32)$as_haudio(sr = 44100)
    /// harray$eq_inner_haudio(haudio) # FALSE
    ///
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// haudio = harray$as_hmatrix(ncols = 4)$as_haudio(sr = 44100)
    /// harray$eq_inner_haudio(haudio) # TRUE
    /// ```
    ///
    pub fn eq_inner_haudio(&self, other: &HAudio) -> bool {
        self.0.eq_inner(&*other.0)
    }

    /// HArray
    /// `clone() -> HArray`
    /// Creates a new HArray, with the underlying data pointing to the same place in memory.
    ///
    /// # Returns
    ///
    /// An HArray.
    ///
    /// # Examples
    ///
    /// ```
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// harray2 = harray$clone()
    ///
    /// harray == harray2 # TRUE
    /// harray$eq_inner(harray2) # TRUE
    /// ```
    ///
    pub fn clone(&self) -> HArray {
        std::clone::Clone::clone(self)
    }

    /// HArray
    /// `as_hmatrix(ncols: integer) -> HMatrix`
    /// Creates a new HMatrix, with the underlying data pointing to the same place in memory.
    ///
    /// # Returns
    ///
    /// An HMatrix.
    ///
    /// # Examples
    ///
    /// ```
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// hmatrix = harray$as_hmatrix(ncols = 2)
    ///
    /// harray$eq_inner_hmatrix(hmatrix) # TRUE
    /// ```
    ///
    pub fn as_hmatrix(&self, ncols: i32) -> HMatrix {
        HMatrix(self.0.as_hmatrix(ncols))
    }

    /// HArray
    /// Collect to an atomic vector.
    pub fn collect(&self) -> Robj {
        self.0.collect()
    }

    /// HArray
    /// The inner array's memory adress.
    pub fn mem_adress(&self) -> String {
        self.0.mem_adress()
    }

    /// HArray
    /// The inner array's data type.
    pub fn data_type(&self) -> HDataType {
        self.0.data_type()
    }

    /// HArray
    /// Returns true if the inner Arc is shared.
    pub fn is_shared(&self) -> bool {
        Arc::weak_count(&self.0) + Arc::strong_count(&self.0) != 1
    }

    /// HArray
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

    pub fn fft(&self) -> HArray {
        HArray(self.0.fft())
    }
}

impl HArray {
    #[doc(hidden)]
    pub fn get_inner_mut(&mut self) -> &mut dyn HArrayR {
        if Arc::weak_count(&self.0) + Arc::strong_count(&self.0) != 1 {
            self.0 = self.0.clone_inner();
        }
        Arc::get_mut(&mut self.0).expect("implementation error")
    }
}

impl HArrayR for HFloatArray<f32> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn slice(&mut self, offset: i32, length: i32) {
        HFloatArray::slice(self, offset.try_into().unwrap(), length.try_into().unwrap());
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_hmatrix(&self, ncols: i32) -> Arc<dyn HMatrixR> {
        let hmatrix = self
            .clone() // ARC clone for the underlying data (O(1)). Underlying data is not copied.
            .into_hmatrix(usize::try_from(ncols).unwrap())
            .unwrap();
        Arc::new(hmatrix)
    }

    fn collect(&self) -> Robj {
        let values = self.inner().values();
        let doubles = values
            .iter()
            .map(|x| Rfloat(*x as f64))
            .collect::<Doubles>();
        doubles.try_into().unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.inner().values().as_slice();
        format!("{:p}", p)
    }

    fn data_type(&self) -> HDataType {
        HDataType::Float32
    }

    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema) {
        HFloatArray::export_c_arrow(self)
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftFloatArray::<f32>::fft(self);
        Arc::new(harray)
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }
}

impl HArrayR for HFloatArray<f64> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn slice(&mut self, offset: i32, length: i32) {
        HFloatArray::slice(self, offset.try_into().unwrap(), length.try_into().unwrap());
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_hmatrix(&self, ncols: i32) -> Arc<dyn HMatrixR> {
        let hmatrix = self
            .clone() // ARC clone for the underlying data (O(1)). Underlying data is not copied.
            .into_hmatrix(usize::try_from(ncols).unwrap())
            .unwrap();
        Arc::new(hmatrix)
    }

    fn collect(&self) -> Robj {
        let values = self.inner().values();
        let doubles = values.iter().map(|x| Rfloat(*x)).collect::<Doubles>();
        doubles.try_into().unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.inner().values().as_slice();
        format!("{:p}", p)
    }

    fn data_type(&self) -> HDataType {
        HDataType::Float64
    }

    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema) {
        HFloatArray::export_c_arrow(self)
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftFloatArray::<f64>::fft(self);
        Arc::new(harray)
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }
}

impl HArrayR for HComplexArray<f32> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn slice(&mut self, offset: i32, length: i32) {
        HComplexArray::slice(self, offset.try_into().unwrap(), length.try_into().unwrap());
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_hmatrix(&self, ncols: i32) -> Arc<dyn HMatrixR> {
        let hmatrix = self
            .clone() // ARC clone for the underlying data (O(1)). Underlying data is not copied.
            .into_hmatrix(usize::try_from(ncols).unwrap())
            .unwrap();
        Arc::new(hmatrix)
    }

    fn collect(&self) -> Robj {
        let values = self.inner().values();
        let complexes = values
            .chunks_exact(2)
            .map(|x| Rcplx::new((*x)[0] as f64, (*x)[1] as f64))
            .collect::<Complexes>();
        complexes.try_into().unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.inner().values().as_slice();
        format!("{:p}", p)
    }

    fn data_type(&self) -> HDataType {
        HDataType::Complex32
    }

    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema) {
        HComplexArray::export_c_arrow(self)
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftComplexArray::fft(self);
        Arc::new(harray)
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }
}

impl HArrayR for HComplexArray<f64> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn slice(&mut self, offset: i32, length: i32) {
        HComplexArray::slice(self, offset.try_into().unwrap(), length.try_into().unwrap());
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_hmatrix(&self, ncols: i32) -> Arc<dyn HMatrixR> {
        let hmatrix = self
            .clone() // ARC clone for the underlying data (O(1)). Underlying data is not copied.
            .into_hmatrix(usize::try_from(ncols).unwrap())
            .unwrap();
        Arc::new(hmatrix)
    }

    fn collect(&self) -> Robj {
        let values = self.inner().values();
        let complexes = values
            .chunks_exact(2)
            .map(|x| Rcplx::new((*x)[0], (*x)[1]))
            .collect::<Complexes>();
        complexes.try_into().unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.inner().values().as_slice();
        format!("{:p}", p)
    }

    fn data_type(&self) -> HDataType {
        HDataType::Complex64
    }

    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema) {
        HComplexArray::export_c_arrow(self)
    }

    fn fft(&self) -> Arc<dyn HArrayR> {
        let harray = FftComplexArray::fft(self);
        Arc::new(harray)
    }

    fn clone_inner(&self) -> Arc<dyn HArrayR> {
        Arc::new(self.clone())
    }
}

extendr_module! {
    mod harray;
    impl HArray;
}
