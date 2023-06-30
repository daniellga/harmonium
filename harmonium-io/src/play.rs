use arrow2::types::NativeType;
use harmonium_core::{
    errors::{HError, HResult},
    structs::HFloatAudio,
};
use num_traits::{Float, ToPrimitive};
use rodio::{
    buffer::SamplesBuffer,
    cpal::{traits::HostTrait, SupportedBufferSize},
    DeviceTrait, OutputStream, Sink,
};

use crate::decode::decode_arrow::decode;

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
    pub fn append_from_haudio<T>(&self, haudio: &HFloatAudio<T>)
    where
        T: NativeType + Float + ToPrimitive,
    {
        let nchannels = haudio.nchannels();
        let nframes = haudio.nframes();
        let sr = haudio.sr();

        let mut data_interleaved: Vec<f32> = Vec::with_capacity(nchannels * nframes);
        let values = haudio.inner().inner().inner();

        for f in 0..nframes {
            for ch in 0..nchannels {
                data_interleaved.push(values.value(f + ch * nframes).to_f32().unwrap());
            }
        }

        let source = SamplesBuffer::new(u16::try_from(nchannels).unwrap(), sr, data_interleaved);

        self.sink.append(source);
    }

    /// Appends a sound to the queue of sounds to play.
    pub fn append_from_file(&self, fpath: &str) -> HResult<()> {
        let haudio = decode::<f32>(fpath, None, None)?;
        self.append_from_haudio(&haudio);

        Ok(())
    }

    /// Resumes playback of a paused sink.
    /// No effect if not paused.
    pub fn play(&self) {
        self.sink.play();
    }

    /// Stops the sink by emptying the queue.
    /// The sink will keep its previous state (play or pause).
    pub fn stop(&self) {
        self.sink.stop();
    }

    /// Pauses playback of this sink.
    /// No effect if already paused.
    /// A paused sink can be resumed with play().
    pub fn pause(&self) {
        self.sink.pause();
    }

    /// Gets if a sink is paused.
    /// Sinks can be paused and resumed using pause() and play(). This returns true if the sink is paused.
    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    /// Gets the volume of the sound.
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value.
    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    /// Changes the volume of the sound.
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value.
    pub fn set_volume(&self, value: f32) {
        self.sink.set_volume(value);
    }

    /// Gets the speed of the sound.
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.
    pub fn speed(&self) -> f32 {
        self.sink.speed()
    }

    /// Changes the speed of the sound.
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.
    pub fn set_speed(&self, value: f32) {
        self.sink.set_speed(value);
    }

    /// Destroys the sink without stopping the sounds that are still playing.
    pub fn detach(self) {
        self.sink.detach();
    }

    /// Sleeps the current thread until the sound ends.
    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
    }

    /// Returns the number of sounds currently in the queue.
    pub fn len(&self) -> usize {
        self.sink.len()
    }

    /// Returns true if this sink has no more sounds to play.
    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    /// Removes all currently loaded `Source`s from the `Sink` and pauses it.
    pub fn clear(&self) {
        self.sink.clear()
    }

    /// Skips to the next `Source` in the `Sink`.
    /// If there are more `Source`s appended to the `Sink` at the time, will play the next one. Otherwise, the `Sink` will finish
    /// as if it had finished playing a `Source` all the way through.
    pub fn skip_one(&self) {
        self.sink.skip_one()
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

    #[ignore]
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

    #[ignore]
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

    #[ignore]
    #[test]
    fn audio_devices_test() {
        audio_default_device().unwrap();
        audio_supported_configs().unwrap();
        audio_output_devices().unwrap();
    }
}
