use std::{error, fmt};

use extendr_api::prelude::*;

#[derive(Debug)]
pub enum HError {
    Error1,
    Error2,
}

impl fmt::Display for HError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "oh la la!!!!")
    }
}

impl error::Error for HError {}

#[extendr]
impl HError {
    fn to_error1() -> HError {
        HError::Error1
    }

    fn to_error2() -> HError {
        HError::Error2
    }
}

extendr_module! {
    mod herror;
    impl HError;
}
