use crate::{conversions::AsScalar, errors::HErrorR, harray::HArray, hdatatype::HDataType};
use ndarray::IxDyn;
use num_complex::Complex;
use realfft::RealFftPlanner;
use rustfft::FftPlanner;
use savvy::{r_println, savvy, Sexp};
use std::sync::Arc;

pub trait HFftPlannerR: Send {
    fn fft(&mut self, harray: &mut HArray) -> savvy::Result<()>;
    fn ifft(&mut self, harray: &mut HArray) -> savvy::Result<()>;
    fn dtype(&self) -> savvy::Result<HDataType>;
    fn print(&self) -> savvy::Result<()>;
}

pub trait HRealFftPlannerR: Send {
    fn rfft(&mut self, harray: &mut HArray) -> savvy::Result<()>;
    fn irfft(&mut self, harray: &mut HArray, length: usize) -> savvy::Result<()>;
    fn dtype(&self) -> savvy::Result<HDataType>;
    fn print(&self) -> savvy::Result<()>;
}

/// HFftPlanner
/// A planner is used to create FFTs. It caches results internally, so when making more than one FFT it is advisable to reuse the same planner. \
///
/// # Methods
///
#[savvy]
pub struct HFftPlanner(pub Box<dyn HFftPlannerR>);

/// HRealFftPlanner
/// A planner is used to create FFTs. It caches results internally, so when making more than one FFT it is advisable to reuse the same planner. \
/// This planner is used to calculate FFTs of real valued inputs and its inverse operation. \
///
/// # Methods
///
#[savvy]
pub struct HRealFftPlanner(pub Box<dyn HRealFftPlannerR>);

#[savvy]
impl HFftPlanner {
    /// HFftPlanner
    /// ## new
    ///
    /// `new(dtype: HDataType) -> HFftPlanner` \
    ///
    /// Creates a new `HFftPlanner` instance. \
    /// If you plan on creating multiple FFT instances, it is recommended to reuse the same planner for all of them. This is because the planner re-uses internal data
    /// across FFT instances wherever possible, saving memory and reducing setup time. (FFT instances created with one planner will never re-use data and buffers with
    /// FFT instances created by a different planner) \
    /// In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan FFTs with
    /// the fastest available instruction set. If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD FFTs. \
    ///
    /// #### Arguments
    ///
    /// * `dtype` \
    /// A complex `HDataType` to indicate the dtype that the `HFftPlanner` will be working with. \
    ///
    /// #### Returns
    ///
    /// An `HFftPlanner`. \
    /// Will return an error if dtype is of a float type. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// fft_planner = HFftPlanner$new(harray$dtype())
    /// ```
    ///
    /// _________
    ///
    fn new(dtype: &HDataType) -> savvy::Result<HFftPlanner> {
        match dtype {
            HDataType::Float32 => Err("The HFftPlanner is for Complex dtypes.".into()),
            HDataType::Float64 => Err("The HFftPlanner is for Complex dtypes.".into()),
            HDataType::Complex32 => Ok(HFftPlanner(Box::new(FftPlanner::<f32>::new()))),
            HDataType::Complex64 => Ok(HFftPlanner(Box::new(FftPlanner::<f64>::new()))),
        }
    }

    /// HFftPlanner
    /// ## fft
    ///
    /// `fft(harray: HArray)` \
    ///
    /// Computes the fast fourier transform of a complex `HArray`. \
    /// The operation is done in-place. \
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
    /// #### Arguments
    ///
    /// * `harray` \
    /// A complex `HArray`. \
    ///
    /// #### Returns
    ///
    /// Will return an error if: \
    /// - The `HArray`'s dtype is incompatible with the `HFftPlanner`'s dtype. \
    /// - The `HArray`'s `ndim` is greater than 2. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// fft_planner = HFftPlanner$new(harray$dtype())
    /// fft_planner$fft(harray)
    /// ```
    ///
    /// _________
    ///
    fn fft(&mut self, harray: &mut HArray) -> savvy::Result<()> {
        self.0.fft(harray)
    }

    /// HFftPlanner
    /// ## ifft
    ///
    /// `ifft(harray: HArray)` \
    ///
    /// Computes the inverse fast fourier transform of a complex `HArray`. \
    /// The operation is done in-place. \
    /// FFT (Fast Fourier Transform) refers to a way the discrete Fourier Transform (DFT) can be calculated efficiently,
    /// by using symmetries in the calculated terms. The symmetry is highest when n is a power of 2, and the transform
    /// is therefore most efficient for these sizes. \
    ///
    /// The function does not normalize outputs. Callers must manually normalize the results by scaling each element by
    /// `1/sqrt(n)`. Multiple normalization steps can be merged into one via pairwise multiplication, so when doing
    /// a forward FFT followed by an inverse callers can normalize once by scaling each element by `1/n`. \
    ///
    /// #### Arguments
    ///
    /// * `harray` \
    /// A complex `HArray`. \
    ///
    /// #### Returns
    ///
    /// Will return an error if: \
    /// - The `HArray`'s dtype is incompatible with the `HFftPlanner`'s dtype. \
    /// - The `HArray`'s `ndim` is greater than 2. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i), c(3,2))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// fft_planner = HFftPlanner$new(harray$dtype())
    /// fft_planner$ifft(harray)
    /// ```
    ///
    /// _________
    ///
    fn ifft(&mut self, harray: &mut HArray) -> savvy::Result<()> {
        self.0.ifft(harray)
    }

    /// HFftPlanner
    /// ## dtype
    ///
    /// `dtype() -> HDataType` \
    ///
    /// Gets the `HFftPlanner`'s dtype.
    ///
    /// #### Returns
    ///
    /// An `HDataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i), c(3,2))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// fft_planner = HFftPlanner$new(harray$dtype())
    /// fft_planner$dtype()
    /// ```
    ///
    /// _________
    ///
    fn dtype(&self) -> savvy::Result<HDataType> {
        self.0.dtype()
    }

    /// HFftPlanner
    /// ## print
    ///
    /// `print()` \
    ///
    /// Print the `HFftPlanner`. \
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i), c(3,2))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// fft_planner = HFftPlanner$new(harray$dtype())
    /// fft_planner$print()
    ///
    /// # or similarly:
    /// print(fft_planner)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        self.0.print()
    }
}

#[savvy]
impl HRealFftPlanner {
    /// HRealFftPlanner
    /// ## new
    ///
    /// `new(dtype: HDataType) -> HRealFftPlanner` \
    ///
    /// Creates a new `HRealFftPlanner` instance. \
    /// If you plan on creating multiple FFT instances, it is recommended to reuse the same planner for all of them. This is because the planner re-uses internal data
    /// across FFT instances wherever possible, saving memory and reducing setup time. (FFT instances created with one planner will never re-use data and buffers with
    /// FFT instances created by a different planner) \
    /// In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan FFTs with
    /// the fastest available instruction set. If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD FFTs. \
    ///
    /// #### Arguments
    ///
    /// * `dtype` \
    /// A float `HDataType` to indicate the dtype that the `HFftPlanner` will be working with. \
    ///
    /// #### Returns
    ///
    /// An `HRealFftPlanner`. \
    /// Will return an error if dtype is of a complex type. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// real_fft_planner = HRealFftPlanner$new(harray$dtype())
    /// ```
    ///
    /// _________
    ///
    fn new(dtype: &HDataType) -> savvy::Result<HRealFftPlanner> {
        match dtype {
            HDataType::Float32 => Ok(HRealFftPlanner(Box::new(RealFftPlanner::<f32>::new()))),
            HDataType::Float64 => Ok(HRealFftPlanner(Box::new(RealFftPlanner::<f64>::new()))),
            HDataType::Complex32 => panic!(),
            HDataType::Complex64 => panic!(),
        }
    }

    /// HRealFftPlanner
    /// ## rfft
    ///
    /// `rfft(harray: HArray)` \
    ///
    /// Computes the fast fourier transform of a float `HArray`. Transforms a real signal of length `N` to a complex-valued spectrum of length `N/2+1` (with `N/2` rounded down). \
    /// The operation is not done in-place, although the same external pointer is used to store the new HArray. \
    /// The FFT of a real signal is Hermitian-symmetric, X[i] = conj(X[-i]) so the output contains only the positive frequencies
    /// below the Nyquist frequency. \
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
    /// #### Arguments
    ///
    /// * `harray` \
    /// A float `HArray`. \
    ///
    /// #### Returns
    ///
    /// Will return an error if: \
    /// - The `HArray`'s dtype is incompatible with the `HFftPlanner`'s dtype. \
    /// - The `HArray`'s `ndim` is greater than 2. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// real_fft_planner = HRealFftPlanner$new(harray$dtype())
    /// real_fft_planner$rfft(harray)
    /// ```
    ///
    /// _________
    ///
    fn rfft(&mut self, harray: &mut HArray) -> savvy::Result<()> {
        self.0.rfft(harray)
    }

    /// HRealFftPlanner
    /// ## irfft
    ///
    /// `irfft(harray: HArray, length: integer)` \
    ///
    /// Computes the inverse fast fourier transform of a complex `HArray`. Transforms a complex spectrum of length `N/2+1` (with `N/2` rounded down) to a real-valued
    /// signal of length `N`. \
    /// The operation is not done in-place, although the same external pointer is used to store the new HArray. \
    /// The FFT of a real signal is Hermitian-symmetric, X[i] = conj(X[-i]) so the output contains only the positive frequencies
    /// below the Nyquist frequency. \
    /// FFT (Fast Fourier Transform) refers to a way the discrete Fourier Transform (DFT) can be calculated efficiently,
    /// by using symmetries in the calculated terms. The symmetry is highest when n is a power of 2, and the transform
    /// is therefore most efficient for these sizes. \
    ///
    /// The function does not normalize outputs. Callers must manually normalize the results by scaling each element by
    /// `1/sqrt(n)`. Multiple normalization steps can be merged into one via pairwise multiplication, so when doing
    /// a forward FFT followed by an inverse callers can normalize once by scaling each element by `1/n`. \
    ///
    /// #### Arguments
    ///
    /// * `harray` \
    /// A complex `HArray`. The `HArray`'s dtype must be the complex equivalent of the `HRealFftPlanner`'s dtype. For example if `HRealFftPlanner` dtype is `Float64`,
    /// the `HArray`'s dtype must be `Complex64`. \
    /// * `length` \
    /// An integer. The output length of the signal. Since the spectrum is `N/2+1`, the length can be `N` and `N+1`, if `N` is even, or can be `N` and `N-1` if `N` is odd.  \
    ///
    /// #### Returns
    ///
    /// Will return an error if: \
    /// - The `HArray`'s dtype is incompatible with the `HFftPlanner`'s dtype. \
    /// - The `HArray`'s `ndim` is greater than 2. \
    /// - The `length` argument is not compatible with the spectrum length. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// r = as.double(sample(100, 4, replace = TRUE))
    /// i = as.double(sample(100, 3, replace = TRUE))
    /// arr = array(complex(real=r, imaginary=c(0,i)), c(4,1))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// real_fft_planner = HRealFftPlanner$new(HDataType$Float32)
    /// real_fft_planner$irfft(harray, 7L)
    /// ```
    ///
    /// _________
    ///
    fn irfft(&mut self, harray: &mut HArray, length: Sexp) -> savvy::Result<()> {
        let length: i32 = length.as_scalar()?;
        let length: usize = length
            .try_into()
            .map_err(|_| savvy::Error::new("Cannot convert i32 to usize."))?;
        self.0.irfft(harray, length)
    }

    /// HRealFftPlanner
    /// ## dtype
    ///
    /// `dtype() -> HDataType` \
    ///
    /// Gets the `HRealFftPlanner`'s dtype.
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
    /// real_fft_planner = HRealFftPlanner$new(harray$dtype())
    /// real_fft_planner$dtype()
    /// ```
    ///
    /// _________
    ///
    fn dtype(&self) -> savvy::Result<HDataType> {
        self.0.dtype()
    }

    /// HRealFftPlanner
    /// ## print
    ///
    /// `print()` \
    ///
    /// Print the `HRealFftPlanner`. \
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3,4))
    /// dtype = HDataType$Float32
    /// harray = HArray$new_from_values(arr, dtype)
    /// real_fft_planner = HRealFftPlanner$new(harray$dtype())
    /// real_fft_planner$print()
    ///
    /// # or similarly:
    /// print(real_fft_planner)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        self.0.print()
    }
}

macro_rules! impl_hfftplannerr {
    ($(($t1:ty, $t2:ty, $e1:expr, $e2: expr)),+) => {
        $(
            impl HFftPlannerR for $t1 {
                fn fft(&mut self, harray: &mut HArray) -> savvy::Result<()> {
                    let harray = harray.get_inner_mut().as_any_mut().downcast_mut::<$t2>().ok_or_else(|| savvy::Error::new("HArray and HFftPlanner must have the same HDataType."))?;
                    harmonium_fft::fft::ProcessFft::fft(self, harray).map_err(|err| savvy::Error::from(HErrorR::from(err)))
                }

                fn ifft(&mut self, harray: &mut HArray) -> savvy::Result<()> {
                    let harray = harray.get_inner_mut().as_any_mut().downcast_mut::<$t2>().ok_or_else(|| savvy::Error::new("HArray and HFftPlanner must have the same HDataType."))?;
                    harmonium_fft::fft::ProcessFft::ifft(self, harray).map_err(|err| savvy::Error::from(HErrorR::from(err)))
                }

                fn dtype(&self) -> savvy::Result<HDataType> {
                    Ok($e1)
                }

                fn print(&self) -> savvy::Result<()> {
                    r_println!($e2);
                    Ok(())
                }
            }
        )+
    };
}

impl_hfftplannerr!(
    (
        FftPlanner<f32>,
        harmonium_core::array::HArray<Complex<f32>, IxDyn>,
        HDataType::Complex32,
        "FftPlanner<f32>"
    ),
    (
        FftPlanner<f64>,
        harmonium_core::array::HArray<Complex<f64>, IxDyn>,
        HDataType::Complex64,
        "FftPlanner<f64>"
    )
);

macro_rules! impl_hrealfftplannerr {
    ($(($t1:ty, $t2:ty, $t3:ty, $e1:expr, $e2: expr)),+) => {
        $(
            impl HRealFftPlannerR for $t1 {
                fn rfft(&mut self, harray: &mut HArray) -> savvy::Result<()> {
                    let harray_downcasted = harray.get_inner_mut().as_any_mut().downcast_mut::<$t2>().ok_or_else(|| savvy::Error::new("HArray and HFftPlanner must have the same HDataType."))?;
                    let result = harmonium_fft::fft::ProcessRealFft::rfft(self, harray_downcasted).map_err(|err| savvy::Error::from(HErrorR::from(err)))?;
                    *harray = HArray(Arc::new(result));
                    Ok(())
                }

                fn irfft(&mut self, harray: &mut HArray, length: usize) -> savvy::Result<()> {
                    let harray_downcasted = harray.get_inner_mut().as_any_mut().downcast_mut::<$t3>().ok_or_else(|| savvy::Error::new("HArray must be the complex equivalent of HFftPlanner's HDataType."))?;
                    let result = harmonium_fft::fft::ProcessRealFft::irfft(self, harray_downcasted, length).map_err(|err| savvy::Error::from(HErrorR::from(err)))?;
                    *harray = HArray(Arc::new(result));
                    Ok(())
                }

                fn dtype(&self) -> savvy::Result<HDataType> {
                    Ok($e1)
                }

                fn print(&self) -> savvy::Result<()> {
                    r_println!($e2);
                    Ok(())
                }
            }
        )+
    };
}

impl_hrealfftplannerr!(
    (
        RealFftPlanner<f32>,
        harmonium_core::array::HArray<f32, IxDyn>,
        harmonium_core::array::HArray<Complex<f32>, IxDyn>,
        HDataType::Float32,
        "RealFftPlanner<f32>"
    ),
    (
        RealFftPlanner<f64>,
        harmonium_core::array::HArray<f64, IxDyn>,
        harmonium_core::array::HArray<Complex<f64>, IxDyn>,
        HDataType::Float64,
        "RealFftPlanner<f64>"
    )
);
