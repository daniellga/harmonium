use rubato::PolynomialDegree;
use savvy::{r_println, savvy, OwnedLogicalSexp, Sexp};
use std::fmt;

/// HPolynomialDegree
/// Degree of the polynomial used for interpolation. A higher degree gives a higher quality result, while taking longer to compute. \
///
/// * `Septic` \
/// Septic polynomial, fitted using 8 sample points. \
///
/// * `Quintic`. \
/// Quintic polynomial, fitted using 6 sample points. \
///
/// * `Cubic`. \
/// Cubic polynomial, fitted using 4 sample points. \
///
/// * `Linear`. \
/// Linear polynomial, fitted using 2 sample points. \
///
/// * `Nearest`. \
/// Nearest, uses the nearest sample point without any fitting. \
///
/// # Methods
///
#[derive(PartialEq)]
#[savvy]
pub enum HPolynomialDegree {
    Septic,
    Quintic,
    Cubic,
    Linear,
    Nearest,
}

#[savvy]
impl HPolynomialDegree {
    /// HPolynomialDegree
    /// ## Septic
    ///
    /// `septic -> HPolynomialDegree` \
    ///
    /// Creates a `Septic` `HPolynomialDegree`. \
    ///
    /// #### Returns
    ///
    /// An `HPolynomialDegree`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// pol_deg = HPolynomialDegree$septic
    /// ```
    ///
    /// _________
    ///
    fn septic() -> Self {
        Self::Septic
    }

    /// HPolynomialDegree
    /// ## Quintic
    ///
    /// `quintic -> HPolynomialDegree` \
    ///
    /// Creates a `Quintic` `HPolynomialDegree`. \
    ///
    /// #### Returns
    ///
    /// An `HPolynomialDegree`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// pol_deg = HPolynomialDegree$quintic
    /// ```
    ///
    /// _________
    ///
    fn quintic() -> Self {
        Self::Quintic
    }

    /// HPolynomialDegree
    /// ## Cubic
    ///
    /// `cubic -> HPolynomialDegree` \
    ///
    /// Creates a `Cubic` `HPolynomialDegree`. \
    ///
    /// #### Returns
    ///
    /// An `HPolynomialDegree`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// pol_deg = HPolynomialDegree$cubic
    /// ```
    ///
    /// _________
    ///
    fn cubic() -> Self {
        Self::Cubic
    }

    /// HPolynomialDegree
    /// ## Linear
    ///
    /// `linear -> HPolynomialDegree` \
    ///
    /// Creates a `Linear` `HPolynomialDegree`. \
    ///
    /// #### Returns
    ///
    /// An `HPolynomialDegree`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// pol_deg = HPolynomialDegree$linear
    /// ```
    ///
    /// _________
    ///
    fn linear() -> Self {
        Self::Linear
    }

    /// HPolynomialDegree
    /// ## Nearest
    ///
    /// `nearest -> HPolynomialDegree` \
    ///
    /// Creates a `Nearest` `HPolynomialDegree`. \
    ///
    /// #### Returns
    ///
    /// An `HPolynomialDegree`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// pol_deg = HPolynomialDegree$nearest
    /// ```
    ///
    /// _________
    ///
    fn nearest() -> Self {
        Self::Nearest
    }

    /// HPolynomialDegree
    /// ## print
    ///
    /// `print()` \
    ///
    /// Prints the `HPolynomialDegree`. \
    /// Differently from R's behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// pol_deg = HPolynomialDegree$complex64
    /// pol_deg$print()
    ///
    /// # or similarly:
    /// print(pol_deg)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        r_println!("{}", self);
        Ok(())
    }

    /// HPolynomialDegree
    /// ## eq
    ///
    /// `eq(other: HPolynomialDegree) -> bool` \
    ///
    /// Equality with another `HPolynomialDegree`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HPolynomialDegree`. \
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// pol_deg1 = HPolynomialDegree$quintic
    /// pol_deg2 = HPolynomialDegree$quintic
    /// pol_deg1$eq(pol_deg2) # TRUE
    ///
    /// # or similarly:
    /// pol_deg1 == pol_deg2
    /// ```
    ///
    /// _________
    ///
    fn eq(&self, other: &HPolynomialDegree) -> savvy::Result<Sexp> {
        let eq = std::cmp::PartialEq::eq(self, other);
        let logical_sexp: OwnedLogicalSexp = eq.try_into()?;
        logical_sexp.into()
    }

    /// HPolynomialDegree
    /// ## ne
    ///
    /// `ne(other: HPolynomialDegree) -> bool` \
    ///
    /// Difference with another `HPolynomialDegree`.
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HPolynomialDegree`. \
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// pol_deg1 = HPolynomialDegree$quintic
    /// pol_deg2 = HPolynomialDegree$nearest
    /// pol_deg1$ne(pol_deg2) # TRUE
    ///
    /// # or similarly:
    /// pol_deg1 != pol_deg2
    /// ```
    ///
    /// _________
    ///
    fn ne(&self, other: &HPolynomialDegree) -> savvy::Result<Sexp> {
        let ne = std::cmp::PartialEq::ne(self, other);
        let logical_sexp: OwnedLogicalSexp = ne.try_into()?;
        logical_sexp.into()
    }
}

impl fmt::Display for HPolynomialDegree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HPolynomialDegree::Septic => write!(f, "Septic")?,
            HPolynomialDegree::Quintic => write!(f, "Quintic")?,
            HPolynomialDegree::Cubic => write!(f, "Cubic")?,
            HPolynomialDegree::Linear => write!(f, "Linear")?,
            HPolynomialDegree::Nearest => write!(f, "Nearest")?,
        }
        Ok(())
    }
}

impl From<&HPolynomialDegree> for PolynomialDegree {
    fn from(item: &HPolynomialDegree) -> Self {
        match item {
            HPolynomialDegree::Septic => PolynomialDegree::Septic,
            HPolynomialDegree::Quintic => PolynomialDegree::Quintic,
            HPolynomialDegree::Cubic => PolynomialDegree::Cubic,
            HPolynomialDegree::Linear => PolynomialDegree::Linear,
            HPolynomialDegree::Nearest => PolynomialDegree::Nearest,
        }
    }
}
