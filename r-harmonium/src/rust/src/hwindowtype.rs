use savvy::{r_println, savvy, OwnedLogicalSexp, Sexp};
use std::fmt;

/// HWindowType
/// A window type representation. \
/// Supports `Blackman`, `Blackman2`, `BlackmanHarris`, `BlackmanHarris2`, `Hann` and `Hann2` types. \
///
/// # Methods
///
#[derive(Clone, Debug, PartialEq)]
#[savvy]
pub enum HWindowType {
    Blackman,
    Blackman2,
    BlackmanHarris,
    BlackmanHarris2,
    Hann,
    Hann2,
}

#[savvy]
impl HWindowType {
    /// HWindowType
    /// ## print
    ///
    /// `print()` \
    ///
    /// Prints the `HWindowType`. \
    /// Differently from R's behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// windowtype = HWindowType$Blackman
    /// windowtype$print()
    ///
    /// # or similarly:
    /// print(windowtype)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        r_println!("{}", self);
        Ok(())
    }

    /// HWindowType
    /// ## eq
    ///
    /// `eq(other: HWindowType) -> bool` \
    ///
    /// Equality with another `HWindowType`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HWindowType`. \
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// windowtype1 = HWindowType$Blackman
    /// windowtype2 = HWindowType$Blackman
    /// windowtype1$eq(windowtype2) # TRUE
    ///
    /// # or similarly:
    /// windowtype1 == windowtype2
    /// ```
    ///
    /// _________
    ///
    fn eq(&self, other: &HWindowType) -> savvy::Result<Sexp> {
        let eq = std::cmp::PartialEq::eq(self, other);
        let logical_sexp: OwnedLogicalSexp = eq.try_into()?;
        logical_sexp.into()
    }

    /// HWindowType
    /// ## ne
    ///
    /// `ne(other: HWindowType) -> bool` \
    ///
    /// Difference with another `HWindowType`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HWindowType`. \
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// windowtype1 = HWindowType$Blackman
    /// windowtype2 = HWindowType$Blackman
    /// windowtype1$ne(windowtype2) # FALSE
    ///
    /// # or similarly:
    /// windowtype1 != windowtype2
    /// ```
    ///
    /// _________
    ///
    fn ne(&self, other: &HWindowType) -> savvy::Result<Sexp> {
        let ne = std::cmp::PartialEq::ne(self, other);
        let logical_sexp: OwnedLogicalSexp = ne.try_into()?;
        logical_sexp.into()
    }
}

impl fmt::Display for HWindowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HWindowType::Blackman => write!(f, "Blackman"),
            HWindowType::Blackman2 => write!(f, "Blackman2"),
            HWindowType::BlackmanHarris => write!(f, "BlackmanHarris"),
            HWindowType::BlackmanHarris2 => write!(f, "BlackmanHarris2"),
            HWindowType::Hann => write!(f, "Hann"),
            HWindowType::Hann2 => write!(f, "Hann2"),
        }
    }
}
