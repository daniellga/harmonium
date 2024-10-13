use crate::{
    conversions::{try_from_i32_to_usize, ToScalar},
    errors::HErrorR,
    harray::HArray,
    hdatatype::HDataType,
};
use harmonium_fft::fft::{Fft, RealFftForward, RealFftInverse};
use ndarray::IxDyn;
use num_complex::Complex;
use savvy::{r_println, savvy, OwnedLogicalSexp, Sexp};
use std::sync::Arc;

pub trait HFftR: Send + Sync {
    fn process(&mut self, harray: &mut HArray) -> savvy::Result<()>;
    fn dtype(&self) -> savvy::Result<HDataType>;
    fn clone_inner(&self) -> Arc<dyn HFftR>;
}

/// HFft
/// An `HFft` is used to create FFTs. It caches results internally, so when making more than one FFT it is advisable to reuse the same `HFft` instance.
///
/// # Methods
///
#[savvy]
#[derive(Clone)]
pub struct HFft(pub Arc<dyn HFftR>);

#[savvy]
impl HFft {
    /// HFft
    /// ## new_forward
    ///
    /// `new_forward(length: integer, dtype: HDataType) -> HFft`
    ///
    /// Creates a new `HFft` instance which will be used to calculate forward FFTs.
    ///
    /// If you plan on creating multiple FFT instances, it is recommended to reuse the same planner for all of them. This is because the planner re-uses internal data
    /// across FFT instances wherever possible, saving memory and reducing setup time (FFT instances created with one planner will never re-use data and buffers with
    /// FFT instances created by a different planner).
    ///
    /// In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan FFTs with
    /// the fastest available instruction set. If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD FFTs.
    ///
    /// #### Arguments
    ///
    /// - `length`
    ///
    /// An integer denoting the length of the input. For 2D `HArray`'s, nrows must
    /// be provided.
    ///
    /// - `dtype`
    ///
    /// A complex `HDataType` to indicate the dtype that the `HFft` will be working with.
    ///
    /// #### Returns
    ///
    /// An `HFft`.
    ///
    /// Will return an error if dtype is of a float type.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Complex32
    /// hfft = HFft$new_forward(3L, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_forward(length: Sexp, dtype: &HDataType) -> savvy::Result<HFft> {
        let length: i32 = length.to_scalar()?;
        let length = try_from_i32_to_usize(length)?;
        match dtype {
            HDataType::Float32 => Err("This HFft is for Complex dtypes.".into()),
            HDataType::Float64 => Err("This HFft is for Complex dtypes.".into()),
            HDataType::Complex32 => Ok(HFft(Arc::new(Fft::<f32>::new_forward(length)))),
            HDataType::Complex64 => Ok(HFft(Arc::new(Fft::<f64>::new_forward(length)))),
        }
    }

    /// HFft
    /// ## new_inverse
    ///
    /// `new_inverse(length: integer, dtype: HDataType) -> HFft`
    ///
    /// Creates a new `HFft` instance which will be used to calculate inverse FFTs.
    ///
    /// If you plan on creating multiple FFT instances, it is recommended to reuse the same planner for all of them. This is because the planner re-uses internal data
    /// across FFT instances wherever possible, saving memory and reducing setup time (FFT instances created with one planner will never re-use data and buffers with
    /// FFT instances created by a different planner).
    ///
    /// In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan FFTs with
    /// the fastest available instruction set. If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD FFTs.
    ///
    /// #### Arguments
    ///
    /// - `length`
    ///
    /// An integer denoting the length of the input. For 2D `HArray`'s, nrows must
    /// be provided.
    ///
    /// - `dtype`
    ///
    /// A complex `HDataType` to indicate the dtype that the `HFft` will be working with.
    ///
    /// #### Returns
    ///
    /// An `HFft`.
    ///
    /// Will return an error if dtype is of a float type.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Complex32
    /// hfft = HFft$new_inverse(3L, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_inverse(length: Sexp, dtype: &HDataType) -> savvy::Result<HFft> {
        let length: i32 = length.to_scalar()?;
        let length = try_from_i32_to_usize(length)?;
        match dtype {
            HDataType::Float32 => Err("This HFft is for Complex dtypes.".into()),
            HDataType::Float64 => Err("This HFft is for Complex dtypes.".into()),
            HDataType::Complex32 => Ok(HFft(Arc::new(Fft::<f32>::new_inverse(length)))),
            HDataType::Complex64 => Ok(HFft(Arc::new(Fft::<f64>::new_inverse(length)))),
        }
    }

    /// HFft
    /// ## new_real_forward
    ///
    /// `new_real_forward(length: integer, dtype: HDataType) -> HFft`
    ///
    /// Creates a new `HFft` instance which will be used to calculate real forward FFTs.
    ///
    /// If you plan on creating multiple FFT instances, it is recommended to reuse the same planner for all of them. This is because the planner re-uses internal data
    /// across FFT instances wherever possible, saving memory and reducing setup time (FFT instances created with one planner will never re-use data and buffers with
    /// FFT instances created by a different planner).
    ///
    /// In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan FFTs with
    /// the fastest available instruction set. If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD FFTs.
    ///
    /// #### Arguments
    ///
    /// - `length`
    ///
    /// An integer denoting the length of the input. For 2D `HArray`'s, nrows must be provided.
    ///
    /// - `dtype`
    ///
    /// A float `HDataType` to indicate the dtype that the `HFft` will be working with.
    ///
    /// #### Returns
    ///
    /// An `HFft`.
    ///
    /// Will return an error if dtype is of complex type.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Float32
    /// hfft = HFft$new_real_forward(3L, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_real_forward(length: Sexp, dtype: &HDataType) -> savvy::Result<HFft> {
        let length: i32 = length.to_scalar()?;
        let length = try_from_i32_to_usize(length)?;
        match dtype {
            HDataType::Float32 => Ok(HFft(Arc::new(RealFftForward::<f32>::new(length)))),
            HDataType::Float64 => Ok(HFft(Arc::new(RealFftForward::<f64>::new(length)))),
            HDataType::Complex32 => Err("This HFft is for float dtypes.".into()),
            HDataType::Complex64 => Err("This HFft is for float dtypes.".into()),
        }
    }

    /// HFft
    /// ## new_real_inverse
    ///
    /// `new_real_inverse(length: integer, dtype: HDataType) -> HFft`
    ///
    /// Creates a new `HFft` instance which will be used to calculate real inverse FFTs.
    ///
    /// If you plan on creating multiple FFT instances, it is recommended to reuse the same planner for all of them. This is because the planner re-uses internal data
    /// across FFT instances wherever possible, saving memory and reducing setup time (FFT instances created with one planner will never re-use data and buffers with
    /// FFT instances created by a different planner).
    ///
    /// In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan FFTs with
    /// the fastest available instruction set. If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD FFTs.
    ///
    /// #### Arguments
    ///
    /// - `length`
    ///
    /// An integer denoting the length of the output. For 2D `HArray`'s, nrows of the output must be provided.
    ///
    /// - `dtype`
    ///
    /// A complex `HDataType` to indicate the dtype that the `HFft` will be working with.
    ///
    /// #### Returns
    ///
    /// An `HFft`.
    ///
    /// Will return an error if dtype is of float type.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Float32
    /// hfft = HFft$new_real_inverse(3L, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_real_inverse(length: Sexp, dtype: &HDataType) -> savvy::Result<HFft> {
        let length: i32 = length.to_scalar()?;
        let length = try_from_i32_to_usize(length)?;
        match dtype {
            HDataType::Float32 => Err("This HFft is for complex dtypes.".into()),
            HDataType::Float64 => Err("This HFft is for complex dtypes.".into()),
            HDataType::Complex32 => Ok(HFft(Arc::new(RealFftInverse::<f32>::new(length)))),
            HDataType::Complex64 => Ok(HFft(Arc::new(RealFftInverse::<f64>::new(length)))),
        }
    }

    /// HFft
    /// ## process
    ///
    /// `process(harray: HArray)`
    ///
    /// Computes the fast fourier transform of an `HArray`.
    /// The FFT computed may be forward or inverse, depending on the `HFft` created.
    /// For a real forward FFT, transforms a real signal of length `N` to a complex-valued spectrum of length `N/2+1` (with `N/2` rounded down).
    /// For a real inverse FFT, transforms a complex spectrum of length `N/2+1` (with `N/2` rounded down) to a real-valued
    /// signal of length `N`.
    ///
    /// The operation is done in-place for FFT.
    /// The operation is done in-place for real FFT, which means the same external pointer will be used to store it. A new `HArray` is created in this case.
    ///
    /// FFT (Fast Fourier Transform) refers to a way the discrete Fourier Transform (DFT) can be calculated efficiently,
    /// by using symmetries in the calculated terms. The symmetry is highest when n is a power of 2, and the transform
    /// is therefore most efficient for these sizes.
    ///
    /// The function does not normalize outputs. Callers must manually normalize the results by scaling each element by
    /// `1/sqrt(n)`. Multiple normalization steps can be merged into one via pairwise multiplication, so when doing
    /// a forward FFT followed by an inverse callers can normalize once by scaling each element by `1/n`.
    ///
    /// Elements in the output are ordered by ascending frequency, with the first element corresponding to frequency 0.
    ///
    /// #### Arguments
    ///
    /// - `harray`
    ///
    /// A complex `HArray`.
    ///
    /// #### Returns
    ///
    /// Will return an error if:
    ///
    /// - The `HArray`'s dtype is incompatible with the `HFft`'s dtype.
    ///
    /// - The `HArray`'s `ndim` is greater than 2.
    ///
    /// #### Examples
    ///
    /// ```r
    /// # Forward FFT.
    /// library(harmonium)
    /// arr = array(c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i), c(3,2))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// hfft = HFft$new_forward(3L, harray$dtype())
    /// hfft$process(harray)
    ///
    /// # Inverse FFT.
    /// arr = array(c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i), c(3,2))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// hfft = HFft$new_inverse(3L, harray$dtype())
    /// hfft$process(harray)
    /// ```
    ///
    /// _________
    ///
    fn process(&mut self, harray: &mut HArray) -> savvy::Result<()> {
        let inner_mut = self.get_inner_mut();
        inner_mut.process(harray)
    }

    /// HFft
    /// ## dtype
    ///
    /// `dtype() -> HDataType`
    ///
    /// Gets the `HFft`'s dtype.
    ///
    /// #### Returns
    ///
    /// An `HDataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Complex32
    /// hfft = HFft$new_forward(3L, dtype)
    /// hfft$dtype()
    /// ```
    ///
    /// _________
    ///
    fn dtype(&self) -> savvy::Result<HDataType> {
        self.0.dtype()
    }

    /// HFft
    /// ## print
    ///
    /// `print()`
    ///
    /// Prints the `HFft`.
    ///
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Complex32
    /// hfft = HFft$new_forward(3L, dtype)
    /// hfft$print()
    ///
    /// # or similarly:
    /// print(hfft)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        r_println!("HFft");
        Ok(())
    }

    /// HFft
    /// ## clone
    ///
    /// `clone() -> HFft`
    ///
    /// Clones the `HFft`.
    ///
    /// Creates a new `HFft`, with the underlying data pointing to the same place in memory.
    /// When `HFFT` is cloned, thus having more than one reference to the same internal struct, and `process` is run,
    /// it uses the same cached `Fft` instance, but a new scratch buffer will have to be allocated.
    ///
    /// #### Returns
    ///
    /// An `HFft`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Complex32
    /// hfft = HFft$new_forward(3L, dtype)
    /// hfft$clone()
    /// ```
    ///
    /// _________
    ///
    fn clone(&self) -> savvy::Result<HFft> {
        Ok(std::clone::Clone::clone(self))
    }

    /// HFft
    /// ## is_unique
    ///
    /// `is_unique() -> bool`
    ///
    /// Checks if the object is unique.
    ///
    /// Since `HFft` has a COW ([clone-on-write](https://doc.rust-lang.org/std/borrow/enum.Cow.html)) behaviour, this function is useful to check if a new
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
    /// dtype = HDataType$Complex32
    /// hfft = HFft$new_forward(3L, dtype)
    /// hfft$is_unique() # TRUE.
    ///
    /// hfft2 = hfft$clone()
    /// hfft$is_unique() # FALSE, hfft shares the same inner object with hfft2.
    /// ```
    ///
    /// _________
    ///
    fn is_unique(&mut self) -> savvy::Result<Sexp> {
        // Requires &mut to avoid race condition.
        let bool = Arc::strong_count(&self.0) == 1;
        let logical_sexp: OwnedLogicalSexp = bool.try_into()?;
        logical_sexp.into()
    }

    /// HFft
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
    /// dtype = HDataType$Complex32
    /// hfft = HFft$new_forward(3L, dtype)
    /// hfft$invalidate()
    /// ```
    ///
    /// _________
    ///
    pub fn invalidate(self) -> savvy::Result<()> {
        Ok(())
    }
}

macro_rules! impl_hfft {
    ($(($t1:ty, $e1:expr)),+) => {
        $(
            impl HFftR for Fft<$t1> {
                fn process(&mut self, harray: &mut HArray) -> savvy::Result<()> {
                    let harray_inner = harray.get_inner_mut().as_any_mut().downcast_mut::<harmonium_core::array::HArray<Complex<$t1>, IxDyn>>().ok_or_else(|| savvy::Error::new("HArray and HFft must have the same HDataType."))?;
                    harmonium_fft::fft::ProcessFft::process(self, harray_inner).map_err(|err| savvy::Error::from(HErrorR::from(err)))
                }

                fn dtype(&self) -> savvy::Result<HDataType> {
                    Ok($e1)
                }

                fn clone_inner(&self) -> Arc<dyn HFftR> {
                    Arc::new(self.clone())
                }
            }
        )+
    };
}

impl_hfft!((f32, HDataType::Complex32), (f64, HDataType::Complex64));

macro_rules! impl_hrealfftforward {
    ($(($t1:ty, $t2:ty, $e1:expr)),+) => {
        $(
            impl HFftR for $t1 {
                fn process(&mut self, harray: &mut HArray) -> savvy::Result<()> {
                    let harray_inner = harray.get_inner_mut().as_any_mut().downcast_mut::<$t2>().ok_or_else(|| savvy::Error::new("HArray and HFft must have the same HDataType."))?;
                    let result = harmonium_fft::fft::ProcessFft::process(self, harray_inner).map_err(|err| savvy::Error::from(HErrorR::from(err)))?;
                    *harray = HArray(Arc::new(result));
                    Ok(())
                }

                fn dtype(&self) -> savvy::Result<HDataType> {
                    Ok($e1)
                }

                fn clone_inner(&self) -> Arc<dyn HFftR> {
                    Arc::new(self.clone())
                }
            }
        )+
    };
}

impl_hrealfftforward!(
    (
        RealFftForward<f32>,
        harmonium_core::array::HArray<f32, IxDyn>,
        HDataType::Float32
    ),
    (
        RealFftForward<f64>,
        harmonium_core::array::HArray<f64, IxDyn>,
        HDataType::Float64
    ),
    (
        RealFftInverse<f32>,
        harmonium_core::array::HArray<Complex<f32>, IxDyn>,
        HDataType::Complex32
    ),
    (
        RealFftInverse<f64>,
        harmonium_core::array::HArray<Complex<f64>, IxDyn>,
        HDataType::Complex64
    )
);

impl HFft {
    #[doc(hidden)]
    pub fn get_inner_mut(&mut self) -> &mut dyn HFftR {
        // Weak references are never used.
        if Arc::strong_count(&self.0) != 1 {
            self.0 = self.0.clone_inner();
        }
        // Safety: reference count was checked.
        // Use get_mut_unchecked when stable.
        unsafe { Arc::get_mut(&mut self.0).unwrap_unchecked() }
    }
}
