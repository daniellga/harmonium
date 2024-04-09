use harmonium_core::errors::HError;

#[derive(Debug)]
pub struct HErrorR(HError);

impl From<HError> for HErrorR {
    fn from(error: HError) -> Self {
        Self(error)
    }
}

impl From<HErrorR> for savvy::Error {
    fn from(error: HErrorR) -> Self {
        match error.0 {
            HError::IoError(err) => Self::new(err.as_str()),
            HError::DecodeError(err) => Self::new(err.as_str()),
            HError::SeekError(err) => Self::new(err.as_str()),
            HError::LimitError(err) => Self::new(err.as_str()),
            HError::ResampleError(err) => Self::new(err.as_str()),
            HError::OutOfSpecError(err) => Self::new(err.as_str()),
            HError::PlayError(err) => Self::new(err.as_str()),
            HError::OtherError(err) => Self::new(err.as_str()),
        }
    }
}
