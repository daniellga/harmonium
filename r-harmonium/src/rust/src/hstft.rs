use crate::{conversions::ToScalar, errors::HErrorR, harray::HArray, hdatatype::HDataType};
use harmonium_fft::stft::{RealStftForward, Stft};
use ndarray::IxDyn;
use num_complex::Complex;
use savvy::{r_println, savvy, OwnedLogicalSexp, Sexp};
use std::{num::NonZero, sync::Arc};

pub trait HStftR: Send + Sync {
    fn process(
        &mut self,
        harray: &mut HArray,
        hop_length: NonZero<usize>,
        window_length: NonZero<usize>,
        window: Option<&HArray>,
    ) -> savvy::Result<()>;
    fn dtype(&self) -> savvy::Result<HDataType>;
    fn clone_inner(&self) -> Arc<dyn HStftR>;
}

/// HStft
/// An `HStft` is used to create STFTs. It caches results internally, so when making more than one Stft it is advisable to reuse the same `HStft` instance.
///
/// # Methods
///
#[savvy]
#[derive(Clone)]
pub struct HStft(pub Arc<dyn HStftR>);

#[savvy]
impl HStft {
    /// HStft
    /// ## new_forward
    ///
    /// `new_forward(length: integer, dtype: HDataType) -> HStft`
    ///
    /// Creates a new `HStft` instance which will be used to calculate forward STFTs.
    ///
    /// If you plan on creating multiple STFT instances, it is recommended to reuse the same planner for all of them. This is because the planner re-uses internal data
    /// across STFT instances wherever possible, saving memory and reducing setup time (STFT instances created with one planner will never re-use data and buffers with
    /// STFT instances created by a different planner).
    ///
    /// In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan STFTs with
    /// the fastest available instruction set. If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD STFTs.
    ///
    /// #### Arguments
    ///
    /// - `length` - an integer denoting the length of the input. For 2D `HArray`'s, nrows must be provided.
    ///
    /// - `dtype` - a complex `HDataType` to indicate the dtype that the `HStft` will be working with.
    ///
    /// #### Returns
    ///
    /// An `HStft`.
    ///
    /// Will return an error if dtype is of a float type.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Complex32
    /// hstft = HStft$new_forward(3L, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_forward(length: Sexp, dtype: &HDataType) -> savvy::Result<HStft> {
        let length: i32 = length.to_scalar()?;
        let length: usize = length
            .try_into()
            .map_err(|_| savvy::Error::new("Cannot convert i32 to usize."))?;
        match dtype {
            HDataType::Float32 => Err("This HStft is for Complex dtypes.".into()),
            HDataType::Float64 => Err("This HStft is for Complex dtypes.".into()),
            HDataType::Complex32 => Ok(HStft(Arc::new(Stft::<f32>::new_forward(length)))),
            HDataType::Complex64 => Ok(HStft(Arc::new(Stft::<f64>::new_forward(length)))),
        }
    }

    /// HStft
    /// ## new_real_forward
    ///
    /// `new_real_forward(length: integer, dtype: HDataType) -> HStft`
    ///
    /// Creates a new `HStft` instance which will be used to calculate real forward STFTs.
    ///
    /// If you plan on creating multiple STFT instances, it is recommended to reuse the same planner for all of them. This is because the planner re-uses internal data
    /// across STFT instances wherever possible, saving memory and reducing setup time (STFT instances created with one planner will never re-use data and buffers with
    /// STFT instances created by a different planner).
    ///
    /// In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan STFTs with
    /// the fastest available instruction set. If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD STFTs.
    ///
    /// #### Arguments
    ///
    /// - `length` - an integer denoting the length of the input. For 2D `HArray`'s, nrows must be provided.
    ///
    /// - `dtype` - a float `HDataType` to indicate the dtype that the `HStft` will be working with.
    ///
    /// #### Returns
    ///
    /// An `HStft`.
    ///
    /// Will return an error if dtype is of complex type.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Float32
    /// hstft = HStft$new_real_forward(3L, dtype)
    /// ```
    ///
    /// _________
    ///
    fn new_real_forward(length: Sexp, dtype: &HDataType) -> savvy::Result<HStft> {
        let length: i32 = length.to_scalar()?;
        let length: usize = length
            .try_into()
            .map_err(|_| savvy::Error::new("Cannot convert i32 to usize."))?;
        match dtype {
            HDataType::Float32 => Ok(HStft(Arc::new(RealStftForward::<f32>::new(length)))),
            HDataType::Float64 => Ok(HStft(Arc::new(RealStftForward::<f64>::new(length)))),
            HDataType::Complex32 => Err("This HStft is for float dtypes.".into()),
            HDataType::Complex64 => Err("This HStft is for float dtypes.".into()),
        }
    }

    /// HStft
    /// ## process
    ///
    /// `process(harray: HArray, hop_length: Integer, window_length: Integer, window: Optional<HArray>)`
    ///
    /// Computes the STFT of a complex `HArray`.
    /// The STFT computed may be forward or inverse, depending on the `HStft` created.
    ///
    /// The STFT computes the Fourier transform of short overlapping windows of the input. This giving frequency components of the signal as they change over time.
    ///
    /// The operation is done in-place, which means, in this case, although a new `HArray` is created, the same external pointer will be used to store it.
    ///
    /// For a forward STFT, the `HArray` output will have the shape:
    /// - `(fft_length, n_fft)` if 1D input HArray.
    /// - `(ncols, fft_length, n_fft)` if 2D input HArray.
    ///
    /// For a real forward STFT, it will have the shape:
    /// - `(fft_length / 2 + 1, n_fft)` if 1D input HArray.
    /// - `(ncols, fft_length / 2 + 1, n_fft)` if 2D input HArray.
    ///
    /// Where `ncols` is the number of columns of the input HArray, `fft_length` is the length provided when the `HStft` is created, `n_fft` is the number of frames and `fft_length / 2`
    /// is a floor division
    ///
    /// #### Arguments
    ///
    /// - `harray` - A complex 1D or 2D `HArray`.
    ///
    /// - `hop_length` - the distance between neighboring sliding window frames.
    ///
    /// - `window_length` - Each column of the HArray is windowed by window of length `window_length` and then padded with zeros to match n_fft. Padding is added on both
    /// the left and the right side of the window so that the window is centered within the frame. Smaller values improve the temporal resolution of the STFT (i.e. the ability
    /// to discriminate impulses that are closely spaced in time) at the expense of frequency resolution (i.e. the ability to discriminate pure tones that are closely
    /// spaced in frequency).
    ///
    /// - `window` - A float `HArray` representing a window function. This input is optional.
    ///
    /// #### Returns
    ///
    /// Will return an error if:
    ///
    /// - The `HArray`'s dtype is incompatible with the `HStft`'s dtype.
    ///
    /// - The `HArray`'s `ndim` is greater than 2.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// arr = as.array(c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i))
    /// dtype = HDataType$Complex32
    /// harray = HArray$new_from_values(arr, dtype)
    /// hstft = HStft$new_forward(5L, dtype)
    /// hop_length = 2L
    /// window_length = 3L
    /// window = HArray$new_from_values(as.array(c(1,2,3)), HDataType$Float32)
    /// hstft$process(harray, hop_length, window_length, window)
    /// ```
    ///
    /// _________
    ///
    fn process(
        &mut self,
        harray: &mut HArray,
        hop_length: Sexp,
        window_length: Sexp,
        window: Option<&HArray>,
    ) -> savvy::Result<()> {
        let inner_mut = self.get_inner_mut();
        let hop_length: i32 = hop_length.to_scalar()?;
        let hop_length: usize = hop_length
            .try_into()
            .map_err(|_| savvy::Error::new("Cannot convert i32 to usize."))?;
        let hop_length = NonZero::new(hop_length)
            .ok_or_else(|| savvy::Error::new("hop_length can't be zero."))?;
        let window_length: i32 = window_length.to_scalar()?;
        let window_length: usize = window_length
            .try_into()
            .map_err(|_| savvy::Error::new("Cannot convert i32 to usize."))?;
        let window_length = NonZero::new(window_length)
            .ok_or_else(|| savvy::Error::new("window_length can't be zero."))?;
        inner_mut.process(harray, hop_length, window_length, window)
    }

    /// HStft
    /// ## dtype
    ///
    /// `dtype() -> HDataType`
    ///
    /// Gets the `HStft`'s dtype.
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
    /// hstft = HStft$new_forward(3L, dtype)
    /// hstft$dtype()
    /// ```
    ///
    /// _________
    ///
    fn dtype(&self) -> savvy::Result<HDataType> {
        self.0.dtype()
    }

    /// HStft
    /// ## print
    ///
    /// `print()`
    ///
    /// Prints the `HStft`.
    ///
    /// Differently from R's normal behaviour, `print` doesn't return the value invisibly.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Complex32
    /// hstft = HStft$new_forward(3L, dtype)
    /// hstft$print()
    ///
    /// # or similarly:
    /// print(hstft)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        r_println!("HStft");
        Ok(())
    }

    /// HStft
    /// ## clone
    ///
    /// `clone() -> HStft`
    ///
    /// Clones the `HStft`.
    ///
    /// Creates a new `HStft`, with the underlying data pointing to the same place in memory.
    /// When `HSTFT` is cloned, thus having more than one reference to the same internal struct, and `process` is run,
    /// it uses the same cached `Fft` instance, but a new scratch buffer will have to be allocated.
    ///
    /// #### Returns
    ///
    /// An `HStft`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// dtype = HDataType$Complex32
    /// hstft = HStft$new_forward(3L, dtype)
    /// hstft$clone()
    /// ```
    ///
    /// _________
    ///
    fn clone(&self) -> savvy::Result<HStft> {
        Ok(std::clone::Clone::clone(self))
    }

    /// HStft
    /// ## is_unique
    ///
    /// `is_unique() -> bool`
    ///
    /// Checks if the object is unique.
    ///
    /// Since `HStft` has a COW ([clone-on-write](https://doc.rust-lang.org/std/borrow/enum.Cow.html)) behaviour, this function is useful to check if a new
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
    /// hstft = HStft$new_forward(3L, dtype)
    /// hstft$is_unique() # TRUE.
    ///
    /// hstft2 = hstft$clone()
    /// hstft$is_unique() # FALSE, hstft shares the same inner object with hstft2.
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

    /// HStft
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
    /// hstft = HStft$new_forward(3L, dtype)
    /// hstft$invalidate()
    /// ```
    ///
    /// _________
    ///
    pub fn invalidate(self) -> savvy::Result<()> {
        Ok(())
    }
}

macro_rules! impl_hstft {
    ($(($t1:ty, $e1:expr)),+) => {
        $(
            impl HStftR for Stft<$t1> {
                fn process(&mut self, harray: &mut HArray, hop_length: NonZero<usize>, window_length: NonZero<usize>, window: Option<&HArray>) -> savvy::Result<()> {
                    let harray_inner = harray.get_inner_mut().as_any_mut().downcast_mut::<harmonium_core::array::HArray<Complex<$t1>, IxDyn>>().ok_or_else(|| savvy::Error::new("HArray and HStft must have the same HDataType."))?;
                    let window = if let Some(harray_window) = window {
                        Some(harray_window.0.as_any().downcast_ref::<harmonium_core::array::HArray<$t1, IxDyn>>().ok_or_else(|| savvy::Error::new("HArray and HStft must have the same HDataType."))?.as_slice().unwrap())
                    } else {
                        None
                    };
                    let harray_output = harmonium_fft::stft::ProcessStft::process(self, harray_inner, hop_length, window_length, window).map_err(|err| savvy::Error::from(HErrorR::from(err)))?;
                    *harray = HArray(Arc::new(harray_output));
                    Ok(())

                }

                fn dtype(&self) -> savvy::Result<HDataType> {
                    Ok($e1)
                }

                fn clone_inner(&self) -> Arc<dyn HStftR> {
                    Arc::new(self.clone())
                }
            }
        )+
    };
}

impl_hstft!((f32, HDataType::Complex32), (f64, HDataType::Complex64));

macro_rules! impl_hrealstftforward {
    ($(($t1:ty, $e1:expr)),+) => {
        $(
            impl HStftR for RealStftForward<$t1> {
                fn process(&mut self, harray: &mut HArray, hop_length: NonZero<usize>, window_length: NonZero<usize>, window: Option<&HArray>) -> savvy::Result<()> {
                    let harray_inner = harray.get_inner_mut().as_any_mut().downcast_mut::<harmonium_core::array::HArray<$t1, IxDyn>>().ok_or_else(|| savvy::Error::new("HArray and HStft must have the same HDataType."))?;
                    let window = if let Some(harray_window) = window {
                        Some(harray_window.0.as_any().downcast_ref::<harmonium_core::array::HArray<$t1, IxDyn>>().ok_or_else(|| savvy::Error::new("HArray and HStft must have the same HDataType."))?.as_slice().unwrap())
                    } else {
                        None
                    };
                    let harray_output = harmonium_fft::stft::ProcessStft::process(self, harray_inner, hop_length, window_length, window).map_err(|err| savvy::Error::from(HErrorR::from(err)))?;
                    *harray = HArray(Arc::new(harray_output));
                    Ok(())

                }

                fn dtype(&self) -> savvy::Result<HDataType> {
                    Ok($e1)
                }

                fn clone_inner(&self) -> Arc<dyn HStftR> {
                    Arc::new(self.clone())
                }
            }
        )+
    };
}

impl_hrealstftforward!((f32, HDataType::Complex32), (f64, HDataType::Complex64));

impl HStft {
    #[doc(hidden)]
    pub fn get_inner_mut(&mut self) -> &mut dyn HStftR {
        // Weak references are never used.
        if Arc::strong_count(&self.0) != 1 {
            self.0 = self.0.clone_inner();
        }
        // Safety: reference count was checked.
        // Use get_mut_unchecked when stable.
        unsafe { Arc::get_mut(&mut self.0).unwrap_unchecked() }
    }
}
