use crate::{conversions::Conversions, harray::HArray, hdatatype::HDataType};
use harmonium_core::conversions::IntoDynamic;
use harmonium_window::windows::*;
use savvy::{savvy, Sexp};
use std::sync::Arc;

/// HWindow
/// A collection of window functions. \
///
/// # Methods
///
pub struct HWindow;

#[savvy]
impl HWindow {
    /// HWindow
    /// ## barthann
    ///
    /// `barthann(npoints: integer, sym: bool, dtype: HDataType) -> HArray` \
    ///
    /// Returns a modified Bartlett-Hann window. \
    ///
    /// The maximum value is normalized to 1 (though the value 1
    /// does not appear if `npoints` is even and `window_type` is symmetric). \
    ///
    /// #### Arguments
    ///
    /// * `npoints` \
    /// An `integer`. Number of points in the output window. \
    /// * `sym` \
    /// A `bool`. \
    /// When `TRUE`, generates a symmetric window, for use in filter design. \
    /// When `FALSE`, generates a periodic window, for use in spectral analysis. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// Must be a float dtype. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HWindow$barthann(npoints = 10L, sym = TRUE, dtype = HDataType$Float64)
    /// ```
    ///
    fn barthann(npoints: Sexp, sym: Sexp, dtype: &HDataType) -> savvy::Result<HArray> {
        let npoints: i32 = npoints.to_scalar()?;
        let npoints = npoints.try_into().unwrap();
        let sym: bool = sym.to_scalar()?;

        let window_type = if sym {
            WindowType::Symmetric
        } else {
            WindowType::Periodic
        };

        match dtype {
            HDataType::Float32 => {
                let harray = harmonium_window::windows::barthann::<f32>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            HDataType::Float64 => {
                let harray = harmonium_window::windows::barthann::<f64>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            _ => Err("Not a valid dtype.".into()),
        }
    }

    /// HWindow
    /// ## bartlett
    ///
    /// `bartlett(npoints: integer, sym: bool, dtype: HDataType) -> HArray` \
    ///
    /// $w(n) = \frac{2}{npoints-1} (\frac{npoints-1}{2} - |n - \frac{npoints-1}{2}|)$
    ///
    /// The Bartlett window is very similar to a triangular window, except
    /// that the end points are at zero.  It is often used in signal
    /// processing for tapering a signal, without generating too much
    /// ripple in the frequency domain. \
    ///
    /// The maximum value is normalized to 1 (though the value 1 does not
    /// appear if `npoints` is even and `window_type` is symmetric. \
    ///
    /// #### Arguments
    ///
    /// * `npoints` \
    /// An `integer`. Number of points in the output window. \
    /// * `sym` \
    /// A `bool`. \
    /// When `TRUE`, generates a symmetric window, for use in filter design. \
    /// When `FALSE`, generates a periodic window, for use in spectral analysis. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// Must be a float dtype. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HWindow$bartlett(npoints = 10L, sym = TRUE, dtype = HDataType$Float64)
    /// ```
    ///
    fn bartlett(npoints: Sexp, sym: Sexp, dtype: &HDataType) -> savvy::Result<HArray> {
        let npoints: i32 = npoints.to_scalar()?;
        let npoints = npoints.try_into().unwrap();

        let sym: bool = sym.to_scalar()?;

        let window_type = if sym {
            WindowType::Symmetric
        } else {
            WindowType::Periodic
        };

        match dtype {
            HDataType::Float32 => {
                let harray = harmonium_window::windows::bartlett::<f32>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            HDataType::Float64 => {
                let harray = harmonium_window::windows::bartlett::<f64>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            _ => Err("Not a valid dtype.".into()),
        }
    }

    /// HWindow
    /// ## blackman
    ///
    /// `blackman(npoints: integer, sym: bool, dtype: HDataType) -> HArray` \
    ///
    /// Returns a Blackman window. \
    ///
    /// $w(n) = 0.42 - 0.5 \cos(2\pi n/npoints) + 0.08 \cos(4\pi n/npoints)$
    ///
    /// The Blackman window is a taper formed by using the first three terms of
    /// a summation of cosines. It was designed to have close to the minimal
    /// leakage possible.  It is close to optimal, only slightly worse than a
    /// Kaiser window. \
    ///
    /// The maximum value is normalized to 1 (though the value 1 does not
    /// appear if `npoints` is even and `sym` is `TRUE`). \
    ///
    /// The "exact Blackman" window was designed to null out the third and fourth
    /// sidelobes, but has discontinuities at the boundaries, resulting in a
    /// 6 dB/oct fall-off.  This window is an approximation of the "exact" window,
    /// which does not null the sidelobes as well, but is smooth at the edges,
    /// improving the fall-off rate to 18 dB/oct. \
    ///
    /// Most references to the Blackman window come from the signal processing
    /// literature, where it is used as one of many windowing functions for
    /// smoothing values.  It is also known as an apodization (which means
    /// "removing the foot", i.e. smoothing discontinuities at the beginning
    /// and end of the sampled signal) or tapering function. It is known as a
    /// "near optimal" tapering function, almost as good (by some measures)
    /// as the Kaiser window. \
    ///
    /// #### Arguments
    ///
    /// * `npoints` \
    /// An `integer`. Number of points in the output window. \
    /// * `sym` \
    /// A `bool`. \
    /// When `TRUE`, generates a symmetric window, for use in filter design. \
    /// When `FALSE`, generates a periodic window, for use in spectral analysis. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// Must be a float dtype. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HWindow$blackman(npoints = 10L, sym = TRUE, dtype = HDataType$Float64)
    /// ```
    ///
    fn blackman(npoints: Sexp, sym: Sexp, dtype: &HDataType) -> savvy::Result<HArray> {
        let npoints: i32 = npoints.to_scalar()?;
        let npoints = npoints.try_into().unwrap();

        let sym: bool = sym.to_scalar()?;

        let window_type = if sym {
            WindowType::Symmetric
        } else {
            WindowType::Periodic
        };

        match dtype {
            HDataType::Float32 => {
                let harray = harmonium_window::windows::blackman::<f32>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            HDataType::Float64 => {
                let harray = harmonium_window::windows::blackman::<f64>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            _ => Err("Not a valid dtype.".into()),
        }
    }

    /// HWindow
    /// ## blackmanharris
    ///
    /// `blackmanharris(npoints: integer, sym: bool, dtype: HDataType) -> HArray` \
    ///
    /// Return a minimum 4-term Blackman-Harris window. \
    ///
    /// The maximum value is normalized to 1 (though the value 1 does not
    /// appear if `npoints` is even and `window_type` is symmetric. \
    ///
    /// #### Arguments
    ///
    /// * `npoints` \
    /// An `integer`. Number of points in the output window. \
    /// * `sym` \
    /// A `bool`. \
    /// When `TRUE`, generates a symmetric window, for use in filter design. \
    /// When `FALSE`, generates a periodic window, for use in spectral analysis. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// Must be a float dtype. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HWindow$blackmanharris(npoints = 10L, sym = TRUE, dtype = HDataType$Float64)
    /// ```
    ///
    fn blackmanharris(npoints: Sexp, sym: Sexp, dtype: &HDataType) -> savvy::Result<HArray> {
        let npoints: i32 = npoints.to_scalar()?;
        let npoints = npoints.try_into().unwrap();

        let sym: bool = sym.to_scalar()?;

        let window_type = if sym {
            WindowType::Symmetric
        } else {
            WindowType::Periodic
        };

        match dtype {
            HDataType::Float32 => {
                let harray = harmonium_window::windows::blackmanharris::<f32>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            HDataType::Float64 => {
                let harray = harmonium_window::windows::blackmanharris::<f64>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            _ => Err("Not a valid dtype.".into()),
        }
    }

    /// HWindow
    /// ## bohman
    ///
    /// `bohman(npoints: integer, sym: bool, dtype: HDataType) -> HArray` \
    ///
    /// Returns a Bohman window. \
    ///
    /// The maximum value is normalized to 1 (though the value 1 does not
    /// appear if `npoints` is even and `window_type` is symmetric). \
    ///
    /// #### Arguments
    ///
    /// * `npoints` \
    /// An `integer`. Number of points in the output window. \
    /// * `sym` \
    /// A `bool`. \
    /// When `TRUE`, generates a symmetric window, for use in filter design. \
    /// When `FALSE`, generates a periodic window, for use in spectral analysis. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// Must be a float dtype. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HWindow$bohman(npoints = 10L, sym = TRUE, dtype = HDataType$Float64)
    /// ```
    ///
    fn bohman(npoints: Sexp, sym: Sexp, dtype: &HDataType) -> savvy::Result<HArray> {
        let npoints: i32 = npoints.to_scalar()?;
        let npoints = npoints.try_into().unwrap();

        let sym: bool = sym.to_scalar()?;

        let window_type = if sym {
            WindowType::Symmetric
        } else {
            WindowType::Periodic
        };

        match dtype {
            HDataType::Float32 => {
                let harray = harmonium_window::windows::bohman::<f32>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            HDataType::Float64 => {
                let harray = harmonium_window::windows::bohman::<f64>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            _ => Err("Not a valid dtype.".into()),
        }
    }

    /// HWindow
    /// ## boxcar
    ///
    /// `boxcar(npoints: integer, dtype: HDataType) -> HArray` \
    ///
    /// Returns a boxcar or rectangular window. \
    ///
    /// Also known as a rectangular window or Dirichlet window, this is equivalent to no window at all. \
    ///
    /// #### Arguments
    ///
    /// * `npoints` \
    /// An `integer`. Number of points in the output window. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// Must be a float dtype. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HWindow$boxcar(npoints = 10L, dtype = HDataType$Float64)
    /// ```
    ///
    fn boxcar(npoints: Sexp, dtype: &HDataType) -> savvy::Result<HArray> {
        let npoints: i32 = npoints.to_scalar()?;
        let npoints = npoints.try_into().unwrap();

        match dtype {
            HDataType::Float32 => {
                let harray = harmonium_window::windows::boxcar::<f32>(npoints);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            HDataType::Float64 => {
                let harray = harmonium_window::windows::boxcar::<f64>(npoints);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            _ => Err("Not a valid dtype.".into()),
        }
    }

    /// HWindow
    /// ## cosine
    ///
    /// `cosine(npoints: integer, sym: bool, dtype: HDataType) -> HArray` \
    ///
    /// Returns a window with a simple cosine shape. \
    ///
    /// The maximum value is normalized to 1 (though the value 1 does not
    /// appear if `npoints` is even and `window_type` is symmetric). \
    ///
    /// #### Arguments
    ///
    /// * `npoints` \
    /// An `integer`. Number of points in the output window. \
    /// * `sym` \
    /// A `bool`. \
    /// When `TRUE`, generates a symmetric window, for use in filter design. \
    /// When `FALSE`, generates a periodic window, for use in spectral analysis. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// Must be a float dtype. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HWindow$cosine(npoints = 10L, sym = TRUE, dtype = HDataType$Float64)
    /// ```
    ///
    fn cosine(npoints: Sexp, sym: Sexp, dtype: &HDataType) -> savvy::Result<HArray> {
        let npoints: i32 = npoints.to_scalar()?;
        let npoints = npoints.try_into().unwrap();

        let sym: bool = sym.to_scalar()?;

        let window_type = if sym {
            WindowType::Symmetric
        } else {
            WindowType::Periodic
        };

        match dtype {
            HDataType::Float32 => {
                let harray = harmonium_window::windows::cosine::<f32>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            HDataType::Float64 => {
                let harray = harmonium_window::windows::cosine::<f64>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            _ => Err("Not a valid dtype.".into()),
        }
    }

    /// HWindow
    /// ## hann
    ///
    /// `hann(npoints: integer, sym: bool, dtype: HDataType) -> HArray` \
    ///
    /// Returns a Hann window. \
    ///
    /// $w(n) = 0.5 - 0.5 \cos\left(\frac{2\pi{n}}{npoints-1}\right) \qquad 0 \leq n \leq npoints-1$
    ///
    /// The maximum value is normalized to 1 (though the value 1 does not
    /// appear if `npoints` is even and `window_type` is symmetric. \
    ///
    /// The Hann window is a taper formed by using a raised cosine or sine-squared
    /// with ends that touch zero. \
    ///
    /// #### Arguments
    ///
    /// * `npoints` \
    /// An `integer`. Number of points in the output window. \
    /// * `sym` \
    /// A `bool`. \
    /// When `TRUE`, generates a symmetric window, for use in filter design. \
    /// When `FALSE`, generates a periodic window, for use in spectral analysis. \
    /// * `dtype` \
    /// An `HDataType` to indicate which type of `HArray` to be created. \
    /// Must be a float dtype. \
    ///
    /// #### Returns
    ///
    /// An `HArray`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HWindow$hann(npoints = 10L, sym = TRUE, dtype = HDataType$Float64)
    /// ```
    ///
    fn hann(npoints: Sexp, sym: Sexp, dtype: &HDataType) -> savvy::Result<HArray> {
        let npoints: i32 = npoints.to_scalar()?;
        let npoints = npoints.try_into().unwrap();

        let sym: bool = sym.to_scalar()?;

        let window_type = if sym {
            WindowType::Symmetric
        } else {
            WindowType::Periodic
        };

        match dtype {
            HDataType::Float32 => {
                let harray = harmonium_window::windows::hann::<f32>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            HDataType::Float64 => {
                let harray = harmonium_window::windows::hann::<f64>(npoints, window_type);
                let harray = harray.into_dynamic();
                Ok(HArray(Arc::new(harray)))
            }
            _ => Err("Not a valid dtype.".into()),
        }
    }
}
