use crate::{
    harray::{HArray, HArrayR},
    haudio::{HAudio, HAudioR},
    hdatatype::HDataType,
};
use arrow2::{
    array::PrimitiveArray,
    ffi::{import_array_from_c, import_field_from_c, ArrowArray, ArrowSchema}, datatypes::PhysicalType, types::PrimitiveType,
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

/// HMatrix
/// A column major matrix representation. \
///
/// # Methods
///
#[derive(Clone)]
pub struct HMatrix(pub Arc<dyn HMatrixR>);

#[extendr]
impl HMatrix {
    /// HMatrix
    /// ## new_from_values
    ///
    /// `new_from_values(values: matrix, dtype: HDataType) -> HMatrix` \
    ///
    /// Creates a new `HMatrix` from a `matrix`. \
    ///
    /// #### Arguments
    ///
    /// * `values` \
    /// A `double` or `complex` `matrix`. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HMatrix` to be created. \
    /// For float dtypes, the `matrix` must be a `double`. For complex dtypes, a `complex`. \
    ///
    /// #### Returns
    ///
    /// An `HMatrix`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// HMatrix$new_from_values(values, dtype)
    /// ```
    ///
    /// _________
    ///
    pub fn new_from_values(values: Robj, dtype: &HDataType) -> HMatrix {
        match (values.rtype(), dtype) {
            (Rtype::Doubles, HDataType::Float32) => {
                let rmatrix: RMatrix<f64> = values.try_into().expect("not valid input types");
                let ncols = rmatrix.ncols();
                let v: Vec<f32> = rmatrix.data().iter().map(|z| *z as f32).collect();
                let hfloatmatrix = HFloatArray::<f32>::new_from_vec(v)
                    .into_hmatrix(ncols)
                    .unwrap();
                let inner = Arc::new(hfloatmatrix);
                HMatrix(inner)
            }
            (Rtype::Doubles, HDataType::Float64) => {
                let rmatrix: RMatrix<f64> = values.try_into().expect("not valid input types");
                let ncols = rmatrix.ncols();
                let slice = rmatrix.data();
                let array = PrimitiveArray::from_slice(slice);
                let hfloatmatrix = HFloatArray::<f64>::new(array).into_hmatrix(ncols).unwrap();
                let inner = Arc::new(hfloatmatrix);
                HMatrix(inner)
            }
            (Rtype::Complexes, HDataType::Complex32) => {
                let rmatrix: RMatrix<Rcplx> = values.try_into().expect("not valid input types");
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
                let rmatrix: RMatrix<Rcplx> = values.try_into().expect("not valid input types");
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

    /// HMatrix
    /// ## new_from_arrow
    ///
    /// `new_from_arrow(values: ArrowArray, ncols: i32, dtype: HDataType) -> HMatrix` \
    ///
    /// Creates a new `HArray` from an R's arrow [`Array`](https://arrow.apache.org/docs/r/reference/array.html). \
    /// The conversion is zero copy. \
    ///
    /// #### Arguments
    ///
    /// * `values` \
    /// A float32 or float64 arrow `Array`.
    /// * `ncols` \
    /// The number of columns of the HMatrix. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HMatrix` to be created. \
    ///
    /// #### Returns
    ///
    /// An `HMatrix`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = arrow::Array$create(1:10, type = arrow::float32())
    /// ncols = 2
    /// dtype = HDataType$complex32
    /// hmatrix = HMatrix$new_from_arrow(values, ncols, dtype)
    ///
    /// # to convert back to R's arrow FixedSizeListArray.
    /// values2 = arrow::as_arrow_array(hmatrix)
    /// all.equal(values, values2) # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn new_from_arrow(values: Robj, ncols: i32, dtype: &HDataType) -> HMatrix {
        if !values.class().unwrap().any(|x| x == "Array") {
            panic!("wrong type");
        }

        let ncols: usize = ncols.try_into().unwrap();

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
                let hmatrix = HFloatMatrix::new(harray, ncols).unwrap();
                HMatrix(Arc::new(hmatrix))
            }
            (HDataType::Float64, PhysicalType::Primitive(PrimitiveType::Float64)) => {
                let arr = arr.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
                let harray = HFloatArray::new(arr.clone());
                let hmatrix = HFloatMatrix::new(harray, ncols).unwrap();
                HMatrix(Arc::new(hmatrix))
            }
            (HDataType::Complex32, PhysicalType::Primitive(PrimitiveType::Float32)) => {
                let arr = arr.as_any().downcast_ref::<PrimitiveArray<f32>>().unwrap();
                let harray = HComplexArray::new(arr.clone());
                let hmatrix = HComplexMatrix::new(harray, ncols).unwrap();
                HMatrix(Arc::new(hmatrix))
            }
            (HDataType::Complex64, PhysicalType::Primitive(PrimitiveType::Float64)) => {
                let arr = arr.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
                let harray = HComplexArray::new(arr.clone());
                let hmatrix = HComplexMatrix::new(harray, ncols).unwrap();
                HMatrix(Arc::new(hmatrix))
            }
            _ => panic!("not valid input"),
        }
    }

    /// HMatrix
    /// ## len
    ///
    /// `len() -> integer` \
    ///
    /// Returns the number of elements of this `HMatrix`. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3)), HDataType$float32)
    /// hmatrix$len()
    /// ```
    ///
    /// _________
    ///
    pub fn len(&self) -> i32 {
        self.0.len()
    }

    /// HMatrix
    /// ## slice
    ///
    /// `slice(offset: integer, length: integer)` \
    ///
    /// Slice the `HMatrix` by an offset and length. \
    /// The operation is done on a column level, which means that `offset` always point at the
    /// start of a column. For instance, an `offset` of 2 and `length` of 3 will slice the columns
    /// 3, 4 and 5. \
    /// This operation is O(1). \
    /// The function will modify in-place the current `HMatrix`. If a clone of the `HMatrix` has been
    /// previously made, it will clone the `HMatrix` and slice it. \
    ///
    /// #### Arguments
    ///
    /// * `offset` \
    /// An `integer` representing the offset starting from 0. \
    /// * `length` \
    /// An `integer` representing the desired length. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4,5,6,7,8,9), ncol = 3L), HDataType$float32)
    /// hmatrix$slice(1L, 1L)
    /// print(hmatrix)
    ///
    /// # if the HMatrix object is not being shared, slicing it will modify the HMatrix in-place.
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4,5,6,7,8,9), ncol = 3L), HDataType$float32)
    /// hmatrix$slice(1L, 1L)
    ///
    /// # if the HMatrix object is being shared, slicing it will create a new HMatrix object.
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4,5,6,7,8,9), ncol = 3L), HDataType$float32)
    /// hmatrix2 = hmatrix$clone()
    /// hmatrix$is_shared() # TRUE
    /// hmatrix$slice(1L, 1L) # now hmatrix is a different object in comparison with hmatrix2, although they share the same underlying data.
    /// hmatrix$is_shared() # FALSE
    /// ```
    ///
    /// _________
    ///
    fn slice(&mut self, offset: i32, length: i32) {
        let inner_mut = self.get_inner_mut();
        inner_mut.slice(offset, length);
    }

    /// HMatrix
    /// ## ncols
    ///
    /// `ncols() -> integer` \
    ///
    /// Returns the number of columns of this `HMatrix`. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3)), HDataType$float32)
    /// hmatrix$ncols()
    /// ```
    ///
    /// _________
    ///
    pub fn ncols(&self) -> i32 {
        self.0.ncols()
    }

    /// HMatrix
    /// ## nrows
    ///
    /// `nrows() -> integer` \
    ///
    /// Returns the number of rows of this `HMatrix`. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3)), HDataType$float32)
    /// hmatrix$nrows()
    /// ```
    ///
    /// _________
    ///
    pub fn nrows(&self) -> i32 {
        self.0.nrows()
    }

    /// HMatrix
    /// ## print
    ///
    /// `print()` \
    ///
    /// Print the `HMatrix`. \
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3)), HDataType$float32)
    /// hmatrix$print()
    ///
    /// # or similarly:
    /// print(hmatrix)
    /// ```
    ///
    /// _________
    ///
    pub fn print(&self) {
        self.0.print();
    }

    /// HMatrix
    /// ## eq
    ///
    /// `eq(other: HMatrix) -> bool` \
    ///
    /// Equality with another `HMatrix`. \
    /// The comparison only checks if the dtype and the values are the same. To compare if the
    /// underlying data is the same in memory, check `mem_adress`. \
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HMatrix`. \
    ///
    /// #### Returns
    ///
    /// A `bool`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix1 = HMatrix$new_from_values(matrix(c(1,2,3)), HDataType$float32)
    /// hmatrix2 = HMatrix$new_from_values(matrix(c(1,2,3)), HDataType$float32)
    /// hmatrix1$eq(hmatrix2) # TRUE
    ///
    /// # or similarly:
    /// hmatrix1 == hmatrix2
    /// ```
    ///
    /// _________
    ///
    pub fn eq(&self, other: &HMatrix) -> bool {
        self.0.eq(&other.0)
    }

    /// HMatrix
    /// ## ne
    ///
    /// `ne(other: HMatrix) -> bool` \
    ///
    /// Difference with another `HMatrix`. \
    /// The comparison only checks if the dtype and the values are the same. To compare if the
    /// underlying data is the same in memory, check `mem_adress`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HMatrix`.
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix1 = HMatrix$new_from_values(matrix(c(1,2,3)), HDataType$float32)
    /// hmatrix2 = HMatrix$new_from_values(matrix(c(1,2,3)), HDataType$float32)
    /// hmatrix1$ne(hmatrix2) # FALSE
    ///
    /// # or similarly:
    /// hmatrix1 != hmatrix2
    /// ```
    ///
    /// _________
    ///
    pub fn ne(&self, other: &HMatrix) -> bool {
        self.0.ne(&other.0)
    }

    /// HMatrix
    /// ## clone
    ///
    /// `clone() -> HMatrix` \
    ///
    /// Creates a new `HMatrix`, with the underlying data pointing to the same place in memory.
    ///
    /// #### Returns
    ///
    /// An `HMatrix`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix1 = HMatrix$new_from_values(matrix(c(1,2,3,4), ncol = 2L), HDataType$float32)
    /// hmatrix2 = hmatrix1$clone()
    /// hmatrix1 == hmatrix2 # TRUE
    /// hmatrix1$mem_adress() == hmatrix2$mem_adress() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn clone(&self) -> HMatrix {
        std::clone::Clone::clone(self)
    }

    /// HMatrix
    /// ## as_harray
    ///
    /// `as_harray() -> HArray` \
    ///
    /// Creates a new `HArray`, with the underlying data pointing to the same place in memory. \
    ///
    /// #### Returns
    ///
    /// An `HArray`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4), ncol = 2L), HDataType$float32)
    /// harray = hmatrix$as_harray()
    ///
    /// harray$mem_adress() == hmatrix$mem_adress() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn as_harray(&self) -> HArray {
        HArray(self.0.as_harray())
    }

    /// HMatrix
    /// ## as_haudio
    ///
    /// `as_haudio(sr: integer) -> HAudio` \
    ///
    /// Creates a new `HAudio`, with the underlying data pointing to the same place in memory. \
    ///
    /// #### Arguments
    ///
    /// * `sr` \
    /// An `integer`. The sampling rate in hz. \
    ///
    /// #### Returns
    ///
    /// An `HAudio`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4), ncol = 2L), HDataType$float32)
    /// haudio = hmatrix$as_haudio(sr = 2L)
    ///
    /// haudio$mem_adress() == hmatrix$mem_adress() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn as_haudio(&self, sr: i32) -> HAudio {
        HAudio(self.0.as_haudio(sr))
    }

    /// HMatrix
    /// ## collect
    ///
    /// `collect() -> matrix` \
    ///
    /// Creates a `matrix` from an `HMatrix`. The type of the `matrix` created (`double` or `complex`) will depend on the `HMatrix`'s dtype.
    ///
    /// #### Returns
    ///
    /// A `matrix` of type `double` or `complex`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4), ncol = 2L), HDataType$float32)
    /// hmatrix$collect()
    /// ```
    ///
    /// _________
    ///
    pub fn collect(&self) -> Robj {
        self.0.collect()
    }

    /// HMatrix
    /// ## mem_adress
    ///
    /// `mem_adress() -> string` \
    ///
    /// The memory adress of the first element of the inner data. \
    /// This is useful to check if different objects share the same underlying data. \
    /// It's important that the offset of both objects is at the same element for this comparison. \
    ///
    /// #### Returns
    ///
    /// A `string`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray = HArray$new_from_values(c(1,2,3,4), HDataType$float64)
    /// hmatrix = harray$as_hmatrix(ncols = 2L)
    /// harray$mem_adress() == hmatrix$mem_adress() # TRUE
    ///
    /// harray$slice(1L,1L) # changing the offset to 1
    /// harray$mem_adress() == hmatrix$mem_adress() # FALSE, even though they still share the same underlying data
    ///
    /// hmatrix2 = hmatrix
    /// hmatrix$mem_adress() == hmatrix2$mem_adress() # TRUE, since `=` operator only creates an alias (hmatrix and hmatrix2 are the same external pointer).
    /// ```
    ///
    /// _________
    ///
    pub fn mem_adress(&self) -> String {
        self.0.mem_adress()
    }

    /// HMatrix
    /// ## dtype
    ///
    /// `dtype() -> HDataType` \
    ///
    /// Gets the `HMatrix`'s dtype as an `HDataType`.
    ///
    /// #### Returns
    ///
    /// An `HDataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4), ncol = 2L), HDataType$float32)
    /// hmatrix$dtype()
    /// ```
    ///
    /// _________
    ///
    pub fn dtype(&self) -> HDataType {
        self.0.dtype()
    }

    /// HMatrix
    /// ## is_shared
    ///
    /// `is_shared() -> bool` \
    ///
    /// Checks if the object is shared. \
    /// Since `HMatrix` has a COW ([clone-on-write](https://doc.rust-lang.org/std/borrow/enum.Cow.html)) behaviour, this function is useful to check if a new
    /// object will be created or if the change will be done in-place. \
    ///
    /// #### Returns
    ///
    /// A `bool`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4), ncol = 2L), HDataType$float32)
    /// hmatrix$is_shared() # FALSE.
    ///
    /// harray = hmatrix$as_harray()
    /// hmatrix$is_shared() # FALSE, since there's only one HMatrix object.
    /// harray$mem_adress() == hmatrix$mem_adress() # TRUE, since they share the same underlying data.
    ///
    /// hmatrix2 = hmatrix$clone()
    /// hmatrix$is_shared() # TRUE, HMatrix object shared with hmatrix2.
    /// hmatrix2$is_shared() # TRUE.
    /// hmatrix$mem_adress() == hmatrix2$mem_adress() # TRUE, since they share the same underlying data.
    ///
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4), ncol = 2L), HDataType$float32)
    /// hmatrix2 = hmatrix$clone()
    /// hmatrix$mem_adress() == hmatrix2$mem_adress() # TRUE
    /// hmatrix$is_shared() # TRUE
    /// hmatrix$slice(0L, 1L)
    /// hmatrix$mem_adress() == hmatrix2$mem_adress() # TRUE. hmatrix and hmatrix2 still share the same underlying data.
    /// hmatrix$is_shared() # FALSE, because a new HMatrix object was created for hmatrix.
    /// ```
    ///
    /// _________
    ///
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

    /// HMatrix
    /// ## fft
    ///
    /// `fft() -> HMatrix` \
    ///
    /// Computes the fast fourier transform of the `HMatrix`. The fft is computed for each column. \
    /// FFT (Fast Fourier Transform) refers to a way the discrete Fourier Transform (DFT) can be calculated efficiently,
    /// by using symmetries in the calculated terms. The symmetry is highest when n is a power of 2, and the transform
    /// is therefore most efficient for these sizes. \
    ///
    /// The function does not normalize outputs. Callers must manually normalize the results by scaling each element by
    /// `1/sqrt(n)`. Multiple normalization steps can be merged into one via pairwise multiplication, so when doing
    /// a forward FFT followed by an inverse callers can normalize once by scaling each element by `1/n`. \
    ///
    /// Elements in the output are ordered by ascending frequency, with the first element corresponding to frequency 0. \
    ///
    /// #### Returns
    ///
    /// An `HMatrix`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4), ncol = 2L), HDataType$float32)
    /// hmatrix$fft()
    /// ```
    ///
    /// _________
    ///
    pub fn fft(&self) -> HMatrix {
        HMatrix(self.0.fft())
    }

    /// HMatrix
    /// ## mean_cols
    ///
    /// `mean_cols()` \
    ///
    /// Takes the average across columns. \
    /// A new inner array is created. The operation is done in-place. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hmatrix = HMatrix$new_from_values(matrix(c(1,2,3,4), ncol = 2L), HDataType$float32)
    /// mem_adress_before = hmatrix$mem_adress()
    /// hmatrix$mean_cols()
    /// hmatrix$mem_adress() != mem_adress_before # TRUE
    /// ```
    ///
    /// _________
    ///
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
        let ncols = self.ncols();
        let nrows = self.nrows();
        self
            .inner
            .inner
            .values()
            .iter()
            .map(|x| *x as f64)
            .collect_rarray([nrows, ncols])
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.as_slice();
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
        *self = HFloatMatrix::<f32>::mean_cols(self).unwrap();
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
        let ncols = self.ncols();
        let nrows = self.nrows();
        self
            .inner
            .inner
            .values()
            .iter()
            .copied()
            .collect_rarray([nrows, ncols])
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.as_slice();
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
        *self = HFloatMatrix::<f64>::mean_cols(self).unwrap();
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
        self
            .inner
            .inner
            .values()
            .chunks_exact(2)
            .map(|x| Rcplx::new(x[0] as f64, x[1] as f64))
            .collect_rarray([nrows, ncols])
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.as_slice();
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
        *self = HComplexMatrix::<f32>::mean_cols(self).unwrap();
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
        self
            .inner
            .inner
            .values()
            .chunks_exact(2)
            .map(|x| Rcplx::new(x[0], x[1]))
            .collect_rarray([nrows, ncols])
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.as_slice();
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
        *self = HComplexMatrix::<f64>::mean_cols(self).unwrap();
    }

    fn clone_inner(&self) -> Arc<dyn HMatrixR> {
        Arc::new(self.clone())
    }
}

extendr_module! {
    mod hmatrix;
    impl HMatrix;
}
