use extendr_api::prelude::*;
use std::fmt;

#[derive(PartialEq)]
pub enum HResamplerType {
    FftFixedIn,
    FftFixedInOut,
    FftFixedOut,
    SincFixedIn,
    SincFixedOut,
}

#[extendr(use_try_from = true)]
impl HResamplerType {
    fn fft_fixed_in() -> Self {
        Self::FftFixedIn
    }
    fn fft_fixed_in_out() -> Self {
        Self::FftFixedInOut
    }
    fn fft_fixed_out() -> Self {
        Self::FftFixedOut
    }
    fn sinc_fixed_in() -> Self {
        Self::SincFixedIn
    }
    fn sinc_fixed_out() -> Self {
        Self::SincFixedOut
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    /// Equality.
    fn eq(&self, other: &HResamplerType) -> bool {
        std::cmp::PartialEq::eq(self, other)
    }

    /// Not equality.
    fn ne(&self, other: &HResamplerType) -> bool {
        std::cmp::PartialEq::ne(self, other)
    }
}

impl fmt::Display for HResamplerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HResamplerType::FftFixedIn => write!(f, "FftFixedIn")?,
            HResamplerType::FftFixedInOut => write!(f, "FftFixedInOut")?,
            HResamplerType::FftFixedOut => write!(f, "FftFixedOut")?,
            HResamplerType::SincFixedIn => write!(f, "SincFixedIn")?,
            HResamplerType::SincFixedOut => write!(f, "SincFixedOut")?,
        }
        Ok(())
    }
}

extendr_module! {
    mod hresamplertype;
    impl HResamplerType;
}
