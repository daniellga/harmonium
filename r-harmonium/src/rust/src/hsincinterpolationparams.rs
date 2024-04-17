use crate::{
    conversions::Conversions, hinterpolationtype::HInterpolationType, hwindowtype::HWindowType,
};
use rubato::{SincInterpolationParameters, SincInterpolationType, WindowFunction};
use savvy::{r_println, savvy, Sexp};

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
    fn new(
        sinc_len: Sexp,
        f_cutoff: Sexp,
        oversampling_factor: Sexp,
        interpolation: &HInterpolationType,
        window: &HWindowType,
    ) -> savvy::Result<HSincInterpolationParameters> {
        let sinc_len: i32 = sinc_len.to_scalar()?;
        let f_cutoff: f64 = f_cutoff.to_scalar()?;
        let oversampling_factor: i32 = oversampling_factor.to_scalar()?;
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
