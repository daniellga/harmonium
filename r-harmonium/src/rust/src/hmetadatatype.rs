use std::fmt;

use savvy::{r_println, savvy, OwnedLogicalSexp, Sexp};

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

#[savvy]
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
    fn print(&self) -> savvy::Result<()> {
        r_println!("{}", self);
        Ok(())
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
    fn eq(&self, other: &HMetadataType) -> savvy::Result<Sexp> {
        let eq = std::cmp::PartialEq::eq(self, other);
        let logical_sexp: OwnedLogicalSexp = eq.try_into()?;
        logical_sexp.into()
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
    fn ne(&self, other: &HMetadataType) -> savvy::Result<Sexp> {
        let ne = std::cmp::PartialEq::ne(self, other);
        let logical_sexp: OwnedLogicalSexp = ne.try_into()?;
        logical_sexp.into()
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
