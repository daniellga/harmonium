use extendr_api::prelude::*;
use std::fmt;

#[derive(PartialEq)]
pub enum HDataType {
    Float32,
    Float64,
    Complex32,
    Complex64,
}

#[extendr(use_try_from = true)]
impl HDataType {
    fn float32() -> Self {
        Self::Float32
    }
    fn float64() -> Self {
        Self::Float64
    }
    fn complex32() -> Self {
        Self::Complex32
    }
    fn complex64() -> Self {
        Self::Complex64
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    /// Equality.
    fn eq(&self, other: &HDataType) -> bool {
        std::cmp::PartialEq::eq(self, other)
    }

    /// Not equality.
    fn ne(&self, other: &HDataType) -> bool {
        std::cmp::PartialEq::ne(self, other)
    }

    pub fn all_hdatatype() -> Vec<String> {
        vec![
            "Float32".into(),
            "Float64".into(),
            "Complex32".into(),
            "Complex64".into(),
        ]
    }
}

impl fmt::Display for HDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HDataType::Float32 => write!(f, "Float32")?,
            HDataType::Float64 => write!(f, "Float64")?,
            HDataType::Complex32 => write!(f, "Complex32")?,
            HDataType::Complex64 => write!(f, "Complex64")?,
        }
        Ok(())
    }
}

extendr_module! {
    mod datatype;
    impl HDataType;
}
