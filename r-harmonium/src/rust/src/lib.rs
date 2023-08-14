use extendr_api::prelude::*;

mod conversions;
mod generate_r_docs;
mod harray;
mod harrayr;
mod haudioop;
mod haudiosink;
mod hdatatype;
mod hfft;
mod hfile;
mod hmetadatatype;
mod hpolynomialdegree;
mod hresampler;
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
    use harray;
    use hsincinterpolationparams;
    use haudiosink;
    use hdatatype;
    use hwindow;
    use hmetadatatype;
    use hresampler;
    use hresamplertype;
    use hfft;
    use haudioop;
    use hfile;
}
