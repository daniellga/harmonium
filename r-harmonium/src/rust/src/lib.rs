use extendr_api::prelude::*;

mod generate_r_docs;
mod harray;
mod haudio;
mod haudiosink;
mod hconfig;
mod hdatatype;
mod herror;
mod hfile;
mod hinterpolationparams;
mod hmatrix;
mod hmetadatatype;
mod hresampler;
mod hresamplertype;
mod hwindow;
mod partialeq;

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod harmonium;
    use hconfig;
    use herror;
    use harray;
    use hinterpolationparams;
    use haudio;
    use haudiosink;
    use hfile;
    use hdatatype;
    use hmatrix;
    use hwindow;
    use hmetadatatype;
    use hresampler;
    use hresamplertype;
}
