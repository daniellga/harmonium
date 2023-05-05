use extendr_api::prelude::*;

mod harraydynamic;
mod haudiodynamic;
mod haudiosink;
mod hdatatype;
mod herror;
mod hfile;
mod hinterpolationparams;
mod hmatrixdynamic;
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
    use herror;
    use harraydynamic;
    use hinterpolationparams;
    use haudiodynamic;
    use haudiosink;
    use hfile;
    use hdatatype;
    use hmatrixdynamic;
    use hwindow;
    use hmetadatatype;
    use hresampler;
    use hresamplertype;
}
