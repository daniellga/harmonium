use crate::{haudio::HAudio, hdatatype::HDataType};
use extendr_api::prelude::*;
use harmonium_core::structs::HFloatAudio;
use harmonium_io::{decode::decode_arrow::decode, play};

/// HAudioSink
/// Handle to an device that outputs sounds.
///
/// ## Methods
pub struct HAudioSink {
    inner: play::HAudioSink,
}

#[extendr]
impl HAudioSink {
    /// HAudioSink
    /// ### new
    ///
    /// `new() -> HAudioSink` \
    ///
    /// Creates a new `HAudioSink` instance. \
    /// The sink is set on "play" mode from the start.
    ///
    /// #### Returns
    ///
    /// An `HAudioSink`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// ```
    ///
    /// _________
    fn new() -> Self {
        Self {
            inner: play::HAudioSink::try_new().unwrap(),
        }
    }

    /// HAudioSink
    /// ### append_from_haudio
    ///
    /// `append_from_haudio(haudio: HAudio)` \
    ///
    /// Appends a sound to the queue of sounds to play.
    ///
    /// #### Arguments
    ///
    /// * `haudio` \
    /// An `HAudio`.
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudio = HAudio$new_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$append_from_haudio(haudio)
    /// ```
    ///
    /// _________
    fn append_from_haudio(&self, haudio: &HAudio) {
        match haudio.0.dtype() {
            HDataType::Float32 => {
                let haudio = haudio
                    .0
                    .as_any()
                    .downcast_ref::<HFloatAudio<f32>>()
                    .unwrap();
                self.inner.append_from_haudio::<f32>(haudio);
            }
            HDataType::Float64 => {
                let haudio = haudio
                    .0
                    .as_any()
                    .downcast_ref::<HFloatAudio<f64>>()
                    .unwrap();
                self.inner.append_from_haudio::<f64>(haudio);
            }
            _ => unreachable!(),
        }
    }

    /// HAudioSink
    /// ### append_from_file
    ///
    /// `append_from_file(fpath: string)` \
    ///
    /// Appends a sound to the queue of sounds to play.
    ///
    /// #### Arguments
    ///
    /// * `fpath` \
    /// The file path as a string.
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// ```
    ///
    /// _________
    pub fn append_from_file(&self, fpath: &str) {
        let haudio = decode::<f32>(fpath, None, None).unwrap();
        self.inner.append_from_haudio(&haudio);
    }

    /// HAudioSink
    /// ### play
    ///
    /// `play()` \
    ///
    /// Resumes playback of a paused sink. \
    /// No effect if not paused.
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$pause()
    /// haudiosink$is_paused() # TRUE
    /// haudiosink$play()
    /// haudiosink$is_paused() # FALSE
    /// ```
    ///
    /// _________
    pub fn play(&self) {
        self.inner.play();
    }

    /// Stops the sink by emptying the queue. \
    /// The sink will keep its previous state (play or pause).
    pub fn stop(&self) {
        self.inner.stop();
    }

    /// Pauses playback of this sink. \
    /// No effect if already paused. \
    /// A paused sink can be resumed with play().
    pub fn pause(&self) {
        self.inner.pause();
    }

    /// Gets if a sink is paused.
    /// Sinks can be paused and resumed using pause() and play(). This returns true if the sink is paused.
    pub fn is_paused(&self) -> bool {
        self.inner.is_paused()
    }

    /// Gets the volume of the sound.
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value.
    pub fn volume(&self) -> f32 {
        self.inner.volume()
    }

    /// Changes the volume of the sound.
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value.
    pub fn set_volume(&self, value: f32) {
        self.inner.set_volume(value);
    }

    /// Gets the speed of the sound.
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.
    pub fn speed(&self) -> f32 {
        self.inner.speed()
    }

    /// Changes the speed of the sound.
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.
    pub fn set_speed(&self, value: f32) {
        self.inner.set_speed(value);
    }

    /// Sleeps the current thread until the sound ends.
    pub fn sleep_until_end(&self) {
        self.inner.sleep_until_end();
    }

    /// Returns the number of sounds currently in the queue.
    pub fn len(&self) -> i32 {
        self.inner.len() as i32
    }

    /// Returns true if this sink has no more sounds to play.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Removes all currently loaded `Source`s from the `Sink` and pauses it.
    pub fn clear(&self) {
        self.inner.clear()
    }

    ///// Skips to the next `Source` in the `Sink`.
    ///// If there are more `Source`s appended to the `Sink` at the time,
    ///// it will play the next one. Otherwise, the `Sink` will finish as if
    ///// it had finished playing a `Source` all the way through.
    //pub fn skip_one(&self) {
    //    self.inner.skip_one()
    //}

    /// Provides a list of available audio output devices.
    /// @return A vector of strings.
    /// @export
    pub fn audio_output_devices() -> Vec<String> {
        play::audio_output_devices().unwrap()
    }

    /// Informs the default audio output device.
    /// @return A string.
    /// @export
    pub fn audio_default_device() -> String {
        play::audio_default_device().unwrap()
    }

    /// Provides the supported configurations for the default audio output device.
    /// The following informations are given:
    /// * Number of channels.
    /// * Minimum and maximum value for the sampling rate.
    /// * Minimum and maximum value for the buffer size.
    /// * Type of data expected by the device.
    /// @return A vector of strings.
    /// @export
    pub fn audio_supported_configs() -> Vec<String> {
        play::audio_supported_configs().unwrap()
    }
}

extendr_module! {
    mod haudiosink;
    impl HAudioSink;
}
