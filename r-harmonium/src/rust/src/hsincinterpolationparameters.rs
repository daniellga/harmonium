use crate::{
    conversions::AsScalar, hinterpolationtype::HInterpolationType, hwindowtype::HWindowType,
};
use rubato::{SincInterpolationParameters, SincInterpolationType, WindowFunction};
use savvy::{r_println, savvy, Sexp};

/// HSincInterpolationParameters
/// Parameters to be used for sinc interpolation. \
///
/// # Methods
///
#[derive(Debug)]
#[savvy]
pub struct HSincInterpolationParameters {
    sinc_len: i32,
    f_cutoff: f64,
    oversampling_factor: i32,
    interpolation: HInterpolationType,
    window: HWindowType,
}

#[savvy]
impl HSincInterpolationParameters {
    /// HSincInterpolationParameters
    /// ## new
    ///
    /// `new(sinc_len: integer, f_cutoff: double, oversampling_factor: integer, interpolation: HInterpolationType, window: HWindowType) -> HSincInterpolationParameters` \
    ///
    /// Creates a new `HSincInterpolationParameters`. \
    ///
    /// #### Arguments
    ///
    /// * `sinc_len` \
    /// An integer. Length of the windowed sinc interpolation filter. Higher values can allow a higher cut-off frequency leading to less high frequency
    /// roll-off at the expense of higher cpu usage. A good starting point should be 256. The value will be rounded up to the nearest multiple of 8. \
    /// * `f_cutoff` \
    /// A double. Relative cutoff frequency of the sinc interpolation filter (relative to the lowest one of `fs_in/2` or `fs_out/2`). Start at 0.95, and
    /// increase if needed. \
    /// * `oversampling_factor` \
    /// An integer. The number of intermediate points to use for interpolation. Higher values use more memory for storing the sinc filters. Only the points actually needed
    /// are calculated during processing so a larger number does not directly lead to higher cpu usage. A lower value helps in keeping the sincs in the cpu
    /// cache. A good starting point should be 128. \
    /// * `interpolation` \
    /// An `HInterpolationType`. The interpolation type. \
    /// * `window` \
    /// An `HWindowType`. The window function to use. \
    ///
    /// #### Returns
    ///
    /// An `HSincInterpolationParameters`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// sinc_len = 256L
    /// f_cutoff = 0.95
    /// oversampling_factor = 128L
    /// interpolation = HInterpolationType$Linear
    /// window = HWindowType$Blackman
    ///
    /// hsincinterpolationparameters = HSincInterpolationParameters$new(sinc_len, f_cutoff, oversampling_factor, interpolation, window)
    /// ```
    ///
    /// _________
    ///
    fn new(
        sinc_len: Sexp,
        f_cutoff: Sexp,
        oversampling_factor: Sexp,
        interpolation: &HInterpolationType,
        window: &HWindowType,
    ) -> savvy::Result<HSincInterpolationParameters> {
        let sinc_len: i32 = sinc_len.as_scalar()?;
        let f_cutoff: f64 = f_cutoff.as_scalar()?;
        let oversampling_factor: i32 = oversampling_factor.as_scalar()?;
        let interpolation = interpolation.clone();
        let window = window.clone();

        Ok(HSincInterpolationParameters {
            sinc_len,
            f_cutoff,
            oversampling_factor,
            interpolation,
            window,
        })
    }

    /// HSincInterpolationParameters
    /// ## print
    ///
    /// `print()` \
    ///
    /// Prints the `HSincInterpolationParameters`. \
    /// Differently from R's behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// sinc_len = 256L
    /// f_cutoff = 0.95
    /// oversampling_factor = 128L
    /// interpolation = HInterpolationType$Linear
    /// window = HWindowType$Blackman
    ///
    /// hsincinterpolationparameters = HSincInterpolationParameters$new(sinc_len, f_cutoff, oversampling_factor, interpolation, window)
    /// hsincinterpolationparameters$print()
    ///
    /// # or similarly:
    /// print(hsincinterpolationparameters)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        r_println!("{:?}", self);
        Ok(())
    }
}

impl TryFrom<&HSincInterpolationParameters> for SincInterpolationParameters {
    type Error = savvy::Error;

    fn try_from(item: &HSincInterpolationParameters) -> savvy::Result<Self> {
        let sinc_len = item
            .sinc_len
            .try_into()
            .map_err(|_| "Cannot convert from i32 to usize")?;
        let f_cutoff = item.f_cutoff as f32;
        let oversampling_factor = item
            .oversampling_factor
            .try_into()
            .map_err(|_| "Cannot convert from i32 to usize")?;
        let interpolation = match item.interpolation {
            HInterpolationType::Cubic => SincInterpolationType::Cubic,
            HInterpolationType::Linear => SincInterpolationType::Linear,
            HInterpolationType::Nearest => SincInterpolationType::Nearest,
            HInterpolationType::Quadratic => SincInterpolationType::Quadratic,
        };
        let window = match item.window {
            HWindowType::Blackman => WindowFunction::Blackman,
            HWindowType::Blackman2 => WindowFunction::Blackman2,
            HWindowType::BlackmanHarris => WindowFunction::BlackmanHarris,
            HWindowType::BlackmanHarris2 => WindowFunction::BlackmanHarris2,
            HWindowType::Hann => WindowFunction::Hann,
            HWindowType::Hann2 => WindowFunction::Hann2,
        };

        Ok(SincInterpolationParameters {
            sinc_len,
            f_cutoff,
            oversampling_factor,
            interpolation,
            window,
        })
    }
}
