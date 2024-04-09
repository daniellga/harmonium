use harmonium_core::errors::HError;

#[derive(Debug)]
pub struct HErrorR(HError);

impl From<HError> for HErrorR {
    fn from(error: HError) -> Self {
        Self(error)
    }
}

impl From<HErrorR> for savvy::Error {
    fn from(err: HErrorR) -> Self {
        Self::GeneralError(err.0.to_string())
    }
}
