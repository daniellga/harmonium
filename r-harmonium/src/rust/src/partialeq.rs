use crate::{harray::HArrayR, haudio::HAudioR, hdatatype::HDataType, hmatrix::HMatrixR};
use harmonium_core::structs::{
    HComplexArray, HComplexMatrix, HFloatArray, HFloatAudio, HFloatMatrix,
};

fn equal_harray(lhs: &dyn HArrayR, rhs: &dyn HArrayR) -> bool {
    if lhs.dtype() != rhs.dtype() {
        return false;
    }

    match lhs.dtype() {
        HDataType::Float32 => {
            let lhs = lhs.as_any().downcast_ref::<HFloatArray<f32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HFloatArray<f32>>().unwrap();
            lhs.eq(rhs)
        }
        HDataType::Float64 => {
            let lhs = lhs.as_any().downcast_ref::<HFloatArray<f64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HFloatArray<f64>>().unwrap();
            lhs.eq(rhs)
        }
        HDataType::Complex32 => {
            let lhs = lhs.as_any().downcast_ref::<HComplexArray<f32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HComplexArray<f32>>().unwrap();
            lhs.eq(rhs)
        }
        HDataType::Complex64 => {
            let lhs = lhs.as_any().downcast_ref::<HComplexArray<f64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HComplexArray<f64>>().unwrap();
            lhs.eq(rhs)
        }
    }
}

fn equal_hmatrix(lhs: &dyn HMatrixR, rhs: &dyn HMatrixR) -> bool {
    if lhs.dtype() != rhs.dtype() {
        return false;
    }

    match lhs.dtype() {
        HDataType::Float32 => {
            let lhs = lhs.as_any().downcast_ref::<HFloatMatrix<f32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HFloatMatrix<f32>>().unwrap();
            lhs.eq(rhs)
        }
        HDataType::Float64 => {
            let lhs = lhs.as_any().downcast_ref::<HFloatMatrix<f64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HFloatMatrix<f64>>().unwrap();
            lhs.eq(rhs)
        }
        HDataType::Complex32 => {
            let lhs = lhs.as_any().downcast_ref::<HComplexMatrix<f32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HComplexMatrix<f32>>().unwrap();
            lhs.eq(rhs)
        }
        HDataType::Complex64 => {
            let lhs = lhs.as_any().downcast_ref::<HComplexMatrix<f64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HComplexMatrix<f64>>().unwrap();
            lhs.eq(rhs)
        }
    }
}

fn equal_haudio(lhs: &dyn HAudioR, rhs: &dyn HAudioR) -> bool {
    if lhs.sr() != rhs.sr() {
        return false;
    }
    if lhs.dtype() != rhs.dtype() {
        return false;
    }

    match lhs.dtype() {
        HDataType::Float32 => {
            let lhs = lhs.as_any().downcast_ref::<HFloatAudio<f32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HFloatAudio<f32>>().unwrap();
            lhs.eq(rhs)
        }
        HDataType::Float64 => {
            let lhs = lhs.as_any().downcast_ref::<HFloatAudio<f64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<HFloatAudio<f64>>().unwrap();
            lhs.eq(rhs)
        }
        _ => unreachable!(),
    }
}

impl PartialEq for dyn HArrayR + '_ {
    fn eq(&self, other: &dyn HArrayR) -> bool {
        equal_harray(self, other)
    }
}

impl PartialEq<dyn HArrayR> for std::sync::Arc<dyn HArrayR + '_> {
    fn eq(&self, other: &dyn HArrayR) -> bool {
        equal_harray(&**self, other)
    }
}

impl PartialEq for dyn HMatrixR + '_ {
    fn eq(&self, other: &dyn HMatrixR) -> bool {
        equal_hmatrix(self, other)
    }
}

impl PartialEq<dyn HMatrixR> for std::sync::Arc<dyn HMatrixR + '_> {
    fn eq(&self, other: &dyn HMatrixR) -> bool {
        equal_hmatrix(&**self, other)
    }
}

impl PartialEq for dyn HAudioR + '_ {
    fn eq(&self, other: &dyn HAudioR) -> bool {
        equal_haudio(self, other)
    }
}

impl PartialEq<dyn HAudioR> for std::sync::Arc<dyn HAudioR + '_> {
    fn eq(&self, that: &dyn HAudioR) -> bool {
        equal_haudio(&**self, that)
    }
}
