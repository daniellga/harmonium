use extendr_api::prelude::*;
use std::fmt;

/// HResamplerType
/// A resampler type representation. \
/// Supports `FftFixedIn`, `FftFixedInOut`, `FftFixedOut`, `SincFixedIn` and `SincFixedOut` types. \
///
/// # Methods
///
#[derive(PartialEq)]
pub enum HResamplerType {
    FftFixedIn,
    FftFixedInOut,
    FftFixedOut,
    SincFixedIn,
    SincFixedOut,
    FastFixedIn,
    FastFixedOut,
}

#[extendr(use_try_from = true)]
impl HResamplerType {
    /// HResamplerType
    /// ## fft_fixed_in
    ///
    /// `fft_fixed_in -> HResamplerType` \
    ///
    /// Creates a `FftFixedIn` `HResamplerType`. \
    ///
    /// #### Returns
    ///
    /// An `HResamplerType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype = HResamplerType$fft_fixed_in
    /// ```
    ///
    /// _________
    ///
    fn fft_fixed_in() -> Self {
        Self::FftFixedIn
    }

    /// HResamplerType
    /// ## fft_fixed_in_out
    ///
    /// `fft_fixed_in_out -> HResamplerType` \
    ///
    /// Creates a `FftFixedInOut` `HResamplerType`. \
    ///
    /// #### Returns
    ///
    /// An `HResamplerType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype = HResamplerType$fft_fixed_in_out
    /// ```
    ///
    /// _________
    ///
    fn fft_fixed_in_out() -> Self {
        Self::FftFixedInOut
    }

    /// HResamplerType
    /// ## fft_fixed_out
    ///
    /// `fft_fixed_out -> HResamplerType` \
    ///
    /// Creates a `FftFixedOut` `HResamplerType`. \
    ///
    /// #### Returns
    ///
    /// An `HResamplerType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype = HResamplerType$fft_fixed_out
    /// ```
    ///
    /// _________
    ///
    fn fft_fixed_out() -> Self {
        Self::FftFixedOut
    }

    /// HResamplerType
    /// ## sinc_fixed_in
    ///
    /// `sinc_fixed_in -> HResamplerType` \
    ///
    /// Creates a `SincFixedIn` `HResamplerType`. \
    ///
    /// #### Returns
    ///
    /// An `HResamplerType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype = HResamplerType$sinc_fixed_in
    /// ```
    ///
    /// _________
    ///
    fn sinc_fixed_in() -> Self {
        Self::SincFixedIn
    }

    /// HResamplerType
    /// ## sinc_fixed_out
    ///
    /// `sinc_fixed_out -> HResamplerType` \
    ///
    /// Creates a `SincFixedOut` `HResamplerType`. \
    ///
    /// #### Returns
    ///
    /// An `HResamplerType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype = HResamplerType$sinc_fixed_out
    /// ```
    ///
    /// _________
    ///
    fn sinc_fixed_out() -> Self {
        Self::SincFixedOut
    }

    /// HResamplerType
    /// ## fast_fixed_in
    ///
    /// `fast_fixed_in -> HResamplerType` \
    ///
    /// Creates a `FastFixedIn` `HResamplerType`. \
    ///
    /// #### Returns
    ///
    /// An `HResamplerType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype = HResamplerType$fast_fixed_in
    /// ```
    ///
    /// _________
    ///
    fn fast_fixed_in() -> Self {
        Self::FastFixedIn
    }

    /// HResamplerType
    /// ## fast_fixed_out
    ///
    /// `fast_fixed_out -> HResamplerType` \
    ///
    /// Creates a `FastFixedOut` `HResamplerType`. \
    ///
    /// #### Returns
    ///
    /// An `HResamplerType`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// hresamplertype = HResamplerType$fast_fixed_out
    /// ```
    ///
    /// _________
    ///
    fn fast_fixed_out() -> Self {
        Self::FastFixedOut
    }

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
    /// hresamplertype = HResamplerType$sinc_fixed_in
    /// hresamplertype$print()
    ///
    /// # or similarly:
    /// print(hresamplertype)
    /// ```
    ///
    /// _________
    ///
    fn print(&self) {
        rprintln!("{}", self);
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
    /// hresamplertype1 = HResamplerType$sinc_fixed_in
    /// hresamplertype2 = HResamplerType$sinc_fixed_in
    /// hresamplertype1$eq(hresamplertype2) # TRUE
    ///
    /// # or similarly:
    /// hresamplertype1 == hresamplertype2
    /// ```
    ///
    /// _________
    ///
    fn eq(&self, other: &HResamplerType) -> bool {
        std::cmp::PartialEq::eq(self, other)
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
    /// hresamplertype1 = HResamplerType$sinc_fixed_in
    /// hresamplertype2 = HResamplerType$sinc_fixed_in
    /// hresamplertype1$ne(hresamplertype2) # FALSE
    ///
    /// # or similarly:
    /// hresamplertype1 != hresamplertype2
    /// ```
    ///
    /// _________
    ///
    fn ne(&self, other: &HResamplerType) -> bool {
        std::cmp::PartialEq::ne(self, other)
    }
}

impl fmt::Display for HResamplerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HResamplerType::FftFixedIn => write!(f, "FftFixedIn")?,
            HResamplerType::FftFixedInOut => write!(f, "FftFixedInOut")?,
            HResamplerType::FftFixedOut => write!(f, "FftFixedOut")?,
            HResamplerType::SincFixedIn => write!(f, "SincFixedIn")?,
            HResamplerType::SincFixedOut => write!(f, "SincFixedOut")?,
            HResamplerType::FastFixedIn => write!(f, "FastFixedIn")?,
            HResamplerType::FastFixedOut => write!(f, "FastFixedOut")?,
        }
        Ok(())
    }
}

extendr_module! {
    mod hresamplertype;
    impl HResamplerType;
}
