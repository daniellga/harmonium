use std::{error, fmt, io};

#[allow(clippy::enum_variant_names)]
pub enum OwnError {
    /// IO error.
    IoError(String),
    /// The stream contained malformed data and could not be decoded or demuxed.
    DecodeError(String),
    /// The stream could not be seeked.
    SeekError(String),
    /// A default or user-defined limit was reached while decoding or demuxing the stream. Limits
    /// are used to prevent denial-of-service attacks from malicious streams.
    LimitError(String),
    /// Resampling errors
    ResampleError(String),
    /// Other types of error.
    OtherError(String),
}

impl fmt::Display for OwnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OwnError::IoError(msg) => write!(f, "io_error: {}", msg),
            OwnError::DecodeError(msg) => write!(f, "decode_error: {}", msg),
            OwnError::SeekError(msg) => write!(f, "seek_error: {}", msg),
            OwnError::LimitError(msg) => write!(f, "limit_error: {}", msg),
            OwnError::ResampleError(msg) => write!(f, "resample_error: {}", msg),
            OwnError::OtherError(msg) => write!(f, "other_error: {}", msg),
        }
    }
}

impl fmt::Debug for OwnError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self)
    }
}

impl error::Error for OwnError {}

impl From<io::Error> for OwnError {
    fn from(err: io::Error) -> Self {
        OwnError::IoError(err.to_string())
    }
}

impl From<symphonia::core::errors::Error> for OwnError {
    fn from(err: symphonia::core::errors::Error) -> Self {
        match err {
            symphonia::core::errors::Error::IoError(_) => OwnError::IoError(err.to_string()),
            symphonia::core::errors::Error::DecodeError(_) => {
                OwnError::DecodeError(err.to_string())
            }
            symphonia::core::errors::Error::SeekError(_) => OwnError::SeekError(err.to_string()),
            symphonia::core::errors::Error::Unsupported(_) => OwnError::IoError(err.to_string()),
            symphonia::core::errors::Error::LimitError(_) => OwnError::LimitError(err.to_string()),
            symphonia::core::errors::Error::ResetRequired => OwnError::OtherError(err.to_string()),
        }
    }
}

impl From<rubato::ResampleError> for OwnError {
    fn from(err: rubato::ResampleError) -> Self {
        OwnError::ResampleError(err.to_string())
    }
}

impl From<rubato::ResamplerConstructionError> for OwnError {
    fn from(err: rubato::ResamplerConstructionError) -> Self {
        OwnError::ResampleError(err.to_string())
    }
}

pub type OwnResult<T> = Result<T, OwnError>;
