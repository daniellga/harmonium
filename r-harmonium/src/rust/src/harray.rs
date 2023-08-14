use crate::{conversions::RobjConversions, harrayr::HArrayR, hdatatype::HDataType};
use extendr_api::prelude::*;
use num_complex::Complex;
use std::sync::Arc;

/// HArray
/// An array representation. \
///
/// # Methods
///
#[derive(Clone)]
pub struct HArray(pub Arc<dyn HArrayR>);

#[extendr]
impl HArray {
    /// HArray
    /// ## new_from_values
    ///
    /// `new_from_array(arr: array, dtype: HDataType) -> HArray` \
    ///
    /// Creates a new `HArray` from an R array. \
    ///
    /// #### Arguments
    ///
    /// * `arr` \
    /// A `double` or `complex` array.
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// For float dtypes, the atomic vector must be a `double`. For complex dtypes, a `complex` atomic vector.
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// HArray$new_from_values(arr, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_from_values(arr: Robj, dtype: &HDataType) -> HArray {
        assert!(arr.is_array());

        // Ok to unwrap since it was checked that robj is an array.
        let mut dim: Vec<usize> = arr
            .dim()
            .unwrap()
            .iter()
            .map(|z| z.inner() as usize)
            .collect();
        dim.reverse();

        match (arr.rtype(), dtype) {
            (Rtype::Doubles, HDataType::Float32) => {
                let slice: &[f64] = arr.robj_to_slice();
                let v: Vec<f32> = slice.iter().map(|x| *x as f32).collect();
                let harray = harmonium_core::array::HArray::new_from_shape_vec(dim, v).unwrap();
                let data = Arc::new(harray);
                HArray(data)
            }
            (Rtype::Doubles, HDataType::Float64) => {
                let v: Vec<f64> = arr.robj_to_slice().to_vec();
                let harray = harmonium_core::array::HArray::new_from_shape_vec(dim, v).unwrap();
                let data = Arc::new(harray);
                HArray(data)
            }
            (Rtype::Complexes, HDataType::Complex32) => {
                let slice: &[Rcplx] = arr.robj_to_slice();
                let v: Vec<Complex<f32>> = slice
                    .iter()
                    .map(|z| Complex::new(z.re().inner() as f32, z.im().inner() as f32))
                    .collect();
                let harray = harmonium_core::array::HArray::new_from_shape_vec(dim, v).unwrap();
                let data = Arc::new(harray);
                HArray(data)
            }
            (Rtype::Complexes, HDataType::Complex64) => {
                let slice: &[Rcplx] = arr.robj_to_slice();
                let v: Vec<Complex<f64>> = slice
                    .iter()
                    .map(|z| Complex::new(z.re().inner(), z.im().inner()))
                    .collect();
                let harray = harmonium_core::array::HArray::new_from_shape_vec(dim, v).unwrap();
                let data = Arc::new(harray);
                HArray(data)
            }
            _ => panic!("not valid input types"),
        }
    }

    /// HArray
    /// ## len
    ///
    /// `len() -> integer` \
    ///
    /// Returns the number of elements of this `Harray`. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$len()
    /// ```
    ///
    /// _________
    ///
    fn len(&self) -> Robj {
        let len: i32 = self.0.len().try_into().unwrap();
        let rint = Rint::from(len);
        rint.into()
    }

    /// HArray
    /// ## shape
    ///
    /// `shape() -> integers` \
    ///
    /// Returns the shape of this `HArray`. \
    ///
    /// #### Returns
    ///
    /// A vector of integers. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$shape()
    /// ```
    ///
    /// _________
    ///
    fn shape(&self) -> Robj {
        let shape = self.0.shape();
        let integers: Integers = shape.iter().map(|z| Rint::from(*z as i32)).collect();
        integers.into()
    }

    /// HArray
    /// ## ndim
    ///
    /// `ndim() -> integer` \
    ///
    /// Returns the number of dimensions of this `HArray`. \
    ///
    /// #### Returns
    ///
    /// An integer. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$ndim()
    /// ```
    ///
    /// _________
    ///
    fn ndim(&self) -> Robj {
        let ndim = self.0.ndim() as i32;
        let rint = Rint::from(ndim);
        rint.into()
    }

    /// HArray
    /// ## print
    ///
    /// `print()` \
    ///
    /// Prints the `HArray`. \
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$print()
    ///
    /// # or similarly:
    /// print(harray)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) {
        self.0.print();
    }

    /// HArray
    /// ## eq
    ///
    /// `eq(other: HArray) -> bool` \
    ///
    /// Equality with another `HArray`. \
    /// The comparison only checks if the dtype and the values are the same. To compare if the
    /// underlying data is the same in memory, check `mem_adress`. \
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HArray`. \
    ///
    /// #### Returns
    ///
    /// A `bool`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray1 = HArray$new_from_values(arr, dtype)
    ///
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray2 = HArray$new_from_values(arr, dtype)
    ///
    /// harray1$eq(harray2) # TRUE
    ///
    /// # or similarly:
    /// harray1 == harray2
    /// ```
    ///
    /// _________
    ///
    fn eq(&self, other: &HArray) -> Robj {
        let eq = self.0.eq(&other.0);
        let rbool = Rbool::from(eq);
        rbool.into()
    }

    /// HArray
    /// ## ne
    ///
    /// `ne(other: HArray) -> bool` \
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
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray1 = HArray$new_from_values(arr, dtype)
    ///
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray2 = HArray$new_from_values(arr, dtype)
    ///
    /// harray1$ne(harray2) # FALSE
    ///
    /// # or similarly:
    /// harray1 != harray2
    /// ```
    ///
    /// _________
    ///
    fn ne(&self, other: &HArray) -> Robj {
        let ne = self.0.ne(&other.0);
        let rbool = Rbool::from(ne);
        rbool.into()
    }

    /// HArray
    /// ## clone
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
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray1 = HArray$new_from_values(arr, dtype)
    /// harray2 = harray1$clone()
    /// harray1 == harray2 # TRUE
    /// ```
    ///
    /// _________
    ///
    fn clone(&self) -> HArray {
        std::clone::Clone::clone(self)
    }

    /// HArray
    /// ## collect
    ///
    /// `collect() -> array` \
    ///
    /// Creates an R array from an `HArray`. The type of the array created (`double` or `complex`) will depend on the `HArray`'s dtype.
    ///
    /// #### Returns
    ///
    /// An array of type `double` or `complex`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$collect()
    /// ```
    ///
    /// _________
    ///
    fn collect(&self) -> Robj {
        self.0.collect()
    }

    /// HArray
    /// ## dtype
    ///
    /// `dtype() -> HDataType` \
    ///
    /// Gets the `HArray`'s dtype as an `HDataType`.
    ///
    /// #### Returns
    ///
    /// An `HDataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$dtype()
    /// ```
    ///
    /// _________
    ///
    fn dtype(&self) -> HDataType {
        self.0.dtype()
    }

    /// HArray
    /// ## is_shared
    ///
    /// `is_shared() -> bool` \
    ///
    /// Checks if the object is shared. \
    /// Since `HArray` has a COW ([clone-on-write](https://doc.rust-lang.org/std/borrow/enum.Cow.html)) behaviour, this function is useful to check if a new
    /// object will be created or if the change will be done in-place. \
    ///
    /// #### Returns
    ///
    /// A `bool`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$float32
    /// harray1 = HArray$new_from_values(arr, dtype)
    /// harray1$is_shared() # FALSE.
    ///
    /// harray2 = harray1$clone()
    /// harray$is_shared() # TRUE, HArray object shared with harray2.
    /// ```
    ///
    /// _________
    ///
    fn is_shared(&self) -> Robj {
        let bool = Arc::weak_count(&self.0) + Arc::strong_count(&self.0) != 1;
        let rbool = Rbool::from(bool);
        rbool.into()
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

extendr_module! {
    mod harray;
    impl HArray;
}
