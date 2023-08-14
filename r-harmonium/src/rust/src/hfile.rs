use extendr_api::prelude::*;
use harmonium_core::conversions::IntoDynamic;
use harmonium_io::decode;
use std::sync::Arc;

use crate::{harray::HArray, hdatatype::HDataType, hmetadatatype::HMetadataType};

struct HFile;

#[extendr]
impl HFile {
    /// HFile
    /// ## decode
    ///
    /// `decode(fpath: string, dtype: HDataType) -> list[HArray, integer]` \
    ///
    /// Decode an audio file, providing its decoded data and the sampling rate. \
    ///
    /// #### Arguments
    ///
    /// * `fpath` \
    /// The file path as a string. \
    ///
    /// #### Returns
    ///
    /// A list containing: \
    /// * The decoded audio as a float HArray. \
    /// * The sampling rate as an integer. \
    ///
    /// #### Examples
    ///
    /// ```r
    /// fpath = "testfiles/gs-16b-2c-44100hz.flac"
    /// dtype = HDataType$float32
    /// HFile$decode(fpath, dtype)
    /// ```
    ///
    /// _________
    ///
    fn decode(fpath: &str, dtype: &HDataType) -> Robj {
        match dtype {
            HDataType::Float32 => {
                let (harray, sr) = harmonium_io::decode::decode::<f32>(fpath).unwrap();
                let harray = harray.into_dynamic();
                let harray = HArray(Arc::new(harray));
                let sr = Rint::from(sr as i32);
                List::from_values(&[Robj::from(harray), Robj::from(sr)]).into()
            }
            HDataType::Float64 => {
                let (harray, sr) = harmonium_io::decode::decode::<f64>(fpath).unwrap();
                let harray = harray.into_dynamic();
                let harray = HArray(Arc::new(harray));
                let sr = Rint::from(sr as i32);
                List::from_values(&[Robj::from(harray), Robj::from(sr)]).into()
            }
            _ => panic!("Operation only allowed for float dtypes."),
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
    /// fpath = "testfiles/gs-16b-2c-44100hz.mp3"
    /// metadata_type = HMetadataType$text
    /// HFile$decode(fpath, metadata_type)
    /// ```
    ///
    /// _________
    ///
    fn metadata(fpath: &str, metadata_type: &HMetadataType) -> Robj {
        let metadata_type = match metadata_type {
            HMetadataType::All => decode::HMetadataType::All,
            HMetadataType::Text => decode::HMetadataType::Text,
            HMetadataType::Visual => decode::HMetadataType::Visual,
        };

        let opt_metadata = decode::metadata_from_file(fpath, metadata_type).unwrap();
        if let Some(metadata) = opt_metadata {
            match metadata {
                decode::HMetadata::All((text, visual)) => {
                    let list1 = list_from_textmetadata(text);
                    let list2 = list_from_visualmetadata(visual);
                    List::from_names_and_values(&["text", "visual"], &[list1, list2])
                        .unwrap()
                        .into()
                }
                decode::HMetadata::Text(text) => list_from_textmetadata(text).into(),
                decode::HMetadata::Visual(visual) => list_from_visualmetadata(visual).into(),
            }
        } else {
            List::new(0).into()
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
    /// fpath = "testfiles/gs-16b-2c-44100hz.flac"
    /// HFile$params(fpath)
    /// ```
    ///
    /// _________
    ///
    fn params(fpath: &str) -> Robj {
        let (sr, nframes, nchannels, duration) = decode::get_params_from_file(fpath).unwrap();
        let arr = [sr as f64, nframes as f64, nchannels as f64, duration];
        let doubles: Doubles = arr.iter().map(|x| Rfloat::from(*x)).collect();
        doubles.into()
    }

    /// Verify an audio file.
    /// @description
    ///
    /// @param fpath \[String\] - The path to the input file.
    /// @examples
    /// fname = "audio_test_files/gs-16b-2c-44100hz.flac"
    /// verify_file(fname)

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
    /// fpath = "testfiles/gs-16b-2c-44100hz.flac"
    /// HFile$verify(fpath)
    /// ```
    ///
    /// _________
    ///
    fn verify(fpath: &str) -> &str {
        match decode::verify_file(fpath).unwrap() {
            decode::HVerifyDecode::Passed => "passed",
            decode::HVerifyDecode::Failed => "failed",
            decode::HVerifyDecode::NotSupported => "not_supported",
        }
    }
}

fn list_from_textmetadata(text: decode::HTextMetadata) -> List {
    if let Some(tags_vec) = text.0 {
        let iter = tags_vec.iter().map(|x| {
            let strings =
                Strings::from_values(&[&x.tag_key[..], &x.tag_std_key[..], &x.tag_value[..]]);
            let mut robj: Robj = strings.into();
            robj.set_names(&["tag_key", "tag_std_key", "tag_value"])
                .unwrap();
            robj
        });
        List::from_values(iter)
    } else {
        List::new(0)
    }
}

fn list_from_visualmetadata(visual: decode::HVisualMetadata) -> List {
    if let Some(svm_vec) = visual.0 {
        let iter = svm_vec.iter().map(|x| {
            let strings = Strings::from_values(&[
                &x.usage[..],
                &x.media_type[..],
                &x.dimensions[..],
                &x.bpp[..],
                &x.color_mode[..],
                &x.size[..],
            ]);

            let mut robj1: Robj = strings.into();
            robj1
                .set_names(&[
                    "usage",
                    "media_type",
                    "dimensions",
                    "bpp",
                    "color_mode",
                    "size",
                ])
                .unwrap();

            let robj2: Robj = if let Some(tags_vec) = &x.tags_vec {
                let inner_iter = tags_vec.iter().map(|z| {
                    let strings = Strings::from_values(&[
                        &z.tag_key[..],
                        &z.tag_std_key[..],
                        &z.tag_value[..],
                    ]);
                    let mut robj: Robj = strings.into();
                    robj.set_names(&["tag_key", "tag_std_key", "tag_value"])
                        .unwrap();
                    robj
                });
                List::from_values(inner_iter).into()
            } else {
                List::new(0).into()
            };

            List::from_values(&[robj1, robj2])
        });
        List::from_values(iter)
    } else {
        List::new(0)
    }
}

extendr_module! {
    mod hfile;
    impl HFile;
}
