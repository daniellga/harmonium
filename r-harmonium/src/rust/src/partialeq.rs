use ndarray::IxDyn;
use num_complex::Complex;
use std::sync::Arc;

use crate::{harrayr::HArrayR, hdatatype::HDataType};

fn equal_harray(lhs: &dyn HArrayR, rhs: &dyn HArrayR) -> bool {
    if lhs.dtype() != rhs.dtype() {
        return false;
    }

    match lhs.dtype() {
        HDataType::Float32 => {
            let lhs = lhs
                .as_any()
                .downcast_ref::<harmonium_core::array::HArray<f32, IxDyn>>()
                .expect("Should not panic since the type was checked.");
            let rhs = rhs
                .as_any()
                .downcast_ref::<harmonium_core::array::HArray<f32, IxDyn>>()
                .expect("Should not panic since the type was checked.");
            lhs.eq(rhs)
        }
        HDataType::Float64 => {
            let lhs = lhs
                .as_any()
                .downcast_ref::<harmonium_core::array::HArray<f64, IxDyn>>()
                .expect("Should not panic since the type was checked.");
            let rhs = rhs
                .as_any()
                .downcast_ref::<harmonium_core::array::HArray<f64, IxDyn>>()
                .expect("Should not panic since the type was checked.");
            lhs.eq(rhs)
        }
        HDataType::Complex32 => {
            let lhs = lhs
                .as_any()
                .downcast_ref::<harmonium_core::array::HArray<Complex<f32>, IxDyn>>()
                .expect("Should not panic since the type was checked.");
            let rhs = rhs
                .as_any()
                .downcast_ref::<harmonium_core::array::HArray<Complex<f32>, IxDyn>>()
                .expect("Should not panic since the type was checked.");
            lhs.eq(rhs)
        }
        HDataType::Complex64 => {
            let lhs = lhs
                .as_any()
                .downcast_ref::<harmonium_core::array::HArray<Complex<f64>, IxDyn>>()
                .expect("Should not panic since the type was checked.");
            let rhs = rhs
                .as_any()
                .downcast_ref::<harmonium_core::array::HArray<Complex<f64>, IxDyn>>()
                .expect("Should not panic since the type was checked.");
            lhs.eq(rhs)
        }
    }
}

impl PartialEq for dyn HArrayR + '_ {
    fn eq(&self, other: &dyn HArrayR) -> bool {
        equal_harray(self, other)
    }
}

impl PartialEq<dyn HArrayR> for Arc<dyn HArrayR + '_> {
    fn eq(&self, other: &dyn HArrayR) -> bool {
        equal_harray(&**self, other)
    }
}
