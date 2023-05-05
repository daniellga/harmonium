use crate::{harraydynamic::HArray, hdatatype::HDataType};
use extendr_api::prelude::*;
use harmonium_window::windows::*;
use std::sync::Arc;

pub struct HWindow;

#[extendr]
impl HWindow {
    fn blackman(npoints: i32, sym: bool, dtype: &HDataType) -> HArray {
        let npoints = npoints.try_into().unwrap();

        let window_type = if sym {
            WindowType::Symmetric
        } else {
            WindowType::Periodic
        };

        match dtype {
            HDataType::Float32 => {
                let hfloatarray = harmonium_window::windows::blackman::<f32>(npoints, window_type);
                HArray(Arc::new(hfloatarray))
            }
            HDataType::Float64 => {
                let hfloatarray = harmonium_window::windows::blackman::<f64>(npoints, window_type);
                HArray(Arc::new(hfloatarray))
            }
            _ => panic!("not a valid dtype"),
        }
    }
}

extendr_module! {
    mod hwindow;
    impl HWindow;
}
