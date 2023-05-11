use crate::{
    hdatatype::HDataType,
    hmatrix::{HMatrix, HMatrixR},
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
    fn dtype(&self) -> HDataType;
    fn export_c_arrow(&self) -> (ArrowArray, ArrowSchema);
    fn fft(&self) -> Arc<dyn HArrayR>;
    fn clone_inner(&self) -> Arc<dyn HArrayR>;
}

/// HArray
/// An array representation. \
/// Supports Float32, Float64, Complex32 and Complex64 types. \
///
/// ## Methods
#[derive(Clone)]
pub struct HArray(pub Arc<dyn HArrayR>);

#[extendr]
impl HArray {
    /// HArray
    /// ### new_from_values
    ///
    /// `new_from_values(values: atomicvector, dtype: HDataType)` \
    ///
    /// Creates a new `HArray` from an R atomic vector.
    ///
    /// #### Arguments
    ///
    /// * `values` \
    /// A double or complex atomic vector.
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// For float dtypes, the atomic vector must be a double. For complex dtypes, a complex atomic vector.
    ///
    /// #### Returns
    ///
    /// An `HArray` external pointer
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = c(1,2,3,4,5,6,7,8,9,10,11,12)
    /// dtype = HDataType$float32
    /// HArray$new_from_values(values, dtype)
    /// ```
    ///
    /// _________
    pub fn new_from_values(values: Robj, dtype: &HDataType) -> HArray {
        match (values.rtype(), dtype) {
            (Rtype::Doubles, HDataType::Float32) => {
                let slice = values.as_real_slice().unwrap();
                let v = slice.iter().map(|x| *x as f32).collect();
                let hfloatarray = HFloatArray::<f32>::new_from_vec(v);
                let data = Arc::new(hfloatarray);
                HArray(data)
            }
            (Rtype::Doubles, HDataType::Float64) => {
                let v = values.as_real_vector().unwrap();
                let hfloatarray = HFloatArray::<f64>::new_from_vec(v);
                let data = Arc::new(hfloatarray);
                HArray(data)
            }
            (Rtype::Complexes, HDataType::Complex32) => {
                let slice: &[Rcplx] = values.as_typed_slice().unwrap();
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
                let slice: &[Rcplx] = values.as_typed_slice().unwrap();
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
    /// ### new_from_arrow
    ///
    /// `new_from_arrow(values: Array, dtype: HDataType)` \
    ///
    /// Creates a new `HArray` from an R's arrow `Array`. \
    /// The conversion is zero copy.
    ///
    /// #### Arguments
    ///
    /// * `values` \
    /// A float32 or float64 arrow `Array`.
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created.
    ///
    /// #### Returns
    ///
    /// An `HArray` external pointer
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = arrow::Array$create(1:10, type = float32())
    /// dtype = HDataType$complex32
    /// HArray$new_from_arrow(values, dtype)
    /// ```
    ///
    /// _________
    pub fn new_from_arrow(values: Robj, dtype: &HDataType) -> HArray {
        if !values.class().unwrap().any(|x| x == "Array") {
            panic!("wrong type");
        }

        let array = ArrowArray::empty();
        let schema = ArrowSchema::empty();

        let array_ptr = (&array as *const ArrowArray) as usize;
        let schema_ptr = (&schema as *const ArrowSchema) as usize;

        values
            .dollar("export_to_c")
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
    /// ### len
    ///
    /// `len() -> integer` \
    ///
    /// Returns the length of this `Harray`.
    ///
    /// #### Returns
    ///
    /// An integer.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray$len()
    /// ```
    ///
    /// _________
    pub fn len(&self) -> i32 {
        self.0.len()
    }

    /// HArray
    /// ### slice
    ///
    /// `slice(offset: integer, length: integer)` \
    ///
    /// Slice the `HArray` by an offset and length. \
    /// This operation is O(1). \
    /// The function will modify in-place the current `HArray`. If a clone of the `HArray` has been
    /// previously made, it will clone the `HArray` and slice it.
    ///
    /// #### Arguments
    ///
    /// * `offset` \
    /// An integer representing the offset starting from 0.
    /// * `length` \
    /// An integer representing the desired length.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray$slice(2, 3)
    /// print(harray)
    ///
    /// # if the HArray object is not being shared, slicing it will modify the HArray in-place.
    /// harray = HArray$new_from_values(c(1,2,3), HDataType$float64)
    /// harray$slice(1, 1)
    ///
    /// # if the HArray object is being shared, slicing it will create HArray object.
    /// harray = HArray$new_from_values(c(1,2,3), HDataType$float64)
    /// harray2 = harray$clone()
    /// harray$is_shared() # TRUE
    /// harray$slice(1, 1) # now harray is a different object in comparison with harray2, although
    /// they share the same underlying data.
    /// harray$is_shared() # FALSE
    ///
    /// ```
    ///
    /// _________
    pub fn slice(&mut self, offset: i32, length: i32) {
        let inner_mut = self.get_inner_mut();
        inner_mut.slice(offset, length);
    }

    /// HArray
    /// ### print
    ///
    /// `print()` \
    ///
    /// Print the `HArray`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray$print()
    ///
    /// # or similarly:
    /// print(harray)
    /// ```
    ///
    /// _________
    pub fn print(&self) {
        self.0.print();
    }

    /// HArray
    /// ### eq
    ///
    /// `eq(other: HArray) -> logical` \
    ///
    /// Equality with another HArray. \
    /// The comparison only checks if the dtype and the values are the same. To compare if the
    /// underlying data is the same in memory, check `mem_adress`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HArray`.
    ///
    /// #### Returns
    ///
    /// A logical.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray1 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray2 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray1$eq(harray2) # TRUE
    ///
    /// # or similarly:
    /// harray1 == harray2
    /// ```
    ///
    /// _________
    pub fn eq(&self, other: &HArray) -> bool {
        self.0.eq(&other.0)
    }

    /// HArray
    /// ### ne
    ///
    /// `ne(other: HArray) -> logical` \
    ///
    /// Difference with another `HArray`. \
    /// The comparison only checks if the dtype and the values are the same. To compare if the
    /// underlying data is the same in memory, check `mem_adress`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HArray`.
    ///
    /// #### Returns
    ///
    /// A logical.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray1 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray2 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray1$ne(harray2) # FALSE
    ///
    /// # or similarly:
    /// harray1 != harray2
    /// ```
    ///
    /// _________
    pub fn ne(&self, other: &HArray) -> bool {
        self.0.ne(&other.0)
    }

    /// HArray
    /// ### clone
    ///
    /// `clone() -> HArray` \
    ///
    /// Creates a new `HArray`, with the underlying data pointing to the same place in memory.
    ///
    /// #### Returns
    ///
    /// An `HArray`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// harray2 = harray$clone()
    ///
    /// harray == harray2 # TRUE
    /// harray$mem_adress() == harray2$mem_adress() # TRUE
    /// ```
    ///
    /// _________
    pub fn clone(&self) -> HArray {
        std::clone::Clone::clone(self)
    }

    /// HArray
    /// ### as_hmatrix
    ///
    /// `as_hmatrix(ncols: integer) -> HMatrix` \
    ///
    /// Creates a new `HMatrix`, with the underlying data pointing to the same place in memory.
    ///
    /// #### Arguments
    ///
    /// * `ncols` \
    /// An integer representing the number of columns desired. \
    /// Will return an error if `ncols` is not a divider of the length of the `HArray`.
    ///
    /// #### Returns
    ///
    /// An `HMatrix`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// hmatrix = harray$as_hmatrix(ncols = 2)
    ///
    /// harray$mem_adress() == hmatrix$mem_adress() # TRUE
    /// ```
    ///
    /// _________
    pub fn as_hmatrix(&self, ncols: i32) -> HMatrix {
        HMatrix(self.0.as_hmatrix(ncols))
    }

    /// HArray
    /// ### collect
    ///
    /// `collect() -> atomicvector` \
    ///
    /// Create an R atomic vector from an `HArray`. The type of the atomic vector created (double or complex) will depend on the `HArray`'s dtype.
    ///
    /// #### Returns
    ///
    /// An atomic vector of type double or complex.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray$collect()
    /// ```
    ///
    /// _________
    pub fn collect(&self) -> Robj {
        self.0.collect()
    }

    /// HArray
    /// ### mem_adress
    ///
    /// `mem_adress() -> character` \
    ///
    /// The memory adress of the first element of the inner data. \
    /// This is useful to check if different objects share the same underlying data. \
    /// It's important that the offset of both objects is at the same element for this comparison.
    ///
    /// #### Returns
    ///
    /// A Character.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray = HArray$new_from_values(c(1,2,3,4), HDataType$float64)
    /// hmatrix = harray$as_hmatrix(ncols = 2)
    /// harray$mem_adress() == hmatrix$mem_adress() # TRUE
    ///
    /// harray$slice(1,1) # changing the offset to 1
    /// harray$mem_adress() == hmatrix$mem_adress() # FALSE, even though they still share the same underlying data
    ///
    /// harray2 = harray
    /// harray$mem_adress() == harray3$mem_adress() # TRUE, since `=` operator only creates an alias (harray and harray3 are the same external pointer).
    /// ```
    ///
    /// _________
    pub fn mem_adress(&self) -> String {
        self.0.mem_adress()
    }

    /// HArray
    /// ### dtype
    ///
    /// `dtype() -> HDataType` \
    ///
    /// Get the `HArray`'s dtype as an `HDataType`.
    ///
    /// #### Returns
    ///
    /// An `HDataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray$dtype()
    /// ```
    ///
    /// _________
    pub fn dtype(&self) -> HDataType {
        self.0.dtype()
    }

    /// HArray
    /// ### is_shared
    ///
    /// `is_shared() -> logical` \
    ///
    /// Checks if the object is shared. \
    /// Since HArray has a COW ([clone-on-write](https://doc.rust-lang.org/std/borrow/enum.Cow.html)) behaviour, this function is useful to check if a new
    /// object will be created or if the change will be done in-place.
    ///
    /// #### Returns
    ///
    /// A logical.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray = HArray$new_from_values(c(1,2,3,4), HDataType$float64)
    /// harray$is_shared() # FALSE.
    ///
    /// hmatrix = harray$as_hmatrix(ncols = 2)
    /// harray$is_shared() # FALSE, since there's only one HArray object.
    /// harray$mem_adress() == hmatrix$mem_adress() # TRUE, since they share the same underlying data.
    ///
    /// harray2 = harray$clone()
    /// harray$is_shared() # TRUE, HArray object shared with harray2.
    /// harray2$is_shared() # TRUE.
    /// harray$mem_adress() == harray2$mem_adress() # TRUE, since they share the same underlying data.
    ///
    /// harray = HArray$new_from_values(c(1,2,3,4), HDataType$float64)
    /// harray2 = harray$clone()
    /// harray$mem_adress() == harray2$mem_adress() # TRUE.
    /// harray$is_shared() # TRUE
    /// harray$slice(0, 1)
    /// harray$mem_adress() == harray2$mem_adress() # TRUE. harray and harray2 still share the same underlying data.
    /// harray$is_shared() # FALSE, because a new HArray object was created for harray.
    /// ```
    ///
    /// _________
    pub fn is_shared(&self) -> bool {
        Arc::weak_count(&self.0) + Arc::strong_count(&self.0) != 1
    }

    /// Export the underlying array to Arrow C interface.
    /// For internal use only.
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

    /// HArray
    /// ### fft
    ///
    /// `fft() -> HArray` \
    ///
    /// Computes the fast fourier transform of the HArray. \
    /// FFT (Fast Fourier Transform) refers to a way the discrete Fourier Transform (DFT) can be calculated efficiently,
    /// by using symmetries in the calculated terms. The symmetry is highest when n is a power of 2, and the transform
    /// is therefore most efficient for these sizes. \
    ///
    /// The function does not normalize outputs. Callers must manually normalize the results by scaling each element by
    /// 1/len().sqrt(). Multiple normalization steps can be merged into one via pairwise multiplication, so when doing
    /// a forward FFT followed by an inverse callers can normalize once by scaling each element by 1/len(). \
    ///
    /// Elements in the output are ordered by ascending frequency, with the first element corresponding to frequency 0.
    ///
    /// #### Returns
    ///
    /// A complex HArray.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray$fft()
    /// ```
    ///
    /// _________
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

    fn dtype(&self) -> HDataType {
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

    fn dtype(&self) -> HDataType {
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

    fn dtype(&self) -> HDataType {
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

    fn dtype(&self) -> HDataType {
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
