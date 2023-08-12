use extendr_api::prelude::*;

mod generate_r_docs;
mod harray;
mod harrayr;
mod haudioop;
mod haudiosink;
mod hdatatype;
mod herror;
mod hfft;
mod hmetadatatype;
mod hpolynomialdegree;
mod hresamplertype;
mod hsincinterpolationparams;
mod hwindow;
mod partialeq;

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod harmonium;
    use hpolynomialdegree;
    use herror;
    use harray;
    use hsincinterpolationparams;
    use haudiosink;
    use hdatatype;
    use hwindow;
    use hmetadatatype;
    use hresamplertype;
    use hfft;
    use haudioop;
}
