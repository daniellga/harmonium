use savvy::{r_println, savvy, OwnedLogicalSexp, Sexp};
use std::fmt;

/// HResamplerType
/// A resampler type representation. \
/// Supports `FftFixedIn`, `FftFixedInOut`, `FftFixedOut`, `SincFixedIn`, `SincFixedOut`,
/// `FastFixedIn` and `FastFixedOut` types. \
///
/// # Methods
///
#[derive(PartialEq)]
#[savvy]
pub enum HResamplerType {
    FftFixedIn,
    FftFixedInOut,
    FftFixedOut,
    SincFixedIn,
    SincFixedOut,
    FastFixedIn,
    FastFixedOut,
}

#[savvy]
impl HResamplerType {
    /// HResamplerType
    /// ## print
    ///
    /// `print()` \
    ///
    /// Prints the `HResamplerType`. \
    /// Differently from R's behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype = HResamplerType$SincFixedIn
    /// hresamplertype$print()
    ///
    /// # or similarly:
    /// print(hresamplertype)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        r_println!("{}", self);
        Ok(())
    }

    /// HResamplerType
    /// ## eq
    ///
    /// `eq(other: HResamplerType) -> bool` \
    ///
    /// Equality with another `HResamplerType`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HResamplerType`. \
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype1 = HResamplerType$SincFixedIn
    /// hresamplertype2 = HResamplerType$SincFixedIn
    /// hresamplertype1$eq(hresamplertype2) # TRUE
    ///
    /// # or similarly:
    /// hresamplertype1 == hresamplertype2
    /// ```
    ///
    /// _________
    ///
    fn eq(&self, other: &HResamplerType) -> savvy::Result<Sexp> {
        let eq = std::cmp::PartialEq::eq(self, other);
        let logical_sexp: OwnedLogicalSexp = eq.try_into()?;
        logical_sexp.into()
    }

    /// HResamplerType
    /// ## ne
    ///
    /// `ne(other: HResamplerType) -> bool` \
    ///
    /// Difference with another `HResamplerType`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HResamplerType`. \
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype1 = HResamplerType$SincFixedIn
    /// hresamplertype2 = HResamplerType$SincFixedIn
    /// hresamplertype1$ne(hresamplertype2) # FALSE
    ///
    /// # or similarly:
    /// hresamplertype1 != hresamplertype2
    /// ```
    ///
    /// _________
    ///
    fn ne(&self, other: &HResamplerType) -> savvy::Result<Sexp> {
        let ne = std::cmp::PartialEq::ne(self, other);
        let logical_sexp: OwnedLogicalSexp = ne.try_into()?;
        logical_sexp.into()
    }
}

impl fmt::Display for HResamplerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HResamplerType::FftFixedIn => write!(f, "FftFixedIn"),
            HResamplerType::FftFixedInOut => write!(f, "FftFixedInOut"),
            HResamplerType::FftFixedOut => write!(f, "FftFixedOut"),
            HResamplerType::SincFixedIn => write!(f, "SincFixedIn"),
            HResamplerType::SincFixedOut => write!(f, "SincFixedOut"),
            HResamplerType::FastFixedIn => write!(f, "FastFixedIn"),
            HResamplerType::FastFixedOut => write!(f, "FastFixedOut"),
        }
    }
}
