use savvy::{r_println, savvy, OwnedLogicalSexp, Sexp};
use std::fmt;

/// HInterpolationType
/// A interpolation type representation.
///
/// Supports `Cubic`, `Linear`, `Quadratic` and `Nearest` types.
///
/// # Methods
///
#[derive(Clone, Debug, PartialEq)]
#[savvy]
pub enum HInterpolationType {
    Cubic,
    Linear,
    Quadratic,
    Nearest,
}

#[savvy]
impl HInterpolationType {
    /// HInterpolationType
    /// ## print
    ///
    /// `print()`
    ///
    /// Prints the `HInterpolationType`.
    ///
    /// Differently from R's behaviour, `print` doesn't return the value invisibly.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// interpolationtype = HInterpolationType$Cubic
    /// interpolationtype$print()
    ///
    /// # or similarly:
    /// print(interpolationtype)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        r_println!("{}", self);
        Ok(())
    }

    /// HInterpolationType
    /// ## eq
    ///
    /// `eq(other: HInterpolationType) -> bool`
    ///
    /// Equality with another `HInterpolationType`.
    ///
    /// #### Arguments
    ///
    /// - `other`
    ///
    /// An `HInterpolationType`.
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// interpolationtype1 = HInterpolationType$Cubic
    /// interpolationtype2 = HInterpolationType$Cubic
    /// interpolationtype1$eq(interpolationtype2) # TRUE
    ///
    /// # or similarly:
    /// interpolationtype1 == interpolationtype2
    /// ```
    ///
    /// _________
    ///
    fn eq(&self, other: &HInterpolationType) -> savvy::Result<Sexp> {
        let eq = std::cmp::PartialEq::eq(self, other);
        let logical_sexp: OwnedLogicalSexp = eq.try_into()?;
        logical_sexp.into()
    }

    /// HInterpolationType
    /// ## ne
    ///
    /// `ne(other: HInterpolationType) -> bool`
    ///
    /// Difference with another `HInterpolationType`.
    ///
    /// #### Arguments
    ///
    /// - `other`
    ///
    /// An `HInterpolationType`.
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// interpolationtype1 = HInterpolationType$Cubic
    /// interpolationtype2 = HInterpolationType$Cubic
    /// interpolationtype1$ne(interpolationtype2) # FALSE
    ///
    /// # or similarly:
    /// interpolationtype1 != interpolationtype2
    /// ```
    ///
    /// _________
    ///
    fn ne(&self, other: &HInterpolationType) -> savvy::Result<Sexp> {
        let ne = std::cmp::PartialEq::ne(self, other);
        let logical_sexp: OwnedLogicalSexp = ne.try_into()?;
        logical_sexp.into()
    }
}

impl fmt::Display for HInterpolationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HInterpolationType::Cubic => write!(f, "Cubic"),
            HInterpolationType::Linear => write!(f, "Linear"),
            HInterpolationType::Nearest => write!(f, "Nearest"),
            HInterpolationType::Quadratic => write!(f, "Quadratic"),
        }
    }
}
