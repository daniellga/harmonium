use crate::{
    hdatatype::HDataType,
    hmatrix::{HMatrix, HMatrixR},
};
use extendr_api::prelude::*;
use harmonium_core::structs;
use harmonium_io::decode::decode_arrow::decode;
use std::{any::Any, sync::Arc};

pub trait HAudioR: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn len(&self) -> i32;
    fn nchannels(&self) -> i32;
    fn nframes(&self) -> i32;
    fn print(&self);
    fn as_hmatrix(&self) -> Arc<dyn HMatrixR>;
    fn collect(&self) -> Robj;
    fn sr(&self) -> i32;
    fn mem_adress(&self) -> String;
    fn dtype(&self) -> HDataType;
    fn as_mono(&mut self);
    fn clone_inner(&self) -> Arc<dyn HAudioR>;
}

/// HAudio
/// A structure to represent audio data. Composed by: \
///
/// * `HMatrix` \
/// The decoded audio data as a floating point time series. Each column represents a channel. \
/// Must have a float dtype. \
///
/// * `integer`. \
/// The sampling rate in hz. \
///
/// # Methods
///
#[derive(Clone)]
pub struct HAudio(pub Arc<dyn HAudioR>);

#[extendr(use_try_from = true)]
impl HAudio {
    /// HAudio
    /// ## new_from_file
    ///
    /// `new_from_file(fpath: string, offset: double, duration: double, dtype: HDataType) -> HAudio` \
    ///
    /// Creates a new `HAudio` from an audio file. \
    ///
    /// #### Arguments
    ///
    /// * `fpath` \
    /// A `string` for the input file path. \
    /// * `offset` (default = `NA`) \
    /// A `double`. Start reading the file after `offset`, in seconds. \
    /// If `NA`, will load from the beginning of the file. \
    /// * `duration` (default = `NA`) \
    /// A `double`. Duration to be loaded, in seconds, counting from `offset`. Will load the file till the end if `offset + duration >= file length`. \
    /// If `NA`, will load until the end of the file. \
    /// * `dtype` \
    /// A float `HDataType` to indicate which type of `HAudio` to be created. \
    ///
    /// #### Returns
    ///
    /// An `HAudio`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav", dtype = dtype)
    /// haudio2 = HAudio$new_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav", offset = 1, duration = 2, dtype = dtype) # Reads the file from 1 second to 3 seconds.
    /// ```
    ///
    /// _________
    ///
    pub fn new_from_file(
        fpath: &str,
        #[default = "NA_real_"] offset: Option<f64>,
        #[default = "NA_real_"] duration: Option<f64>,
        dtype: &HDataType,
    ) -> HAudio {
        let inner: Arc<dyn HAudioR> = match dtype {
            HDataType::Float32 => Arc::new(decode::<f32>(fpath, offset, duration).unwrap()),
            HDataType::Float64 => Arc::new(decode::<f64>(fpath, offset, duration).unwrap()),
            _ => panic!("not a valid dtype"),
        };
        HAudio(inner)
    }

    /// HAudio
    /// ## new_from_values
    ///
    /// `new_from_values(values: matrix, sr: integer, dtype: HDataType) -> HAudio` \
    ///
    /// Creates a new `HAudio` from a matrix. \
    ///
    /// #### Arguments
    ///
    /// * `values` \
    /// A `double` `matrix`. \
    /// * `sr` \
    /// An `integer`. The sampling rate in hz. \
    /// * `dtype` \
    /// A float `HDataType` to indicate which type of `HAudio` to be created. \
    ///
    /// #### Returns
    ///
    /// An `HAudio`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// ```
    ///
    /// _________
    ///
    pub fn new_from_values(values: Robj, sr: i32, dtype: &HDataType) -> HAudio {
        let hmatrix = HMatrix::new_from_values(values, dtype);
        hmatrix.as_haudio(sr)
    }

    /// HAudio
    /// ## len
    ///
    /// `len() -> integer` \
    ///
    /// Returns the number of elements of this `HAudio`. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio$len()
    /// ```
    ///
    /// _________
    ///
    pub fn len(&self) -> i32 {
        self.0.len()
    }

    /// HAudio
    /// ## nchannels
    ///
    /// `nchannels() -> integer` \
    ///
    /// Returns the number of channels of this `HAudio`. \
    /// This is the same as the number of columns of the inner `HMatrix`. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio$nchannels()
    /// ```
    ///
    /// _________
    ///
    pub fn nchannels(&self) -> i32 {
        self.0.nchannels()
    }

    /// HAudio
    /// ## nframes
    ///
    /// `nframes() -> integer` \
    ///
    /// Returns the number of frames of this `HAudio`. \
    /// This is the same as the number of rows of the inner `HMatrix`. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio$nframes()
    /// ```
    ///
    /// _________
    ///
    pub fn nframes(&self) -> i32 {
        self.0.nframes()
    }

    /// HAudio
    /// ## print
    ///
    /// `print()` \
    ///
    /// Print the `HAudio`. \
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio$print()
    ///
    /// # or similarly:
    /// print(haudio)
    /// ```
    ///
    /// _________
    ///
    pub fn print(&self) {
        self.0.print();
    }

    /// HAudio
    /// ## as_hmatrix
    ///
    /// `as_hmatrix() -> HMatrix` \
    ///
    /// Creates a new `HMatrix`, with the underlying data pointing to the same place in memory. \
    ///
    /// #### Returns
    ///
    /// An `HMatrix`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// hmatrix = haudio$as_hmatrix()
    ///
    /// haudio$mem_adress() == hmatrix$mem_adress() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn as_hmatrix(&self) -> HMatrix {
        HMatrix(self.0.as_hmatrix())
    }

    /// HAudio
    /// ## eq
    ///
    /// `eq(other: HAudio) -> bool` \
    ///
    /// Equality with another `HAudio`. \
    /// The comparison only checks if the dtype, the sampling rate and the values are the same. To compare if the
    /// underlying data is the same in memory, check `mem_adress`. \
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HAudio`. \
    ///
    /// #### Returns
    ///
    /// A `bool`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio1 = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio2 = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio1$eq(haudio2) # TRUE
    ///
    /// # or similarly:
    /// haudio1 == haudio2
    /// ```
    ///
    /// _________
    ///
    pub fn eq(&self, other: &HAudio) -> bool {
        self.0.eq(&other.0)
    }

    /// HAudio
    /// ## ne
    ///
    /// `ne(other: HAudio) -> bool` \
    ///
    /// Difference with another `HAudio`. \
    /// The comparison only checks if the dtype, the sampling rate and the values are the same. To compare if the
    /// underlying data is the same in memory, check `mem_adress`. \
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HAudio`. \
    ///
    /// #### Returns
    ///
    /// A `bool`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio1 = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio2 = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio1$ne(haudio2) # FALSE
    ///
    /// # or similarly:
    /// haudio1 != haudio2
    /// ```
    ///
    /// _________
    ///
    pub fn ne(&self, other: &HAudio) -> bool {
        self.0.ne(&other.0)
    }

    /// HAudio
    /// ## clone
    ///
    /// `clone() -> HAudio` \
    ///
    /// Creates a new `HAudio`, with the underlying data pointing to the same place in memory.
    ///
    /// #### Returns
    ///
    /// An `HAudio`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio1 = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio2 = haudio1$clone()
    /// haudio1 == haudio2 # TRUE
    /// haudio1$mem_adress() == haudio2$mem_adress() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn clone(&self) -> HAudio {
        std::clone::Clone::clone(self)
    }

    /// HAudio
    /// ## collect
    ///
    /// `collect() -> matrix` \
    ///
    /// Creates a `matrix` from an `HAudio`.
    ///
    /// #### Returns
    ///
    /// A `matrix` of type `double`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio$collect()
    /// ```
    ///
    /// _________
    ///
    pub fn collect(&self) -> Robj {
        self.0.collect()
    }

    /// HAudio
    /// ## sr
    ///
    /// `sr() -> integer` \
    ///
    /// Get the sampling rate.
    ///
    /// #### Returns
    ///
    /// An `integer`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio$sr()
    /// ```
    ///
    /// _________
    ///
    pub fn sr(&self) -> i32 {
        self.0.sr()
    }

    /// HAudio
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
    /// haudio = hmatrix$as_haudio(sr = 3L)
    /// harray$mem_adress() == hmatrix$mem_adress() # TRUE
    /// harray$mem_adress() == haudio$mem_adress() # TRUE
    ///
    /// harray$slice(1L,1L) # changing the offset to 1
    /// harray$mem_adress() == haudio$mem_adress() # FALSE, even though they still share the same underlying data
    ///
    /// haudio2 = haudio
    /// haudio$mem_adress() == haudio2$mem_adress() # TRUE, since `=` operator only creates an alias (haudio and haudio2 are the same external pointer).
    /// ```
    ///
    /// _________
    ///
    pub fn mem_adress(&self) -> String {
        self.0.mem_adress()
    }

    /// HAudio
    /// ## dtype
    ///
    /// `dtype() -> HDataType` \
    ///
    /// Gets the `HAudio`'s dtype as an `HDataType`.
    ///
    /// #### Returns
    ///
    /// An `HDataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// haudio$dtype()
    /// ```
    ///
    /// _________
    ///
    pub fn dtype(&self) -> HDataType {
        self.0.dtype()
    }

    /// HAudio
    /// ## as_mono
    ///
    /// `as_mono()` \
    ///
    /// Convert to 1 channel by taking the average across channels. \
    /// A new inner array is created. The operation is done in-place. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
    /// dtype = HDataType$float32
    /// haudio = HAudio$new_from_values(values = values, sr = 3L, dtype = dtype)
    /// mem_adress_before = haudio$mem_adress()
    /// haudio$as_mono()
    /// haudio$mem_adress() != mem_adress_before # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn as_mono(&mut self) {
        let inner_mut = self.get_inner_mut();
        inner_mut.as_mono();
    }
}

impl HAudio {
    #[doc(hidden)]
    pub fn get_inner_mut(&mut self) -> &mut dyn HAudioR {
        if Arc::weak_count(&self.0) + Arc::strong_count(&self.0) != 1 {
            self.0 = self.0.clone_inner();
        }
        Arc::get_mut(&mut self.0).expect("implementation error")
    }
}

impl HAudioR for structs::HFloatAudio<f32> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn nchannels(&self) -> i32 {
        i32::try_from(self.nchannels()).unwrap()
    }

    fn nframes(&self) -> i32 {
        i32::try_from(self.nframes()).unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_hmatrix(&self) -> Arc<dyn HMatrixR> {
        let hmatrix = self.inner().clone();
        Arc::new(hmatrix)
    }

    fn collect(&self) -> Robj {
        let hmatrix = self.inner();
        let ncols = hmatrix.ncols();
        let nrows = hmatrix.nrows();
        hmatrix
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

    fn sr(&self) -> i32 {
        i32::try_from(self.sr()).unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.inner().as_slice();
        format!("{:p}", p)
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float32
    }

    fn as_mono(&mut self) {
        self.as_mono().unwrap();
    }

    fn clone_inner(&self) -> Arc<dyn HAudioR> {
        Arc::new(self.clone())
    }
}

impl HAudioR for structs::HFloatAudio<f64> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn len(&self) -> i32 {
        i32::try_from(self.len()).unwrap()
    }

    fn nchannels(&self) -> i32 {
        i32::try_from(self.nchannels()).unwrap()
    }

    fn nframes(&self) -> i32 {
        i32::try_from(self.nframes()).unwrap()
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    fn as_hmatrix(&self) -> Arc<dyn HMatrixR> {
        let hmatrix = self.inner().clone();
        Arc::new(hmatrix)
    }

    fn collect(&self) -> Robj {
        let hmatrix = self.inner();
        let ncols = hmatrix.ncols();
        let nrows = hmatrix.nrows();
        hmatrix
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

    fn sr(&self) -> i32 {
        i32::try_from(self.sr()).unwrap()
    }

    fn mem_adress(&self) -> String {
        let p = self.inner().as_slice();
        format!("{:p}", p)
    }

    fn dtype(&self) -> HDataType {
        HDataType::Float64
    }

    fn as_mono(&mut self) {
        self.as_mono().unwrap();
    }

    fn clone_inner(&self) -> Arc<dyn HAudioR> {
        Arc::new(self.clone())
    }
}

extendr_module! {
    mod haudio;
    impl HAudio;
}
