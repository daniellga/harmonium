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

#[extendr]
fn inttimes2_or_error(x: Robj) -> Robj {
    match x.rtype() {
        Rtype::Integers => {
            x.as_integers().unwrap().iter_mut().for_each(|z| *z *= 2);
            ().into()
        }
        _ => HError::to_error1().into(),
    }
}

extendr_module! {
    mod herror;
    impl HError;
    fn inttimes2_or_error;
}
