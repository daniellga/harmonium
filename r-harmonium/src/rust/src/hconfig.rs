use extendr_api::prelude::*;
use harmonium_core::fmt::FloatFmt;

#[extendr]
fn set_float_fmt(fmt: &str) {
    let fmt = match fmt {
        "full" => FloatFmt::Full,
        "mixed" => FloatFmt::Mixed,
        _ => panic!("\"fmt\" must be one of [\"full\", \"mixed\"]."),
    };
    harmonium_core::fmt::set_float_fmt(fmt);
}

#[extendr]
fn get_float_fmt() -> String {
    let strfmt = match harmonium_core::fmt::get_float_fmt() {
        FloatFmt::Full => "full",
        FloatFmt::Mixed => "mixed",
    };
    strfmt.to_string()
}

extendr_module! {
    mod hconfig;
    fn set_float_fmt;
    fn get_float_fmt;
}
