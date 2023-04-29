use extendr_api::prelude::*;

mod datatype;
mod harraydynamic;
mod haudiodynamic;
mod haudiosink;
mod hfile;
mod hmatrixdynamic;
mod partialeq;
mod windows;

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod harmonium_io;
    use harraydynamic;
    use haudiodynamic;
    use haudiosink;
    use hfile;
    use datatype;
    use hmatrixdynamic;
    use windows;
}
