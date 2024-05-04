use crate::{
    conversions::Conversions, errors::HErrorR, harray::HArray, hdatatype::HDataType,
    hmetadatatype::HMetadataType,
};
use harmonium_core::conversions::IntoDynamic;
use harmonium_io::decode;
use savvy::{savvy, OwnedIntegerSexp, OwnedListSexp, OwnedRealSexp, OwnedStringSexp, Sexp};
use std::sync::Arc;

#[savvy]
struct HFile;
#[savvy]
struct HDecoderStream(Box<dyn HDecoderStreamR>);
#[savvy]
struct HDecodedAudio {
    harray: HArray,
    sr: u32,
}

#[savvy]
impl HDecodedAudio {
    /// HDecodedAudio
    /// ## harray
    ///
    /// `harray() -> HArray` \
    ///
    /// Get the decoded HArray.
    ///
    /// #### Returns
    ///
    /// A float HArray. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// fpath = "../../../testfiles/gs-16b-2c-44100hz.flac"
    /// dtype = HDataType$Float32
    /// hdecodedaudio = HFile$decode(fpath, dtype)
    /// hdecodedaudio$harray()
    ///
    /// ```
    ///
    /// _________
    ///
    fn harray(&self) -> savvy::Result<HArray> {
        self.harray.clone()
    }
    /// HDecodedAudio
    /// ## sr
    ///
    /// `sr() -> integer` \
    ///
    /// Get the sampling rate of the decoded audio.
    ///
    ///
    /// #### Returns
    ///
    /// An integer. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// fpath = "../../../testfiles/gs-16b-2c-44100hz.flac"
    /// dtype = HDataType$Float32
    /// hdecodedaudio = HFile$decode(fpath, dtype)
    /// hdecodedaudio$sr()
    ///
    /// ```
    ///
    /// _________
    ///
    fn sr(&self) -> savvy::Result<Sexp> {
        let sr: i32 = self
            .sr
            .try_into()
            .map_err(|_| savvy::Error::new("Cannot convert u32 to i32."))?;

        let integer_sexp = OwnedIntegerSexp::try_from_scalar(sr)?;
        integer_sexp.into()
    }
    /// HDecodedAudio
    /// ## invalidate
    ///
    /// `invalidate()` \
    ///
    /// Replaces the inner value of the external pointer, invalidating it. \
    /// This function is useful to remove one of the shared references of the inner pointer in rust. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// fpath = "../../../testfiles/gs-16b-2c-44100hz.flac"
    /// dtype = HDataType$Float32
    /// hdecodedaudio = HFile$decode(fpath, dtype)
    /// harray = hdecodedaudio$harray() # now the inner HArray struct has 2 references.
    /// hdecodedaudio$invalidate() # back to 1 reference.
    /// ```
    ///
    /// _________
    ///
    pub fn invalidate(self) -> savvy::Result<()> {
        Ok(())
    }
}

#[savvy]
impl HFile {
    /// HFile
    /// ## decode
    ///
    /// `decode(fpath: string, dtype: HDataType) -> HDecodedAudio` \
    ///
    /// Decode an audio file, providing its decoded data and the sampling rate. \
    /// The samples are normalized to fit in the range of \[-1.0, 1.0\].
    ///
    /// #### Arguments
    ///
    /// * `fpath` \
    /// The file path as a string. \
    /// * `dtype` \
    /// A float `HDataType`. \
    ///
    /// #### Returns
    ///
    /// An HDecodedAudio containing: \
    /// * The decoded audio as a float HArray. \
    /// * The sampling rate as an integer. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// fpath = "../../../testfiles/gs-16b-2c-44100hz.flac"
    /// dtype = HDataType$Float32
    /// HFile$decode(fpath, dtype)
    /// ```
    ///
    /// _________
    ///
    fn decode(fpath: Sexp, dtype: &HDataType) -> savvy::Result<HDecodedAudio> {
        let fpath: &str = fpath.to_scalar()?;
        match dtype {
            HDataType::Float32 => {
                let (harray, sr) =
                    harmonium_io::decode::decode::<f32>(fpath).map_err(HErrorR::from)?;
                let harray = harray.into_dynamic();
                let harray = HArray(Arc::new(harray));
                Ok(HDecodedAudio { harray, sr })
            }
            HDataType::Float64 => {
                let (harray, sr) =
                    harmonium_io::decode::decode::<f64>(fpath).map_err(HErrorR::from)?;
                let harray = harray.into_dynamic();
                let harray = HArray(Arc::new(harray));
                Ok(HDecodedAudio { harray, sr })
            }
            _ => Err("Operation only allowed for float dtypes.".into()),
        }
    }

    /// HFile
    /// ## stream
    ///
    /// `decode_stream(fpath: string, frames: integer, dtype: HDataType) -> HDecoderStream` \
    ///
    /// Creates an `HDecoderStream`, used as an iterator to stream frames of decoded audio. \
    ///
    ///
    /// #### Arguments
    ///
    /// * `fpath` \
    /// The file path as a string. \
    /// * `frames` \
    /// Number of frames to decode per iteration. \
    /// * `dtype` \
    /// A float `HDataType`. \
    ///
    /// #### Returns
    ///
    /// An `HDecoderStream`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// fpath = "../../../testfiles/gs-16b-2c-44100hz.flac"
    /// dtype = HDataType$Float32
    /// frames = 1000L
    /// HFile$decode_stream(fpath, frames, dtype)
    /// ```
    ///
    /// _________
    ///
    fn decode_stream(
        fpath: Sexp,
        frames: Sexp,
        dtype: &HDataType,
    ) -> savvy::Result<HDecoderStream> {
        let fpath: &str = fpath.to_scalar()?;
        let frames: i32 = frames.to_scalar()?;
        let frames = usize::try_from(frames)
            .map_err(|_| savvy::Error::new("Cannot convert i32 to usize."))?;
        match dtype {
            HDataType::Float32 => {
                let streamer =
                    harmonium_io::decode::stream::<f32>(fpath, frames).map_err(HErrorR::from)?;
                Ok(HDecoderStream(Box::new(streamer)))
            }
            HDataType::Float64 => {
                let streamer =
                    harmonium_io::decode::stream::<f64>(fpath, frames).map_err(HErrorR::from)?;
                Ok(HDecoderStream(Box::new(streamer)))
            }
            _ => Err("Operation only allowed for float dtypes.".into()),
        }
    }

    /// HFile
    /// ## metadata
    ///
    /// `metadata(fpath: string, metadata_type: HMetadataType) -> list` \
    ///
    /// Extract text and visual metadata from a file. \
    /// Tags that are part of the container format are preferentially extracted. Additional tags that were found while probing will not be extracted. \
    /// The following metadata tagging formats are supported. \
    ///
    /// * ID3v1 \
    /// * ID3v2 \
    /// * ISO/MP4 \
    /// * RIFF \
    /// * Vorbis Comment (in OGG & FLAC) \
    ///
    /// Each `TextMetadata` will be comprised of a `Tag`, which contains the following fields: \
    /// * tag_key \
    ///     A key string indicating the type, meaning, or purpose of the Tags value. Note: The meaning of key is dependant on the underlying metadata format. \
    /// * tag_std_key \
    ///     If the Tag’s key string is commonly associated with a typical type, meaning, or purpose, then if recognized a StandardTagKey will be assigned
    ///     to this Tag. This is a best effort guess since not all metadata formats have a well defined or specified tag mapping. However, it is recommended that
    ///     consumers prefer std_key over key, if provided. \
    ///     Check [`StandardTagKey`] for all the variants. \
    /// * tag_value \
    ///     The value of the Tag. \
    ///
    /// Each `VisualMetadata` will be comprised of the following fields: \
    /// * usage: \
    ///     The usage and/or content of the Visual. A string version of `symphonia_core::meta::StandardVisualKey`, which is an enumeration providing
    ///     standardized keys for common visual dispositions. A demuxer may assign a StandardVisualKey to a Visual if the disposition of the attached visual
    ///     is known and can be mapped to a standard key. The visual types listed here are derived from, though do not entirely cover, the ID3v2 APIC frame specification. \
    /// * media_type \
    ///     The Media Type (MIME Type) used to encode the Visual. \
    /// * dimensions \
    ///     The dimensions (width and height) of the Visual, represented in pixels. \
    ///     Note: This value may not be accurate as it comes from metadata, not the
    ///     embedded graphic itself. Consider it only a hint. \
    /// * bits_per_pixel \
    ///     The number of bits-per-pixel (aka bit-depth) of the unencoded image. \
    /// * color_mode \
    ///     Indicates how the color of a pixel is encoded in a Visual. Variants: \
    ///     **Discrete \
    ///         Each pixel in the Visual stores its own color information. \
    ///     **Indexed(NonZeroU32) \
    ///         Each pixel in the Visual stores an index into a color palette containing the color information. The value stored by this variant indicates the number \
    ///         of colors in the color palette. \
    /// * size \
    ///     Size of the image in bytes. \
    /// * tag \
    ///     `Tag` with the following fields: \
    ///         ** tag_key \
    ///             A key string indicating the type, meaning, or purpose of the Tags value. Note: The meaning of key is dependant on the underlying metadata format. \
    ///         ** tag_std_key \
    ///             If the Tag’s key string is commonly associated with a typical type, meaning, or purpose, then if recognized a StandardTagKey will be assigned
    ///             to this Tag. This is a best effort guess since not all metadata formats have a well defined or specified tag mapping. However, it is recommended that
    ///             consumers prefer std_key over key, if provided. \
    ///             Check [`StandardTagKey`] for all the variants. \
    ///         ** tag_value \
    ///             The value of the Tag. \
    ///
    /// #### Arguments
    ///
    /// * `fpath` \
    /// The file path as a string. \
    /// * `metadata_type` \
    /// An `HMetadataType`. \
    ///
    /// #### Returns
    ///
    /// A list of metadata. An empty list will be returned if there is no metadata in the file. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// fpath = "../../../testfiles/gs-16b-2c-44100hz.mp3"
    /// metadata_type = HMetadataType$Text
    /// HFile$metadata(fpath, metadata_type)
    /// ```
    ///
    /// _________
    ///
    fn metadata(fpath: Sexp, metadata_type: &HMetadataType) -> savvy::Result<Sexp> {
        let fpath: &str = fpath.to_scalar()?;
        let metadata_type = match metadata_type {
            HMetadataType::All => decode::HMetadataType::All,
            HMetadataType::Text => decode::HMetadataType::Text,
            HMetadataType::Visual => decode::HMetadataType::Visual,
        };

        let opt_metadata =
            decode::metadata_from_file(fpath, metadata_type).map_err(HErrorR::from)?;
        if let Some(metadata) = opt_metadata {
            match metadata {
                decode::HMetadata::All((text, visual)) => {
                    let list1 = list_from_textmetadata(text)?;
                    let list2 = list_from_visualmetadata(visual)?;
                    let mut list = OwnedListSexp::new(2, true)?;
                    unsafe { list.set_value_unchecked(0, Sexp::from(list1).0) };
                    unsafe { list.set_value_unchecked(1, Sexp::from(list2).0) };
                    unsafe {
                        list.set_name_unchecked(
                            0,
                            Sexp::from(OwnedStringSexp::try_from_scalar("text")?).0,
                        )
                    };
                    unsafe {
                        list.set_name_unchecked(
                            1,
                            Sexp::from(OwnedStringSexp::try_from_scalar("visual")?).0,
                        )
                    };
                    Ok(list.into())
                }
                decode::HMetadata::Text(text) => list_from_textmetadata(text)?.into(),
                decode::HMetadata::Visual(visual) => list_from_visualmetadata(visual)?.into(),
            }
        } else {
            let list = OwnedListSexp::new(0, false)?;
            Ok(list.into())
        }
    }

    /// HFile
    /// ## params
    ///
    /// `params(fpath: string) -> atomicvector` \
    ///
    /// Get audio parameters from a file. \
    /// Note that this avoids loading the contents into memory, and is therefore useful for querying these parameters from long files. \
    ///
    /// #### Arguments
    ///
    /// * `fpath` \
    /// The file path as a string. \
    ///
    /// #### Returns
    ///
    /// A double atomic vector containing, in order: \
    /// * sampling rate in Hz. \
    /// * number of frames. \
    /// * number of channels. \
    /// * duration in seconds. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// fpath = "../../../testfiles/gs-16b-2c-44100hz.flac"
    /// HFile$params(fpath)
    /// ```
    ///
    /// _________
    ///
    fn params(fpath: Sexp) -> savvy::Result<Sexp> {
        let fpath: &str = fpath.to_scalar()?;
        let (sr, nframes, nchannels, duration) =
            decode::get_params_from_file(fpath).map_err(HErrorR::from)?;
        let arr = [sr as f64, nframes as f64, nchannels as f64, duration];
        let real_sexp = OwnedRealSexp::try_from_slice(arr.as_slice())?;
        Ok(real_sexp.into())
    }

    /// HFile
    /// ## verify
    ///
    /// `verify(fpath: string) -> string \
    ///
    /// Verify an audio file, if supported by the decoder. \
    /// The verification is done after the decoding process is finished. \
    ///
    /// #### Arguments
    ///
    /// * `fpath` \
    /// The file path as a string. \
    ///
    /// #### Returns
    ///
    /// A string. \
    /// One of \["passed", "failed", "not_supported"\] \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// fpath = "../../../testfiles/gs-16b-2c-44100hz.flac"
    /// HFile$verify(fpath)
    /// ```
    ///
    /// _________
    ///
    fn verify(fpath: Sexp) -> savvy::Result<Sexp> {
        let fpath: &str = fpath.to_scalar()?;
        let verified = match decode::verify_file(fpath).map_err(HErrorR::from)? {
            decode::HVerifyDecode::Passed => "passed",
            decode::HVerifyDecode::Failed => "failed",
            decode::HVerifyDecode::NotSupported => "not_supported",
        };
        let string_sexp: OwnedStringSexp = verified.try_into()?;
        Ok(string_sexp.into())
    }
}

/// HDecoderStream
/// An iterator that decodes audio in streams. \
///
/// # Methods
///
#[savvy]
impl HDecoderStream {
    /// HDecoderStream
    /// ## stream
    ///
    /// `stream() -> HArray` \
    ///
    /// Gets the next wave of frames as an `HArray`. \
    /// Returns an error if it's end of stream or if an error ocurred in
    /// the decoding process. \
    ///
    /// #### Returns
    ///
    /// The decoded audio as a float HArray. \
    /// The number of frames streamed is the one used as input in the creation of `HDecoderStream`. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// library(harmonium)
    /// fpath = "../../../testfiles/gs-16b-2c-44100hz.flac"
    /// dtype = HDataType$Float32
    /// frames = 1000L
    /// hdecoder_stream = HFile$decode_stream(fpath, frames, dtype)
    /// hdecoder_stream$stream()
    /// ```
    ///
    /// _________
    ///
    fn stream(&mut self) -> savvy::Result<HArray> {
        self.0.next()
    }
}

pub trait HDecoderStreamR {
    fn next(&mut self) -> savvy::Result<HArray>;
}

impl HDecoderStreamR for harmonium_io::decode::DecoderStream<f32> {
    fn next(&mut self) -> savvy::Result<HArray> {
        if let Some(x) = std::iter::Iterator::next(self) {
            Ok(HArray(Arc::new(x.into_dynamic())))
        } else {
            Err("The iterator has no more values to yield.".into())
        }
    }
}

impl HDecoderStreamR for harmonium_io::decode::DecoderStream<f64> {
    fn next(&mut self) -> savvy::Result<HArray> {
        if let Some(x) = std::iter::Iterator::next(self) {
            Ok(HArray(Arc::new(x.into_dynamic())))
        } else {
            Err("The iterator has no more values to yield.".into())
        }
    }
}

fn list_from_textmetadata(text: decode::HTextMetadata) -> savvy::Result<OwnedListSexp> {
    if let Some(tags_vec) = text.0 {
        let mut list = OwnedListSexp::new(tags_vec.len(), false)?;
        for (i, htag) in tags_vec.as_slice().iter().enumerate() {
            let mut string_sexp = OwnedStringSexp::try_from_slice([
                htag.tag_key.as_str(),
                htag.tag_std_key.as_str(),
                htag.tag_value.as_str(),
            ])?;
            string_sexp.set_names(&["tag_key", "tag_std_key", "tag_value"])?;
            unsafe { list.set_value_unchecked(i, Sexp::from(string_sexp).0) };
        }
        Ok(list)
    } else {
        OwnedListSexp::new(0, false)
    }
}

fn list_from_visualmetadata(visual: decode::HVisualMetadata) -> savvy::Result<OwnedListSexp> {
    if let Some(svm_vec) = visual.0 {
        let mut list = OwnedListSexp::new(svm_vec.len(), false)?;
        for (i, hsvm) in svm_vec.as_slice().iter().enumerate() {
            let mut inner_list = OwnedListSexp::new(2, false)?;
            let mut string_sexp = OwnedStringSexp::try_from_slice([
                hsvm.usage.as_str(),
                hsvm.media_type.as_str(),
                hsvm.dimensions.as_str(),
                hsvm.bpp.as_str(),
                hsvm.color_mode.as_str(),
                hsvm.size.as_str(),
            ])?;
            string_sexp.set_names(&[
                "usage",
                "media_type",
                "dimensions",
                "bpp",
                "color_mode",
                "size",
            ])?;
            unsafe { inner_list.set_value_unchecked(0, Sexp::from(string_sexp).0) };

            let inner_inner_list = if let Some(tags_vec) = &hsvm.tags_vec {
                let mut inner_inner_list = OwnedListSexp::new(tags_vec.len(), false)?;
                for (i, htag) in tags_vec.as_slice().iter().enumerate() {
                    let mut string_sexp = OwnedStringSexp::try_from_slice([
                        htag.tag_key.as_str(),
                        htag.tag_std_key.as_str(),
                        htag.tag_value.as_str(),
                    ])?;
                    string_sexp.set_names(&["tag_key", "tag_std_key", "tag_value"])?;
                    unsafe { inner_inner_list.set_value_unchecked(i, Sexp::from(string_sexp).0) };
                }
                inner_inner_list
            } else {
                OwnedListSexp::new(0, false)?
            };

            unsafe { inner_list.set_value_unchecked(1, Sexp::from(inner_inner_list).0) };

            unsafe { list.set_value_unchecked(i, Sexp::from(inner_list).0) };
        }
        Ok(list)
    } else {
        OwnedListSexp::new(0, false)
    }
}
