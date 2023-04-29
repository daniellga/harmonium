use std::{error, fmt, io};

#[allow(clippy::enum_variant_names)]
pub enum HError {
    /// IO error.
    IoError(String),
    /// The stream contained malformed data and could not be decoded or demuxed.
    DecodeError(String),
    /// The stream could not be seeked.
    SeekError(String),
    /// A default or user-defined limit was reached while decoding or demuxing the stream. Limits
    /// are used to prevent denial-of-service attacks from malicious streams.
    LimitError(String),
    /// Resampling errors.
    ResampleError(String),
    /// Arguments out of specification.
    OutOfSpecError(String),
    /// Errors related to playing audio.
    PlayError(String),
    /// Other types of error.
    OtherError(String),
}

impl fmt::Display for HError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HError::IoError(msg) => write!(f, "IoError: {}", msg),
            HError::DecodeError(msg) => write!(f, "DecodeError: {}", msg),
            HError::SeekError(msg) => write!(f, "SeekError: {}", msg),
            HError::LimitError(msg) => write!(f, "LimitError: {}", msg),
            HError::ResampleError(msg) => write!(f, "ResampleError: {}", msg),
            HError::OutOfSpecError(msg) => write!(f, "OutOfSpecError: {}", msg),
            HError::PlayError(msg) => write!(f, "PlayError: {}", msg),
            HError::OtherError(msg) => write!(f, "OtherError: {}", msg),
        }
    }
}

impl fmt::Debug for HError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self)
    }
}

impl error::Error for HError {}

impl From<io::Error> for HError {
    fn from(err: io::Error) -> Self {
        HError::IoError(err.to_string())
    }
}

impl From<symphonia::core::errors::Error> for HError {
    fn from(err: symphonia::core::errors::Error) -> Self {
        match err {
            symphonia::core::errors::Error::IoError(_) => HError::IoError(err.to_string()),
            symphonia::core::errors::Error::DecodeError(_) => HError::DecodeError(err.to_string()),
            symphonia::core::errors::Error::SeekError(_) => HError::SeekError(err.to_string()),
            symphonia::core::errors::Error::Unsupported(_) => HError::IoError(err.to_string()),
            symphonia::core::errors::Error::LimitError(_) => HError::LimitError(err.to_string()),
            symphonia::core::errors::Error::ResetRequired => HError::OtherError(err.to_string()),
        }
    }
}

impl From<rubato::ResampleError> for HError {
    fn from(err: rubato::ResampleError) -> Self {
        HError::ResampleError(err.to_string())
    }
}

impl From<rubato::ResamplerConstructionError> for HError {
    fn from(err: rubato::ResamplerConstructionError) -> Self {
        HError::ResampleError(err.to_string())
    }
}

impl From<arrow2::error::Error> for HError {
    fn from(err: arrow2::error::Error) -> Self {
        match err {
            arrow2::error::Error::OutOfSpec(_) => HError::OutOfSpecError(err.to_string()),
            _ => HError::OtherError(err.to_string()),
        }
    }
}

impl From<rodio::StreamError> for HError {
    fn from(err: rodio::StreamError) -> Self {
        HError::PlayError(err.to_string())
    }
}

impl From<rodio::PlayError> for HError {
    fn from(err: rodio::PlayError) -> Self {
        HError::PlayError(err.to_string())
    }
}

impl From<rodio::DevicesError> for HError {
    fn from(err: rodio::DevicesError) -> Self {
        HError::PlayError(err.to_string())
    }
}

impl From<rodio::cpal::DeviceNameError> for HError {
    fn from(err: rodio::cpal::DeviceNameError) -> Self {
        HError::PlayError(err.to_string())
    }
}

impl From<rodio::cpal::SupportedStreamConfigsError> for HError {
    fn from(err: rodio::cpal::SupportedStreamConfigsError) -> Self {
        HError::PlayError(err.to_string())
    }
}

pub type HResult<T> = Result<T, HError>;
