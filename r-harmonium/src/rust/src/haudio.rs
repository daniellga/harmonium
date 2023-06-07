use crate::{
    hdatatype::HDataType,
    hmatrix::{HMatrix, HMatrixR},
};
use arrow2::array::PrimitiveArray;
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

    ///// Resample the audio data from sr_in to sr_out.
    ///// fftfixedin: A synchronous resampler that needs a fixed number of audio frames for input and returns a variable number of frames. The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter before it’s inverse transformed to get the resampled waveforms. \cr
    ///// Synchronous resampling: is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data. This type of resampler is considerably faster but doesn’t support changing the resampling ratio.
    ///// # Arguments
    ///// `sr_out` - Target sampling rate.
    ///// `chunk_size_in` - Size of input data in frames.
    ///// `sub_chunks` - Desired number of subchunks for processing, actual number may be different.
    ///// # Examples
    /////
    ///// ```
    ///// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
    ///// //let offset = None;
    ///// //let duration = None;
    ///// //let mut decoded_audio =
    ///// //    DecodedAudio::<f64, OwnedRepr<f64>>::load(fname, offset, duration, VerifyDecode::Verify).unwrap();
    ///// //decoded_audio.resample_fftfixedin(22000, 1024, 2).unwrap();
    ///// ```
    //fn resample_fftfixedin(&mut self, sr_out: i32, chunk_size_in: i32, sub_chunks: i32) {
    //    let inner_mut = self.get_inner_mut();
    //    inner_mut.resample_fftfixedin(sr_out, chunk_size_in, sub_chunks);
    //}

    ///// Resample the audio data from sr_in to sr_out.
    ///// fftfixedinout: A synchronous resampler that accepts a fixed number of audio frames for input and returns a fixed number of frames. The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter before it’s inverse transformed to get the resampled waveforms.
    ///// Synchronous resampling: is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data. This type of resampler is considerably faster but doesn’t support changing the resampling ratio.
    ///// # Arguments
    ///// `sr_out` - Target sampling rate.
    ///// `chunk_size_in` - Size of input data in frames.
    ///// # Examples
    /////
    ///// ```
    ///// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
    ///// //let offset = None;
    ///// //let duration = None;
    ///// //let mut decoded_audio =
    ///// //    DecodedAudio::<f64, OwnedRepr<f64>>::load(fname, offset, duration, VerifyDecode::Verify).unwrap();
    ///// //decoded_audio.resample_fftfixedinout(22000, 1024).unwrap();
    ///// ```
    //fn resample_fftfixedinout(&mut self, sr_out: i32, chunk_size_in: i32) {
    //    let inner_mut = self.get_inner_mut();
    //    inner_mut.resample_fftfixedinout(sr_out, chunk_size_in);
    //}

    ///// Resample the audio data from sr_in to sr_out.
    ///// fftfixedout: A synchronous resampler that needs a varying number of audio frames for input and returns a fixed number of frames. The resampling is done by FFTing the input data. The spectrum is then extended or truncated as well as multiplied with an antialiasing filter before it’s inverse transformed to get the resampled waveforms.
    ///// Synchronous resampling: is implemented via FFT. The data is FFTed, the spectrum modified, and then inverse FFTed to get the resampled data. This type of resampler is considerably faster but doesn’t support changing the resampling ratio.
    ///// # Arguments
    ///// `sr_out` - Target sampling rate.
    ///// `chunk_size_out` - Size of output data in frames.
    ///// `sub_chunks` - Desired number of subchunks for processing, actual number may be different.
    ///// # Examples
    /////
    ///// ```
    ///// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
    ///// //let offset = None;
    ///// //let duration = None;
    ///// //let mut decoded_audio =
    ///// //    DecodedAudio::<f64, OwnedRepr<f64>>::load(fname, offset, duration, VerifyDecode::Verify).unwrap();
    ///// //decoded_audio.resample_fftfixedout(22000, 1024, 2).unwrap();
    ///// ```
    //fn resample_fftfixedout(&mut self, sr_out: i32, chunk_size_out: i32, sub_chunks: i32) {
    //    let inner_mut = self.get_inner_mut();
    //    inner_mut.resample_fftfixedout(sr_out, chunk_size_out, sub_chunks);
    //}

    ///// Resample the audio data from sr_in to sr_out.
    ///// sincfixedin: An asynchronous resampler that accepts a fixed number of audio frames for input and returns a variable number of frames. The resampling is done by creating a number of intermediate points (defined by oversampling_factor) by sinc interpolation. The new samples are then calculated by interpolating between these points.
    ///// Asynchronous resampling: the resampling is based on band-limited interpolation using sinc interpolation filters. The sinc interpolation upsamples by an adjustable factor, and then the new sample points are calculated by interpolating between these points. The resampling ratio can be updated at any time.
    ///// # Arguments
    ///// `sr_out` - Target sampling rate.
    ///// `max_resample_ratio_relative` - Maximum ratio that can be set with Resampler::set_resample_ratio relative to resample_ratio, must be >= 1.0. The minimum relative ratio is the reciprocal of the maximum. For example, with max_resample_ratio_relative of 10.0, the ratio can be set between resample_ratio * 10.0 and resample_ratio / 10.0.
    ///// `sinc_len` - Length of the windowed sinc interpolation filter. Higher values can allow a higher cut-off frequency leading to less high frequency roll-off at the expense of higher cpu usage. 256 is a good starting point. The value will be rounded up to the nearest multiple of 8.
    ///// `f_cutoff` - Relative cutoff frequency of the sinc interpolation filter (relative to the lowest one of fs_in/2 or fs_out/2). Start at 0.95, and increase if needed.
    ///// `oversampling_factor` - The number of intermediate points to use for interpolation. Higher values use more memory for storing the sinc filters. Only the points actually needed are calculated during processing so a larger number does not directly lead to higher cpu usage. But keeping it down helps in keeping the sincs in the cpu cache. Starts at 128.
    ///// `interpolation` - Interpolation type. One of \["cubic", "linear", "nearest"\]. \cr
    ///// For asynchronous interpolation where the ratio between input and output sample rates can be any number, it’s not possible to pre-calculate all the needed interpolation filters. Instead they have to be computed as needed, which becomes impractical since the sincs are very expensive to generate in terms of cpu time. It’s more efficient to combine the sinc filters with some other interpolation technique. Then sinc filters are used to provide a fixed number of interpolated points between input samples, and then the new value is calculated by interpolation between those points. \cr
    ///// Variants:
    ///// \itemize{
    ///// \item "cubic": For cubic interpolation, the four nearest intermediate points are calculated using sinc interpolation. Then a cubic polynomial is fitted to these points, and is then used to calculate the new sample value. The computation time as about twice the one for linear interpolation, but it requires much fewer intermediate points for a good result.
    ///// \item "linear": With linear interpolation the new sample value is calculated by linear interpolation between the two nearest points. This requires two intermediate points to be calculated using sinc interpolation, and te output is a weighted average of these two. This is relatively fast, but needs a large number of intermediate points to push the resampling artefacts below the noise floor.
    ///// \item "nearest": The Nearest mode doesn’t do any interpolation, but simply picks the nearest intermediate point. This is useful when the nearest point is actually the correct one, for example when upsampling by a factor 2, like 48kHz->96kHz. Then setting the oversampling_factor to 2, and using Nearest mode, no unnecessary computations are performed and the result is the same as for synchronous resampling. This also works for other ratios that can be expressed by a fraction. For 44.1kHz -> 48 kHz, setting oversampling_factor to 160 gives the desired result (since 48kHz = 160/147 * 44.1kHz).
    ///// }
    ///// `window` - Window function to use. \cr
    ///// Variants:
    ///// \itemize{
    ///// \item "blackman": Intermediate rolloff and intermediate attenuation.
    ///// \item "blackman2": Slower rolloff but better attenuation than Blackman.
    ///// \item "blackmanharris": Slow rolloff but good attenuation.
    ///// \item "blackmanharris2": Slower rolloff but better attenuation than Blackman-Harris.
    ///// \item "hann": Fast rolloff but not very high attenuation.
    ///// \item "hann2": Slower rolloff and higher attenuation than simple Hann.
    ///// }
    ///// `chunk_size_in` - Size of input data in frames.
    ///// # Examples
    /////
    ///// ```
    ///// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
    ///// //let offset = None;
    ///// //let duration = None;
    ///// //decoded_audio
    ///// //    .resample_sincfixedin(
    ///// //        22000,
    ///// //        2.,
    ///// //        256,
    ///// //        0.95,
    ///// //        128,
    ///// //        InterpolationType::Linear,
    ///// //        WindowFunction::Blackman2,
    ///// //        1024,
    ///// //    )
    ///// //    .unwrap();
    ///// ```
    //fn resample_sincfixedin(
    //    &mut self,
    //    sr_out: i32,
    //    max_resample_ratio_relative: f64,
    //    sinc_len: i32,
    //    f_cutoff: f64,
    //    oversampling_factor: i32,
    //    interpolation: &str,
    //    window: &str,
    //    chunk_size_in: i32,
    //) {
    //    let inner_mut = self.get_inner_mut();
    //    inner_mut.resample_sincfixedin(
    //        sr_out,
    //        max_resample_ratio_relative,
    //        sinc_len,
    //        f_cutoff,
    //        oversampling_factor,
    //        interpolation,
    //        window,
    //        chunk_size_in,
    //    )
    //}

    ///// Resample the audio data from sr_in to sr_out.
    ///// sincfixedout: An asynchronous resampler that return a fixed number of audio frames. The number of input frames required is given by the input_frames_next function. The resampling is done by creating a number of intermediate points (defined by oversampling_factor) by sinc interpolation. The new samples are then calculated by interpolating between these points.
    ///// Asynchronous resampling: the resampling is based on band-limited interpolation using sinc interpolation filters. The sinc interpolation upsamples by an adjustable factor, and then the new sample points are calculated by interpolating between these points. The resampling ratio can be updated at any time.
    ///// # Arguments
    ///// `sr_out` - Target sampling rate.
    ///// `max_resample_ratio_relative` - Maximum ratio that can be set with Resampler::set_resample_ratio relative to resample_ratio, must be >= 1.0. The minimum relative ratio is the reciprocal of the maximum. For example, with max_resample_ratio_relative of 10.0, the ratio can be set between resample_ratio * 10.0 and resample_ratio / 10.0.
    ///// `sinc_len` - Length of the windowed sinc interpolation filter. Higher values can allow a higher cut-off frequency leading to less high frequency roll-off at the expense of higher cpu usage. 256 is a good starting point. The value will be rounded up to the nearest multiple of 8.
    ///// `f_cutoff` - Relative cutoff frequency of the sinc interpolation filter (relative to the lowest one of fs_in/2 or fs_out/2). Start at 0.95, and increase if needed.
    ///// `oversampling_factor` - The number of intermediate points to use for interpolation. Higher values use more memory for storing the sinc filters. Only the points actually needed are calculated during processing so a larger number does not directly lead to higher cpu usage. But keeping it down helps in keeping the sincs in the cpu cache. Starts at 128.
    ///// `interpolation` - Interpolation type. One of \["cubic", "linear", "nearest"\]. \cr
    ///// For asynchronous interpolation where the ratio between input and output sample rates can be any number, it’s not possible to pre-calculate all the needed interpolation filters. Instead they have to be computed as needed, which becomes impractical since the sincs are very expensive to generate in terms of cpu time. It’s more efficient to combine the sinc filters with some other interpolation technique. Then sinc filters are used to provide a fixed number of interpolated points between input samples, and then the new value is calculated by interpolation between those points. \cr
    ///// Variants:
    ///// \itemize{
    ///// \item "cubic": For cubic interpolation, the four nearest intermediate points are calculated using sinc interpolation. Then a cubic polynomial is fitted to these points, and is then used to calculate the new sample value. The computation time as about twice the one for linear interpolation, but it requires much fewer intermediate points for a good result.
    ///// \item "linear": With linear interpolation the new sample value is calculated by linear interpolation between the two nearest points. This requires two intermediate points to be calculated using sinc interpolation, and te output is a weighted average of these two. This is relatively fast, but needs a large number of intermediate points to push the resampling artefacts below the noise floor.
    ///// \item "nearest": The Nearest mode doesn’t do any interpolation, but simply picks the nearest intermediate point. This is useful when the nearest point is actually the correct one, for example when upsampling by a factor 2, like 48kHz->96kHz. Then setting the oversampling_factor to 2, and using Nearest mode, no unnecessary computations are performed and the result is the same as for synchronous resampling. This also works for other ratios that can be expressed by a fraction. For 44.1kHz -> 48 kHz, setting oversampling_factor to 160 gives the desired result (since 48kHz = 160/147 * 44.1kHz).
    ///// }
    ///// `window` - Window function to use. \cr
    ///// Variants:
    ///// \itemize{
    ///// \item "blackman": Intermediate rolloff and intermediate attenuation.
    ///// \item "blackman2": Slower rolloff but better attenuation than Blackman.
    ///// \item "blackmanharris": Slow rolloff but good attenuation.
    ///// \item "blackmanharris2": Slower rolloff but better attenuation than Blackman-Harris.
    ///// \item "hann": Fast rolloff but not very high attenuation.
    ///// \item "hann2": Slower rolloff and higher attenuation than simple Hann.
    ///// }
    ///// `chunk_size_out` - Size of output data in frames.
    ///// # Examples
    /////
    ///// ```
    ///// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
    ///// //let offset = None;
    ///// //let duration = None;
    ///// //decoded_audio
    ///// //    .resample_sincfixedout(
    ///// //        22000,
    ///// //        2.,
    ///// //        256,
    ///// //        0.95,
    ///// //        128,
    ///// //        InterpolationType::Linear,
    ///// //        WindowFunction::Blackman2,
    ///// //        1024,
    ///// //    )
    ///// //    .unwrap();
    ///// ```
    //fn resample_sincfixedout(
    //    &mut self,
    //    sr_out: i32,
    //    max_resample_ratio_relative: f64,
    //    sinc_len: i32,
    //    f_cutoff: f64,
    //    oversampling_factor: i32,
    //    interpolation: &str,
    //    window: &str,
    //    chunk_size_out: i32,
    //) {
    //    let inner_mut = self.get_inner_mut();
    //    inner_mut.resample_sincfixedout(
    //        sr_out,
    //        max_resample_ratio_relative,
    //        sinc_len,
    //        f_cutoff,
    //        oversampling_factor,
    //        interpolation,
    //        window,
    //        chunk_size_out,
    //    )
    //}
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
        let list_array = self.inner().inner();
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

    //fn resample_fftfixedin(&mut self, sr_out: i32, chunk_size_in: i32, sub_chunks: i32) {
    //    Resampler::resample_fftfixedin(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        chunk_size_in.try_into().unwrap(),
    //        sub_chunks.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}

    //fn resample_fftfixedinout(&mut self, sr_out: i32, chunk_size_in: i32) {
    //    Resampler::resample_fftfixedinout(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        chunk_size_in.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}

    //fn resample_fftfixedout(&mut self, sr_out: i32, chunk_size_out: i32, sub_chunks: i32) {
    //    Resampler::resample_fftfixedout(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        chunk_size_out.try_into().unwrap(),
    //        sub_chunks.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}

    //fn resample_sincfixedin(
    //    &mut self,
    //    sr_out: i32,
    //    max_resample_ratio_relative: f64,
    //    sinc_len: i32,
    //    f_cutoff: f64,
    //    oversampling_factor: i32,
    //    interpolation: &str,
    //    window: &str,
    //    chunk_size_in: i32,
    //) {
    //    let interpolation = match interpolation {
    //        "cubic" => rubato::InterpolationType::Cubic,
    //        "linear" => rubato::InterpolationType::Linear,
    //        "nearest" => rubato::InterpolationType::Nearest,
    //        _ => panic!("not a valid interpolation type"),
    //    };

    //    let window = match window {
    //        "blackman" => rubato::WindowFunction::Blackman,
    //        "blackman2" => rubato::WindowFunction::Blackman2,
    //        "blackmanharris" => rubato::WindowFunction::BlackmanHarris,
    //        "blackmanharris2" => rubato::WindowFunction::BlackmanHarris2,
    //        "hann" => rubato::WindowFunction::Hann,
    //        "hann2" => rubato::WindowFunction::Hann2,
    //        _ => panic!("not a valid window type"),
    //    };

    //    let interpolation_params = InterpolationParameters {
    //        sinc_len: sinc_len.try_into().unwrap(),
    //        f_cutoff: f_cutoff as f32,
    //        oversampling_factor: oversampling_factor.try_into().unwrap(),
    //        interpolation,
    //        window,
    //    };

    //    Resampler::resample_sincfixedin(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        max_resample_ratio_relative,
    //        interpolation_params,
    //        chunk_size_in.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}

    //fn resample_sincfixedout(
    //    &mut self,
    //    sr_out: i32,
    //    max_resample_ratio_relative: f64,
    //    sinc_len: i32,
    //    f_cutoff: f64,
    //    oversampling_factor: i32,
    //    interpolation: &str,
    //    window: &str,
    //    chunk_size_out: i32,
    //) {
    //    let interpolation = match interpolation {
    //        "cubic" => rubato::InterpolationType::Cubic,
    //        "linear" => rubato::InterpolationType::Linear,
    //        "nearest" => rubato::InterpolationType::Nearest,
    //        _ => panic!("not a valid interpolation type"),
    //    };

    //    let window = match window {
    //        "blackman" => rubato::WindowFunction::Blackman,
    //        "blackman2" => rubato::WindowFunction::Blackman2,
    //        "blackmanharris" => rubato::WindowFunction::BlackmanHarris,
    //        "blackmanharris2" => rubato::WindowFunction::BlackmanHarris2,
    //        "hann" => rubato::WindowFunction::Hann,
    //        "hann2" => rubato::WindowFunction::Hann2,
    //        _ => panic!("not a valid window type"),
    //    };

    //    let interpolation_params = InterpolationParameters {
    //        sinc_len: sinc_len.try_into().unwrap(),
    //        f_cutoff: f_cutoff as f32,
    //        oversampling_factor: oversampling_factor.try_into().unwrap(),
    //        interpolation,
    //        window,
    //    };

    //    Resampler::resample_sincfixedout(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        max_resample_ratio_relative,
    //        interpolation_params,
    //        chunk_size_out.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}
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
        let list_array = self.inner().inner();
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

    //fn resample_fftfixedin(&mut self, sr_out: i32, chunk_size_in: i32, sub_chunks: i32) {
    //    Resampler::resample_fftfixedin(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        chunk_size_in.try_into().unwrap(),
    //        sub_chunks.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}

    //fn resample_fftfixedinout(&mut self, sr_out: i32, chunk_size_in: i32) {
    //    Resampler::resample_fftfixedinout(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        chunk_size_in.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}

    //fn resample_fftfixedout(&mut self, sr_out: i32, chunk_size_out: i32, sub_chunks: i32) {
    //    Resampler::resample_fftfixedout(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        chunk_size_out.try_into().unwrap(),
    //        sub_chunks.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}

    //fn resample_sincfixedin(
    //    &mut self,
    //    sr_out: i32,
    //    max_resample_ratio_relative: f64,
    //    sinc_len: i32,
    //    f_cutoff: f64,
    //    oversampling_factor: i32,
    //    interpolation: &str,
    //    window: &str,
    //    chunk_size_in: i32,
    //) {
    //    let interpolation = match interpolation {
    //        "cubic" => rubato::InterpolationType::Cubic,
    //        "linear" => rubato::InterpolationType::Linear,
    //        "nearest" => rubato::InterpolationType::Nearest,
    //        _ => panic!("not a valid interpolation type"),
    //    };

    //    let window = match window {
    //        "blackman" => rubato::WindowFunction::Blackman,
    //        "blackman2" => rubato::WindowFunction::Blackman2,
    //        "blackmanharris" => rubato::WindowFunction::BlackmanHarris,
    //        "blackmanharris2" => rubato::WindowFunction::BlackmanHarris2,
    //        "hann" => rubato::WindowFunction::Hann,
    //        "hann2" => rubato::WindowFunction::Hann2,
    //        _ => panic!("not a valid window type"),
    //    };

    //    let interpolation_params = InterpolationParameters {
    //        sinc_len: sinc_len.try_into().unwrap(),
    //        f_cutoff: f_cutoff as f32,
    //        oversampling_factor: oversampling_factor.try_into().unwrap(),
    //        interpolation,
    //        window,
    //    };

    //    Resampler::resample_sincfixedin(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        max_resample_ratio_relative,
    //        interpolation_params,
    //        chunk_size_in.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}

    //fn resample_sincfixedout(
    //    &mut self,
    //    sr_out: i32,
    //    max_resample_ratio_relative: f64,
    //    sinc_len: i32,
    //    f_cutoff: f64,
    //    oversampling_factor: i32,
    //    interpolation: &str,
    //    window: &str,
    //    chunk_size_out: i32,
    //) {
    //    let interpolation = match interpolation {
    //        "cubic" => rubato::InterpolationType::Cubic,
    //        "linear" => rubato::InterpolationType::Linear,
    //        "nearest" => rubato::InterpolationType::Nearest,
    //        _ => panic!("not a valid interpolation type"),
    //    };

    //    let window = match window {
    //        "blackman" => rubato::WindowFunction::Blackman,
    //        "blackman2" => rubato::WindowFunction::Blackman2,
    //        "blackmanharris" => rubato::WindowFunction::BlackmanHarris,
    //        "blackmanharris2" => rubato::WindowFunction::BlackmanHarris2,
    //        "hann" => rubato::WindowFunction::Hann,
    //        "hann2" => rubato::WindowFunction::Hann2,
    //        _ => panic!("not a valid window type"),
    //    };

    //    let interpolation_params = InterpolationParameters {
    //        sinc_len: sinc_len.try_into().unwrap(),
    //        f_cutoff: f_cutoff as f32,
    //        oversampling_factor: oversampling_factor.try_into().unwrap(),
    //        interpolation,
    //        window,
    //    };

    //    Resampler::resample_sincfixedout(
    //        self,
    //        sr_out.try_into().unwrap(),
    //        max_resample_ratio_relative,
    //        interpolation_params,
    //        chunk_size_out.try_into().unwrap(),
    //    )
    //    .unwrap();
    //}
}

extendr_module! {
    mod haudio;
    impl HAudio;
}
