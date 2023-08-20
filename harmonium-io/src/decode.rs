use std::{fs::File, path::Path};

use harmonium_core::{
    array::HArray,
    errors::{HError, HResult},
};
use ndarray::{ArcArray2, Ix2};
use num_traits::{Float, FloatConst};
use symphonia::core::{
    audio::SampleBuffer,
    codecs::{Decoder, DecoderOptions},
    conv::ConvertibleSample,
    formats::{FormatOptions, FormatReader},
    io::MediaSourceStream,
    meta::{MetadataOptions, MetadataRevision},
    probe::Hint,
};

#[derive(Debug, PartialEq)]
pub enum HVerifyDecode {
    Passed,
    Failed,
    NotSupported,
}

#[derive(Debug, PartialEq)]
pub struct HTag {
    pub tag_key: String,
    pub tag_std_key: String,
    pub tag_value: String,
}

#[derive(Debug, PartialEq)]
pub struct HTextMetadata(pub Option<Vec<HTag>>);

#[derive(Debug, PartialEq)]
pub struct HVisualMetadata(pub Option<Vec<HSingleVisualMetadata>>);

#[derive(Debug, PartialEq)]
pub struct HSingleVisualMetadata {
    pub usage: String,
    pub media_type: String,
    pub dimensions: String,
    pub bpp: String,
    pub color_mode: String,
    pub size: String,
    pub tags_vec: Option<Vec<HTag>>,
}

#[derive(Debug, PartialEq)]
pub enum HMetadata {
    All((HTextMetadata, HVisualMetadata)),
    Text(HTextMetadata),
    Visual(HVisualMetadata),
}

pub enum HMetadataType {
    All,
    Text,
    Visual,
}

/// Decode an audio file as an HFloatAudio.
/// The decoded samples are normalized to fit in the range of \[-1.0, 1.0\].
///
/// # Arguments
///
/// * `fpath` - The input file.
/// * `offset` - Start reading the file after offset, in seconds.
/// If `None`, will decode from the beginning of the file.
/// * `duration` - Duration to be loaded, in seconds, counting from `offset`. Will load the file till the end if `offset + duration >= file length`.
/// If `None`, will decode until the end of the file.
///
/// # Examples
///
/// ```
/// //let test_file = "../testfiles/gs-16b-2c-44100hz.wav";
/// //load(test_file, Some(1_f64), Some(1_f64))
/// ```
pub fn decode<T>(fpath: &str) -> HResult<(HArray<T, Ix2>, u32)>
where
    T: Float + FloatConst + ConvertibleSample,
{
    let fpath = Path::new(fpath);
    let ext = Path::extension(fpath)
        .ok_or_else(|| HError::IoError("couldn't extract the file extension".into()))?
        .to_str()
        .ok_or_else(|| HError::IoError("cannot convert from &OsStr to &str".into()))?;
    // Create a media source. Note that the MediaSource trait is automatically implemented for File, among other types.
    let file = Box::new(File::open(fpath)?);
    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());
    // Create a hint to help the format registry guess what format reader is appropriate.
    let mut hint = Hint::new();
    hint.with_extension(ext);
    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();
    // Probe the media source stream for a format.
    let probed =
        symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
    // Get the format reader yielded by the probe operation.
    let mut reader = probed.format;
    // Get the default track.
    let track = reader
        .default_track()
        .ok_or_else(|| HError::DecodeError("no tracks were detected".into()))?;
    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;
    let codec_params = decoder.codec_params();
    let channels = codec_params
        .channels
        .ok_or_else(|| HError::DecodeError("cannot retrieve the number of channels".into()))?
        .count();
    // Total number of frames. In PCM nframes is the same as nsamples / nchannels.
    let nframes = codec_params
        .n_frames
        .ok_or_else(|| HError::DecodeError("cannot retrieve the number of frames".into()))?;
    let sr = codec_params
        .sample_rate
        .ok_or_else(|| HError::DecodeError("cannot retrieve the sample rate".into()))?;

    let track_id = track.id;
    let mut sample_buf: Option<SampleBuffer<T>> = None;

    let mut ndarray = ArcArray2::zeros((channels, usize::try_from(nframes).unwrap()));
    let mut ndarray_t = ndarray.view_mut().reversed_axes();
    let mut ndarray_iter = ndarray_t.iter_mut();
    let ndarray_iter_by_ref = ndarray_iter.by_ref();

    loop {
        // Get the next packet from the format reader.
        let packet = match reader.next_packet() {
            Ok(packet_ok) => packet_ok,
            Err(symphonia::core::errors::Error::IoError(ref packet_err))
                if packet_err.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(packet_err) => Err(packet_err)?,
        };

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // The decoded audio samples may now be accessed via the audio buffer if per-channel
                // slices of samples in their native decoded format is desired. Use-cases where
                // the samples need to be accessed in an interleaved order or converted into
                // another sample format, or a byte buffer is required, are covered by copying the
                // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                // example below, we will copy the audio buffer into a sample buffer in an
                // interleaved order.

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();
                    // Get the capacity of the decoded buffer.
                    let cap = audio_buf.capacity() as u64;
                    // Create the sample buffer.
                    sample_buf = Some(SampleBuffer::<T>::new(cap, spec));
                }

                if let Some(buf) = &mut sample_buf {
                    // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                    buf.copy_interleaved_ref(audio_buf);

                    // The samples may now be access via the `samples()` function.
                    let samples = buf.samples();

                    for (sample, elem) in samples.iter().zip(&mut *ndarray_iter_by_ref) {
                        *elem = *sample;
                    }
                }
            }
            Err(symphonia::core::errors::Error::DecodeError(err_str)) => {
                Err(symphonia::core::errors::Error::DecodeError(err_str))?
            }
            Err(_) => break,
        }
    }

    let harray = HArray(ndarray);

    Ok((harray, sr))
}

/// stream an audio file as an iterator.
/// The samples are normalized to fit in the range of \[-1.0, 1.0\].
///
/// # Arguments
///
/// `fpath` - The input file.
/// `frames` - Number of frames to decode per iteration.
///
/// # Examples
///
/// ```
/// //let test_file = "../testfiles/gs-16b-2c-44100hz.wav";
/// //stream(test_file, Some(1.0_f64), Some(1.0_f64), 1000)
/// ```
pub fn stream<T>(fpath: &str, frames: usize) -> HResult<DecoderStream<T>>
where
    T: Float + FloatConst + ConvertibleSample,
{
    let fpath = Path::new(fpath);
    let ext = Path::extension(fpath)
        .ok_or_else(|| HError::IoError("couldn't extract the file extension".into()))?
        .to_str()
        .ok_or_else(|| HError::IoError("cannot convert from &OsStr to &str".into()))?;
    // Create a media source. Note that the MediaSource trait is automatically implemented for File, among other types.
    let file = Box::new(File::open(fpath)?);
    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());
    // Create a hint to help the format registry guess what format reader is appropriate.
    let mut hint = Hint::new();
    hint.with_extension(ext);
    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();
    // Probe the media source stream for a format.
    let probed =
        symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
    // Get the format reader yielded by the probe operation.
    let reader = probed.format;
    // Get the default track.
    let track = reader
        .default_track()
        .ok_or_else(|| HError::DecodeError("no tracks were detected".into()))?;
    // Create a decoder for the track.
    let decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;
    let codec_params = decoder.codec_params();
    let channels = codec_params
        .channels
        .ok_or_else(|| HError::DecodeError("cannot retrieve the number of channels".into()))?
        .count();
    // Total number of frames. In PCM nframes is the same as nsamples, but for each channel.
    let nframes = codec_params
        .n_frames
        .ok_or_else(|| HError::DecodeError("cannot retrieve the number of frames".into()))?;
    let track_id = track.id;
    let sample_buf: Option<SampleBuffer<T>> = None;
    let last_idx = 0;

    let stream_struct = DecoderStream::new(
        reader,
        decoder,
        track_id,
        sample_buf,
        channels,
        nframes.try_into().unwrap(),
        frames,
        last_idx,
    );

    Ok(stream_struct)
}

pub struct DecoderStream<T>
where
    T: Float + FloatConst + ConvertibleSample,
{
    // Reader.
    reader: Box<dyn FormatReader>,
    // Decoder.
    decoder: Box<dyn Decoder>,
    // Track id.
    track_id: u32,
    // Buffer to allocate the samples read in a packet.
    sample_buf: Option<SampleBuffer<T>>,
    // Number of channels.
    channels: usize,
    // Number of frames.
    nframes: usize,
    // Number of frames to be returned in each iteration.
    frames: usize,
    // Last index read in the packet. Needed to keep reading from the following sample when in a new iteration but in the same packet.
    last_idx: usize,
}

impl<T> DecoderStream<T>
where
    T: Float + FloatConst + ConvertibleSample,
{
    #![allow(clippy::too_many_arguments)]
    fn new(
        reader: Box<dyn FormatReader>,
        decoder: Box<dyn Decoder>,
        track_id: u32,
        sample_buf: Option<SampleBuffer<T>>,
        channels: usize,
        nframes: usize,
        frames: usize,
        last_idx: usize,
    ) -> Self {
        DecoderStream {
            reader,
            decoder,
            track_id,
            sample_buf,
            channels,
            nframes,
            frames,
            last_idx,
        }
    }
}

impl<T> Iterator for DecoderStream<T>
where
    T: Float + FloatConst + ConvertibleSample,
{
    type Item = HArray<T, Ix2>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nframes < self.frames {
            return None;
        }

        let mut ndarray = ArcArray2::zeros((self.channels, self.frames));
        let mut ndarray_t = ndarray.view_mut().reversed_axes();
        let mut ndarray_iter = ndarray_t.iter_mut();
        let ndarray_iter_by_ref = ndarray_iter.by_ref();

        let nsamples = self.frames * self.channels;
        let mut samples_used = 0_usize;

        // If samples reminiscent from last read packet.
        if self.last_idx != 0 {
            if let Some(audio_buf) = &mut self.sample_buf {
                // The samples may now be access via the `samples()` function.
                let mut samples = audio_buf.samples();

                samples = &samples[self.last_idx..];

                let samples_len = samples.len();
                let remaining_samples = nsamples - samples_used;

                if remaining_samples > samples_len {
                    for (sample, elem) in samples.iter().zip(&mut *ndarray_iter_by_ref) {
                        *elem = *sample;
                        samples_used += 1;
                    }
                } else {
                    for (sample, elem) in samples.iter().zip(&mut *ndarray_iter_by_ref) {
                        *elem = *sample;
                        self.last_idx += 1;
                    }

                    self.nframes -= self.frames;
                    let harray = HArray(ndarray);
                    return Some(harray);
                }
            };
        }

        loop {
            // Get the next packet from the format reader.
            let packet = match self.reader.next_packet() {
                Ok(packet_ok) => packet_ok,
                Err(symphonia::core::errors::Error::IoError(ref packet_err))
                    if packet_err.kind() == std::io::ErrorKind::UnexpectedEof =>
                {
                    return None;
                }
                Err(_) => return None,
            };

            // If the packet does not belong to the selected track, skip it.
            if packet.track_id() != self.track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match self.decoder.decode(&packet) {
                Ok(audio_buf) => {
                    // The decoded audio samples may now be accessed via the audio buffer if per-channel
                    // slices of samples in their native decoded format is desired. Use-cases where
                    // the samples need to be accessed in an interleaved order or converted into
                    // another sample format, or a byte buffer is required, are covered by copying the
                    // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                    // example below, we will copy the audio buffer into a sample buffer in an
                    // interleaved order.

                    // If this is the *first* decoded packet, create a sample buffer matching the
                    // decoded audio buffer format.
                    if (self.sample_buf).is_none() {
                        // Get the audio buffer specification.
                        let spec = *audio_buf.spec();
                        // Get the capacity of the decoded buffer.
                        let cap = audio_buf.capacity() as u64;

                        // Create the sample buffer.
                        self.sample_buf = Some(SampleBuffer::<T>::new(cap, spec));
                    }

                    if let Some(buf) = &mut self.sample_buf {
                        // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                        buf.copy_interleaved_ref(audio_buf);

                        // The samples may now be access via the `samples()` function.
                        let samples = buf.samples();
                        self.last_idx = 0;

                        let samples_len = samples.len();
                        let remaining_samples = nsamples - samples_used;

                        if remaining_samples > samples_len {
                            for (sample, elem) in samples.iter().zip(&mut *ndarray_iter_by_ref) {
                                *elem = *sample;
                                samples_used += 1;
                            }
                        } else {
                            for (sample, elem) in samples.iter().zip(&mut *ndarray_iter_by_ref) {
                                *elem = *sample;
                                self.last_idx += 1;
                            }

                            self.nframes -= self.frames;
                            let harray = HArray(ndarray);
                            return Some(harray);
                        }
                    }
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => {
                    return None;
                }
                Err(_) => return None,
            }
        }
    }
}

/// Extract text and visual metadata from a file.
/// Tags that are part of the container format are preferentially extracted. Additional tags that were found while probing will not be extracted.
/// The following metadata tagging formats are supported.
///
/// * ID3v1
/// * ID3v2
/// * ISO/MP4
/// * RIFF
/// * Vorbis Comment (in OGG & FLAC)
///
/// Each `HTextMetadata` will be comprised of a `HTag`, which contains the following fields:
/// * tag_key
///     A key string indicating the type, meaning, or purpose of the Tags value. Note: The meaning of key is dependant on the underlying metadata format.
/// * tag_std_key
///     If the Tag’s key string is commonly associated with a typical type, meaning, or purpose, then if recognized a StandardTagKey will be assigned
///     to this Tag. This is a best effort guess since not all metadata formats have a well defined or specified tag mapping. However, it is recommended that
///     consumers prefer std_key over key, if provided.
///     Check [`StandardTagKey`] for all the variants.
/// * tag_value
///     The value of the Tag.
///
/// Each `HVisualMetadata` will be comprised of the following fields:
/// * usage
///     The usage and/or content of the Visual. A string version of `symphonia_core::meta::StandardVisualKey`, which is an enumeration providing
///     standardized keys for common visual dispositions. A demuxer may assign a StandardVisualKey to a Visual if the disposition of the attached visual
///     is known and can be mapped to a standard key. The visual types listed here are derived from, though do not entirely cover, the ID3v2 APIC frame specification.
/// * media_type
///     The Media Type (MIME Type) used to encode the Visual.
/// * dimensions
///     The dimensions (width and height) of the Visual, represented in pixels. Note: This value may not be accurate as it comes from metadata, not the
///     embedded graphic itself. Consider it only a hint.
/// * bits_per_pixel:
///     The number of bits-per-pixel (aka bit-depth) of the unencoded image.
/// * color_mode:
///     Indicates how the color of a pixel is encoded in a Visual. Variants:
///     **Discrete
///         Each pixel in the Visual stores its own color information.
///     **Indexed(NonZeroU32)
///         Each pixel in the Visual stores an index into a color palette containing the color information. The value stored by this variant indicates the number
///         of colors in the color palette.
/// * size:
///     Size of the image in bytes.
/// * tag:
///     `Tag` with the following fields:
///         ** tag_key
///             A key string indicating the type, meaning, or purpose of the Tags value. Note: The meaning of key is dependant on the underlying metadata format.
///         ** tag_std_key
///             If the Tag’s key string is commonly associated with a typical type, meaning, or purpose, then if recognized a StandardTagKey will be assigned
///             to this Tag. This is a best effort guess since not all metadata formats have a well defined or specified tag mapping. However, it is recommended that
///             consumers prefer std_key over key, if provided.
///             Check [`StandardTagKey`] for all the variants.
///         ** tag_value
///             The value of the Tag.
///
/// # Examples
///
/// ```
/// //metadata_from_file(../testfiles/gs-16b-1c-44100hz.mp3, HMetadataType::All).unwrap();
/// ```
pub fn metadata_from_file(fpath: &str, metadata_type: HMetadataType) -> HResult<Option<HMetadata>> {
    let fpath = Path::new(fpath);
    let ext = Path::extension(fpath)
        .ok_or_else(|| HError::IoError("couldn't extract the file extension".into()))?
        .to_str()
        .ok_or_else(|| HError::IoError("cannot convert from &OsStr to &str".into()))?;
    // Create a media source. Note that the MediaSource trait is automatically implemented for File, among other types.
    let file = Box::new(File::open(fpath)?);
    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());
    // Create a hint to help the format registry guess what format reader is appropriate.
    let mut hint = Hint::new();
    hint.with_extension(ext);
    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    // Probe the media source stream for a format.
    let mut probed =
        symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

    // Prefer metadata that's provided in the container format, over other tags found during the probe operation.
    let metadata = if let Some(metadata_rev) = probed.format.metadata().current() {
        Some(process_metadata_rev(metadata_rev, metadata_type))
    } else {
        probed
            .metadata
            .get()
            .as_ref()
            .and_then(|m| m.current())
            .map(|metadata_rev| process_metadata_rev(metadata_rev, metadata_type))
    };

    Ok(metadata)
}

fn process_metadata_rev(
    metadata_rev: &MetadataRevision,
    metadata_type: HMetadataType,
) -> HMetadata {
    match metadata_type {
        HMetadataType::Text => HMetadata::Text(process_text_metadata(metadata_rev)),
        HMetadataType::Visual => HMetadata::Visual(process_visual_metadata(metadata_rev)),
        HMetadataType::All => {
            let (text, visual) = (
                process_text_metadata(metadata_rev),
                process_visual_metadata(metadata_rev),
            );
            HMetadata::All((text, visual))
        }
    }
}

fn process_text_metadata(metadata_rev: &MetadataRevision) -> HTextMetadata {
    let tags = metadata_rev.tags();

    if tags.is_empty() {
        HTextMetadata(None)
    } else {
        let mut metadata_vec: Vec<HTag> = Vec::with_capacity(tags.len());

        for tag in tags.iter() {
            let tag_key = tag.key.clone();
            let tag_std_key = match tag.std_key {
                Some(x) => format!("{:?}", x),
                None => "None".to_string(),
            };

            let tag_value = tag.value.to_string();

            let tag_struct = HTag {
                tag_key,
                tag_std_key,
                tag_value,
            };

            metadata_vec.push(tag_struct);
        }
        HTextMetadata(Some(metadata_vec))
    }
}

fn process_visual_metadata(metadata_rev: &MetadataRevision) -> HVisualMetadata {
    let visuals = metadata_rev.visuals();

    if visuals.is_empty() {
        HVisualMetadata(None)
    } else {
        let mut metadata: Vec<HSingleVisualMetadata> = Vec::with_capacity(visuals.len());

        for visual in visuals.iter() {
            let usage = match visual.usage {
                Some(x) => format!("{:?}", x),
                None => "None".to_string(),
            };

            let media_type = visual.media_type.clone();

            let dimensions = match visual.dimensions {
                Some(x) => format!("(width, heigth): ({}, {})", x.width, x.height),
                None => "None".to_string(),
            };

            let bpp = match visual.bits_per_pixel {
                Some(x) => x.to_string(),
                None => "None".to_string(),
            };

            let color_mode = match visual.color_mode {
                Some(x) => format!("{:?}", x),
                None => "None".to_string(),
            };

            let size = visual.data.len().to_string();

            let tags = &visual.tags;
            let tags_vec = if tags.is_empty() {
                None
            } else {
                let mut tags_vec = Vec::with_capacity(tags.len());

                for tag in tags.iter() {
                    let tag_key = tag.key.clone();
                    let tag_std_key = match tag.std_key {
                        Some(x) => format!("{:?}", x),
                        None => "None".to_string(),
                    };

                    let tag_value = tag.value.to_string();

                    let tag_struct = HTag {
                        tag_key,
                        tag_std_key,
                        tag_value,
                    };

                    tags_vec.push(tag_struct);
                }
                Some(tags_vec)
            };

            let visualmetadata = HSingleVisualMetadata {
                usage,
                media_type,
                dimensions,
                bpp,
                color_mode,
                size,
                tags_vec,
            };
            metadata.push(visualmetadata);
        }

        HVisualMetadata(Some(metadata))
    }
}

/// Get audio parameters from a file.
///Returns a tuple `(sr, nframes, nchannels, duration)`.
/// `sr` will be in Hz and `duration` in seconds.
/// Note that this avoids loading the contents into memory, and is therefore useful for querying these parameters from long files.
///
/// # Arguments
/// `fpath` - The path to the input file.
///
/// # Examples
///
/// ```
/// //let fname = "../testfiles/gs-16b-2c-44100hz.wav";
/// //get_params_from_file(fname)
/// ```
pub fn get_params_from_file(fpath: &str) -> HResult<(u32, u64, usize, f64)> {
    let fpath = Path::new(fpath);
    let ext = Path::extension(fpath)
        .ok_or_else(|| HError::IoError("couldn't extract the file extension".into()))?
        .to_str()
        .ok_or_else(|| HError::IoError("cannot convert from &OsStr to &str".into()))?;
    let file = Box::new(File::open(fpath)?);
    let mss = MediaSourceStream::new(file, Default::default());
    let mut hint = Hint::new();
    hint.with_extension(ext);
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let probed =
        symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
    let reader = probed.format;
    let track = reader
        .default_track()
        .ok_or_else(|| HError::DecodeError("no tracks were detected".into()))?;
    let sr = track
        .codec_params
        .sample_rate
        .ok_or_else(|| HError::DecodeError("cannot retrieve the sample rate".into()))?;
    let nframes = track
        .codec_params
        .n_frames
        .ok_or_else(|| HError::DecodeError("cannot retrieve the number of frames".into()))?;
    let nchannels = track
        .codec_params
        .channels
        .ok_or_else(|| HError::DecodeError("cannot retrieve the number of channels".into()))?
        .count();
    let duration = nframes as f64 / sr as f64;

    Ok((sr, nframes, nchannels, duration))
}

/// Verify an audio file, if supported by the decoder.
/// The verification is done after the decoding process is finished.
///
/// # Arguments
///
/// * `fpath` - The input file.
///
/// # Examples
///
/// ```
/// //verify("../testfiles/test.wav");
/// ```
pub fn verify_file(fpath: &str) -> HResult<HVerifyDecode> {
    let fpath = Path::new(fpath);
    let ext = Path::extension(fpath)
        .ok_or_else(|| HError::IoError("couldn't extract the file extension".into()))?
        .to_str()
        .ok_or_else(|| HError::IoError("cannot convert from &OsStr to &str".into()))?;
    // Create a media source. Note that the MediaSource trait is automatically implemented for File, among other types.
    let file = Box::new(File::open(fpath)?);
    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());
    // Create a hint to help the format registry guess what format reader is appropriate.
    let mut hint = Hint::new();
    hint.with_extension(ext);
    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();

    #[allow(clippy::needless_update)]
    let decoder_opts: DecoderOptions = DecoderOptions {
        verify: true,
        ..Default::default() // in case of future options for DecoderOptions in Symphonia
    };
    // Probe the media source stream for a format.
    let probed =
        symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
    // Get the format reader yielded by the probe operation.
    let mut reader = probed.format;
    // Get the default track.
    let track = reader
        .default_track()
        .ok_or_else(|| HError::DecodeError("no tracks were detected".into()))?;
    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;
    let track_id = track.id;

    loop {
        // Get the next packet from the format reader.
        let packet = match reader.next_packet() {
            Ok(packet_ok) => packet_ok,
            Err(symphonia::core::errors::Error::IoError(ref packet_err))
                if packet_err.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(packet_err) => Err(packet_err)?,
        };

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(_) => continue,
            Err(symphonia::core::errors::Error::DecodeError(err_str)) => {
                Err(symphonia::core::errors::Error::DecodeError(err_str))?
            }
            Err(_) => break,
        }
    }

    // check that it works. It doesn't work with the .wav example. Symphonia says it only works
    // with some type of exts.
    // Finalize the decoder to get the verification result.
    let finalize_result = decoder.finalize();

    if let Some(verify_ok) = finalize_result.verify_ok {
        if verify_ok {
            Ok(HVerifyDecode::Passed)
        } else {
            Ok(HVerifyDecode::Failed)
        }
    } else {
        Ok(HVerifyDecode::NotSupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use harmonium_core::haudioop::HAudioOp;
    use ndarray::s;

    macro_rules! decode_test {
            ($input: expr, $results: expr, $($t: ty),+ $(,)*) => {
                let fpath = $input;
                $(
                    let (decoded_harray, sr) = decode::<$t>(fpath).unwrap();
                    let nframes = decoded_harray.nframes();
                    let nchannels = decoded_harray.nchannels();
                    let decoded_ndarray = decoded_harray.0;

                    // test 5 first elements in 1st channel
                    let slice = decoded_ndarray.slice(s![0, 0..5]);
                    let lhs = slice.as_slice().unwrap();

                    let v: Vec<$t> = (*$results.0)
                    .clone()
                    .into_iter()
                    .map(|x| <$t>::from(x))
                    .collect();
                    let rhs = v.as_slice();

                    assert_eq!(lhs, rhs);

                    // test 5 last elements in 1st channel
                    let slice = decoded_ndarray.slice(s![0, nframes-5..nframes]);
                    let lhs = slice.as_slice().unwrap();

                    let v: Vec<$t> = (*$results.1)
                    .clone()
                    .into_iter()
                    .map(|x| <$t>::from(x))
                    .collect();
                    let rhs = v.as_slice();

                    assert_eq!(lhs, rhs);

                    // test 5 last elements in last channel
                    let slice = decoded_ndarray.slice(s![nchannels-1, nframes-5..nframes]);
                    let lhs = slice.as_slice().unwrap();

                    let v: Vec<$t> = (*$results.2)
                    .clone()
                    .into_iter()
                    .map(|x| <$t>::from(x))
                    .collect();
                    let rhs = v.as_slice();

                    assert_eq!(lhs, rhs);

                    assert_eq!(sr, $results.3);
                    assert_eq!(nchannels, $results.4);
                    assert_eq!(nframes, $results.5);
                    )+
            };
        }

    #[test]
    fn decode_test() {
        let fpath = "../testfiles/gs-16b-2c-44100hz.wav";
        let result_head = vec![
            0.0,
            0.000_030_517_578,
            -0.000_061_035_156,
            0.000_061_035_156,
            -0.000_061_035_156,
        ];

        let result_tail = vec![0.000030517578, 0.0, 0.0, 0.0, 0.0];

        let result_last = vec![
            0.0,
            0.000030517578,
            -0.000030517578,
            0.000030517578,
            -0.000030517578,
        ];

        decode_test!(
            fpath,
            (&result_head, &result_tail, &result_last, 44100, 2, 698194),
            f32,
            f64,
        );
    }

    macro_rules! stream_test {
                ($input: expr, $dimensions: expr, $($t: ty),+ $(,)*) => {
                    let fpath = $input;
                    $(
                        let decoded_harray = decode::<$t>(fpath).unwrap().0;
                        let nframes = decoded_harray.nframes();
                        let nchannels = $dimensions.0;
                        let frames = $dimensions.1;
                        let mut stream_struct = stream::<$t>(fpath, frames).unwrap();

                        // test first iteration
                        let harray_next = stream_struct.next().unwrap();
                        let lhs = (harray_next.nchannels(), harray_next.nframes());
                        let rhs = (nchannels, frames);
                        assert_eq!(lhs, rhs);

                        let ndarray_lhs = harray_next.0.slice(s![0, ..]);
                        let lhs = ndarray_lhs.as_slice().unwrap();
                        let ndarray_rhs = decoded_harray.0.slice(s![0, 0..frames]);
                        let rhs = ndarray_rhs.as_slice().unwrap();
                        assert_eq!(lhs, rhs);

                        // test second iteration
                        let harray_next = stream_struct.next().unwrap();
                        let lhs = (harray_next.nchannels(), harray_next.nframes());
                        let rhs = (nchannels, frames);
                        assert_eq!(lhs, rhs);

                        let ndarray_lhs = harray_next.0.slice(s![0, ..]);
                        let lhs = ndarray_lhs.as_slice().unwrap();
                        let ndarray_rhs = decoded_harray.0.slice(s![0, frames..frames + frames]);
                        let rhs = ndarray_rhs.as_slice().unwrap();
                        assert_eq!(lhs, rhs);

                        // test last iteration
                        let harray_next = stream_struct.last().unwrap();
                        let lhs = (harray_next.nchannels(), harray_next.nframes());
                        let rhs = (nchannels, frames);
                        assert_eq!(lhs, rhs);

                        let remaining = nframes % frames;
                        let ndarray_lhs = harray_next.0.slice(s![0, ..]);
                        let lhs = ndarray_lhs.as_slice().unwrap();
                        let ndarray_rhs = decoded_harray.0.slice(s![0, nframes - frames - remaining..nframes - remaining]);
                        let rhs = ndarray_rhs.as_slice().unwrap();
                        assert_eq!(lhs, rhs);
                        )+
                };
            }

    #[test]
    fn stream_test() {
        let fpath = "../testfiles/gs-16b-2c-44100hz.wav";
        let frames = 1000;
        let nchannels = 2;

        stream_test!(fpath, (nchannels, frames), f32, f64);
    }

    #[test]
    fn metadata_from_file_test() {
        let fpath = "../testfiles/gs-16b-2c-44100hz.flac";
        let meta = metadata_from_file(fpath, HMetadataType::Text)
            .unwrap()
            .unwrap();

        // Text metadata.
        if let HMetadata::Text(text) = meta {
            assert_eq!(
                text.0.unwrap()[0],
                HTag {
                    tag_key: "title".into(),
                    tag_std_key: "TrackTitle".into(),
                    tag_value: "Galway".into()
                }
            );
        };

        // No visual metadata in this file.
        let meta = metadata_from_file(fpath, HMetadataType::Visual)
            .unwrap()
            .unwrap();
        if let HMetadata::Visual(visual) = meta {
            assert_eq!(visual.0, None);
        };

        // All metadata.
        let meta = metadata_from_file(fpath, HMetadataType::All)
            .unwrap()
            .unwrap();
        if let HMetadata::All(all) = meta {
            let text = all.0;
            let visual = all.1;
            assert_eq!(
                text.0.unwrap()[0],
                HTag {
                    tag_key: "title".into(),
                    tag_std_key: "TrackTitle".into(),
                    tag_value: "Galway".into()
                }
            );
            assert_eq!(visual.0, None);
        };
    }

    #[test]
    fn get_params_from_file_test() {
        let fpath = "../testfiles/gs-16b-2c-44100hz.wav";
        let params = get_params_from_file(fpath).unwrap();
        assert_eq!(params, (44100, 698194, 2, 15.832063492063492));
    }

    #[test]
    fn verify_test() {
        let fpath = "../testfiles/gs-16b-2c-44100hz.wav";
        let verify_decode = verify_file(fpath).unwrap();
        assert_eq!(verify_decode, HVerifyDecode::NotSupported);
    }
}
