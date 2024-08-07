use std::time::Duration;

use harmonium_core::{
    audioop::{Audio, AudioOp},
    errors::{HError, HResult},
};
use ndarray::Array1;
use num_traits::{Float, FloatConst, FromPrimitive};
use rodio::{
    buffer::SamplesBuffer,
    cpal::{traits::HostTrait, SupportedBufferSize},
    DeviceTrait, OutputStream, Sink,
};

use crate::decode::decode;

pub struct HAudioSink {
    sink: Sink,
    _stream: OutputStream,
}

impl HAudioSink {
    /// Creates a new `HAudioSink` instance. The sink is set on "play" mode from the start.
    pub fn try_new() -> HResult<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(HAudioSink { sink, _stream })
    }

    /// Appends a sound to the queue of sounds to play.
    pub fn append_from_harray<T>(&self, audio: &Audio<T>, sr: u32)
    where
        T: Float + FloatConst + FromPrimitive,
    {
        match audio {
            Audio::D1(harray) => {
                let ndarray = harray.0.mapv(|x| {
                    // This should not panic since it is a conversion from f32 or f64.
                    unsafe { x.to_f32().unwrap_unchecked() }
                });
                let source = SamplesBuffer::new(1, sr, ndarray.as_slice().unwrap());
                self.sink.append(source);
            }
            Audio::D2(harray) => {
                let nchannels = harray.nchannels();
                let nframes = harray.nframes();

                let mut ndarray = Array1::zeros(nchannels * nframes);

                let ndarray_interleaved_t = harray.0.view().reversed_axes();

                for (a, b) in ndarray.iter_mut().zip(ndarray_interleaved_t.iter()) {
                    // This should not panic since it is a conversion from f32 or f64.
                    unsafe {
                        *a = b.to_f32().unwrap_unchecked();
                    }
                }

                let source = SamplesBuffer::new(
                    u16::try_from(nchannels).unwrap(),
                    sr,
                    ndarray.as_slice().unwrap(),
                );

                self.sink.append(source);
            }
            Audio::Dyn(harray) => {
                assert!(harray.ndim() == 2);
                let nchannels = harray.nchannels();
                let nframes = harray.nframes();

                let mut ndarray = Array1::zeros(nchannels * nframes);

                let ndarray_interleaved_t = harray.0.view().reversed_axes();

                for (a, b) in ndarray.iter_mut().zip(ndarray_interleaved_t.iter()) {
                    // This should not panic since it is a conversion from f32 or f64.
                    unsafe {
                        *a = b.to_f32().unwrap_unchecked();
                    }
                }

                let source = SamplesBuffer::new(
                    u16::try_from(nchannels).unwrap(),
                    sr,
                    ndarray.as_slice().unwrap(),
                );

                self.sink.append(source);
            }
        }
    }

    /// Appends a sound to the queue of sounds to play.
    pub fn append_from_file(&self, fpath: &str) -> HResult<()> {
        let (harray, sr) = decode::<f32>(fpath)?;
        let audio = Audio::D2(&harray);
        self.append_from_harray(&audio, sr);
        Ok(())
    }

    /// Removes all currently loaded `Source`s from the `Sink` and pauses it.
    pub fn clear(&self) {
        self.sink.clear()
    }

    /// Destroys the sink without stopping the sounds that are still playing.
    pub fn detach(self) {
        self.sink.detach();
    }

    /// Returns the position of the sound that’s being played.
    /// This takes into account any speedup or delay applied.
    /// Example: if you apply a speedup of 2 to an mp3 decoder source and get_pos() returns 5s then the position in the mp3 recording is 10s from its start.
    pub fn get_pos(&self) -> Duration {
        self.sink.get_pos()
    }

    /// Returns true if this sink has no more sounds to play.
    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    /// Gets if a sink is paused.
    /// Sinks can be paused and resumed using pause() and play(). This returns true if the sink is paused.
    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    /// Returns the number of sounds currently in the queue.
    pub fn len(&self) -> usize {
        self.sink.len()
    }

    /// Pauses playback of this sink.
    /// No effect if already paused.
    /// A paused sink can be resumed with play().
    pub fn pause(&self) {
        self.sink.pause();
    }

    /// Resumes playback of a paused sink.
    /// No effect if not paused.
    pub fn play(&self) {
        self.sink.play();
    }

    /// Changes the speed of the sound.
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.
    pub fn set_speed(&self, value: f32) {
        self.sink.set_speed(value);
    }

    /// Changes the volume of the sound.
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value.
    pub fn set_volume(&self, value: f32) {
        self.sink.set_volume(value);
    }

    /// Skips to the next `Source` in the `Sink`.
    /// If there are more `Source`s appended to the `Sink` at the time, will play the next one. Otherwise, the `Sink` will finish
    /// as if it had finished playing a `Source` all the way through.
    pub fn skip_one(&self) {
        self.sink.skip_one()
    }

    /// Sleeps the current thread until the sound ends.
    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
    }

    /// Gets the speed of the sound.
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.
    pub fn speed(&self) -> f32 {
        self.sink.speed()
    }

    /// Stops the sink by emptying the queue.
    /// The sink will keep its previous state (play or pause).
    pub fn stop(&self) {
        self.sink.stop();
    }

    /// Attempts to seek to a given position in the current source.
    /// This blocks between 0 and ~5 milliseconds.
    /// As long as the duration of the source is known, seek is guaranteed to saturate at the end of the source. For example given a
    /// source that reports a total duration of 42 seconds calling `try_seek()` with 60 seconds as argument will seek to 42 seconds.
    ///
    /// This function will return an error if:
    /// - one of the underlying sources does not support seeking.
    /// - an implementation ran into one during the seek.
    /// - when seeking beyond the end of a source when the duration of the source is not known.
    pub fn try_seek(&self, pos: Duration) -> HResult<()> {
        self.sink.try_seek(pos).map_err(HError::from)
    }

    /// Gets the volume of the sound.
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value.
    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }
}

/// Returns a list of available audio output devices.
pub fn audio_output_devices() -> HResult<Vec<String>> {
    let host = rodio::cpal::default_host();
    let devices = host.output_devices()?;
    let mut strings = Vec::with_capacity(devices.size_hint().0);
    for x in devices {
        strings.push(x.name()?);
    }
    Ok(strings)
}

/// Returns the default audio output device.
pub fn audio_default_device() -> HResult<String> {
    let host = rodio::cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| HError::PlayError("cannot get the default output device".into()))?;
    Ok(device.name()?)
}

/// Returns the supported configurations for the default audio output device.
/// The following informations are provided:
/// * Number of channels.
/// * Minimum and maximum value for the sampling rate.
/// * Minimum and maximum value for the buffer size.
/// * Type of data expected by the device.
pub fn audio_supported_configs() -> HResult<Vec<String>> {
    let host = rodio::cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| HError::PlayError("cannot get the default output device".into()))?;
    let supported_configs = device.supported_output_configs()?;
    let mut strings: Vec<String> = Vec::with_capacity(supported_configs.size_hint().0);
    for x in supported_configs {
        let buffer_size = match x.buffer_size() {
            SupportedBufferSize::Range { min: x, max: y } => format!("({x}, {y})"),
            SupportedBufferSize::Unknown => "Unknown".into(),
        };
        let s = format!(
            "[channels: {}, sample_rate(min, max): ({}, {}), buffer_size(min, max): {}, sample_format: {}]",
            x.channels(),
            x.min_sample_rate().0,
            x.max_sample_rate().0,
            buffer_size,
            x.sample_format()
        );
        strings.push(s);
    }
    Ok(strings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_test() {
        let sink = HAudioSink::try_new().unwrap();
        sink.append_from_file("../testfiles/gs-16b-2c-44100hz.wav")
            .unwrap();
        sink.append_from_file("../testfiles/gs-16b-2c-44100hz.wav")
            .unwrap();
        assert_eq!(sink.len(), 2);

        sink.pause();
        assert!(sink.is_paused());
        sink.play();

        assert_eq!(sink.volume(), 1.);
        sink.set_volume(1.2);
        assert_eq!(sink.volume(), 1.2);

        assert_eq!(sink.speed(), 1.);
        sink.set_speed(3.);
        assert_eq!(sink.speed(), 3.);

        sink.sleep_until_end();
        sink.detach();
    }

    //#[test]
    //fn skip_one_test() {
    //    let sink = HAudioSink::try_new().unwrap();
    //    sink.append_from_file("../testfiles/gs-16b-2c-44100hz.wav")
    //        .unwrap();
    //    sink.append_from_file("../testfiles/gs-16b-2c-44100hz.wav")
    //        .unwrap();
    //    sink.append_from_file("../testfiles/gs-16b-1c-44100hz.wav")
    //        .unwrap();
    //    assert_eq!(sink.len(), 3);
    //    sink.skip_one();
    //    assert_eq!(sink.len(), 2);

    //    sink.sleep_until_end();
    //}

    #[test]
    fn clear_test() {
        let sink = HAudioSink::try_new().unwrap();
        sink.append_from_file("../testfiles/gs-16b-2c-44100hz.wav")
            .unwrap();
        sink.append_from_file("../testfiles/gs-16b-2c-44100hz.wav")
            .unwrap();
        assert_eq!(sink.len(), 2);
        sink.clear();
        assert_eq!(sink.len(), 0);
        sink.append_from_file("../testfiles/gs-16b-2c-44100hz.wav")
            .unwrap();
        assert_eq!(sink.len(), 1);
    }

    #[test]
    fn audio_devices_test() {
        audio_default_device().unwrap();
        audio_supported_configs().unwrap();
        audio_output_devices().unwrap();
    }
}
