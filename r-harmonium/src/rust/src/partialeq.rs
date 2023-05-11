use crate::{harray::HArrayR, haudio::HAudioR, hdatatype::HDataType, hmatrix::HMatrixR};
use harmonium_core::structs::{
    HComplexArray, HComplexMatrix, HFloatArray, HFloatAudio, HFloatMatrix,
};

//pub trait PartialEqInner<Rhs: ?Sized = Self> {
//    fn eq_inner(&self, other: &Rhs) -> bool;
//}

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

//fn harray_equal_inner_harray(lhs: &dyn HArrayR, rhs: &dyn HArrayR) -> bool {
//    if lhs.dtype() != rhs.dtype() {
//        return false;
//    }
//
//    match lhs.dtype() {
//        HDataType::Float32 => {
//            let lhs = harr_as_slice_f32(lhs);
//            let rhs = harr_as_slice_f32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Float64 => {
//            let lhs = harr_as_slice_f64(lhs);
//            let rhs = harr_as_slice_f64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Complex32 => {
//            let lhs = harr_as_slice_c32(lhs);
//            let rhs = harr_as_slice_c32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Complex64 => {
//            let lhs = harr_as_slice_c64(lhs);
//            let rhs = harr_as_slice_c64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//    }
//}
//
//fn harray_equal_inner_hmatrix(lhs: &dyn HArrayR, rhs: &dyn HMatrixR) -> bool {
//    if lhs.dtype() != rhs.dtype() {
//        return false;
//    }
//
//    match lhs.dtype() {
//        HDataType::Float32 => {
//            let lhs = harr_as_slice_f32(lhs);
//            let rhs = hmat_as_slice_f32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Float64 => {
//            let lhs = harr_as_slice_f64(lhs);
//            let rhs = hmat_as_slice_f64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Complex32 => {
//            let lhs = harr_as_slice_c32(lhs);
//            let rhs = hmat_as_slice_c32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Complex64 => {
//            let lhs = harr_as_slice_c64(lhs);
//            let rhs = hmat_as_slice_c64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//    }
//}
//
//fn harray_equal_inner_haudio(lhs: &dyn HArrayR, rhs: &dyn HAudioR) -> bool {
//    if lhs.dtype() != rhs.dtype() {
//        return false;
//    }
//
//    match lhs.dtype() {
//        HDataType::Float32 => {
//            let lhs = harr_as_slice_f32(lhs);
//            let rhs = haudio_as_slice_f32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Float64 => {
//            let lhs = harr_as_slice_f64(lhs);
//            let rhs = haudio_as_slice_f64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        _ => unreachable!(),
//    }
//}

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

//fn hmatrix_equal_inner_hmatrix(lhs: &dyn HMatrixR, rhs: &dyn HMatrixR) -> bool {
//    if lhs.dtype() != rhs.dtype() {
//        return false;
//    }
//
//    match lhs.dtype() {
//        HDataType::Float32 => {
//            let lhs = hmat_as_slice_f32(lhs);
//            let rhs = hmat_as_slice_f32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Float64 => {
//            let lhs = hmat_as_slice_f64(lhs);
//            let rhs = hmat_as_slice_f64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Complex32 => {
//            let lhs = hmat_as_slice_c32(lhs);
//            let rhs = hmat_as_slice_c32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Complex64 => {
//            let lhs = hmat_as_slice_c64(lhs);
//            let rhs = hmat_as_slice_c64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//    }
//}
//
//fn hmatrix_equal_inner_harray(lhs: &dyn HMatrixR, rhs: &dyn HArrayR) -> bool {
//    if lhs.dtype() != rhs.dtype() {
//        return false;
//    }
//
//    match lhs.dtype() {
//        HDataType::Float32 => {
//            let lhs = hmat_as_slice_f32(lhs);
//            let rhs = harr_as_slice_f32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Float64 => {
//            let lhs = hmat_as_slice_f64(lhs);
//            let rhs = harr_as_slice_f64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Complex32 => {
//            let lhs = hmat_as_slice_c32(lhs);
//            let rhs = harr_as_slice_c32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Complex64 => {
//            let lhs = hmat_as_slice_c64(lhs);
//            let rhs = harr_as_slice_c64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//    }
//}
//
//fn hmatrix_equal_inner_haudio(lhs: &dyn HMatrixR, rhs: &dyn HAudioR) -> bool {
//    if lhs.dtype() != rhs.dtype() {
//        return false;
//    }
//
//    match lhs.dtype() {
//        HDataType::Float32 => {
//            let lhs = hmat_as_slice_f32(lhs);
//            let rhs = haudio_as_slice_f32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Float64 => {
//            let lhs = hmat_as_slice_f64(lhs);
//            let rhs = haudio_as_slice_f64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        _ => unreachable!(),
//    }
//}

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

//fn haudio_equal_inner_haudio(lhs: &dyn HAudioR, rhs: &dyn HAudioR) -> bool {
//    if lhs.dtype() != rhs.dtype() {
//        return false;
//    }
//
//    match lhs.dtype() {
//        HDataType::Float32 => {
//            let lhs = haudio_as_slice_f32(lhs);
//            let rhs = haudio_as_slice_f32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Float64 => {
//            let lhs = haudio_as_slice_f64(lhs);
//            let rhs = haudio_as_slice_f64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        _ => unreachable!(),
//    }
//}
//
//fn haudio_equal_inner_harray(lhs: &dyn HAudioR, rhs: &dyn HArrayR) -> bool {
//    if lhs.dtype() != rhs.dtype() {
//        return false;
//    }
//
//    match lhs.dtype() {
//        HDataType::Float32 => {
//            let lhs = haudio_as_slice_f32(lhs);
//            let rhs = harr_as_slice_f32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Float64 => {
//            let lhs = haudio_as_slice_f64(lhs);
//            let rhs = harr_as_slice_f64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        _ => unreachable!(),
//    }
//}
//
//fn haudio_equal_inner_hmatrix(lhs: &dyn HAudioR, rhs: &dyn HMatrixR) -> bool {
//    if lhs.dtype() != rhs.dtype() {
//        return false;
//    }
//
//    match lhs.dtype() {
//        HDataType::Float32 => {
//            let lhs = haudio_as_slice_f32(lhs);
//            let rhs = hmat_as_slice_f32(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        HDataType::Float64 => {
//            let lhs = haudio_as_slice_f64(lhs);
//            let rhs = hmat_as_slice_f64(rhs);
//            std::ptr::eq(lhs, rhs)
//        }
//        _ => unreachable!(),
//    }
//}

macro_rules! harray_or_hmat_as_slice {
    ($(($name:ident, $t1:ty, $t2:ty, $t3:ty)),+) => {
        $(
        fn $name(obj: $t3) -> &[$t2] {
            obj.as_any().downcast_ref::<$t1>().unwrap().as_slice()
        }
        )+
    };
}

harray_or_hmat_as_slice!(
    (harr_as_slice_f32, HFloatArray<f32>, f32, &dyn HArrayR),
    (harr_as_slice_f64, HFloatArray<f64>, f64, &dyn HArrayR),
    (harr_as_slice_c32, HComplexArray<f32>, f32, &dyn HArrayR),
    (harr_as_slice_c64, HComplexArray<f64>, f64, &dyn HArrayR),
    (hmat_as_slice_f32, HFloatMatrix<f32>, f32, &dyn HMatrixR),
    (hmat_as_slice_f64, HFloatMatrix<f64>, f64, &dyn HMatrixR),
    (hmat_as_slice_c32, HComplexMatrix<f32>, f32, &dyn HMatrixR),
    (hmat_as_slice_c64, HComplexMatrix<f64>, f64, &dyn HMatrixR)
);

macro_rules! haudio_as_slice {
    ($(($name:ident, $t1:ty, $t2:ty, $t3:ty)),+) => {
        $(
        fn $name(obj: $t3) -> &[$t2] {
            obj.as_any().downcast_ref::<$t1>().unwrap().inner().as_slice()
        }
        )+
    };
}

haudio_as_slice!(
    (haudio_as_slice_f32, HFloatAudio<f32>, f32, &dyn HAudioR),
    (haudio_as_slice_f64, HFloatAudio<f64>, f64, &dyn HAudioR)
);

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

//impl PartialEqInner for dyn HArrayR + '_ {
//    fn eq_inner(&self, other: &dyn HArrayR) -> bool {
//        harray_equal_inner_harray(self, other)
//    }
//}
//
//impl PartialEqInner<dyn HArrayR> for std::sync::Arc<dyn HArrayR + '_> {
//    fn eq_inner(&self, other: &dyn HArrayR) -> bool {
//        harray_equal_inner_harray(&**self, other)
//    }
//}
//
//impl PartialEqInner<dyn HMatrixR> for dyn HArrayR + '_ {
//    fn eq_inner(&self, other: &dyn HMatrixR) -> bool {
//        harray_equal_inner_hmatrix(self, other)
//    }
//}
//
//impl PartialEqInner<dyn HMatrixR> for std::sync::Arc<dyn HArrayR + '_> {
//    fn eq_inner(&self, other: &dyn HMatrixR) -> bool {
//        harray_equal_inner_hmatrix(&**self, other)
//    }
//}
//
//impl PartialEqInner<dyn HAudioR> for dyn HArrayR + '_ {
//    fn eq_inner(&self, other: &dyn HAudioR) -> bool {
//        harray_equal_inner_haudio(self, other)
//    }
//}
//
//impl PartialEqInner<dyn HAudioR> for std::sync::Arc<dyn HArrayR + '_> {
//    fn eq_inner(&self, other: &dyn HAudioR) -> bool {
//        harray_equal_inner_haudio(&**self, other)
//    }
//}
//
//impl PartialEqInner for dyn HMatrixR + '_ {
//    fn eq_inner(&self, other: &dyn HMatrixR) -> bool {
//        hmatrix_equal_inner_hmatrix(self, other)
//    }
//}
//
//impl PartialEqInner<dyn HMatrixR> for std::sync::Arc<dyn HMatrixR + '_> {
//    fn eq_inner(&self, other: &dyn HMatrixR) -> bool {
//        hmatrix_equal_inner_hmatrix(&**self, other)
//    }
//}
//
//impl PartialEqInner<dyn HArrayR> for dyn HMatrixR + '_ {
//    fn eq_inner(&self, other: &dyn HArrayR) -> bool {
//        hmatrix_equal_inner_harray(self, other)
//    }
//}
//
//impl PartialEqInner<dyn HArrayR> for std::sync::Arc<dyn HMatrixR + '_> {
//    fn eq_inner(&self, other: &dyn HArrayR) -> bool {
//        hmatrix_equal_inner_harray(&**self, other)
//    }
//}
//
//impl PartialEqInner<dyn HAudioR> for dyn HMatrixR + '_ {
//    fn eq_inner(&self, other: &dyn HAudioR) -> bool {
//        hmatrix_equal_inner_haudio(self, other)
//    }
//}
//
//impl PartialEqInner<dyn HAudioR> for std::sync::Arc<dyn HMatrixR + '_> {
//    fn eq_inner(&self, other: &dyn HAudioR) -> bool {
//        hmatrix_equal_inner_haudio(&**self, other)
//    }
//}
//
//impl PartialEqInner for dyn HAudioR + '_ {
//    fn eq_inner(&self, other: &dyn HAudioR) -> bool {
//        haudio_equal_inner_haudio(self, other)
//    }
//}
//
//impl PartialEqInner<dyn HAudioR> for std::sync::Arc<dyn HAudioR + '_> {
//    fn eq_inner(&self, other: &dyn HAudioR) -> bool {
//        haudio_equal_inner_haudio(&**self, other)
//    }
//}
//
//impl PartialEqInner<dyn HArrayR> for dyn HAudioR + '_ {
//    fn eq_inner(&self, other: &dyn HArrayR) -> bool {
//        haudio_equal_inner_harray(self, other)
//    }
//}
//
//impl PartialEqInner<dyn HArrayR> for std::sync::Arc<dyn HAudioR + '_> {
//    fn eq_inner(&self, other: &dyn HArrayR) -> bool {
//        haudio_equal_inner_harray(&**self, other)
//    }
//}
//
//impl PartialEqInner<dyn HMatrixR> for dyn HAudioR + '_ {
//    fn eq_inner(&self, other: &dyn HMatrixR) -> bool {
//        haudio_equal_inner_hmatrix(self, other)
//    }
//}
//
//impl PartialEqInner<dyn HMatrixR> for std::sync::Arc<dyn HAudioR + '_> {
//    fn eq_inner(&self, other: &dyn HMatrixR) -> bool {
//        haudio_equal_inner_hmatrix(&**self, other)
//    }
//}
