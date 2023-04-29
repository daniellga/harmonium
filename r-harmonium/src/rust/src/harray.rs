use crate::hmatrix::{HComplex32MatrixR, HComplex64MatrixR, HFloat32MatrixR, HFloat64MatrixR};
use arrow2::array::PrimitiveArray;
use extendr_api::prelude::*;
use harmonium_core::structs::{HComplexArray, HFloatArray};
use harmonium_fft::fft::fft_arrow::{FftComplexArray, FftFloatArray};
use std::fmt;

#[derive(Debug)]
pub struct HFloat32ArrayR {
    inner: HFloatArray<f32>,
}

#[derive(Debug)]
pub struct HFloat64ArrayR {
    inner: HFloatArray<f64>,
}

#[derive(Debug)]
pub struct HComplex32ArrayR {
    inner: HComplexArray<f32>,
}

#[derive(Debug)]
pub struct HComplex64ArrayR {
    inner: HComplexArray<f64>,
}

impl HFloat32ArrayR {
    pub(crate) fn new(inner: HFloatArray<f32>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HFloatArray<f32> {
        &self.inner
    }
}
impl HFloat64ArrayR {
    pub(crate) fn new(inner: HFloatArray<f64>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HFloatArray<f64> {
        &self.inner
    }
}
impl HComplex32ArrayR {
    pub(crate) fn new(inner: HComplexArray<f32>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HComplexArray<f32> {
        &self.inner
    }
}
impl HComplex64ArrayR {
    pub(crate) fn new(inner: HComplexArray<f64>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HComplexArray<f64> {
        &self.inner
    }
}

#[extendr(use_try_from = true)]
impl HFloat32ArrayR {
    pub fn new_from_values(rvec: Doubles) -> Self {
        let v: Vec<f32> = rvec.iter().map(|x| x.0 as f32).collect();
        let array = PrimitiveArray::from_vec(v);
        let harray = HFloatArray::<f32>::new(array);
        Self::new(harray)
    }

    pub fn len(&self) -> i32 {
        i32::try_from(self.inner().inner().len()).unwrap()
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HFloat32ArrayR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HFloat32ArrayR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HFloat32ArrayR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        Self::new(self.inner().clone())
    }

    /// Creates a new HFloat32ArrayR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let slice = self.inner().inner().values().as_slice();
        let array = PrimitiveArray::<f32>::from_slice(slice);
        let harray = HFloatArray::<f32>::new(array);
        Self::new(harray)
    }

    pub fn fft(&self) -> HComplex32ArrayR {
        HComplex32ArrayR::new(self.inner().fft())
    }

    /// Converts to HFloat32MatrixR. The new HFloat32MatrixR Uses the same underlying data as the HFloat32ArrayR.
    pub fn as_hmatrix(&self, ncols: i32) -> HFloat32MatrixR {
        let hmatrix = self
            .inner()
            .clone()
            .into_hmatrix(usize::try_from(ncols).unwrap())
            .unwrap();
        HFloat32MatrixR::new(hmatrix)
    }

    pub fn collect(&self) -> Doubles {
        let values = self.inner.inner().values();
        let doubles = values
            .iter()
            .map(|x| Rfloat(*x as f64))
            .collect::<Doubles>();
        doubles
    }

    pub fn mem_adress(&self) -> String {
        let p = self.inner().inner().values().as_slice();
        format!("{:p}", p)
    }
}

#[extendr(use_try_from = true)]
impl HFloat64ArrayR {
    pub fn new_from_values(rvec: Doubles) -> Self {
        let v: Vec<f64> = rvec.iter().map(|x| x.0).collect();
        let array = PrimitiveArray::from_vec(v);
        let harray = HFloatArray::<f64>::new(array);
        Self::new(harray)
    }

    pub fn len(&self) -> i32 {
        i32::try_from(self.inner().inner().len()).unwrap()
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HFloat64ArrayR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HFloat64ArrayR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HFloat64ArrayR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        Self::new(self.inner().clone())
    }

    /// Creates a new HFloat64ArrayR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let slice = self.inner().inner().values().as_slice();
        let array = PrimitiveArray::<f64>::from_slice(slice);
        let harray = HFloatArray::<f64>::new(array);
        Self::new(harray)
    }

    pub fn fft(&self) -> HComplex64ArrayR {
        HComplex64ArrayR::new(self.inner().fft())
    }

    /// Converts to HFloat64MatrixR. The new HFloat64MatrixR Uses the same underlying data as the HFloat64ArrayR.
    pub fn as_hmatrix(&self, ncols: i32) -> HFloat64MatrixR {
        let hmatrix = self
            .inner()
            .clone()
            .into_hmatrix(usize::try_from(ncols).unwrap())
            .unwrap();
        HFloat64MatrixR::new(hmatrix)
    }

    pub fn collect(&self) -> Doubles {
        let values = self.inner.inner().values();
        let doubles = values.iter().map(|x| Rfloat(*x)).collect::<Doubles>();
        doubles
    }

    pub fn mem_adress(&self) -> String {
        let p = self.inner().inner().values().as_slice();
        format!("{:p}", p)
    }
}

#[extendr(use_try_from = true)]
impl HComplex32ArrayR {
    pub fn new_from_values(rvec: Complexes) -> Self {
        let length = rvec.len() * 2;
        let mut v: Vec<f32> = Vec::with_capacity(length);
        for x in rvec.iter() {
            v.push(x.re().0 as f32);
            v.push(x.im().0 as f32);
        }
        let arr = PrimitiveArray::from_vec(v);
        let harray = HComplexArray::<f32>::new(arr);
        Self::new(harray)
    }

    pub fn len(&self) -> i32 {
        i32::try_from(self.inner().inner().len()).unwrap()
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HComplex32ArrayR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HComplex32ArrayR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HComplex32ArrayR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        Self::new(self.inner().clone())
    }

    /// Creates a new HComplex32ArrayR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let slice = self.inner().inner().values().as_slice();
        let array = PrimitiveArray::<f32>::from_slice(slice);
        let harray = HComplexArray::<f32>::new(array);
        Self::new(harray)
    }

    pub fn fft(&self) -> Self {
        Self::new(self.inner().fft())
    }

    /// Converts to HComplex32MatrixR. The new HComplex32MatrixR Uses the same underlying data as the HComplex32ArrayR.
    pub fn as_hmatrix(&self, ncols: i32) -> HComplex32MatrixR {
        let hmatrix = self
            .inner()
            .clone()
            .into_hmatrix(usize::try_from(ncols).unwrap())
            .unwrap();
        HComplex32MatrixR::new(hmatrix)
    }

    pub fn collect(&self) -> Complexes {
        let values = self.inner.inner().values();
        let complexes = values
            .chunks_exact(2)
            .map(|x| Rcplx::new((*x)[0] as f64, (*x)[1] as f64))
            .collect::<Complexes>();
        complexes
    }

    pub fn mem_adress(&self) -> String {
        let p = self.inner().inner().values().as_slice();
        format!("{:p}", p)
    }
}

#[extendr(use_try_from = true)]
impl HComplex64ArrayR {
    pub fn new_from_values(rvec: Complexes) -> Self {
        let length = rvec.len() * 2;
        let mut v: Vec<f64> = Vec::with_capacity(length);
        for x in rvec.iter() {
            v.push(x.re().0);
            v.push(x.im().0);
        }
        let arr = PrimitiveArray::from_vec(v);
        let harray = HComplexArray::<f64>::new(arr);
        Self::new(harray)
    }

    pub fn len(&self) -> i32 {
        i32::try_from(self.inner().inner().len()).unwrap()
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HComplex64ArrayR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HComplex64ArrayR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HComplex64ArrayR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        Self::new(self.inner().clone())
    }

    /// Creates a new HComplex64ArrayR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let slice = self.inner().inner().values().as_slice();
        let array = PrimitiveArray::<f64>::from_slice(slice);
        let harray = HComplexArray::<f64>::new(array);
        Self::new(harray)
    }

    pub fn fft(&self) -> Self {
        Self::new(self.inner().fft())
    }

    /// Converts to HComplex64MatrixR. The new HComplex64MatrixR Uses the same underlying data as the HComplex64ArrayR.
    pub fn as_hmatrix(&self, ncols: i32) -> HComplex64MatrixR {
        let hmatrix = self
            .inner
            .clone()
            .into_hmatrix(usize::try_from(ncols).unwrap())
            .unwrap();
        HComplex64MatrixR::new(hmatrix)
    }

    pub fn collect(&self) -> Complexes {
        let values = self.inner.inner().values();
        let complexes = values
            .chunks_exact(2)
            .map(|x| Rcplx::new((*x)[0], (*x)[1]))
            .collect::<Complexes>();
        complexes
    }

    pub fn mem_adress(&self) -> String {
        let p = self.inner().inner().values().as_slice();
        format!("{:p}", p)
    }
}

macro_rules! impl_display_harrayr_float {
    ($($t:ty),+) => {
        $(
            impl fmt::Display for $t {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let length = self.inner().inner().len();
                    let values = self.inner().inner().values();
                    let mut first = true;
                    for item in values.iter().take(5) {
                        if !first {
                            write!(f, ", {}", item)?;
                        } else {
                            write!(f, "[{}", item)?;
                        }
                        first = false;
                    }
                    if length > 5 {
                        write!(f, ", ...]\nlen = {}", length)?;
                    } else {

                        write!(f, "]\nlen = {}", length)?;
                    }
                    Ok(())
                }
            }
        )+
    };
}

impl_display_harrayr_float!(HFloat32ArrayR, HFloat64ArrayR);

macro_rules! impl_display_harrayr_complex {
    ($($t:ty),+) => {
        $(
            impl fmt::Display for $t {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let length = self.inner().inner().len();
                    let values = self.inner().inner().values();
                    let mut first = true;
                    for item in values.chunks_exact(2).take(5) {
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
                    if length > 10 {
                        write!(f, ", ...]\nlen = {}", length / 2)?;
                    } else {
                        write!(f, "]\nlen = {}", length / 2)?;
                    }
                    Ok(())
                }
            }
        )+
    };
}

impl_display_harrayr_complex!(HComplex32ArrayR, HComplex64ArrayR);

extendr_module! {
    mod harray;
    impl HFloat32ArrayR;
    impl HFloat64ArrayR;
    impl HComplex32ArrayR;
    impl HComplex64ArrayR;
}
