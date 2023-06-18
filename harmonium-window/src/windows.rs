use arrow2::types::NativeType;
use harmonium_core::{
    errors::{HError, HResult},
    structs::HFloatArray,
};
use num_traits::Float;
use realfft::{num_complex::Complex, FftNum, RealFftPlanner};
use rustfft::FftPlanner;

pub enum Window {
    Barthann,
    Bartlett,
    Blackman,
    BlackmanHarris,
    Bohman,
    Boxcar,
    Cosine,
    Chebwin,
    Hann,
    Triangle,
}

pub trait FloatConst: Float {
    const PI_FLOAT: Self;
}

impl FloatConst for f32 {
    const PI_FLOAT: f32 = std::f32::consts::PI;
}

impl FloatConst for f64 {
    const PI_FLOAT: f64 = std::f64::consts::PI;
}

pub enum WindowType {
    Symmetric,
    Periodic,
}

/// Returns a modified Bartlett-Hann window.
///
/// The maximum value is normalized to 1 (though the value 1
/// does not appear if `npoints` is even and `window_type` is symmetric).
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn barthann<T>(npoints: usize, window_type: WindowType) -> HFloatArray<T>
where
    T: Float + FloatConst + NativeType,
{
    let np_f = match window_type {
        WindowType::Symmetric => T::from(npoints - 1).unwrap(),
        WindowType::Periodic => T::from(npoints).unwrap(),
    };

    let pi2 = T::from(2.0).unwrap() * T::PI_FLOAT;
    let a = T::from(0.62).unwrap();
    let b = T::from(0.48).unwrap();
    let c = T::from(0.38).unwrap();
    let half = T::from(0.5).unwrap();

    let window: Vec<T> = (0..npoints)
        .map(|x| {
            let x_float = T::from(x).unwrap();
            let fac = (x_float / (np_f) - half).abs();
            a - b * fac + c * (pi2 * fac).cos()
        })
        .collect();

    HFloatArray::new_from_vec(window)
}

/// Returns a Bartlett window.
///
/// The Bartlett window is very similar to a triangular window, except
/// that the end points are at zero.  It is often used in signal
/// processing for tapering a signal, without generating too much
/// ripple in the frequency domain.
///
/// The maximum value is normalized to 1 (though the value 1 does not
/// appear if `npoints` is even and `window_type` is symmetric).
///
/// .. math:: w(n) = \frac{2}{npoints-1} \left(
///           \frac{npoints-1}{2} - \left|n - \frac{npoints-1}{2}\right|
///           \right)
///
/// Most references to the Bartlett window come from the signal
/// processing literature, where it is used as one of many windowing
/// functions for smoothing values.  Note that convolution with this
/// window produces linear interpolation.  It is also known as an
/// apodization (which means"removing the foot", i.e. smoothing
/// discontinuities at the beginning and end of the sampled signal) or
/// tapering function. The Fourier transform of the Bartlett is the product
/// of two sinc functions.
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn bartlett<T>(npoints: usize, window_type: WindowType) -> HFloatArray<T>
where
    T: Float + NativeType,
{
    let np_f = match window_type {
        WindowType::Symmetric => T::from(npoints - 1).unwrap(),
        WindowType::Periodic => T::from(npoints).unwrap(),
    };

    let two = T::from(2.0).unwrap();

    let window: Vec<T> = (0..npoints)
        .map(|x| {
            let x_float = T::from(x).unwrap();
            if x_float <= (np_f) / two {
                two * x_float / (np_f)
            } else {
                two - two * x_float / (np_f)
            }
        })
        .collect();

    HFloatArray::new_from_vec(window)
}

/// Returns a Blackman window.
///
/// The Blackman window is a taper formed by using the first three terms of
/// a summation of cosines. It was designed to have close to the minimal
/// leakage possible.  It is close to optimal, only slightly worse than a
/// Kaiser window.
///
/// The maximum value is normalized to 1 (though the value 1 does not
/// appear if `npoints` is even and `window_type` is symmetric).
///
/// .. math::  w(n) = 0.42 - 0.5 \cos(2\pi n/npoints) + 0.08 \cos(4\pi n/npoints)
///
/// The "exact Blackman" window was designed to null out the third and fourth
/// sidelobes, but has discontinuities at the boundaries, resulting in a
/// 6 dB/oct fall-off.  This window is an approximation of the "exact" window,
/// which does not null the sidelobes as well, but is smooth at the edges,
/// improving the fall-off rate to 18 dB/oct.
///
/// Most references to the Blackman window come from the signal processing
/// literature, where it is used as one of many windowing functions for
/// smoothing values.  It is also known as an apodization (which means
/// "removing the foot", i.e. smoothing discontinuities at the beginning
/// and end of the sampled signal) or tapering function. It is known as a
/// "near optimal" tapering function, almost as good (by some measures)
/// as the Kaiser window.
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn blackman<T>(npoints: usize, window_type: WindowType) -> HFloatArray<T>
where
    T: Float + FloatConst + NativeType,
{
    let np_f = match window_type {
        WindowType::Symmetric => T::from(npoints - 1).unwrap(),
        WindowType::Periodic => T::from(npoints).unwrap(),
    };

    let pi2 = T::from(2.0).unwrap() * T::PI_FLOAT;
    let pi4 = T::from(4.0).unwrap() * T::PI_FLOAT;
    let a = T::from(0.42).unwrap();
    let b = T::from(0.5).unwrap();
    let c = T::from(0.08).unwrap();

    let window: Vec<T> = (0..npoints)
        .map(|x| {
            let x_float = T::from(x).unwrap();
            a - b * (pi2 * x_float / (np_f)).cos() + c * (pi4 * x_float / (np_f)).cos()
        })
        .collect();

    HFloatArray::new_from_vec(window)
}

/// Returns a minimum 4-term Blackman-Harris window.
///
/// The maximum value is normalized to 1 (though the value 1 does not
/// appear if `npoints` is even and `window_type` is symmetric).
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn blackmanharris<T>(npoints: usize, window_type: WindowType) -> HFloatArray<T>
where
    T: Float + FloatConst + NativeType,
{
    let np_f = match window_type {
        WindowType::Symmetric => T::from(npoints - 1).unwrap(),
        WindowType::Periodic => T::from(npoints).unwrap(),
    };

    let pi2 = T::from(2.0).unwrap() * T::PI_FLOAT;
    let pi4 = T::from(4.0).unwrap() * T::PI_FLOAT;
    let pi6 = T::from(6.0).unwrap() * T::PI_FLOAT;
    let a = T::from(0.35875).unwrap();
    let b = T::from(0.48829).unwrap();
    let c = T::from(0.14128).unwrap();
    let d = T::from(0.01168).unwrap();

    let window: Vec<T> = (0..npoints)
        .map(|x| {
            let x_float = T::from(x).unwrap();
            a - b * (pi2 * x_float / (np_f)).cos() + c * (pi4 * x_float / (np_f)).cos()
                - d * (pi6 * x_float / (np_f)).cos()
        })
        .collect();

    HFloatArray::new_from_vec(window)
}

/// Returns a Bohman window.
///
/// The maximum value is normalized to 1 (though the value 1 does not
/// appear if `npoints` is even and `window_type` is symmetric).
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn bohman<T>(npoints: usize, window_type: WindowType) -> HFloatArray<T>
where
    T: Float + FloatConst + NativeType,
{
    let np_f = match window_type {
        WindowType::Symmetric => T::from(npoints - 1).unwrap(),
        WindowType::Periodic => T::from(npoints).unwrap(),
    };

    let pi = T::PI_FLOAT;
    let zero = T::from(0.0).unwrap();
    let one = T::from(1.0).unwrap();
    let two = T::from(2.0).unwrap();
    let step = two / (np_f);
    let mut fac = -one;

    let mut window: Vec<T> = Vec::with_capacity(npoints);

    for n in 0..npoints {
        if n == 0 || n == npoints - 1 {
            window.push(zero);
        } else {
            fac = fac + step;
            let fac_abs = fac.abs();
            window.push((one - fac_abs) * (pi * fac_abs).cos() + one / pi * (pi * fac_abs).sin());
        }
    }

    HFloatArray::new_from_vec(window)
}

/// Returns a boxcar or rectangular window.
///
/// Also known as a rectangular window or Dirichlet window, this is equivalent to no window at all.
///
/// # Arguments
/// `npoints` - Number of points in the output window.
pub fn boxcar<T>(npoints: usize) -> HFloatArray<T>
where
    T: Float + NativeType,
{
    let one = T::from(1.0).unwrap();
    let window: Vec<T> = (0..npoints).map(|_| one).collect();

    HFloatArray::new_from_vec(window)
}

/// Returns a window with a simple cosine shape.
///
/// The maximum value is normalized to 1 (though the value 1 does not
/// appear if `npoints` is even and `window_type` is symmetric).
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn cosine<T>(npoints: usize, window_type: WindowType) -> HFloatArray<T>
where
    T: Float + FloatConst + NativeType,
{
    let np_f = match window_type {
        WindowType::Symmetric => T::from(npoints).unwrap(),
        WindowType::Periodic => T::from(npoints + 1).unwrap(),
    };

    let pi = T::PI_FLOAT;
    let half = T::from(0.5).unwrap();

    let window: Vec<T> = (0..npoints)
        .map(|x| (pi / (np_f) * (T::from(x).unwrap() + half)).sin())
        .collect();

    HFloatArray::new_from_vec(window)
}

/// Returns a Dolph-Chebyshev window.
///
/// The maximum value is normalized to 1.
///
/// This window optimizes for the narrowest main lobe width for a given order
/// `M` and sidelobe equiripple attenuation `at`, using Chebyshev
/// polynomials.  It was originally developed by Dolph to optimize the
/// directionality of radio antenna arrays.
///
/// Unlike most windows, the Dolph-Chebyshev is defined in terms of its
/// frequency response:
///
/// .. math:: W(k) = \frac
///           {\cos\{npoints \cos^{-1}[\beta \cos(\frac{\pi k}{npoints})]\}}
///           {\cosh[npoints \cosh^{-1}(\beta)]}
///
/// where
///
/// .. math:: \beta = \cosh \left [\frac{1}{npoints}
///           \cosh^{-1}(10^\frac{A}{20}) \right ]
///
/// and 0 <= abs(k) <= npoints - 1. A is the attenuation in decibels (`at`).
///
/// The time domain window is then generated using the IFFT, so
/// power-of-two `npoints` are the fastest to generate, and prime number `npoints` are
/// the slowest.
///
/// The equiripple condition in the frequency domain creates impulses in the
/// time domain, which appear at the ends of the window.
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `at` - Attenuation in dB.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn chebwin<T>(npoints: usize, at: T, window_type: WindowType) -> HFloatArray<T>
where
    T: Float + FloatConst + FftNum + NativeType,
{
    let np_f = match window_type {
        WindowType::Symmetric => T::from(npoints).unwrap(),
        WindowType::Periodic => T::from(npoints + 1).unwrap(),
    };
    let np = np_f.to_usize().unwrap();

    let pi = T::PI_FLOAT;
    let zero = T::from(0.0).unwrap();
    let one = T::from(1.0).unwrap();
    let two = T::from(2.0).unwrap();
    let ten = T::from(10.0).unwrap();
    let twenty = T::from(20.0).unwrap();
    let expr = T::from(two * T::from(np % 2).unwrap() - one).unwrap();

    let order = np_f - one;
    let beta = (one / order * (T::powf(ten, at.abs() / twenty)).acosh()).cosh();

    let mut window: Vec<T> = Vec::with_capacity(npoints);
    let mut v: Vec<T> = Vec::with_capacity(np);

    for x in 0..np {
        let x_float = T::from(x).unwrap();
        let y = beta * (pi * x_float / np_f).cos();

        if y > one {
            v.push((order * y.acosh()).cosh());
        } else if y < -one {
            v.push(expr * (order * (-y).acosh()).cosh());
        } else {
            v.push((order * (y.acos())).cos());
        }
    }

    if np % 2 == 1 {
        let mut real_planner = RealFftPlanner::<T>::new();
        let r2c = real_planner.plan_fft_forward(np);
        let mut output_vec = r2c.make_output_vec();
        r2c.process(&mut v, &mut output_vec).unwrap();

        let n = (np + 1) / 2;

        for i in (1..n).rev().chain(0..(npoints + 1) / 2) {
            window.push(output_vec[i].re);
        }
    } else {
        let mut output_vec: Vec<Complex<T>> = Vec::with_capacity(npoints);
        let pi_div = pi / np_f;

        for (i, item) in v.iter().enumerate() {
            let i_float = T::from(i).unwrap();
            let complex = <Complex<T>>::exp(Complex::new(zero, pi_div * i_float));
            output_vec.push(complex * item);
        }

        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(np);
        fft.process(&mut output_vec);

        let n = np / 2 + 1;

        for i in (1..n).rev().chain(1..npoints / 2 + 1) {
            window.push(output_vec[i].re);
        }
    }

    let max = *window
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();

    window.iter_mut().for_each(|z| {
        *z = *z / max;
    });

    HFloatArray::new_from_vec(window)
}

/// Returns an exponential (or Poisson) window.
///
/// The maximum value is normalized to 1 (though the value 1 does not
/// appear if `npoints` is even and `window_type` is symmetric).
///
/// .. math::  w(n) = 0.5 - 0.5 \cos\left(\frac{2\pi{n}}{M-1}\right)
///            \qquad 0 \leq n \leq M-1
///
/// The window was named for Julius von Hann, an Austrian meteorologist. It is
/// also known as the Cosine Bell. It is sometimes erroneously referred to as
/// the "Hanning" window, from the use of "hann" as a verb in the original
/// paper and confusion with the very similar Hamming window.
///
/// Most references to the Hann window come from the signal processing
/// literature, where it is used as one of many windowing functions for
/// smoothing values.  It is also known as an apodization (which means
/// "removing the foot", i.e. smoothing discontinuities at the beginning
/// and end of the sampled signal) or tapering function.
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `center` - Parameter defining the center location of the window function.
///            The default value if not given is ``center = np_f / 2``.  This
///            parameter must take its default value for symmetric windows.
/// `tau` - Parameter defining the decay.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn exponential<T>(
    npoints: usize,
    center: Option<T>,
    tau: T,
    window_type: WindowType,
) -> HResult<HFloatArray<T>>
where
    T: Float + FloatConst + NativeType,
{
    let np_f = match window_type {
        WindowType::Symmetric if center.is_none() => T::from(npoints - 1).unwrap(),
        WindowType::Symmetric => {
            return Err(HError::OutOfSpecError(
                "center must be none for symmetric windows".into(),
            ));
        }
        WindowType::Periodic => T::from(npoints).unwrap(),
    };

    let half = T::from(0.5).unwrap();
    let center = center.unwrap_or_else(|| np_f * half);

    let window = (0..npoints)
        .map(|x| {
            let x_float = T::from(x).unwrap();
            T::exp(-(x_float - center).abs() / tau)
        })
        .collect();

    Ok(HFloatArray::new_from_vec(window))
}

/// Returns a Hann window.
///
/// The maximum value is normalized to 1 (though the value 1 does not
/// appear if `npoints` is even and `window_type` is symmetric).
///
/// The Hann window is a taper formed by using a raised cosine or sine-squared
/// with ends that touch zero.
///
/// .. math::  w(n) = 0.5 - 0.5 \cos\left(\frac{2\pi{n}}{npoints-1}\right)
///            \qquad 0 \leq n \leq npoints-1
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn hann<T>(npoints: usize, window_type: WindowType) -> HFloatArray<T>
where
    T: Float + FloatConst + NativeType,
{
    let np_f = match window_type {
        WindowType::Symmetric => T::from(npoints - 1).unwrap(),
        WindowType::Periodic => T::from(npoints).unwrap(),
    };

    let pi2 = T::from(2.0).unwrap() * T::PI_FLOAT;
    let half = T::from(0.5).unwrap();

    let window: Vec<T> = (0..npoints)
        .map(|x| half - half * (pi2 * T::from(x).unwrap() / (np_f)).cos())
        .collect();

    HFloatArray::new_from_vec(window)
}

/// Returns a triangular window.
///
/// The maximum value is normalized to 1 (though the value 1 does not
/// appear if `npoints` is even and `window_type` is symmetric).
///
/// # Arguments
/// `npoints` - Number of points in the output window.
/// `window_type` - When `WindowType::Symmetric`, generates a symmetric window, for use in filter design.
///                 When `WindowType::Periodic`, generates a periodic window, for use in spectral analysis.
pub fn triangle<T>(npoints: usize, window_type: WindowType) -> HFloatArray<T>
where
    T: Float + NativeType,
{
    let np_f = T::from(npoints).unwrap();
    let one = T::from(1.0).unwrap();
    let two = T::from(2.0).unwrap();

    let mut window: Vec<T> = Vec::with_capacity(npoints);

    match (npoints % 2 == 0, window_type) {
        (true, WindowType::Symmetric) => {
            for x in (1..((npoints + 3) / 2)).chain((1..((npoints + 3) / 2)).rev()) {
                let x_float = T::from(x).unwrap();
                window.push((two * x_float - one) / np_f);
            }
        }
        (false, WindowType::Symmetric) => {
            for x in (1..((npoints + 3) / 2)).chain((1..((npoints + 1) / 2)).rev()) {
                let x_float = T::from(x).unwrap();
                window.push(two * x_float / (np_f + one));
            }
        }
        (true, WindowType::Periodic) => {
            for x in (1..(npoints / 2 + 2)).chain((2..(npoints / 2 + 1)).rev()) {
                let x_float = T::from(x).unwrap();
                window.push(two * x_float / (np_f + two));
            }
        }
        (false, WindowType::Periodic) => {
            for x in (1..((npoints + 3) / 2)).chain((2..((npoints + 3) / 2)).rev()) {
                let x_float = T::from(x).unwrap();
                window.push((two * x_float - one) / (np_f + one));
            }
        }
    }

    HFloatArray::new_from_vec(window)
}

//pub fn kaiser<T>(npoints: usize, beta: T, window_type: WindowType) -> Vec<T>
//where
//    T: Float + FloatConst,
//{
//    let np_f = match window_type {
//        WindowType::Periodic => T::from(npoints - 1).unwrap(),
//        WindowType::Symmetric => T::from(npoints).unwrap(),
//    };
//
//    let pi = T::PI_FLOAT;
//    let a = T::from(2.0).unwrap();
//
//    n = np.arange(0, M)
//    alpha = (M - 1) / a
//    w = (special.i0(beta * np.sqrt(1 - ((n - alpha) / alpha) ** a)) /
//         special.i0(beta))
//
//    let window: Vec<T> = (0..npoints)
//        .map(|x| {
//            let x_float = T::from(x).unwrap();
//            (pi / np_f * (x_float + a)).sin()
//        })
//        .collect();
//
//    window
//}


#[cfg(test)]
mod tests {
    use super::*;

    pub fn compare<T>(lhs: HFloatArray<T>, rhs: HFloatArray<T>) -> bool
    where
        T: NativeType + Float
    {
        if lhs.len() != rhs.len() { return false }
        let mut result = true;
        for i in 0..lhs.len() {
            if (lhs.as_slice()[i] - rhs.as_slice()[i]).abs() >= T::from(1e-4).unwrap() {
                result = false;
            };
        }

        result
    }

    #[test]
    fn bartlett_test() {
        let v_symmetric: HFloatArray<f32> = bartlett(8, WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            0.0, 0.2857143, 0.5714286, 0.85714287, 0.8571428, 0.57142854, 0.28571427, 0.0,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> = bartlett(8, WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![0., 0.25, 0.5, 0.75, 1., 0.75, 0.5, 0.25]);
        assert!(compare(v_periodic, rhs));
    }

    #[test]
    fn barthann_test() {
        let v_symmetric: HFloatArray<f32> = barthann(8, WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            0.0, 0.21164526, 0.60170084, 0.92808247, 0.9280824, 0.6017007, 0.21164526, 0.0,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> = barthann(8, WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            0.0, 0.17129943, 0.49999997, 0.82870054, 1.0, 0.82870054, 0.49999997, 0.17129943,
        ]);
        assert!(compare(v_periodic, rhs));
    }

    #[test]
    fn blackman_test() {
        let v_symmetric: HFloatArray<f32> = blackman(8, WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            -1.4901161e-8,
            0.090453416,
            0.45918298,
            0.9203636,
            0.9203636,
            0.4591827,
            0.090453446,
            -1.4901161e-8,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> = blackman(8, WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            -1.4901161e-8,
            0.0664466,
            0.34000003,
            0.7735534,
            0.99999994,
            0.7735533,
            0.33999997,
            0.066446535,
        ]);
        assert!(compare(v_periodic, rhs));
    }

    #[test]
    fn blackmanharris_test() {
        let v_symmetric: HFloatArray<f32> = blackmanharris(8, WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            5.9968792e-5,
            0.033391707,
            0.3328335,
            0.8893698,
            0.8893698,
            0.33283323,
            0.03339171,
            5.9968792e-5,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> = blackmanharris(8, WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            5.9968792e-5,
            0.02173582,
            0.21747002,
            0.6957641,
            1.0,
            0.69576406,
            0.21746999,
            0.021735793,
        ]);
        assert!(compare(v_periodic, rhs));
    }

    #[test]
    fn bohman_test() {
        let v_symmetric: HFloatArray<f32> = bohman(8, WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            0.0,
            0.070724666,
            0.43748394,
            0.91036844,
            0.9103685,
            0.4374839,
            0.070724666,
            0.0,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> = bohman(8, WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            0.0,
            0.048302367,
            0.31830984,
            0.7554091,
            1.0,
            0.7554091,
            0.31830984,
            0.0,
        ]);
        assert!(compare(v_periodic, rhs));
    }

    #[test]
    fn boxcar_test() {
        let v: HFloatArray<f32> = boxcar(8);
        let rhs = HFloatArray::new_from_vec(vec![1., 1., 1., 1., 1., 1., 1., 1.]);
        assert!(compare(v, rhs));
    }

    #[test]
    fn chebwin_test() {
        let v_symmetric: HFloatArray<f32> = chebwin(8, 70., WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            0.05397676, 0.27194428, 0.66340005, 1.0, 1.0, 0.66340005, 0.27194428, 0.05397676,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_symmetric: HFloatArray<f32> = chebwin(9, 70., WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            0.03807165, 0.19412002, 0.5034264, 0.8467195, 1.0, 0.8467195, 0.5034264, 0.19412002,
            0.03807165,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> = chebwin(8, 70., WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            0.03807165, 0.19412002, 0.5034264, 0.8467195, 1.0, 0.8467195, 0.5034264, 0.19412002,
        ]);
        assert!(compare(v_periodic, rhs));

        let v_periodic: HFloatArray<f32> = chebwin(9, 70., WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            0.030656429,
            0.15511018,
            0.41670975,
            0.75451595,
            1.0,
            1.0,
            0.75451595,
            0.41670975,
            0.15511018,
        ]);
        assert!(compare(v_periodic, rhs));
    }

    #[test]
    fn exponential_test() {
        let v_symmetric: HResult<HFloatArray<f32>> =
            exponential(8, Some(1.0), 3.0, WindowType::Symmetric);
        assert!(v_symmetric.is_err());

        let v_symmetric: HFloatArray<f32> =
            exponential(8, None, 3.0, WindowType::Symmetric).unwrap();
        let rhs = HFloatArray::new_from_vec(vec![
            0.31140324, 0.4345982, 0.60653067, 0.84648174, 0.84648174, 0.60653067, 0.4345982,
            0.31140324,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> =
            exponential(8, Some(1.0), 3.0, WindowType::Periodic).unwrap();
        let rhs = HFloatArray::new_from_vec(vec![
            0.7165313, 1.0, 0.7165313, 0.5134171, 0.36787945, 0.26359713, 0.18887562, 0.13533528,
        ]);
        assert!(compare(v_periodic, rhs));

        let v_periodic: HFloatArray<f32> = exponential(8, None, 3.0, WindowType::Periodic).unwrap();
        let rhs = HFloatArray::new_from_vec(vec![
            0.26359713, 0.36787945, 0.5134171, 0.7165313, 1.0, 0.7165313, 0.5134171, 0.36787945,
        ]);
        assert!(compare(v_periodic, rhs));

        // test with center = 0.
        let v_periodic: HFloatArray<f32> =
            exponential(8, Some(0.0), 3.0, WindowType::Periodic).unwrap();
        let rhs = HFloatArray::new_from_vec(vec![
            1.0,
            0.7165313,
            0.5134171,
            0.36787945,
            0.26359713,
            0.18887562,
            0.13533528,
            0.096971974,
        ]);
        assert!(compare(v_periodic, rhs));

        // test with center = 0.
        let v_periodic: HFloatArray<f32> =
            exponential(9, Some(0.0), 3.0, WindowType::Periodic).unwrap();
        let rhs = HFloatArray::new_from_vec(vec![
            1.0,
            0.7165313,
            0.5134171,
            0.36787945,
            0.26359713,
            0.18887562,
            0.13533528,
            0.096971974,
            0.069483444,
        ]);
        assert!(compare(v_periodic, rhs));
    }

    #[test]
    fn cosine_test() {
        let v_symmetric: HFloatArray<f32> = cosine(8, WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            0.19509032, 0.55557024, 0.83146966, 0.9807853, 0.98078525, 0.83146954, 0.5555702,
            0.19509031,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> = cosine(8, WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            0.1736482, 0.5, 0.7660445, 0.9396927, 1.0, 0.9396926, 0.76604444, 0.49999982,
        ]);
        assert!(compare(v_periodic, rhs));
    }

    #[test]
    fn hann_test() {
        let v_symmetric: HFloatArray<f32> = hann(8, WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            0., 0.1882551, 0.6112605, 0.9504844, 0.9504844, 0.6112603, 0.18825516, 0.,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> = hann(8, WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            0., 0.14644662, 0.5, 0.8535534, 1., 0.8535533, 0.5, 0.1464465,
        ]);
        assert!(compare(v_periodic, rhs));
    }

    #[test]
    fn triangle_test() {
        let v_symmetric: HFloatArray<f32> = triangle(10, WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![0.1, 0.3, 0.5, 0.7, 0.9, 0.9, 0.7, 0.5, 0.3, 0.1]);
        assert!(compare(v_symmetric, rhs));

        let v_symmetric: HFloatArray<f32> = triangle(11, WindowType::Symmetric);
        let rhs = HFloatArray::new_from_vec(vec![
            0.16666667, 0.33333333, 0.5, 0.66666667, 0.83333333, 1., 0.83333333, 0.66666667, 0.5,
            0.33333333, 0.16666667,
        ]);
        assert!(compare(v_symmetric, rhs));

        let v_periodic: HFloatArray<f32> = triangle(10, WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            0.16666667, 0.33333333, 0.5, 0.66666667, 0.83333333, 1., 0.83333333, 0.66666667, 0.5,
            0.33333333,
        ]);
        assert!(compare(v_periodic, rhs));

        let v_periodic: HFloatArray<f32> = triangle(11, WindowType::Periodic);
        let rhs = HFloatArray::new_from_vec(vec![
            0.083333336,
            0.25,
            0.41666667,
            0.58333333,
            0.75,
            0.91666667,
            0.91666667,
            0.75,
            0.58333333,
            0.41666667,
            0.25,
        ]);
        assert!(compare(v_periodic, rhs));
    }
}
