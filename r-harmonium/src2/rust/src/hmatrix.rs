use crate::{
    harray::{HComplex32ArrayR, HComplex64ArrayR, HFloat32ArrayR, HFloat64ArrayR},
    haudio::{HFloat32AudioR, HFloat64AudioR},
};
use arrow2::array::PrimitiveArray;
use extendr_api::prelude::*;
use harmonium_core::structs::{HComplexArray, HComplexMatrix, HFloatArray, HFloatMatrix};
use harmonium_fft::fft::fft_arrow::{FftComplexMatrix, FftFloatMatrix};
use std::fmt;

#[derive(Debug)]
pub struct HFloat32MatrixR {
    inner: HFloatMatrix<f32>,
}

#[derive(Debug)]
pub struct HFloat64MatrixR {
    inner: HFloatMatrix<f64>,
}

#[derive(Debug)]
pub struct HComplex32MatrixR {
    inner: HComplexMatrix<f32>,
}

#[derive(Debug)]
pub struct HComplex64MatrixR {
    inner: HComplexMatrix<f64>,
}

impl HFloat32MatrixR {
    pub(crate) fn new(inner: HFloatMatrix<f32>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HFloatMatrix<f32> {
        &self.inner
    }
}

impl HFloat64MatrixR {
    pub(crate) fn new(inner: HFloatMatrix<f64>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HFloatMatrix<f64> {
        &self.inner
    }
}

impl HComplex32MatrixR {
    pub(crate) fn new(inner: HComplexMatrix<f32>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HComplexMatrix<f32> {
        &self.inner
    }
}

impl HComplex64MatrixR {
    pub(crate) fn new(inner: HComplexMatrix<f64>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HComplexMatrix<f64> {
        &self.inner
    }
}

#[extendr(use_try_from = true)]
impl HFloat32MatrixR {
    pub fn new_from_values(rmatrix: RMatrix<f64>) -> Self {
        let ncols = rmatrix.ncols();
        let v: Vec<f32> = rmatrix.data().iter().map(|z| *z as f32).collect();
        let array = PrimitiveArray::from_vec(v);
        let harray = HFloatArray::<f32>::new(array);
        let harrayr = HFloat32ArrayR::new(harray);
        harrayr.as_hmatrix(i32::try_from(ncols).unwrap())
    }

    /// Converts to HFloat32ArrayR. The new HFloat32ArrayR Uses the same underlying data as the HFloat32MatrixR.
    pub fn as_harray(&self) -> HFloat32ArrayR {
        let array = self
            .inner()
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .clone();
        let harray = HFloatArray::new(array);
        HFloat32ArrayR::new(harray)
    }

    /// Converts to HFloat32AudioR. The new HFloat32AudioR Uses the same underlying data as the HFloat32MatrixR.
    pub fn as_haudio(&self, sr: i32) -> HFloat32AudioR {
        let inner = self.inner().clone().into_haudio(sr as u32);
        HFloat32AudioR::new(inner)
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HFloat32MatrixR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HFloat32MatrixR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HFloat32MatrixR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        Self::new(self.inner().clone())
    }

    /// Creates a new HFloat32MatrixR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let list_array = self.inner().inner();
        let ncols = list_array.len();
        let slice = list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values()
            .as_slice();
        let array = PrimitiveArray::<f32>::from_slice(slice);
        let harray = HFloatArray::<f32>::new(array);
        let hmatrix = harray.into_hmatrix(ncols).unwrap();
        Self::new(hmatrix)
    }

    pub fn fft(&self) -> HComplex32MatrixR {
        HComplex32MatrixR::new(self.inner().fft().unwrap())
    }

    pub fn collect(&self) -> RMatrix<f64> {
        let list_array = self.inner().inner();
        let ncols = list_array.len();
        let nrows = list_array.size();
        let buffer = list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values();

        buffer
            .iter()
            .map(|x| *x as f64)
            .collect_rarray([nrows, ncols])
            .unwrap()
    }

    /// Returns the pointer adress as a string.
    pub fn mem_adress(&self) -> String {
        let p = self
            .inner()
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values()
            .as_slice();
        format!("{:p}", p)
    }

    /// Take the average across columns. The operation is not done in place.
    pub fn mean_cols(&self) -> Self {
        let hfloatmatrix = self.inner.mean_cols().unwrap();
        HFloat32MatrixR::new(hfloatmatrix)
    }
}

#[extendr(use_try_from = true)]
impl HFloat64MatrixR {
    pub fn new_from_values(rmatrix: RMatrix<f64>) -> Self {
        let slice = rmatrix.data();
        let ncols = rmatrix.ncols();
        let array = PrimitiveArray::from_slice(slice);
        let harray = HFloatArray::<f64>::new(array);
        let harrayr = HFloat64ArrayR::new(harray);
        harrayr.as_hmatrix(i32::try_from(ncols).unwrap())
    }

    /// Converts to HFloat64ArrayR. The new HFloat64ArrayR Uses the same underlying data as the HFloat64MatrixR.
    pub fn as_harray(&self) -> HFloat64ArrayR {
        let array = self
            .inner()
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .clone();
        let harray = HFloatArray::new(array);
        HFloat64ArrayR::new(harray)
    }

    /// Converts to HFloat64AudioR. The new HFloat64AudioR Uses the same underlying data as the HFloat64MatrixR.
    pub fn as_haudio(&self, sr: i32) -> HFloat64AudioR {
        let inner = self.inner().clone().into_haudio(sr as u32);
        HFloat64AudioR::new(inner)
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HFloat64MatrixR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HFloat64MatrixR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HFloat64MatrixR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        Self::new(self.inner().clone())
    }

    /// Creates a new HFloat64MatrixR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let list_array = self.inner().inner();
        let ncols = list_array.len();
        let slice = list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values()
            .as_slice();
        let array = PrimitiveArray::<f64>::from_slice(slice);
        let harray = HFloatArray::<f64>::new(array);
        let hmatrix = harray.into_hmatrix(ncols).unwrap();
        Self::new(hmatrix)
    }

    pub fn fft(&self) -> HComplex64MatrixR {
        HComplex64MatrixR::new(self.inner().fft().unwrap())
    }

    pub fn collect(&self) -> RMatrix<f64> {
        let list_array = self.inner().inner();
        let ncols = list_array.len();
        let nrows = list_array.size();
        let buffer = list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values();

        buffer
            .iter()
            .copied()
            .collect_rarray([nrows, ncols])
            .unwrap()
    }

    /// Returns the pointer adress as a string.
    pub fn mem_adress(&self) -> String {
        let p = self
            .inner()
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values()
            .as_slice();
        format!("{:p}", p)
    }

    /// Take the average across columns. The operation is not done in place.
    pub fn mean_cols(&self) -> Self {
        let hfloatmatrix = self.inner.mean_cols().unwrap();
        HFloat64MatrixR::new(hfloatmatrix)
    }
}

#[extendr(use_try_from = true)]
impl HComplex32MatrixR {
    pub fn new_from_values(rmatrix: RMatrix<Rcplx>) -> Self {
        let length = rmatrix.data().len() * 2;
        let ncols = rmatrix.ncols();
        let mut v: Vec<f32> = Vec::with_capacity(length);
        for x in rmatrix.data().iter() {
            v.push(x.re().0 as f32);
            v.push(x.im().0 as f32);
        }
        let array = PrimitiveArray::from_vec(v);
        let harray = HComplexArray::<f32>::new(array);
        let harrayr = HComplex32ArrayR::new(harray);
        harrayr.as_hmatrix(i32::try_from(ncols).unwrap())
    }

    /// Converts to HComplex32ArrayR. The new HComplex32ArrayR Uses the same underlying data as the HComplex32MatrixR.
    pub fn as_harray(&self) -> HComplex32ArrayR {
        let array = self
            .inner()
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .clone();
        let harray = HComplexArray::new(array);
        HComplex32ArrayR::new(harray)
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HComplex32MatrixR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HComplex32MatrixR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HComplex32MatrixR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        Self::new(self.inner.clone())
    }

    /// Creates a new HComplex32MatrixR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let list_array = self.inner().inner();
        let ncols = list_array.len();
        let slice = list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values()
            .as_slice();
        let array = PrimitiveArray::<f32>::from_slice(slice);
        let harray = HComplexArray::<f32>::new(array);
        let hmatrix = harray.into_hmatrix(ncols).unwrap();
        Self::new(hmatrix)
    }

    pub fn fft(&self) -> Self {
        Self::new(self.inner().fft().unwrap())
    }

    pub fn collect(&self) -> RMatrix<Rcplx> {
        let list_array = self.inner().inner();
        let ncols = list_array.len();
        let nrows = list_array.size();
        let buffer = list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values();

        buffer
            .chunks_exact(2)
            .map(|x| Rcplx::new(x[0] as f64, x[1] as f64))
            .collect_rarray([nrows / 2, ncols])
            .unwrap()
    }

    /// Returns the pointer adress as a string.
    pub fn mem_adress(&self) -> String {
        let p = self
            .inner()
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f32>>()
            .unwrap()
            .values()
            .as_slice();
        format!("{:p}", p)
    }
}

#[extendr(use_try_from = true)]
impl HComplex64MatrixR {
    pub fn new_from_values(rmatrix: RMatrix<Rcplx>) -> Self {
        let length = rmatrix.data().len() * 2;
        let ncols = rmatrix.ncols();
        let mut v: Vec<f64> = Vec::with_capacity(length);
        for x in rmatrix.data().iter() {
            v.push(x.re().0);
            v.push(x.im().0);
        }
        let array = PrimitiveArray::from_vec(v);
        let harray = HComplexArray::<f64>::new(array);
        let harrayr = HComplex64ArrayR::new(harray);
        harrayr.as_hmatrix(i32::try_from(ncols).unwrap())
    }

    /// Converts to HComplex64ArrayR. The new HComplex64ArrayR Uses the same underlying data as the HComplex64MatrixR.
    pub fn as_harray(&self) -> HComplex64ArrayR {
        let array = self
            .inner()
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .clone();
        let harray = HComplexArray::new(array);
        HComplex64ArrayR::new(harray)
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HComplex64MatrixR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HComplex64MatrixR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HComplex64MatrixR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        Self::new(self.inner().clone())
    }

    /// Creates a new HComplex64MatrixR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let list_array = self.inner().inner();
        let ncols = list_array.len();
        let slice = list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values()
            .as_slice();
        let array = PrimitiveArray::<f64>::from_slice(slice);
        let harray = HComplexArray::<f64>::new(array);
        let hmatrix = harray.into_hmatrix(ncols).unwrap();
        Self::new(hmatrix)
    }

    pub fn fft(&self) -> Self {
        Self::new(self.inner().fft().unwrap())
    }

    pub fn collect(&self) -> RMatrix<Rcplx> {
        let list_array = self.inner().inner();
        let ncols = list_array.len();
        let nrows = list_array.size();
        let buffer = list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values();

        buffer
            .chunks_exact(2)
            .map(|x| Rcplx::new(x[0], x[1]))
            .collect_rarray([nrows / 2, ncols])
            .unwrap()
    }

    /// Returns the pointer adress as a string.
    pub fn mem_adress(&self) -> String {
        let p = self
            .inner()
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<f64>>()
            .unwrap()
            .values()
            .as_slice();
        format!("{:p}", p)
    }
}

macro_rules! impl_display_hmatrixr_float {
    ($(($t1:ty, $t2:ty)),+) => {
        $(
            impl fmt::Display for $t1 {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let ncols = self.inner().inner().len();
                    let nrows = self.inner().inner().size();
                    let values = self.inner().inner().values().as_any().downcast_ref::<PrimitiveArray<$t2>>().unwrap().values();
                    for rows in values.chunks_exact(nrows).take(5) {
                        let mut first = true;
                        for item in rows.iter().take(5) {
                            if !first {
                                write!(f, ", {}", item)?;
                            } else {
                                write!(f, "[{}", item)?;
                            }
                            first = false;
                        }
                        if nrows > 5 {
                            writeln!(f, ", ...]")?;
                        } else {
                            writeln!(f, "]")?;
                        }
                    }
                    if ncols > 5 {
                        writeln!(f, "...")?;
                    }
                    if nrows == 1 {
                    write!(f, "{nrows} row x ")?;
                    } else {
                    write!(f, "{nrows} rows x ")?;
                    }
                    if ncols == 1 {
                    write!(f, "{ncols} col")?;
                    } else {
                    write!(f, "{ncols} cols")?;
                    }
                    Ok(())
                }
            }
        )+
    };
}

macro_rules! impl_display_hmatrixr_complex {
    ($(($t1:ty, $t2:ty)),+) => {
        $(
            impl fmt::Display for $t1 {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let ncols = self.inner().inner().len();
                    let nrows = self.inner().inner().size();
                    let values = self.inner().inner().values().as_any().downcast_ref::<PrimitiveArray<$t2>>().unwrap().values();
                    for rows in values.chunks_exact(nrows).take(5) {
                        let mut first = true;
                        for item in rows.chunks_exact(2).take(5) {
                            if !first {
                                write!(f, ", {}", item[0])?;
                                if item[1] >= 0. || item[1].is_nan() {
                                    write!(f, "+{}i", item[1])?;
                                } else {
                                    write!(f, "-{}i", item[1].abs())?;
                                }
                            } else {
                                write!(f, "[{}", item[0])?;
                                if item[1] >= 0. || item[1].is_nan() {
                                    write!(f, "+{}i", item[1])?;
                                } else {
                                    write!(f, "-{}i", item[1].abs())?;
                                }
                            }
                            first = false;
                        }
                        if nrows > 10 {
                            writeln!(f, ", ...]")?;
                        } else {
                            writeln!(f, "]")?;
                        }
                    }
                    if ncols > 5 {
                        writeln!(f, "...")?;
                    }
                    if nrows == 2 {
                        write!(f, "{} row x ", nrows / 2)?;
                    } else {
                        write!(f, "{} rows x ", nrows / 2)?;
                    }
                    if ncols == 1 {
                        write!(f, "{ncols} col")?;
                    } else {
                        write!(f, "{ncols} cols")?;
                    }
                    Ok(())
                }
            }
        )+
    };
}

impl_display_hmatrixr_float!((HFloat32MatrixR, f32), (HFloat64MatrixR, f64));
impl_display_hmatrixr_complex!((HComplex32MatrixR, f32), (HComplex64MatrixR, f64));

extendr_module! {
    mod hmatrix;
    impl HFloat32MatrixR;
    impl HFloat64MatrixR;
    impl HComplex32MatrixR;
    impl HComplex64MatrixR;
}
