use crate::fft::{make_mut_slice, Fft, ProcessFft, RealFftForward};
use harmonium_core::{
    array::HArray,
    errors::{HError, HResult},
};
use ndarray::{s, ArcArray, ArcArray2, Axis, Dimension, Ix1, Ix2, Ix3, IxDyn};
use rustfft::{
    num_complex::{Complex, ComplexFloat},
    num_traits::{ConstZero, Float, FloatConst},
    FftNum,
};
use std::{num::NonZero, sync::Arc};

#[derive(Clone)]
pub struct Stft<T>(Fft<T>);

#[derive(Clone)]
pub struct RealStftForward<T> {
    inner: RealFftForward<T>,
    scratch_real_buffer: Arc<[T]>,
}

#[allow(clippy::len_without_is_empty)]
/// A `Stft` is used to process stft. It caches results internally, so when making more than one stft it is advisable to reuse the same `Stft` instance.
impl<T> Stft<T>
where
    T: FftNum + Float + FloatConst + ConstZero,
{
    pub fn new_forward(length: usize) -> Self {
        Stft(Fft::new_forward(length))
    }

    pub fn len(&self) -> usize {
        self.0.fft.len()
    }
}

#[allow(clippy::len_without_is_empty)]
// A `RealStftForward` is used to process real stft. It caches results internally, so when making more than one stft it is advisable to reuse the same
// `RealStftForward` instance.
impl<T> RealStftForward<T>
where
    T: FftNum + Float + FloatConst + ConstZero,
{
    pub fn new(length: usize) -> Self {
        let scratch_real_buffer = vec![T::ZERO; length];
        let scratch_real_buffer: Arc<[T]> = Arc::from(scratch_real_buffer);

        RealStftForward {
            inner: RealFftForward::new(length),
            scratch_real_buffer,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.fft.len()
    }
}

pub trait ProcessStft<D>
where
    D: Dimension,
{
    type U: ComplexFloat;
    type Output;

    /// Computes the Fourier transform of short overlapping windows of the input.
    /// The function does not normalize outputs.
    ///
    /// If real-valued input, for each forward FFT, transforms a real signal of length `N` to a complex-valued spectrum of length `N/2+1` (with `N/2` rounded down).
    ///
    /// # Arguments
    /// `harray` - A 1D or 2D HArray to be used as input.
    /// `hop_length` - The distance between neighboring sliding window frames.
    /// `window_length` - Size of window frame. Must be greater than the fft length.
    /// `window` - An optional window. `window.len()` must be equal to `window_length`.
    fn process(
        &mut self,
        harray: &HArray<Self::U, D>,
        hop_length: NonZero<usize>,
        window_length: NonZero<usize>,
        window: Option<&[<Self::U as ComplexFloat>::Real]>,
    ) -> HResult<Self::Output>;
}

impl<T> ProcessStft<Ix1> for Stft<T>
where
    T: FftNum + Float + FloatConst + ConstZero,
{
    type U = Complex<T>;
    type Output = HArray<Self::U, Ix2>;

    fn process(
        &mut self,
        harray: &HArray<Self::U, Ix1>,
        hop_length: NonZero<usize>,
        window_length: NonZero<usize>,
        window: Option<&[T]>,
    ) -> HResult<Self::Output> {
        // Since fft_length is checked to be >= window_length and window_length is NonZero<usize>, we can be sure fft_length > 0.
        let fft_length = self.len();
        let window_length = window_length.get();
        let hop_length = hop_length.get();
        let length = harray.len();

        validate_fft_length(fft_length, window_length, length)?;
        validate_window_length(window, window_length)?;

        let n_fft = 1 + (length - fft_length) / hop_length;
        let mut stft_ndarray = ArcArray2::<Complex<T>>::zeros((n_fft, fft_length));

        // Center PAD the window if fft_length > window_length.
        let left = (fft_length - window_length) / 2;
        let right = left + window_length;
        let slice_info = s![.., left..right];
        let slice_info_1d = s![left..right];

        for (mut row, win) in stft_ndarray
            .slice_mut(slice_info)
            .lanes_mut(Axis(1))
            .into_iter()
            .zip(harray.0.windows(fft_length).into_iter().step_by(hop_length))
        {
            row.assign(&win.slice(slice_info_1d));
            if let Some(w) = window {
                row.as_slice_mut().unwrap().apply_window(w);
            }
        }

        let mut output = HArray(stft_ndarray);
        self.0.process(&mut output)?;

        Ok(output)
    }
}

impl<T> ProcessStft<Ix2> for Stft<T>
where
    T: FftNum + Float + FloatConst + ConstZero,
{
    type U = Complex<T>;
    type Output = HArray<Self::U, Ix3>;

    fn process(
        &mut self,
        harray: &HArray<Self::U, Ix2>,
        hop_length: NonZero<usize>,
        window_length: NonZero<usize>,
        window: Option<&[T]>,
    ) -> HResult<Self::Output> {
        // Since fft_length is checked to be >= window_length and window_length is NonZero<usize>, we can be sure fft_length > 0.
        let fft_length = self.len();
        let window_length = window_length.get();
        let hop_length = hop_length.get();
        let nrows = harray.0.len_of(Axis(0));
        let ncols = harray.0.len_of(Axis(1));

        validate_fft_length(fft_length, window_length, ncols)?;
        validate_window_length(window, window_length)?;

        let n_fft = 1 + (ncols - fft_length) / hop_length;
        let mut stft_ndarray = ArcArray::<Complex<T>, Ix3>::zeros((nrows, n_fft, fft_length));

        // Center PAD the window if fft_length > window_length.
        let left = (fft_length - window_length) / 2;
        let right = left + window_length;
        let scratch_buffer = make_mut_slice(&mut self.0.scratch_buffer);

        for (mut col, win_col) in stft_ndarray.lanes_mut(Axis(2)).into_iter().zip(
            harray
                .0
                .windows((1, fft_length))
                .into_iter()
                .step_by(hop_length),
        ) {
            let col_slice = col.as_slice_mut().unwrap();
            let col_slice_sliced = &mut col_slice[left..right];

            col_slice_sliced.copy_from_slice(&win_col.as_slice().unwrap()[left..right]);

            if let Some(w) = window {
                col_slice_sliced.apply_window(w);
            }
            self.0.fft.process_with_scratch(col_slice, scratch_buffer);
        }

        let output = HArray(stft_ndarray);

        Ok(output)
    }
}

impl<T> ProcessStft<IxDyn> for Stft<T>
where
    T: FftNum + Float + FloatConst + ConstZero,
{
    type U = Complex<T>;
    type Output = HArray<Self::U, IxDyn>;

    fn process(
        &mut self,
        harray: &HArray<Self::U, IxDyn>,
        hop_length: NonZero<usize>,
        window_length: NonZero<usize>,
        window: Option<&[T]>,
    ) -> HResult<Self::Output> {
        // Since fft_length is checked to be >= window_length and window_length is NonZero<usize>, we can be sure fft_length > 0.
        let fft_length = self.len();
        let window_length = window_length.get();
        let hop_length = hop_length.get();

        // Center PAD the window if fft_length > window_length.
        let left = (fft_length - window_length) / 2;
        let right = left + window_length;

        match harray.ndim() {
            1 => {
                let length = harray.len();

                validate_fft_length(fft_length, window_length, length)?;
                validate_window_length(window, window_length)?;

                let n_fft = 1 + (length - fft_length) / hop_length;
                let mut stft_ndarray = ArcArray2::<Complex<T>>::zeros((n_fft, fft_length));

                let slice_info = s![.., left..right];
                let slice_info_1d = s![left..right];

                for (mut row, win) in stft_ndarray
                    .slice_mut(slice_info)
                    .lanes_mut(Axis(1))
                    .into_iter()
                    .zip(
                        harray
                            .0
                            .windows(IxDyn(&[fft_length]))
                            .into_iter()
                            .step_by(hop_length),
                    )
                {
                    row.assign(&win.slice(slice_info_1d));
                    if let Some(w) = window {
                        row.as_slice_mut().unwrap().apply_window(w);
                    }
                }

                let mut output = HArray(stft_ndarray.into_dyn());
                self.0.process(&mut output)?;

                Ok(output)
            }
            2 => {
                let nrows = harray.0.len_of(Axis(0));
                let ncols = harray.0.len_of(Axis(1));

                validate_fft_length(fft_length, window_length, ncols)?;
                validate_window_length(window, window_length)?;

                let n_fft = 1 + (ncols - fft_length) / hop_length;
                let mut stft_ndarray =
                    ArcArray::<Complex<T>, Ix3>::zeros((nrows, n_fft, fft_length));

                let slice_info = s![.., left..right];
                let slice_info_1d = s![left..right];
                let scratch_buffer = make_mut_slice(&mut self.0.scratch_buffer);

                for (mut matrix, win) in stft_ndarray.axis_iter_mut(Axis(1)).zip(
                    harray
                        .0
                        .windows(IxDyn(&[nrows, fft_length]))
                        .into_iter()
                        .step_by(hop_length),
                ) {
                    matrix.slice_mut(slice_info).assign(&win.slice(slice_info));

                    for mut col in matrix.lanes_mut(Axis(1)) {
                        if let Some(w) = window {
                            col.slice_mut(slice_info_1d)
                                .as_slice_mut()
                                .unwrap()
                                .apply_window(w);
                        }
                        self.0
                            .fft
                            .process_with_scratch(col.as_slice_mut().unwrap(), scratch_buffer);
                    }
                }

                let output = HArray(stft_ndarray.into_dyn());

                Ok(output)
            }
            _ => Err(HError::OutOfSpecError(
                "The HArray's ndim should be 1 or 2.".into(),
            )),
        }
    }
}

impl<T> ProcessStft<Ix1> for RealStftForward<T>
where
    T: FftNum + Float + FloatConst + ConstZero,
{
    type U = T;
    type Output = HArray<Complex<Self::U>, Ix2>;

    fn process(
        &mut self,
        harray: &HArray<Self::U, Ix1>,
        hop_length: NonZero<usize>,
        window_length: NonZero<usize>,
        window: Option<&[T]>,
    ) -> HResult<Self::Output> {
        // Since fft_length is checked to be >= window_length and window_length is NonZero<usize>, we can be sure fft_length > 0.
        let fft_length = self.len();
        let real_fft_length = fft_length / 2 + 1;
        let window_length = window_length.get();
        let hop_length = hop_length.get();
        let length = harray.len();
        let scratch_real_buffer = make_mut_slice(&mut self.scratch_real_buffer);
        let scratch_buffer = make_mut_slice(&mut self.inner.scratch_buffer);

        validate_fft_length(fft_length, window_length, length)?;
        validate_window_length(window, window_length)?;

        let n_fft = 1 + (length - fft_length) / hop_length;
        let mut stft_ndarray = ArcArray2::<Complex<T>>::zeros((n_fft, real_fft_length));

        // Center PAD the window if fft_length > window_length.
        let left = (fft_length - window_length) / 2;
        let right = left + window_length;

        for (mut row, win) in stft_ndarray
            .lanes_mut(Axis(1))
            .into_iter()
            .zip(harray.0.windows(fft_length).into_iter().step_by(hop_length))
        {
            let scratch_real_buffer_slice = &mut scratch_real_buffer[left..right];
            scratch_real_buffer_slice.copy_from_slice(&win.as_slice().unwrap()[left..right]);
            if let Some(w) = window {
                scratch_real_buffer_slice.apply_window(w);
            }
            self.inner
                .fft
                .process_with_scratch(
                    scratch_real_buffer,
                    row.as_slice_mut().unwrap(),
                    scratch_buffer,
                )
                .unwrap();
        }

        let output = HArray(stft_ndarray);

        Ok(output)
    }
}

impl<T> ProcessStft<Ix2> for RealStftForward<T>
where
    T: FftNum + Float + FloatConst + ConstZero,
{
    type U = T;
    type Output = HArray<Complex<Self::U>, Ix3>;

    fn process(
        &mut self,
        harray: &HArray<Self::U, Ix2>,
        hop_length: NonZero<usize>,
        window_length: NonZero<usize>,
        window: Option<&[T]>,
    ) -> HResult<Self::Output> {
        // Since fft_length is checked to be >= window_length and window_length is NonZero<usize>, we can be sure fft_length > 0.
        let fft_length = self.len();
        let real_fft_length = fft_length / 2 + 1;
        let window_length = window_length.get();
        let hop_length = hop_length.get();
        let nrows = harray.0.len_of(Axis(0));
        let ncols = harray.0.len_of(Axis(1));
        let scratch_real_buffer = make_mut_slice(&mut self.scratch_real_buffer);
        let scratch_buffer = make_mut_slice(&mut self.inner.scratch_buffer);

        validate_fft_length(fft_length, window_length, ncols)?;
        validate_window_length(window, window_length)?;

        let n_fft = 1 + (ncols - fft_length) / hop_length;
        let mut stft_ndarray = ArcArray::<Complex<T>, Ix3>::zeros((nrows, n_fft, real_fft_length));

        // Center PAD the window if fft_length > window_length.
        let left = (fft_length - window_length) / 2;
        let right = left + window_length;

        for (mut col, win_col) in stft_ndarray.lanes_mut(Axis(2)).into_iter().zip(
            harray
                .0
                .windows((1, fft_length))
                .into_iter()
                .step_by(hop_length),
        ) {
            let scratch_real_buffer_slice = &mut scratch_real_buffer[left..right];
            scratch_real_buffer_slice.copy_from_slice(&win_col.as_slice().unwrap()[left..right]);

            if let Some(w) = window {
                scratch_real_buffer_slice.apply_window(w);
            }
            self.inner
                .fft
                .process_with_scratch(
                    scratch_real_buffer,
                    col.as_slice_mut().unwrap(),
                    scratch_buffer,
                )
                .unwrap();
        }

        let output = HArray(stft_ndarray);

        Ok(output)
    }
}

impl<T> ProcessStft<IxDyn> for RealStftForward<T>
where
    T: FftNum + Float + FloatConst + ConstZero,
{
    type U = T;
    type Output = HArray<Complex<Self::U>, IxDyn>;

    fn process(
        &mut self,
        harray: &HArray<Self::U, IxDyn>,
        hop_length: NonZero<usize>,
        window_length: NonZero<usize>,
        window: Option<&[T]>,
    ) -> HResult<Self::Output> {
        // Since fft_length is checked to be >= window_length and window_length is NonZero<usize>, we can be sure fft_length > 0.
        let fft_length = self.len();
        let real_fft_length = fft_length / 2 + 1;
        let window_length = window_length.get();
        let hop_length = hop_length.get();

        // Center PAD the window if fft_length > window_length.
        let left = (fft_length - window_length) / 2;
        let right = left + window_length;

        let scratch_real_buffer = make_mut_slice(&mut self.scratch_real_buffer);
        let scratch_buffer = make_mut_slice(&mut self.inner.scratch_buffer);

        match harray.ndim() {
            1 => {
                let length = harray.len();

                validate_fft_length(fft_length, window_length, length)?;
                validate_window_length(window, window_length)?;

                let n_fft = 1 + (length - fft_length) / hop_length;
                let mut stft_ndarray = ArcArray2::<Complex<T>>::zeros((n_fft, real_fft_length));

                for (mut row, win) in stft_ndarray.lanes_mut(Axis(1)).into_iter().zip(
                    harray
                        .0
                        .windows(IxDyn(&[fft_length]))
                        .into_iter()
                        .step_by(hop_length),
                ) {
                    let scratch_real_buffer_slice = &mut scratch_real_buffer[left..right];
                    scratch_real_buffer_slice
                        .copy_from_slice(&win.as_slice().unwrap()[left..right]);
                    if let Some(w) = window {
                        scratch_real_buffer_slice.apply_window(w);
                    }
                    self.inner
                        .fft
                        .process_with_scratch(
                            scratch_real_buffer,
                            row.as_slice_mut().unwrap(),
                            scratch_buffer,
                        )
                        .unwrap();
                }

                let output = HArray(stft_ndarray.into_dyn());

                Ok(output)
            }
            2 => {
                let nrows = harray.0.len_of(Axis(0));
                let ncols = harray.0.len_of(Axis(1));

                validate_fft_length(fft_length, window_length, ncols)?;
                validate_window_length(window, window_length)?;

                let n_fft = 1 + (ncols - fft_length) / hop_length;
                let mut stft_ndarray =
                    ArcArray::<Complex<T>, Ix3>::zeros((nrows, n_fft, real_fft_length));

                for (mut col, win_col) in stft_ndarray.lanes_mut(Axis(2)).into_iter().zip(
                    harray
                        .0
                        .windows(IxDyn(&[1, fft_length]))
                        .into_iter()
                        .step_by(hop_length),
                ) {
                    let scratch_real_buffer_slice = &mut scratch_real_buffer[left..right];
                    scratch_real_buffer_slice
                        .copy_from_slice(&win_col.as_slice().unwrap()[left..right]);

                    if let Some(w) = window {
                        scratch_real_buffer_slice.apply_window(w);
                    }
                    self.inner
                        .fft
                        .process_with_scratch(
                            scratch_real_buffer,
                            col.as_slice_mut().unwrap(),
                            scratch_buffer,
                        )
                        .unwrap();
                }

                let output = HArray(stft_ndarray.into_dyn());

                Ok(output)
            }
            _ => Err(HError::OutOfSpecError(
                "The HArray's ndim should be 1 or 2.".into(),
            )),
        }
    }
}

trait ApplyWindow<T> {
    fn apply_window(&mut self, window: &[T]);
}

impl<T> ApplyWindow<T> for [Complex<T>]
where
    T: Float,
{
    fn apply_window(&mut self, window: &[T]) {
        for (i, w) in self.iter_mut().zip(window.iter()) {
            *i = (*i).scale(*w);
        }
    }
}

impl<T> ApplyWindow<T> for [T]
where
    T: Float,
{
    fn apply_window(&mut self, window: &[T]) {
        for (i, w) in self.iter_mut().zip(window.iter()) {
            *i = *i * *w;
        }
    }
}

fn validate_fft_length(fft_length: usize, lower: usize, upper: usize) -> HResult<()> {
    if fft_length < lower || fft_length > upper {
        Err(HError::OutOfSpecError(format!(
            "Expected {upper} >= fft_length >= {lower}. Got {fft_length}."
        )))
    } else {
        Ok(())
    }
}

fn validate_window_length<T>(window: Option<&[T]>, window_length: usize) -> HResult<()> {
    if let Some(slice) = window {
        if slice.len() != window_length {
            let got_length = slice.len();
            Err(HError::OutOfSpecError(format!(
                "Expected window length == {window_length}. Got {got_length}."
            )))
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use harmonium_core::{comparison::compare_harray_complex, conversions::IntoDynamic};

    #[test]
    fn stft_1d_test() {
        let fft_length = [3_usize, 5, 5, 5];
        let one_hop_length = NonZero::<usize>::new(1).unwrap();
        let two_hop_length = NonZero::<usize>::new(2).unwrap();
        let hop_length = [
            one_hop_length,
            one_hop_length,
            two_hop_length,
            two_hop_length,
        ];
        let result_no_pad = vec![
            Complex::new(9.0, 12.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(15.0, 18.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(21.0, 24.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(27.0, 30.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
        ];
        let result_pad = vec![
            Complex::new(15.0, 18.0),
            Complex::new(-6.15250, -11.76777),
            Complex::new(5.534407, -2.575299),
            Complex::new(-2.972101, 4.755639),
            Complex::new(-11.40981, -8.41257),
            Complex::new(21.0, 24.0),
            Complex::new(-6.86842, -16.28792),
            Complex::new(6.328012, -4.132835),
            Complex::new(-4.529637, 5.549243),
            Complex::new(-15.92996, -9.12849),
        ];
        let result_pad_hop_length = vec![
            Complex::new(15.0, 18.0),
            Complex::new(-6.1525, -11.76777),
            Complex::new(5.534407, -2.575299),
            Complex::new(-2.972101, 4.755639),
            Complex::new(-11.40981, -8.41257),
        ];
        let result_pad_hop_length_window = vec![
            Complex::new(34.0, 40.0),
            Complex::new(-27.40167, -24.27608),
            Complex::new(20.9163, -4.33643),
            Complex::new(-6.61134, 20.11352),
            Complex::new(-20.90328, -31.50101),
        ];

        let result = [
            result_no_pad,
            result_pad,
            result_pad_hop_length,
            result_pad_hop_length_window,
        ];

        let window = [None, None, None, Some([1., 2., 3.].as_slice())];

        let input = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let length = input.len();
        let window_length = NonZero::new(3).unwrap();

        for (((fft_length, hop_length), result), window) in fft_length
            .into_iter()
            .zip(hop_length.into_iter())
            .zip(result.iter())
            .zip(window.into_iter())
        {
            // Ix1 test.
            let harray = HArray::new_from_shape_vec(length, input.clone()).unwrap();
            let mut stft = Stft::<f32>::new_forward(fft_length);
            let stft_harray = stft
                .process(&harray, hop_length, window_length, window)
                .unwrap();
            let n_fft = 1 + (harray.len() - fft_length) / hop_length;
            let rhs = HArray::new_from_shape_vec((n_fft, fft_length), result.clone()).unwrap();
            assert!(compare_harray_complex(&stft_harray, &rhs));

            // IxDyn test.
            let harray = HArray::new_from_shape_vec(length, input.clone())
                .unwrap()
                .into_dynamic();
            let mut stft = Stft::<f32>::new_forward(fft_length);
            let stft_harray = stft
                .process(&harray, hop_length, window_length, window)
                .unwrap();
            let n_fft = 1 + (harray.len() - fft_length) / hop_length;
            let rhs = HArray::new_from_shape_vec((n_fft, fft_length), result.clone())
                .unwrap()
                .into_dynamic();
            assert!(compare_harray_complex(&stft_harray, &rhs));
        }
    }

    #[test]
    fn stft_2d_test() {
        let fft_length = [3_usize, 5, 5, 5];
        let one_hop_length = NonZero::<usize>::new(1).unwrap();
        let two_hop_length = NonZero::<usize>::new(2).unwrap();
        let hop_length = [
            one_hop_length,
            one_hop_length,
            two_hop_length,
            two_hop_length,
        ];
        let result_no_pad = vec![
            Complex::new(9.0, 12.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(15.0, 18.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(21.0, 24.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(27.0, 30.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(9.0, 12.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(15.0, 18.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(21.0, 24.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
            Complex::new(27.0, 30.0),
            Complex::new(-4.732051, -1.2679492),
            Complex::new(-1.2679492, -4.732051),
        ];
        let result_pad = vec![
            Complex::new(15.0, 18.0),
            Complex::new(-6.15250, -11.76777),
            Complex::new(5.534407, -2.575299),
            Complex::new(-2.972101, 4.755639),
            Complex::new(-11.40981, -8.41257),
            Complex::new(21.0, 24.0),
            Complex::new(-6.86842, -16.28792),
            Complex::new(6.328012, -4.132835),
            Complex::new(-4.529637, 5.549243),
            Complex::new(-15.92996, -9.12849),
            Complex::new(15.0, 18.0),
            Complex::new(-6.15250, -11.76777),
            Complex::new(5.534407, -2.575299),
            Complex::new(-2.972101, 4.755639),
            Complex::new(-11.40981, -8.41257),
            Complex::new(21.0, 24.0),
            Complex::new(-6.86842, -16.28792),
            Complex::new(6.328012, -4.132835),
            Complex::new(-4.529637, 5.549243),
            Complex::new(-15.92996, -9.12849),
        ];
        let result_pad_hop_length = vec![
            Complex::new(15.0, 18.0),
            Complex::new(-6.1525, -11.76777),
            Complex::new(5.534407, -2.575299),
            Complex::new(-2.972101, 4.755639),
            Complex::new(-11.40981, -8.41257),
            Complex::new(15.0, 18.0),
            Complex::new(-6.1525, -11.76777),
            Complex::new(5.534407, -2.575299),
            Complex::new(-2.972101, 4.755639),
            Complex::new(-11.40981, -8.41257),
        ];
        let result_pad_hop_length_window = vec![
            Complex::new(34.0, 40.0),
            Complex::new(-27.40167, -24.27608),
            Complex::new(20.9163, -4.33643),
            Complex::new(-6.61134, 20.11352),
            Complex::new(-20.90328, -31.50101),
            Complex::new(34.0, 40.0),
            Complex::new(-27.40167, -24.27608),
            Complex::new(20.9163, -4.33643),
            Complex::new(-6.61134, 20.11352),
            Complex::new(-20.90328, -31.50101),
        ];

        let result = [
            result_no_pad,
            result_pad,
            result_pad_hop_length,
            result_pad_hop_length_window,
        ];

        let window = [None, None, None, Some([1., 2., 3.].as_slice())];

        let input = vec![
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
            Complex::new(1_f32, 2_f32),
            Complex::new(3_f32, 4_f32),
            Complex::new(5_f32, 6_f32),
            Complex::new(7_f32, 8_f32),
            Complex::new(9_f32, 10_f32),
            Complex::new(11_f32, 12_f32),
        ];
        let length = input.len();
        let window_length = NonZero::new(3).unwrap();

        for (((fft_length, hop_length), result), window) in fft_length
            .into_iter()
            .zip(hop_length.into_iter())
            .zip(result.iter())
            .zip(window.into_iter())
        {
            // Ix2 test.
            let harray = HArray::new_from_shape_vec((2, length / 2), input.clone()).unwrap();
            let mut stft = Stft::<f32>::new_forward(fft_length);
            let stft_harray = stft
                .process(&harray, hop_length, window_length, window)
                .unwrap();
            let ncols = harray.0.len_of(Axis(1));
            let n_fft = 1 + (ncols - fft_length) / hop_length;
            let rhs = HArray::new_from_shape_vec((2, n_fft, fft_length), result.clone()).unwrap();
            assert!(compare_harray_complex(&stft_harray, &rhs));

            // IxDyn test.
            let harray = HArray::new_from_shape_vec((2, length / 2), input.clone())
                .unwrap()
                .into_dynamic();
            let mut stft = Stft::<f32>::new_forward(fft_length);
            let stft_harray = stft
                .process(&harray, hop_length, window_length, window)
                .unwrap();
            let ncols = harray.0.len_of(Axis(1));
            let n_fft = 1 + (ncols - fft_length) / hop_length;
            let rhs = HArray::new_from_shape_vec((2, n_fft, fft_length), result.clone())
                .unwrap()
                .into_dynamic();
            assert!(compare_harray_complex(&stft_harray, &rhs));
        }
    }

    #[test]
    fn real_stft_1d_test() {
        let fft_length = [3_usize, 5, 5, 5];
        let one_hop_length = NonZero::<usize>::new(1).unwrap();
        let two_hop_length = NonZero::<usize>::new(2).unwrap();
        let hop_length = [
            one_hop_length,
            one_hop_length,
            two_hop_length,
            two_hop_length,
        ];
        let result_no_pad = vec![
            Complex::new(6.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(9.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(12.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(15.0, 0.0),
            Complex::new(-1.5, 0.8660254),
        ];
        let result_pad = vec![
            Complex::new(9.0, 0.0),
            Complex::new(-5.045085, -1.314328),
            Complex::new(0.5450851, -2.126627),
            Complex::new(12.0, 0.0),
            Complex::new(-6.354102, -2.265384),
            Complex::new(0.3541019, -2.714412),
        ];
        let result_pad_hop_length = vec![
            Complex::new(9.0, 0.0),
            Complex::new(-5.045085, -1.314328),
            Complex::new(0.5450851, -2.126627),
        ];
        let result_pad_hop_length_window = vec![
            Complex::new(20.0, 0.0),
            Complex::new(-13.94427, 1.624598),
            Complex::new(3.944272, -6.88191),
        ];

        let result = [
            result_no_pad,
            result_pad,
            result_pad_hop_length,
            result_pad_hop_length_window,
        ];

        let window = [None, None, None, Some([1., 2., 3.].as_slice())];

        let input = vec![1., 2., 3., 4., 5., 6.];
        let length = input.len();
        let window_length = NonZero::new(3).unwrap();

        for (((fft_length, hop_length), result), window) in fft_length
            .into_iter()
            .zip(hop_length.into_iter())
            .zip(result.iter())
            .zip(window.into_iter())
        {
            // Ix1 test.
            let harray = HArray::new_from_shape_vec(length, input.clone()).unwrap();
            let mut stft = RealStftForward::<f32>::new(fft_length);
            let stft_harray = stft
                .process(&harray, hop_length, window_length, window)
                .unwrap();
            let n_fft = 1 + (harray.len() - fft_length) / hop_length;
            let rhs =
                HArray::new_from_shape_vec((n_fft, fft_length / 2 + 1), result.clone()).unwrap();
            assert!(compare_harray_complex(&stft_harray, &rhs));

            // IxDyn test.
            let harray = HArray::new_from_shape_vec(length, input.clone())
                .unwrap()
                .into_dynamic();
            let mut stft = RealStftForward::<f32>::new(fft_length);
            let stft_harray = stft
                .process(&harray, hop_length, window_length, window)
                .unwrap();
            let n_fft = 1 + (harray.len() - fft_length) / hop_length;
            let rhs = HArray::new_from_shape_vec((n_fft, fft_length / 2 + 1), result.clone())
                .unwrap()
                .into_dynamic();
            assert!(compare_harray_complex(&stft_harray, &rhs));
        }
    }

    #[test]
    fn real_stft_2d_test() {
        let fft_length = [3_usize, 5, 5, 5];
        let one_hop_length = NonZero::<usize>::new(1).unwrap();
        let two_hop_length = NonZero::<usize>::new(2).unwrap();
        let hop_length = [
            one_hop_length,
            one_hop_length,
            two_hop_length,
            two_hop_length,
        ];
        let result_no_pad = vec![
            Complex::new(6.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(9.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(12.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(15.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(6.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(9.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(12.0, 0.0),
            Complex::new(-1.5, 0.8660254),
            Complex::new(15.0, 0.0),
            Complex::new(-1.5, 0.8660254),
        ];
        let result_pad = vec![
            Complex::new(9.0, 0.0),
            Complex::new(-5.045085, -1.314328),
            Complex::new(0.5450851, -2.126627),
            Complex::new(12.0, 0.0),
            Complex::new(-6.354102, -2.265384),
            Complex::new(0.3541019, -2.714412),
            Complex::new(9.0, 0.0),
            Complex::new(-5.045085, -1.314328),
            Complex::new(0.5450851, -2.126627),
            Complex::new(12.0, 0.0),
            Complex::new(-6.354102, -2.265384),
            Complex::new(0.3541019, -2.714412),
        ];
        let result_pad_hop_length = vec![
            Complex::new(9.0, 0.0),
            Complex::new(-5.045085, -1.314328),
            Complex::new(0.5450851, -2.126627),
            Complex::new(9.0, 0.0),
            Complex::new(-5.045085, -1.314328),
            Complex::new(0.5450851, -2.126627),
        ];
        let result_pad_hop_length_window = vec![
            Complex::new(20.0, 0.0),
            Complex::new(-13.94427, 1.624598),
            Complex::new(3.944272, -6.88191),
            Complex::new(20.0, 0.0),
            Complex::new(-13.94427, 1.624598),
            Complex::new(3.944272, -6.88191),
        ];

        let result = [
            result_no_pad,
            result_pad,
            result_pad_hop_length,
            result_pad_hop_length_window,
        ];

        let window = [None, None, None, Some([1., 2., 3.].as_slice())];

        let input = vec![1., 2., 3., 4., 5., 6., 1., 2., 3., 4., 5., 6.];
        let length = input.len();
        let window_length = NonZero::new(3).unwrap();

        for (((fft_length, hop_length), result), window) in fft_length
            .into_iter()
            .zip(hop_length.into_iter())
            .zip(result.iter())
            .zip(window.into_iter())
        {
            // Ix2 test.
            let harray = HArray::new_from_shape_vec((2, length / 2), input.clone()).unwrap();
            let mut stft = RealStftForward::<f32>::new(fft_length);
            let stft_harray = stft
                .process(&harray, hop_length, window_length, window)
                .unwrap();
            let ncols = harray.0.len_of(Axis(1));
            let n_fft = 1 + (ncols - fft_length) / hop_length;
            let rhs =
                HArray::new_from_shape_vec((2, n_fft, fft_length / 2 + 1), result.clone()).unwrap();
            assert!(compare_harray_complex(&stft_harray, &rhs));

            // IxDyn test.
            let harray = HArray::new_from_shape_vec((2, length / 2), input.clone())
                .unwrap()
                .into_dynamic();
            let mut stft = RealStftForward::<f32>::new(fft_length);
            let stft_harray = stft
                .process(&harray, hop_length, window_length, window)
                .unwrap();
            let ncols = harray.0.len_of(Axis(1));
            let n_fft = 1 + (ncols - fft_length) / hop_length;
            let rhs = HArray::new_from_shape_vec((2, n_fft, fft_length / 2 + 1), result.clone())
                .unwrap()
                .into_dynamic();
            assert!(compare_harray_complex(&stft_harray, &rhs));
        }
    }
}
