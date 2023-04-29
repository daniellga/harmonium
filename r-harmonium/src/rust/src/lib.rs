use extendr_api::prelude::*;

mod harraydynamic;
mod haudiodynamic;
mod haudiosink;
mod hdatatype;
mod hfile;
mod hmatrixdynamic;
mod hwindow;
mod partialeq;

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod harmonium;
    use harraydynamic;
    use haudiodynamic;
    use haudiosink;
    use hfile;
    use hdatatype;
    use hmatrixdynamic;
    use hwindow;
}
