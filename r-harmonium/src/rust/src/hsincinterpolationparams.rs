use rubato::{SincInterpolationParameters, SincInterpolationType, WindowFunction};
use savvy::{r_println, savvy, Sexp};

use crate::conversions::Conversions;

#[derive(Debug)]
#[savvy]
pub struct HSincInterpolationParameters {
    sinc_len: i32,
    f_cutoff: f64,
    oversampling_factor: i32,
    interpolation: &'static str,
    window: &'static str,
}

#[savvy]
impl HSincInterpolationParameters {
    fn new(
        sinc_len: Sexp,
        f_cutoff: Sexp,
        oversampling_factor: Sexp,
        interpolation: Sexp,
        window: Sexp,
    ) -> savvy::Result<HSincInterpolationParameters> {
        let sinc_len: i32 = sinc_len.to_scalar()?;
        let f_cutoff: f64 = f_cutoff.to_scalar()?;
        let oversampling_factor: i32 = oversampling_factor.to_scalar()?;
        let interpolation: &str = interpolation.to_scalar()?;
        let window: &str = window.to_scalar()?;

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

impl From<&HSincInterpolationParameters> for SincInterpolationParameters {
    fn from(item: &HSincInterpolationParameters) -> Self {
        let sinc_len = item.sinc_len.try_into().unwrap();
        let f_cutoff = item.f_cutoff as f32;
        let oversampling_factor = item.oversampling_factor.try_into().unwrap();
        let interpolation = match item.interpolation {
            "cubic" => SincInterpolationType::Cubic,
            "linear" => SincInterpolationType::Linear,
            "nearest" => SincInterpolationType::Nearest,
            _ => panic!("Not a valid interpolation type."),
        };
        let window = match item.window {
            "blackman" => WindowFunction::Blackman,
            "blackman2" => WindowFunction::Blackman2,
            "blackmanharris" => WindowFunction::BlackmanHarris,
            "blackmanharris2" => WindowFunction::BlackmanHarris2,
            "hann" => WindowFunction::Hann,
            "hann2" => WindowFunction::Hann2,
            _ => panic!("Not a valid window."),
        };

        SincInterpolationParameters {
            sinc_len,
            f_cutoff,
            oversampling_factor,
            interpolation,
            window,
        }
    }
}
