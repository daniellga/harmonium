use extendr_api::prelude::*;
use rubato::{InterpolationParameters, InterpolationType, WindowFunction};

#[derive(Debug)]
pub struct HInterpolationParams {
    sinc_len: i32,
    f_cutoff: f64,
    oversampling_factor: i32,
    interpolation: String,
    window: String,
}

#[extendr]
impl HInterpolationParams {
    fn new(
        sinc_len: i32,
        f_cutoff: f64,
        oversampling_factor: i32,
        interpolation: String,
        window: String,
    ) -> HInterpolationParams {
        HInterpolationParams {
            sinc_len,
            f_cutoff,
            oversampling_factor,
            interpolation,
            window,
        }
    }

    fn print(&self) {
        rprintln!("{:?}", self);
    }
}

impl From<&HInterpolationParams> for InterpolationParameters {
    fn from(item: &HInterpolationParams) -> Self {
        let sinc_len = item.sinc_len.try_into().unwrap();
        let f_cutoff = item.f_cutoff as f32;
        let oversampling_factor = item.oversampling_factor.try_into().unwrap();
        let interpolation = match item.interpolation.as_str() {
            "cubic" => InterpolationType::Cubic,
            "linear" => InterpolationType::Linear,
            "nearest" => InterpolationType::Nearest,
            _ => panic!("Not a valid interpolation type."),
        };
        let window = match item.window.as_str() {
            "blackman" => WindowFunction::Blackman,
            "blackman2" => WindowFunction::Blackman2,
            "blackmanharris" => WindowFunction::BlackmanHarris,
            "blackmanharris2" => WindowFunction::BlackmanHarris2,
            "hann" => WindowFunction::Hann,
            "hann2" => WindowFunction::Hann2,
            _ => panic!("Not a valid window."),
        };

        InterpolationParameters {
            sinc_len,
            f_cutoff,
            oversampling_factor,
            interpolation,
            window,
        }
    }
}

extendr_module! {
    mod hinterpolationparams;
    impl HInterpolationParams;
}
