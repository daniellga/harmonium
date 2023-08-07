use crate::{harrayr::HArrayR, hdatatype::HDataType};
use extendr_api::{prelude::*, AsTypedSlice};
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
    /// `new_from_array(robj: array, dtype: HDataType) -> HArray` \
    ///
    /// Creates a new `HArray` from an R array. \
    ///
    /// #### Arguments
    ///
    /// * `robj` \
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
    /// values = c(1,2,3,4,5,6,7,8,9,10,11,12)
    /// dtype = HDataType$float32
    /// HArray$new_from_values(values, dtype)
    /// ```
    ///
    /// _________
    ///
    pub fn new_from_values(robj: Robj, dtype: &HDataType) -> HArray {
        assert!(robj.is_array());

        // Ok to unwrap since it was checked that robj is an array.
        let mut dim: Vec<usize> = robj.dim().unwrap().iter().map(|z| z.0 as usize).collect();
        dim.reverse();

        match (robj.rtype(), dtype) {
            (Rtype::Doubles, HDataType::Float32) => {
                let slice: &[f64] = robj.as_typed_slice().unwrap();
                let v: Vec<f32> = slice.iter().map(|x| *x as f32).collect();
                let harray = harmonium_core::array::HArray::new_from_shape_vec(&dim, v).unwrap();
                let data = Arc::new(harray);
                HArray(data)
            }
            (Rtype::Doubles, HDataType::Float64) => {
                let v: Vec<f64> = robj.as_real_vector().unwrap();
                let harray = harmonium_core::array::HArray::new_from_shape_vec(&dim, v).unwrap();
                let data = Arc::new(harray);
                HArray(data)
            }
            (Rtype::Complexes, HDataType::Complex32) => {
                let slice: &[Rcplx] = robj.as_typed_slice().unwrap();
                let v: Vec<Complex<f32>> = slice
                    .iter()
                    .map(|z| Complex::new(z.re().0 as f32, z.im().0 as f32))
                    .collect();
                let harray = harmonium_core::array::HArray::new_from_shape_vec(&dim, v).unwrap();
                let data = Arc::new(harray);
                HArray(data)
            }
            (Rtype::Complexes, HDataType::Complex64) => {
                let slice: &[Rcplx] = robj.as_typed_slice().unwrap();
                let v: Vec<Complex<f64>> = slice
                    .iter()
                    .map(|z| Complex::new(z.re().0 as f64, z.im().0 as f64))
                    .collect();
                let harray = harmonium_core::array::HArray::new_from_shape_vec(&dim, v).unwrap();
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
    /// harray = HArray$new_from_values(c(1,2,3), HDataType$float32)
    /// harray$len()
    /// ```
    ///
    /// _________
    ///
    pub fn len(&self) -> i32 {
        self.0.len()
    }

    /// HArray
    /// ## print
    ///
    /// `print()` \
    ///
    /// Print the `HArray`. \
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray = HArray$new_from_values(c(1,2,3), HDataType$float32)
    /// harray$print()
    ///
    /// # or similarly:
    /// print(harray)
    /// ```
    ///
    /// _________
    ///
    pub fn print(&self) {
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
    /// harray1 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray2 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray1$eq(harray2) # TRUE
    ///
    /// # or similarly:
    /// harray1 == harray2
    /// ```
    ///
    /// _________
    ///
    pub fn eq(&self, other: &HArray) -> bool {
        self.0.eq(&other.0)
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
    /// harray1 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray2 = HArray$new_from_values(c(1,2,3,4,5,6,7), HDataType$float32)
    /// harray1$ne(harray2) # FALSE
    ///
    /// # or similarly:
    /// harray1 != harray2
    /// ```
    ///
    /// _________
    ///
    pub fn ne(&self, other: &HArray) -> bool {
        self.0.ne(&other.0)
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
    /// harray1 = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// harray2 = harray1$clone()
    /// harray1 == harray2 # TRUE
    /// harray1$mem_adress() == harray2$mem_adress() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn clone(&self) -> HArray {
        std::clone::Clone::clone(self)
    }

    /// HArray
    /// ## collect
    ///
    /// `collect() -> atomicvector` \
    ///
    /// Creates an R atomic vector from an `HArray`. The type of the atomic vector created (`double` or `complex`) will depend on the `HArray`'s dtype.
    ///
    /// #### Returns
    ///
    /// An atomic vector of type `double` or `complex`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// harray = HArray$new_from_values(c(1,2,3,4,5,6,7,8), HDataType$float32)
    /// harray$collect()
    /// ```
    ///
    /// _________
    ///
    pub fn collect(&self) -> Robj {
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
    /// harray = HArray$new_from_values(c(1,2,3,4), HDataType$float64)
    /// harray$dtype()
    /// ```
    ///
    /// _________
    ///
    pub fn dtype(&self) -> HDataType {
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
    /// harray = HArray$new_from_values(c(1,2,3,4), HDataType$float64)
    /// harray$is_shared() # FALSE.
    ///
    /// hmatrix = harray$as_hmatrix(ncols = 2L)
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
    /// harray$slice(0L, 1L)
    /// harray$mem_adress() == harray2$mem_adress() # TRUE. harray and harray2 still share the same underlying data.
    /// harray$is_shared() # FALSE, because a new HArray object was created for harray.
    /// ```
    ///
    /// _________
    ///
    pub fn is_shared(&self) -> bool {
        Arc::weak_count(&self.0) + Arc::strong_count(&self.0) != 1
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
