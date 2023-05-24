use std::fmt;

use extendr_api::prelude::*;

/// HMetadataType
/// A metadata type representation. \
/// Supports `All`, `Text` and `Visual` types. \
///
/// # Methods
///
#[derive(PartialEq)]
pub enum HMetadataType {
    All,
    Text,
    Visual,
}

#[extendr]
impl HMetadataType {
    /// HMetadataType
    /// ## all
    ///
    /// `all -> HMetadataType` \
    ///
    /// Creates an All `HMetadataType`. \
    ///
    /// #### Returns
    ///
    /// An `HMetadataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// HMetadataType$all
    /// ```
    ///
    /// _________
    ///
    fn all() -> Self {
        Self::All
    }

    /// HMetadataType
    /// ## text
    ///
    /// `text -> HMetadataType` \
    ///
    /// Creates a Text `HMetadataType`. \
    ///
    /// #### Returns
    ///
    /// An `HMetadataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// HMetadataType$text
    /// ```
    ///
    /// _________
    ///
    fn text() -> Self {
        Self::Text
    }

    /// HMetadataType
    /// ## visual
    ///
    /// `visual -> HMetadataType` \
    ///
    /// Creates a Visual `HMetadataType`. \
    ///
    /// #### Returns
    ///
    /// An `HMetadataType`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// HMetadataType$visual
    /// ```
    ///
    /// _________
    ///
    fn visual() -> Self {
        Self::Visual
    }

    /// HMetadataType
    /// ## print
    ///
    /// `print()` \
    ///
    /// Prints the `HMetadataType`. \
    /// Differently from R's behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// metadatatype = HMetadataType$complex64
    /// metadatatype$print()
    ///
    /// # or similarly:
    /// print(metadatatype)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) {
        rprintln!("{}", self);
    }

    /// HMetadataType
    /// ## eq
    ///
    /// `eq(other: HMetadataType) -> bool` \
    ///
    /// Equality with another `HMetadataType`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HMetadataType`. \
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// metadatatype1 = HMetadataType$all
    /// metadatatype2 = HMetadataType$all
    /// metadatatype1$eq(metadatatype2) # TRUE
    ///
    /// # or similarly:
    /// metadatatype1 == metadatatype2
    /// ```
    ///
    /// _________
    ///
    fn eq(&self, other: &HMetadataType) -> bool {
        std::cmp::PartialEq::eq(self, other)
    }

    /// HMetadataType
    /// ## ne
    ///
    /// `ne(other: HMetadataType) -> bool` \
    ///
    /// Difference with another `HMetadataType`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HMetadataType`. \
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// metadatatype1 = HMetadataType$all
    /// metadatatype2 = HMetadataType$all
    /// metadatatype1$ne(metadatatype2) # FALSE
    ///
    /// # or similarly:
    /// metadatatype1 != metadatatype2
    /// ```
    ///
    /// _________
    ///
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
