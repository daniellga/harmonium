use crate::hmatrix::{HFloat32MatrixR, HFloat64MatrixR};
use arrow2::array::PrimitiveArray;
use extendr_api::prelude::*;
use harmonium_core::structs::{HAudio, HFloatArray};
use harmonium_io::decode::decode_arrow::decode;
use std::fmt;

#[derive(Debug)]
pub struct HFloat32AudioR {
    inner: HAudio<f32>,
}

#[derive(Debug)]
pub struct HFloat64AudioR {
    inner: HAudio<f64>,
}

impl HFloat32AudioR {
    pub(crate) fn new(inner: HAudio<f32>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HAudio<f32> {
        &self.inner
    }
}

impl HFloat64AudioR {
    pub(crate) fn new(inner: HAudio<f64>) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &HAudio<f64> {
        &self.inner
    }
}

#[extendr(use_try_from = true)]
impl HFloat32AudioR {
    pub fn new_from_file(
        fpath: &str,
        #[default = "NA_real_"] offset: Option<f64>,
        #[default = "NA_real_"] duration: Option<f64>,
    ) -> Self {
        let haudio = decode::<f32>(fpath, offset, duration).unwrap();
        Self::new(haudio)
    }

    pub fn new_from_values(rmatrix: RMatrix<f64>, sr: i32) -> Self {
        let ncols = rmatrix.ncols();
        let v: Vec<f32> = rmatrix.data().iter().map(|z| *z as f32).collect();
        let array = PrimitiveArray::from_vec(v);
        let harray = HFloatArray::<f32>::new(array);
        let haudio = harray
            .into_hmatrix(ncols)
            .unwrap()
            .into_haudio(u32::try_from(sr).unwrap());
        Self::new(haudio)
    }

    /// Converts to HComplex64ArrayR. The new HComplex64ArrayR Uses the same underlying data as the HComplex64MatrixR.
    pub fn as_hmatrix(&self) -> HFloat32MatrixR {
        let harray = self.inner().inner().clone();
        HFloat32MatrixR::new(harray)
    }

    pub fn sr(&self) -> i32 {
        i32::try_from(self.inner().sr()).unwrap()
    }

    pub fn nchannels(&self) -> i32 {
        i32::try_from(self.inner().nchannels()).unwrap()
    }

    pub fn nframes(&self) -> i32 {
        i32::try_from(self.inner().nframes()).unwrap()
    }

    pub fn duration(&self) -> f64 {
        self.inner().duration()
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HFloat32AudioR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HFloat32AudioR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HFloat32AudioR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        HFloat32AudioR::new(self.inner().clone())
    }

    /// Creates a new HFloat32AudioR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let sr = self.inner().sr();
        let list_array = self.inner().inner().inner();
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
        let haudio = harray.into_hmatrix(ncols).unwrap().into_haudio(sr);
        HFloat32AudioR::new(haudio)
    }

    pub fn collect(&self) -> RMatrix<f64> {
        let hmatrix = self.inner().inner();
        let hmatrixr = HFloat32MatrixR::new(hmatrix.clone());
        hmatrixr.collect()
    }

    /// Returns the pointer adress as a string.
    pub fn mem_adress(&self) -> String {
        let p = self
            .inner()
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

    /// Convert to 1 channel taking the average across channels. A new matrix is created.
    pub fn to_mono(&mut self) {
        self.inner.to_mono().unwrap();
    }
}

#[extendr(use_try_from = true)]
impl HFloat64AudioR {
    pub fn new_from_file(
        fpath: &str,
        #[default = "NA_real_"] offset: Option<f64>,
        #[default = "NA_real_"] duration: Option<f64>,
    ) -> Self {
        let haudio = decode::<f64>(fpath, offset, duration).unwrap();
        Self::new(haudio)
    }

    pub fn new_from_values(rmatrix: RMatrix<f64>, sr: i32) -> Self {
        let ncols = rmatrix.ncols();
        let v: Vec<f64> = rmatrix.data().to_vec();
        let array = PrimitiveArray::from_vec(v);
        let harray = HFloatArray::<f64>::new(array);
        let haudio = harray.into_hmatrix(ncols).unwrap().into_haudio(sr as u32);
        Self::new(haudio)
    }

    /// Converts to HComplex64ArrayR. The new HComplex64ArrayR Uses the same underlying data as the HComplex64MatrixR.
    pub fn as_hmatrix(&self) -> HFloat64MatrixR {
        let harray = self.inner().inner().clone();
        HFloat64MatrixR::new(harray)
    }

    pub fn sr(&self) -> i32 {
        i32::try_from(self.inner().sr()).unwrap()
    }

    pub fn nchannels(&self) -> i32 {
        i32::try_from(self.inner().nchannels()).unwrap()
    }

    pub fn nframes(&self) -> i32 {
        i32::try_from(self.inner().nframes()).unwrap()
    }

    pub fn duration(&self) -> f64 {
        self.inner().duration()
    }

    pub fn print(&self) {
        rprintln!("{}", self);
    }

    pub fn eq(&self, other: &HFloat64AudioR) -> bool {
        self.inner().eq(other.inner())
    }

    pub fn ne(&self, other: &HFloat64AudioR) -> bool {
        self.inner().ne(other.inner())
    }

    /// Creates a new HFloat64AudioR, with the underlying data pointing to the same place in memory.
    pub fn clone(&self) -> Self {
        HFloat64AudioR::new(self.inner().clone())
    }

    /// Creates a new HFloat64AudioR, similar to the first one, but with the underlying data pointing to a different place in memory.
    pub fn copy(&self) -> Self {
        let sr = self.inner().sr();
        let list_array = self.inner().inner().inner();
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
        let haudio = harray.into_hmatrix(ncols).unwrap().into_haudio(sr);
        HFloat64AudioR::new(haudio)
    }

    pub fn collect(&self) -> RMatrix<f64> {
        let hmatrix = self.inner().inner();
        let hmatrixr = HFloat64MatrixR::new(hmatrix.clone());
        hmatrixr.collect()
    }

    /// Returns the pointer adress as a string.
    pub fn mem_adress(&self) -> String {
        let p = self
            .inner()
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

    /// Convert to 1 channel taking the average across channels. A new matrix is created.
    pub fn to_mono(&mut self) {
        self.inner.to_mono().unwrap();
    }
}

macro_rules! impl_display_haudior {
    ($(($t1:ty, $t2:ty)),+) => {
        $(
            impl fmt::Display for $t1 {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "sr = {}\n", self.sr())?;
                    let ncols = self.inner().inner().inner().len();
                    let nrows = self.inner().inner().inner().size();
                    let values = self.inner().inner().inner().values().as_any().downcast_ref::<PrimitiveArray<$t2>>().unwrap().values();
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
                            write!(f, ", ...]\n")?;
                        } else {
                            write!(f, "]\n")?;
                        }
                    }
                    if ncols > 5 {
                        write!(f, "...\n")?;
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

impl_display_haudior!((HFloat32AudioR, f32), (HFloat64AudioR, f64));

extendr_module! {
    mod haudio;
    impl HFloat32AudioR;
    impl HFloat64AudioR;
}
