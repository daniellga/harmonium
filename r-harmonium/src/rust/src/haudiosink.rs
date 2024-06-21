use crate::{
    conversions::{try_from_usize_to_int_sexp, AsScalar},
    errors::HErrorR,
    harray::HArray,
    hdatatype::HDataType,
};
use harmonium_core::audioop::Audio;
use harmonium_io::{decode::decode, play};
use ndarray::IxDyn;
use savvy::{savvy, OwnedIntegerSexp, OwnedLogicalSexp, OwnedRealSexp, OwnedStringSexp, Sexp};

/// HAudioSink
/// Handle to a device that outputs sounds.
///
/// # Methods
///
#[savvy]
pub struct HAudioSink(play::HAudioSink);

#[savvy]
impl HAudioSink {
    /// HAudioSink
    /// ## new
    ///
    /// `new() -> HAudioSink`
    ///
    /// Creates a new `HAudioSink` instance.
    ///
    /// The sink is set on "play" mode from the start.
    ///
    /// #### Returns
    ///
    /// An `HAudioSink`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// ```
    ///
    /// _________
    ///
    fn new() -> savvy::Result<Self> {
        Ok(Self(play::HAudioSink::try_new().map_err(HErrorR::from)?))
    }

    /// HAudioSink
    /// ## append_from_harray
    ///
    /// `append_from_harray(harray: HArray, sr: integer)`
    ///
    /// Appends a sound to the queue of sounds to play.
    ///
    /// #### Arguments
    ///
    /// - `harray`
    ///
    /// An `HArray`.
    ///
    /// - `sr`
    ///
    /// An integer. The audio sampling rate.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// hdecodedaudio = HFile$decode(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav", dtype = HDataType$Float32)
    /// harray = hdecodedaudio$harray()
    /// sr = hdecodedaudio$sr()
    /// haudiosink$append_from_harray(harray, sr)
    /// ```
    ///
    /// _________
    ///
    fn append_from_harray(&self, harray: &HArray, sr: Sexp) -> savvy::Result<()> {
        let sr: i32 = sr.as_scalar()?;
        let sr = sr
            .try_into()
            .map_err(|_| savvy::Error::new("Cannot convert i32 to u32."))?;

        match harray.0.dtype() {
            HDataType::Float32 => {
                let harray = unsafe {
                    harray
                        .0
                        .as_any()
                        .downcast_ref::<harmonium_core::array::HArray<f32, IxDyn>>()
                        // Should not panic since the type was checked.
                        .unwrap_unchecked()
                };
                let audio = Audio::Dyn(harray);
                self.0.append_from_harray::<f32>(&audio, sr);
                Ok(())
            }
            HDataType::Float64 => {
                let harray = unsafe {
                    harray
                        .0
                        .as_any()
                        .downcast_ref::<harmonium_core::array::HArray<f64, IxDyn>>()
                        // Should not panic since the type was checked.
                        .unwrap_unchecked()
                };
                let audio = Audio::Dyn(harray);
                self.0.append_from_harray::<f64>(&audio, sr);
                Ok(())
            }
            _ => Err("Not a valid HDataType.".into()),
        }
    }

    /// HAudioSink
    /// ## append_from_file
    ///
    /// `append_from_file(fpath: string)`
    ///
    /// Appends a sound to the queue of sounds to play.
    ///
    /// #### Arguments
    ///
    /// - `fpath`
    ///
    /// The file path as a `string`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// ```
    ///
    /// _________
    ///
    fn append_from_file(&self, fpath: Sexp) -> savvy::Result<()> {
        let fpath: &str = fpath.as_scalar()?;

        let (harray, sr) = decode::<f32>(fpath).map_err(HErrorR::from)?;
        let audio = Audio::D2(&harray);
        self.0.append_from_harray(&audio, sr);
        Ok(())
    }

    /// HAudioSink
    /// ## audio_default_device
    ///
    /// `audio_default_device() -> string`
    ///
    /// Informs the default audio output device.
    ///
    /// #### Returns
    ///
    /// A `string`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// HAudioSink$audio_default_device()
    /// ```
    ///
    /// _________
    ///
    fn audio_default_device() -> savvy::Result<Sexp> {
        let default_device = play::audio_default_device().map_err(HErrorR::from)?;
        let string_sexp = OwnedStringSexp::try_from(default_device)?;
        string_sexp.into()
    }

    /// HAudioSink
    /// ## audio_output_devices
    ///
    /// `audio_output_devices() -> characteratomicvector`
    ///
    /// Provides a list of available audio output devices.
    ///
    /// #### Returns
    ///
    /// A character atomic vector.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// HAudioSink$audio_output_devices()
    /// ```
    ///
    /// _________
    ///
    fn audio_output_devices() -> savvy::Result<Sexp> {
        let output_devices = play::audio_output_devices().map_err(HErrorR::from)?;
        let string_sexp = OwnedStringSexp::try_from(output_devices)?;
        string_sexp.into()
    }

    /// HAudioSink
    /// ## audio_supported_configs
    ///
    /// `audio_supported_configs() -> atomicvector`
    ///
    /// Provides the supported configurations for the default audio output device.
    ///
    /// The following informations are given:
    ///
    /// - Number of channels.
    ///
    /// - Minimum and maximum value for the sampling rate.
    ///
    /// - Minimum and maximum value for the buffer size.
    ///
    /// - Type of data expected by the device.
    ///
    /// #### Returns
    ///
    /// A character atomic vector.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// HAudioSink$audio_supported_configs()
    /// ```
    ///
    /// _________
    ///
    fn audio_supported_configs() -> savvy::Result<Sexp> {
        let supported_configs = play::audio_supported_configs().map_err(HErrorR::from)?;
        let string_sexp = OwnedStringSexp::try_from(supported_configs)?;
        string_sexp.into()
    }

    /// HAudioSink
    /// ## clear
    ///
    /// `clear()`
    ///
    /// Removes all currently loaded `Source`s from the `Sink` and pauses it.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$clear()
    /// haudiosink$is_empty() # TRUE
    /// haudiosink$is_paused() # TRUE
    /// ```
    ///
    /// _________
    ///
    fn clear(&self) -> savvy::Result<()> {
        self.0.clear();
        Ok(())
    }

    /// HAudioSink
    /// ## is_empty
    ///
    /// `is_empty() -> bool`
    ///
    /// Returns true if this sink has no more sounds to play.
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$is_empty() # TRUE
    /// ```
    ///
    /// _________
    ///
    fn is_empty(&self) -> savvy::Result<Sexp> {
        let is_empty = self.0.is_empty();
        let logical_sexp: OwnedLogicalSexp = is_empty.try_into()?;
        logical_sexp.into()
    }

    /// HAudioSink
    /// ## is_paused
    ///
    /// `is_paused() -> bool`
    ///
    /// Gets if a sink is paused.
    ///
    /// Sinks can be paused and resumed using pause() and play(). This returns true if the sink is paused .
    ///
    /// #### Returns
    ///
    /// A `bool`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$is_paused() # FALSE
    /// haudiosink$pause()
    /// haudiosink$is_paused() # TRUE
    /// ```
    ///
    /// _________
    ///
    fn is_paused(&self) -> savvy::Result<Sexp> {
        let is_paused = self.0.is_paused();
        let logical_sexp: OwnedLogicalSexp = is_paused.try_into()?;
        logical_sexp.into()
    }

    /// HAudioSink
    /// ## len
    ///
    /// `len() -> integer`
    ///
    /// Returns the number of sounds currently in the queue.
    ///
    /// #### Returns
    ///
    /// An `integer`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$len() == 0 # TRUE
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$len() == 2 # TRUE
    /// ```
    ///
    /// _________
    ///
    fn len(&self) -> savvy::Result<Sexp> {
        let integer_sexp: OwnedIntegerSexp = try_from_usize_to_int_sexp(self.0.len())?;
        integer_sexp.into()
    }

    /// HAudioSink
    /// ## pause
    ///
    /// `pause()`
    ///
    /// Pauses playback of this sink.
    ///
    /// No effect if already paused.
    ///
    /// A paused sink can be resumed with play().
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$is_paused() # FALSE
    /// haudiosink$pause()
    /// haudiosink$is_paused() # TRUE
    /// ```
    ///
    /// _________
    ///
    fn pause(&self) -> savvy::Result<()> {
        self.0.pause();
        Ok(())
    }

    /// HAudioSink
    /// ## play
    ///
    /// `play()`
    ///
    /// Resumes playback of a paused sink.
    ///
    /// No effect if not paused.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$pause()
    /// haudiosink$is_paused() # TRUE
    /// haudiosink$play()
    /// haudiosink$is_paused() # FALSE
    /// ```
    ///
    /// _________
    ///
    fn play(&self) -> savvy::Result<()> {
        self.0.play();
        Ok(())
    }

    /// HAudioSink
    /// ## set_speed
    ///
    /// `set_speed(value: double)`
    ///
    /// Changes the speed of the sound.
    ///
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.
    ///
    /// #### Arguments
    ///
    /// - `value`
    ///
    /// A `double`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$set_speed(2)
    /// haudiosink$speed() == 2 # TRUE
    /// ```
    ///
    /// _________
    ///
    fn set_speed(&self, value: Sexp) -> savvy::Result<()> {
        let value: f64 = value.as_scalar()?;
        self.0.set_speed(value as f32);
        Ok(())
    }

    /// HAudioSink
    /// ## set_volume
    ///
    /// `set_volume(value: double)`
    ///
    /// Changes the volume of the sound.
    ///
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value.
    ///
    /// #### Arguments
    ///
    /// - `value`
    ///
    /// A `double`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$set_volume(2)
    /// haudiosink$volume() == 2 # TRUE
    /// ```
    ///
    /// _________
    ///
    fn set_volume(&self, value: Sexp) -> savvy::Result<()> {
        let value: f64 = value.as_scalar()?;
        self.0.set_volume(value as f32);
        Ok(())
    }

    /// HAudioSink
    /// ## skip_one
    ///
    /// `skip_one()`
    ///
    /// Skips to the next `Source` in the `Sink`.
    ///
    /// If there are more `Source`s appended to the `Sink` at the time, it will play the next one.
    /// Otherwise, the `Sink` will finish as if it had finished playing a `Source` all the way through.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$len() == 2 # TRUE
    /// haudiosink$skip_one()
    /// haudiosink$len() == 1 # TRUE
    /// ```
    ///
    /// _________
    ///
    fn skip_one(&self) -> savvy::Result<()> {
        self.0.skip_one();
        Ok(())
    }

    /// HAudioSink
    /// ## sleep_until_end
    ///
    /// `sleep_until_end()`
    ///
    /// Sleeps the current thread until the sound ends.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$sleep_until_end()
    /// ```
    ///
    /// _________
    ///
    fn sleep_until_end(&self) -> savvy::Result<()> {
        self.0.sleep_until_end();
        Ok(())
    }

    /// HAudioSink
    /// ## speed
    ///
    /// `speed() -> double`
    ///
    /// Gets the speed of the sound.
    ///
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.
    ///
    /// #### Returns
    ///
    /// A `double`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$speed()
    /// ```
    ///
    /// _________
    ///
    fn speed(&self) -> savvy::Result<Sexp> {
        let speed = self.0.speed() as f64;
        let real_sexp: OwnedRealSexp = speed.try_into()?;
        real_sexp.into()
    }

    /// HAudioSink
    /// ## stop
    ///
    /// `stop()`
    ///
    /// Stops the sink by emptying the queue.
    ///
    /// The sink will keep its previous state (play or pause).
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$len() == 2 # TRUE
    /// haudiosink$stop()
    /// haudiosink$len() == 0 # TRUE
    /// haudiosink$is_paused() # FALSE
    /// ```
    ///
    /// _________
    ///
    fn stop(&self) -> savvy::Result<()> {
        self.0.stop();
        Ok(())
    }

    /// HAudioSink
    /// ## try_seek
    ///
    /// `try_seek(pos: f64)`
    ///
    /// Attempts to seek to a given position in the current source.
    ///
    /// This blocks between 0 and ~5 milliseconds.
    ///
    /// As long as the duration of the source is known, seek is guaranteed to saturate at the end of the source. For example given a
    /// source that reports a total duration of 42 seconds calling `try_seek()` with 60 seconds as argument will seek to 42 seconds.
    ///
    /// This function will return an error if:
    ///
    /// - one of the underlying sources does not support seeking.
    ///
    /// - an implementation ran into one during the seek.
    ///
    /// - when seeking beyond the end of a source when the duration of the source is not known.
    ///
    /// #### Arguments
    ///
    /// - `pos`
    ///
    /// A `double`. The time to seek to in seconds.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$try_seek(2)
    /// ```
    ///
    /// _________
    ///
    fn try_seek(&self, pos: Sexp) -> savvy::Result<()> {
        let pos: f64 = pos.as_scalar()?;
        let pos = std::time::Duration::from_secs_f64(pos);
        self.0.try_seek(pos).map_err(HErrorR::from)?;
        Ok(())
    }

    /// HAudioSink
    /// ## volume
    ///
    /// `volume() -> double`
    ///
    /// Gets the volume of the sound.
    ///
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value.
    ///
    /// #### Returns
    ///
    /// A `double`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "./r-harmonium/testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$volume()
    /// ```
    ///
    /// _________
    ///
    fn volume(&self) -> savvy::Result<Sexp> {
        let volume = self.0.volume() as f64;
        let real_sexp: OwnedRealSexp = volume.try_into()?;
        real_sexp.into()
    }
}
