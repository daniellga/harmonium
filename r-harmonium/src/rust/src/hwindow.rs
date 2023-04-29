use std::sync::Arc;
use crate::{hdatatype::HDataType, harraydynamic::HArray};
use extendr_api::prelude::*;
use harmonium_core::structs::HFloatArray;
use harmonium_window::windows::*;

pub struct HWindow;

#[extendr]
impl HWindow {
    fn blackman(npoints: i32, sym: bool, data_type: &HDataType) -> HArray {
        let npoints = npoints.try_into().unwrap();

        let window_type = if sym {
            WindowType::Symmetric
        } else {
            WindowType::Periodic
        };

        match data_type {
            HDataType::Float32 => {
                let v = harmonium_window::windows::blackman::<f32>(npoints, window_type);
                let hfloatarray = HFloatArray::new_from_vec(v);
                HArray(Arc::new(hfloatarray))
            }
            HDataType::Float64 => {
                let v = harmonium_window::windows::blackman::<f64>(npoints, window_type);
                let hfloatarray = HFloatArray::new_from_vec(v);
                HArray(Arc::new(hfloatarray))
            }
            _ => panic!("not a valid data_type"),
        }
    }

    fn all_hwindow() -> Vec<String> {
        vec![
            "blackman".into(),
            "blackmanharris".into(),
            "bohman".into(),
            "chebsaw".into(),
            "cosine".into(),
            "hann".into(),
        ]
    }
}

extendr_module! {
    mod hwindow;
    impl HWindow;
}
