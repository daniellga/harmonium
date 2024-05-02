use std::fmt;

use savvy::{r_println, savvy, OwnedLogicalSexp, Sexp};

/// HMetadataType
/// A metadata type representation. \
/// Supports `All`, `Text` and `Visual` types. \
///
/// # Methods
///
#[derive(PartialEq)]
#[savvy]
pub enum HMetadataType {
    All,
    Text,
    Visual,
}

#[savvy]
impl HMetadataType {
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
    /// metadatatype = HMetadataType$All
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
    /// metadatatype1 = HMetadataType$All
    /// metadatatype2 = HMetadataType$All
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
    /// metadatatype1 = HMetadataType$All
    /// metadatatype2 = HMetadataType$All
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
            HMetadataType::All => write!(f, "All"),
            HMetadataType::Text => write!(f, "Text"),
            HMetadataType::Visual => write!(f, "Visual"),
        }
    }
}
