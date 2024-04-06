use savvy::{r_println, savvy, OwnedLogicalSexp, Sexp};
use std::fmt;

/// HDataType
/// A type representation. \
/// Supports `Float32`, `Float64`, `Complex32` and `Complex64` types. \
///
/// # Methods
///
#[derive(PartialEq)]
#[savvy]
pub enum HDataType {
    Float32,
    Float64,
    Complex32,
    Complex64,
}

#[savvy]
impl HDataType {
    /// HDataType
    /// ## float32
    ///
    /// `float32 -> HDataType` \
    ///
    /// Creates a Float32 `HDataType`. \
    ///
    /// #### Returns
    ///
    /// An `HDataType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// dtype = HDataType$float32
    /// ```
    ///
    /// _________
    ///
    fn float32() -> Self {
        Self::Float32
    }

    /// HDataType
    /// ## float64
    ///
    /// `float64 -> HDataType` \
    ///
    /// Creates a Float64 `HDataType`. \
    ///
    /// #### Returns
    ///
    /// An `HDataType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// dtype = HDataType$float64
    /// ```
    ///
    /// _________
    ///
    fn float64() -> Self {
        Self::Float64
    }

    /// HDataType
    /// ## complex32
    ///
    /// `complex32 -> HDataType` \
    ///
    /// Creates a Complex32 `HDataType`. \
    ///
    /// #### Returns
    ///
    /// An `HDataType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// dtype = HDataType$complex32
    /// ```
    ///
    /// _________
    ///
    fn complex32() -> Self {
        Self::Complex32
    }

    /// HDataType
    /// ## complex64
    ///
    /// `complex64 -> HDataType` \
    ///
    /// Creates a Complex64 `HDataType`. \
    ///
    /// #### Returns
    ///
    /// An `HDataType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// dtype = HDataType$complex64
    /// ```
    ///
    /// _________
    ///
    fn complex64() -> Self {
        Self::Complex64
    }

    /// HDataType
    /// ## print
    ///
    /// `print()` \
    ///
    /// Prints the `HDataType`. \
    /// Differently from R's behaviour, `print` doesn't return the value invisibly. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// dtype = HDataType$complex64
    /// dtype$print()
    ///
    /// # or similarly:
    /// print(dtype)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) -> savvy::Result<()> {
        r_println!("{}", self);
        Ok(())
    }

    /// HDataType
    /// ## eq
    ///
    /// `eq(other: HDataType) -> bool` \
    ///
    /// Equality with another `HDataType`. \
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HDataType`. \
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// hdatatype1 = HDataType$float32
    /// hdatatype2 = HDataType$float32
    /// hdatatype1$eq(hdatatype2) # TRUE
    ///
    /// # or similarly:
    /// hdatatype1 == hdatatype2
    /// ```
    ///
    /// _________
    ///
    fn eq(&self, other: &HDataType) -> savvy::Result<Sexp> {
        let eq = std::cmp::PartialEq::eq(self, other);
        let logical_sexp: OwnedLogicalSexp = eq.try_into()?;
        logical_sexp.into()
    }

    /// HDataType
    /// ## ne
    ///
    /// `ne(other: HDataType) -> bool` \
    ///
    /// Difference with another `HDataType`. \
    ///
    /// #### Arguments
    ///
    /// * `other` \
    /// An `HDataType`. \
    ///
    /// #### Returns
    ///
    /// A `bool`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hdatatype1 = HDataType$float32
    /// hdatatype2 = HDataType$float64
    /// hdatatype1$ne(hdatatype2) # TRUE
    ///
    /// # or similarly:
    /// hdatatype1 != hdatatype2
    /// ```
    ///
    /// _________
    ///
    fn ne(&self, other: &HDataType) -> savvy::Result<Sexp> {
        let ne = std::cmp::PartialEq::ne(self, other);
        let logical_sexp: OwnedLogicalSexp = ne.try_into()?;
        logical_sexp.into()
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
