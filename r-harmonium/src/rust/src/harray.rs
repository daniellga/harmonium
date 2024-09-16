use crate::{
    conversions::try_from_usize_to_int_sexp, errors::HErrorR, harrayr::HArrayR,
    hdatatype::HDataType,
};
use ndarray::{IxDyn, ShapeError, SliceInfo, SliceInfoElem};
use num_complex::Complex;
use savvy::{
    savvy, IntegerSexp, ListSexp, NotAvailableValue, OwnedIntegerSexp, OwnedLogicalSexp,
    OwnedStringSexp, Sexp, TypedSexp,
};
use std::sync::Arc;

/// HArray
/// An array representation.
///
/// # Methods
///
#[derive(Clone)]
#[savvy]
pub struct HArray(pub Arc<dyn HArrayR>);

#[savvy]
impl HArray {
    /// HArray
    /// ## new_from_values
    ///
    /// `new_from_values(arr: array, dtype: HDataType) -> HArray`
    ///
    /// Creates a new `HArray` from an R array.
    ///
    /// #### Arguments
    ///
    /// - `arr`
    ///
    /// A `double` or `complex` array.
    ///
    /// - `dtype`
    ///
    /// An `HDataType` to indicate which type of `HArray` to be created.
    ///
    /// For float dtypes, the atomic vector must be a `double`. For complex dtypes, a `complex` atomic vector.
    ///
    /// #### Returns
    ///
    /// An `HArray`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// HArray$new_from_values(arr, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_from_values(arr: Sexp, dtype: &HDataType) -> savvy::Result<HArray> {
        if let Some(dim) = arr.get_dim() {
            let dim: Vec<usize> = dim.iter().map(|z| *z as usize).rev().collect();

            match (arr.into_typed(), dtype) {
                (TypedSexp::Real(arr), HDataType::Float32) => {
                    let slice: &[f64] = arr.as_slice();
                    let v: Vec<f32> = slice.iter().map(|x| *x as f32).collect();
                    let harray = harmonium_core::array::HArray::new_from_shape_vec(dim, v)
                        .map_err(HErrorR::from)?;
                    let data = Arc::new(harray);
                    Ok(HArray(data))
                }
                (TypedSexp::Real(arr), HDataType::Float64) => {
                    let v: Vec<f64> = arr.as_slice().to_vec();
                    let harray = harmonium_core::array::HArray::new_from_shape_vec(dim, v)
                        .map_err(HErrorR::from)?;
                    let data = Arc::new(harray);
                    Ok(HArray(data))
                }
                (TypedSexp::Complex(arr), HDataType::Complex32) => {
                    let slice: &[Complex<f64>] = arr.as_slice();
                    let v: Vec<Complex<f32>> = slice
                        .iter()
                        .map(|z| Complex::new(z.re as f32, z.im as f32))
                        .collect();
                    let harray = harmonium_core::array::HArray::new_from_shape_vec(dim, v)
                        .map_err(HErrorR::from)?;
                    let data = Arc::new(harray);
                    Ok(HArray(data))
                }
                (TypedSexp::Complex(arr), HDataType::Complex64) => {
                    let v: Vec<Complex<f64>> = arr.as_slice().to_vec();
                    let harray = harmonium_core::array::HArray::new_from_shape_vec(dim, v)
                        .map_err(HErrorR::from)?;
                    let data = Arc::new(harray);
                    Ok(HArray(data))
                }
                _ => Err("not valid input types".into()),
            }
        } else {
            Err("arr must be of array type.".into())
        }
    }

    /// HArray
    /// ## len
    ///
    /// `len() -> integer`
    ///
    /// Returns the number of elements of this `Harray`.
    ///
    /// #### Returns
    ///
    /// An integer.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$len()
    /// ```
    ///
    /// _________
    ///
    fn len(&self) -> savvy::Result<Sexp> {
        let integer_sexp: OwnedIntegerSexp = try_from_usize_to_int_sexp(self.0.len())?;
        integer_sexp.into()
    }

    /// HArray
    /// ## shape
    ///
    /// `shape() -> integeratomicvector`
    ///
    /// Returns the shape of this `HArray`.
    ///
    /// #### Returns
    ///
    /// An integer atomic vector.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$shape()
    /// ```
    ///
    /// _________
    ///
    fn shape(&self) -> savvy::Result<Sexp> {
        let shape = self.0.shape();
        let mut integer_sexp = unsafe { OwnedIntegerSexp::new_without_init(shape.len())? };
        shape
            .iter()
            .map(|z| *z as i32)
            .zip(integer_sexp.as_mut_slice().iter_mut())
            .for_each(|(sh, int_sxp)| *int_sxp = sh);
        integer_sexp.into()
    }

    /// HArray
    /// ## ndim
    ///
    /// `ndim() -> integer`
    ///
    /// Returns the number of dimensions of this `HArray`.
    ///
    /// #### Returns
    ///
    /// An integer.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$ndim()
    /// ```
    ///
    /// _________
    ///
    fn ndim(&self) -> savvy::Result<Sexp> {
        let integer_sexp: OwnedIntegerSexp = try_from_usize_to_int_sexp(self.0.ndim())?;
        integer_sexp.into()
    }

    /// HArray
    /// ## slice
    ///
    /// `slice(range: list[atomicvector]) -> HArray`
    ///
    /// Slices the HArray.
    ///
    /// This operation has a COW ([clone-on-write](https://doc.rust-lang.org/std/borrow/enum.Cow.html)) behaviour. The created slice shares the inner data with
    /// the original array until one of them is modified.
    ///
    /// #### Arguments
    ///
    /// - `range`
    ///
    /// A list of vectors of integers.
    ///
    /// The number of vectors in the list must be equal to the number of dimensions in the original HArray as they represent the slice information for each axis.
    ///
    /// Each vector must be composed of 1 or 3 elements
    ///
    /// For 1 element: A single index. An index to use for taking a subview with respect to that axis. The index is selected, then the axis is removed.
    ///
    /// For 3 elements: [start, end, step]. All 3 values can be positive or negative, although step can't be 0.
    /// Negative start or end indexes are counted from the back of the axis. If end is None, the slice extends to the end of the axis.
    /// A `c(NA_integer_, NA_integer_, NA_integer_)` value for start will mean start = 0, end = axis_length, step = 1.
    ///
    /// #### Returns
    ///
    /// An `HArray`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20), c(4,5))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$slice(list(c(0L, 2L, 1L), c(1L, 3L, 1L)))
    /// harray$slice(list(c(0L, 4L, 1L), c(1L, NA, 1L)))
    /// harray$slice(list(c(0L, NA, 1L), c(1L, 3L, 1L)))
    /// harray$slice(list(0L, c(NA_integer_, NA, NA))) # using index
    /// x = c(NA_integer_, NA_integer_, NA_integer_)
    /// harray$slice(list(x, x)) == harray # TRUE
    /// ```
    ///
    /// _________
    ///
    fn slice(&self, range: Sexp) -> savvy::Result<HArray> {
        // ndarray already panics if an index is out of bounds or step size is zero. Also panics if D is IxDyn and info does not match the number of array axes.
        let range = ListSexp::try_from(range)?;
        let list_len = range.len();

        if list_len != self.0.ndim() {
            return Err("The list must have the same length as the number of dimensions.".into());
        }

        let mut vec_ranges: Vec<SliceInfoElem> = Vec::with_capacity(list_len);
        for obj in range.values_iter() {
            let integer_sexp = IntegerSexp::try_from(obj)?;
            let slice: &[i32] = integer_sexp.as_slice();
            if slice.len() == 1 {
                // Safety: the vector is checked to be length 1.
                let index = unsafe { *slice.get_unchecked(0) as isize };
                let slice_info_elem = SliceInfoElem::Index(index);
                vec_ranges.push(slice_info_elem);
            } else if slice.len() == 3 {
                // Safety: the vector is checked to be length 3.
                let (start, end, step) = (
                    unsafe { *slice.get_unchecked(0) },
                    unsafe { *slice.get_unchecked(1) },
                    unsafe { *slice.get_unchecked(2) },
                );

                let start = if start.is_na() { 0 } else { start as isize };
                let end = if end.is_na() {
                    None
                } else {
                    Some(end as isize)
                };
                let step = if step.is_na() { 1 } else { step as isize };

                let slice_info_elem = SliceInfoElem::Slice { start, end, step };
                vec_ranges.push(slice_info_elem);
            } else {
                return Err("Each element must have a length of 1 or 3.".into());
            }
        }

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, IxDyn> = vec_ranges
            .try_into()
            .map_err(|err: ShapeError| savvy::Error::new(err.to_string().as_str()))?;

        Ok(HArray(self.0.slice(slice_info)))
    }

    /// HArray
    /// ## print
    ///
    /// `print()`
    ///
    /// Prints the `HArray`.
    ///
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$print()
    ///
    /// # or similarly:
    /// print(harray)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        self.0.print();
        Ok(())
    }

    /// HArray
    /// ## eq
    ///
    /// `eq(other: HArray) -> bool`
    ///
    /// Equality with another `HArray`.
    ///
    /// The comparison only checks if the dtype and the values are the same. To compare if the
    /// underlying data is the same in memory, check `mem_adress`.
    ///
    /// #### Arguments
    ///
    /// - `other`
    ///
    /// An `HArray`.
    ///
    /// #### Returns
    ///
    /// A bool.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray1 = HArray$new_from_values(arr, dtype)
    ///
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
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
    fn eq(&self, other: &HArray) -> savvy::Result<Sexp> {
        let eq = self.0.eq(&other.0);
        let logical_sexp: OwnedLogicalSexp = eq.try_into()?;
        logical_sexp.into()
    }

    /// HArray
    /// ## ne
    ///
    /// `ne(other: HArray) -> bool`
    ///
    /// Difference with another `HArray`.
    ///
    /// The comparison only checks if the dtype and the values are the same. To compare if the
    /// underlying data is the same in memory, check `mem_adress`.
    ///
    /// #### Arguments
    ///
    /// - `other`
    ///
    /// An `HArray`.
    ///
    /// #### Returns
    ///
    /// A bool.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray1 = HArray$new_from_values(arr, dtype)
    ///
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
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
    fn ne(&self, other: &HArray) -> savvy::Result<Sexp> {
        let ne = self.0.ne(&other.0);
        let logical_sexp: OwnedLogicalSexp = ne.try_into()?;
        logical_sexp.into()
    }

    /// HArray
    /// ## clone
    ///
    /// `clone() -> HArray`
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
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray1 = HArray$new_from_values(arr, dtype)
    /// harray2 = harray1$clone()
    /// harray1 == harray2 # TRUE
    /// ```
    ///
    /// _________
    ///
    fn clone(&self) -> savvy::Result<HArray> {
        Ok(std::clone::Clone::clone(self))
    }

    /// HArray
    /// ## collect
    ///
    /// `collect() -> array`
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
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$collect()
    /// ```
    ///
    /// _________
    ///
    fn collect(&self) -> savvy::Result<Sexp> {
        self.0.collect()
    }

    /// HArray
    /// ## dtype
    ///
    /// `dtype() -> HDataType`
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
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$dtype()
    /// ```
    ///
    /// _________
    ///
    fn dtype(&self) -> savvy::Result<HDataType> {
        Ok(self.0.dtype())
    }

    /// HArray
    /// ## mem_adress
    ///
    /// `mem_adress() -> string`
    ///
    /// The memory adress of the first element of the inner array.
    ///
    /// This is useful to check if different objects share the same underlying data.
    ///
    /// #### Returns
    ///
    /// A string.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$mem_adress()
    /// ```
    ///
    /// _________
    ///
    pub fn mem_adress(&self) -> savvy::Result<Sexp> {
        let string_sexp: OwnedStringSexp = self.0.mem_adress().try_into()?;
        string_sexp.into()
    }

    /// HArray
    /// ## is_standard_layout
    ///
    /// `is_standard_layout() -> bool`
    ///
    /// Returns true if the array data is laid out in contiguous “C order” in memory (where the last index is the most rapidly varying).
    ///
    /// Returns false otherwise, i.e. the array is possibly not contiguous in memory, it has custom strides, etc.
    ///
    /// This function is useful mainly to check if an `HArray` is contiguous after some operation as, for example, `slice()`.
    ///
    /// #### Returns
    ///
    /// A bool.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$is_standard_layout() # TRUE, contiguous data
    /// sliced_harray = harray$slice(list(c(0L, 2L, 1L), c(1L, 3L, 1L)))
    /// sliced_harray$is_standard_layout() # FALSE, non contiguous data
    /// ```
    ///
    /// _________
    ///
    pub fn is_standard_layout(&self) -> savvy::Result<Sexp> {
        self.0.is_standard_layout()
    }

    /// HArray
    /// ## is_unique
    ///
    /// `is_unique() -> bool`
    ///
    /// Checks if the object is shared.
    ///
    /// Since `HArray` has a COW ([clone-on-write](https://doc.rust-lang.org/std/borrow/enum.Cow.html)) behaviour, this function is useful to check if a new
    /// object will be created or if the change will be done in-place.
    ///
    /// #### Returns
    ///
    /// A bool.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray1 = HArray$new_from_values(arr, dtype)
    /// harray1$is_unique() # TRUE.
    ///
    /// harray2 = harray1$clone()
    /// harray1$is_unique() # FALSE, HArray object shared with harray2.
    /// ```
    ///
    /// _________
    ///
    fn is_unique(&mut self) -> savvy::Result<Sexp> {
        // Requires &mut to avoid race condition.
        let inner_mut = self.get_inner_mut();
        inner_mut.is_unique()
    }

    /// HArray
    /// ## invalidate
    ///
    /// `invalidate()`
    ///
    /// Replaces the inner value of the external pointer, invalidating it.
    /// This function is useful to remove one of the shared references of the inner pointer in rust.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// harray$invalidate()
    /// ```
    ///
    /// _________
    ///
    pub fn invalidate(self) -> savvy::Result<()> {
        Ok(())
    }
}

impl HArray {
    #[doc(hidden)]
    pub fn get_inner_mut(&mut self) -> &mut dyn HArrayR {
        // Weak references are never used.
        if Arc::strong_count(&self.0) != 1 {
            self.0 = self.0.clone_inner();
        }
        // Safety: reference count was checked.
        // Use get_mut_unchecked when stable.
        unsafe { Arc::get_mut(&mut self.0).unwrap_unchecked() }
    }
}
