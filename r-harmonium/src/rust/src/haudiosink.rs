use crate::{haudio::HAudio, hdatatype::HDataType};
use extendr_api::prelude::*;
use harmonium_core::structs::HFloatAudio;
use harmonium_io::{decode::decode_arrow::decode, play};

/// HAudioSink
/// Handle to a device that outputs sounds. \
///
/// # Methods
///
pub struct HAudioSink {
    inner: play::HAudioSink,
}

#[extendr]
impl HAudioSink {
    /// HAudioSink
    /// ## new
    ///
    /// `new() -> HAudioSink` \
    ///
    /// Creates a new `HAudioSink` instance. \
    /// The sink is set on "play" mode from the start. \
    ///
    /// #### Returns
    ///
    /// An `HAudioSink`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// ```
    ///
    /// _________
    ///
    fn new() -> Self {
        Self {
            inner: play::HAudioSink::try_new().unwrap(),
        }
    }

    /// HAudioSink
    /// ## append_from_haudio
    ///
    /// `append_from_haudio(haudio: HAudio)` \
    ///
    /// Appends a sound to the queue of sounds to play. \
    ///
    /// #### Arguments
    ///
    /// * `haudio` \
    /// An `HAudio`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudio = HAudio$new_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav", dtype = HDataType$float32)
    /// haudiosink$append_from_haudio(haudio)
    /// ```
    ///
    /// _________
    ///
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
    /// ## append_from_file
    ///
    /// `append_from_file(fpath: string)` \
    ///
    /// Appends a sound to the queue of sounds to play. \
    ///
    /// #### Arguments
    ///
    /// * `fpath` \
    /// The file path as a `string`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// ```
    ///
    /// _________
    ///
    pub fn append_from_file(&self, fpath: &str) {
        let haudio = decode::<f32>(fpath, None, None).unwrap();
        self.inner.append_from_haudio(&haudio);
    }

    /// HAudioSink
    /// ## play
    ///
    /// `play()` \
    ///
    /// Resumes playback of a paused sink. \
    /// No effect if not paused. \
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
    ///
    pub fn play(&self) {
        self.inner.play();
    }

    /// HAudioSink
    /// ## stop
    ///
    /// `stop()` \
    ///
    /// Stops the sink by emptying the queue. \
    /// The sink will keep its previous state (play or pause). \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$len() == 2 # TRUE
    /// haudiosink$stop()
    /// haudiosink$len() == 0 # TRUE
    /// haudiosink$is_paused() # FALSE
    /// ```
    ///
    /// _________
    ///
    pub fn stop(&self) {
        self.inner.stop();
    }

    /// HAudioSink
    /// ## pause
    ///
    /// `pause()` \
    ///
    /// Pauses playback of this sink. \
    /// No effect if already paused. \
    /// A paused sink can be resumed with play(). \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$is_paused() # FALSE
    /// haudiosink$pause()
    /// haudiosink$is_paused() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn pause(&self) {
        self.inner.pause();
    }

    /// HAudioSink
    /// ## is_paused
    ///
    /// `is_paused() -> bool` \
    ///
    /// Gets if a sink is paused. \
    /// Sinks can be paused and resumed using pause() and play(). This returns true if the sink is paused. \
    ///
    /// #### Returns
    ///
    /// A `bool`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$is_paused() # FALSE
    /// haudiosink$pause()
    /// haudiosink$is_paused() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn is_paused(&self) -> bool {
        self.inner.is_paused()
    }

    /// HAudioSink
    /// ## volume
    ///
    /// `volume() -> double` \
    ///
    /// Gets the volume of the sound. \
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value. \
    ///
    /// #### Returns
    ///
    /// A `double`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$volume()
    /// ```
    ///
    /// _________
    ///
    pub fn volume(&self) -> f64 {
        self.inner.volume() as f64
    }

    /// HAudioSink
    /// ## set_volume
    ///
    /// `set_volume(value: double)` \
    ///
    /// Changes the volume of the sound. \
    /// The value 1.0 is the “normal” volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value. \
    ///
    /// #### Arguments
    ///
    /// * `value` \
    /// A `double`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$set_volume(2)
    /// haudiosink$volume() == 2 # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn set_volume(&self, value: f64) {
        self.inner.set_volume(value as f32);
    }

    /// HAudioSink
    /// ## speed
    ///
    /// `speed() -> double` \
    ///
    /// Gets the speed of the sound. \
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound. \
    ///
    /// #### Returns
    ///
    /// A `double`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$speed()
    /// ```
    ///
    /// _________
    ///
    pub fn speed(&self) -> f64 {
        self.inner.speed() as f64
    }

    /// HAudioSink
    /// ## set_speed
    ///
    /// `set_speed(value: double)` \
    ///
    /// Changes the speed of the sound. \
    /// The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound. \
    ///
    /// #### Arguments
    ///
    /// * `value` \
    /// A `double`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$set_speed(2)
    /// haudiosink$speed() == 2 # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn set_speed(&self, value: f64) {
        self.inner.set_speed(value as f32);
    }

    /// HAudioSink
    /// ## sleep_until_end
    ///
    /// `sleep_until_end()` \
    ///
    /// Sleeps the current thread until the sound ends. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$sleep_until_end()
    /// ```
    ///
    /// _________
    ///
    pub fn sleep_until_end(&self) {
        self.inner.sleep_until_end();
    }

    /// HAudioSink
    /// ## len
    ///
    /// `len() -> integer` \
    ///
    /// Returns the number of sounds currently in the queue. \
    ///
    /// #### Returns
    ///
    /// An `integer`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$len() == 0 # TRUE
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$len() == 2 # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn len(&self) -> i32 {
        self.inner.len() as i32
    }

    /// HAudioSink
    /// ## is_empty
    ///
    /// `is_empty() -> bool` \
    ///
    /// Returns true if this sink has no more sounds to play. \
    ///
    /// #### Returns
    ///
    /// A `bool`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$is_empty() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// HAudioSink
    /// ## clear
    ///
    /// `clear()` \
    ///
    /// Removes all currently loaded `Source`s from the `Sink` and pauses it. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$clear()
    /// haudiosink$is_empty() # TRUE
    /// haudiosink$is_paused() # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn clear(&self) {
        self.inner.clear()
    }

    /// HAudioSink
    /// ## skip_one
    ///
    /// `skip_one()` \
    ///
    /// Skips to the next `Source` in the `Sink`. \
    /// If there are more `Source`s appended to the `Sink` at the time, it will play the next one.
    /// Otherwise, the `Sink` will finish as if it had finished playing a `Source` all the way through. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// haudiosink = HAudioSink$new()
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$append_from_file(fpath = "../../../testfiles/gs-16b-2c-44100hz.wav")
    /// haudiosink$len() == 2 # TRUE
    /// haudiosink$skip_one()
    /// haudiosink$len() == 1 # TRUE
    /// ```
    ///
    /// _________
    ///
    pub fn skip_one(&self) {
        self.inner.skip_one()
    }

    /// HAudioSink
    /// ## audio_output_devices
    ///
    /// `audio_output_devices() -> atomicvector` \
    ///
    /// Provides a list of available audio output devices. \
    ///
    /// #### Returns
    ///
    /// A character atomic vector. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HAudioSink$audio_output_devices()
    /// ```
    ///
    /// _________
    ///
    pub fn audio_output_devices() -> Strings {
        let v = play::audio_output_devices().unwrap();
        Strings::from_values(v)
    }

    /// HAudioSink
    /// ## audio_default_device
    ///
    /// `audio_default_device() -> string` \
    ///
    /// Informs the default audio output device. \
    ///
    /// #### Returns
    ///
    /// A `string`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HAudioSink$audio_default_device()
    /// ```
    ///
    /// _________
    ///
    pub fn audio_default_device() -> String {
        play::audio_default_device().unwrap()
    }

    /// HAudioSink
    /// ## audio_supported_configs
    ///
    /// `audio_supported_configs() -> atomicvector` \
    ///
    /// Provides the supported configurations for the default audio output device. \
    /// The following informations are given: \
    ///
    /// * Number of channels. \
    /// * Minimum and maximum value for the sampling rate. \
    /// * Minimum and maximum value for the buffer size. \
    /// * Type of data expected by the device. \
    ///
    /// #### Returns
    ///
    /// A character atomic vector. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// HAudioSink$audio_supported_configs()
    /// ```
    ///
    /// _________
    ///
    pub fn audio_supported_configs() -> Strings {
        let v = play::audio_supported_configs().unwrap();
        Strings::from_values(v)
    }
}

extendr_module! {
    mod haudiosink;
    impl HAudioSink;
}
