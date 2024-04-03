use rubato::{SincInterpolationParameters, SincInterpolationType, WindowFunction};
use savvy::{r_println, savvy, Sexp, TypedSexp};

#[derive(Debug)]
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
        let sinc_len: i32 = match sinc_len.into_typed() {
            TypedSexp::Integer(integer_sexp) if integer_sexp.len() == 1 => {
                integer_sexp.as_slice()[0]
            }
            _ => panic!("sinc_len must be an integer of length 1."),
        };

        let f_cutoff: f64 = match f_cutoff.into_typed() {
            TypedSexp::Real(real_sexp) if real_sexp.len() == 1 => real_sexp.as_slice()[0],
            _ => panic!("f_cutoff must be a double of length 1."),
        };

        let oversampling_factor: i32 = match oversampling_factor.into_typed() {
            TypedSexp::Integer(integer_sexp) if integer_sexp.len() == 1 => {
                integer_sexp.as_slice()[0]
            }
            _ => panic!("oversampling_factor must be an integer of length 1."),
        };

        let interpolation = match interpolation.into_typed() {
            TypedSexp::String(string_sexp) if string_sexp.len() == 1 => {
                // Ok to unwrap since the size was checked.
                string_sexp.iter().next().unwrap()
            }
            _ => panic!("interpolation must be a string of length 1."),
        };

        let window = match window.into_typed() {
            TypedSexp::String(string_sexp) if string_sexp.len() == 1 => {
                // Ok to unwrap since the size was checked.
                string_sexp.iter().next().unwrap()
            }
            _ => panic!("window must be a string of length 1."),
        };

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
