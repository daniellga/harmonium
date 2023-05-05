use std::fmt;

use extendr_api::prelude::*;

#[derive(PartialEq)]
pub enum HMetadataType {
    All,
    Text,
    Visual,
}

#[extendr]
impl HMetadataType {
    fn all() -> Self {
        Self::All
    }
    fn text() -> Self {
        Self::Text
    }
    fn visual() -> Self {
        Self::Visual
    }

    fn print(&self) {
        rprintln!("{}", self);
    }

    /// Equality.
    fn eq(&self, other: &HMetadataType) -> bool {
        std::cmp::PartialEq::eq(self, other)
    }

    /// Not equality.
    fn ne(&self, other: &HMetadataType) -> bool {
        std::cmp::PartialEq::ne(self, other)
    }
}

impl fmt::Display for HMetadataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HMetadataType::All => write!(f, "All")?,
            HMetadataType::Text => write!(f, "Text")?,
            HMetadataType::Visual => write!(f, "Visual")?,
        }
        Ok(())
    }
}

extendr_module! {
    mod hmetadatatype;
    impl HMetadataType;
}
