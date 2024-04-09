use savvy::{OwnedIntegerSexp, Sexp, TypedSexp};

pub(crate) trait Conversions<T> {
    fn to_scalar(self: Self) -> savvy::Result<T>;
}

impl Conversions<&'static str> for Sexp {
    fn to_scalar(self) -> savvy::Result<&'static str> {
        match self.into_typed() {
            TypedSexp::String(string_sexp) if string_sexp.len() == 1 => {
                // Ok to unwrap since the size was checked.
                Ok(string_sexp.iter().next().unwrap())
            }
            _ => {
                let err = format!("Argument must be a string of length 1.");
                Err(err.into())
            }
        }
    }
}

impl Conversions<i32> for Sexp {
    fn to_scalar(self) -> savvy::Result<i32> {
        match self.into_typed() {
            TypedSexp::Integer(integer_sexp) if integer_sexp.len() == 1 => {
                Ok(integer_sexp.as_slice()[0])
            }
            _ => {
                let err = format!("Argument must be an integer of length 1.");
                Err(err.into())
            }
        }
    }
}

impl Conversions<f64> for Sexp {
    fn to_scalar(self) -> savvy::Result<f64> {
        match self.into_typed() {
            TypedSexp::Real(real_sexp) if real_sexp.len() == 1 => Ok(real_sexp.as_slice()[0]),
            _ => {
                let err = format!("Argument must be a double of length 1.");
                Err(err.into())
            }
        }
    }
}

impl Conversions<bool> for Sexp {
    fn to_scalar(self) -> savvy::Result<bool> {
        match self.into_typed() {
            TypedSexp::Logical(logical_sexp) if logical_sexp.len() == 1 => {
                Ok(logical_sexp.as_slice_raw()[0] == 1)
            }
            _ => {
                let err = format!("Argument must be a logical of length 1.");
                Err(err.into())
            }
        }
    }
}

pub fn try_from_usize_to_int_sexp(n: usize) -> savvy::Result<OwnedIntegerSexp> {
    let n: i32 = n
        .try_into()
        .map_err(|_| savvy::Error::new("Cannot convert usize to i32."))?;
    n.try_into()
}
