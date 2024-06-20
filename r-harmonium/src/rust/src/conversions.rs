use savvy::{OwnedIntegerSexp, Sexp, TypedSexp};

pub(crate) trait AsScalar<T> {
    fn as_scalar(self) -> savvy::Result<T>;
}

impl AsScalar<&'static str> for Sexp {
    fn as_scalar(self) -> savvy::Result<&'static str> {
        match self.into_typed() {
            TypedSexp::String(string_sexp) if string_sexp.len() == 1 => unsafe {
                Ok(string_sexp
                    .iter()
                    .next()
                    // Should never panic since the size was checked.
                    .unwrap_unchecked())
            },
            _ => {
                let err = "Argument must be a string of length 1.".to_string();
                Err(err.into())
            }
        }
    }
}

impl AsScalar<i32> for Sexp {
    fn as_scalar(self) -> savvy::Result<i32> {
        match self.into_typed() {
            TypedSexp::Integer(integer_sexp) if integer_sexp.len() == 1 => {
                Ok(integer_sexp.as_slice()[0])
            }
            _ => {
                let err = "Argument must be a string of length 1.".to_string();
                Err(err.into())
            }
        }
    }
}

impl AsScalar<f64> for Sexp {
    fn as_scalar(self) -> savvy::Result<f64> {
        match self.into_typed() {
            TypedSexp::Real(real_sexp) if real_sexp.len() == 1 => Ok(real_sexp.as_slice()[0]),
            _ => {
                let err = "Argument must be a string of length 1.".to_string();
                Err(err.into())
            }
        }
    }
}

impl AsScalar<bool> for Sexp {
    fn as_scalar(self) -> savvy::Result<bool> {
        match self.into_typed() {
            TypedSexp::Logical(logical_sexp) if logical_sexp.len() == 1 => {
                Ok(logical_sexp.as_slice_raw()[0] == 1)
            }
            _ => {
                let err = "Argument must be a string of length 1.".to_string();
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
