
#include <stdint.h>
#include <Rinternals.h>
#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}


SEXP HArray_new_from_values__impl(SEXP arr, SEXP dtype) {
    SEXP res = HArray_new_from_values(arr, dtype);
    return handle_result(res);
}

SEXP HArray_len__impl(SEXP self__) {
    SEXP res = HArray_len(self__);
    return handle_result(res);
}

SEXP HArray_shape__impl(SEXP self__) {
    SEXP res = HArray_shape(self__);
    return handle_result(res);
}

SEXP HArray_ndim__impl(SEXP self__) {
    SEXP res = HArray_ndim(self__);
    return handle_result(res);
}

SEXP HArray_slice__impl(SEXP self__, SEXP range) {
    SEXP res = HArray_slice(self__, range);
    return handle_result(res);
}

SEXP HArray_print__impl(SEXP self__) {
    SEXP res = HArray_print(self__);
    return handle_result(res);
}

SEXP HArray_eq__impl(SEXP self__, SEXP other) {
    SEXP res = HArray_eq(self__, other);
    return handle_result(res);
}

SEXP HArray_ne__impl(SEXP self__, SEXP other) {
    SEXP res = HArray_ne(self__, other);
    return handle_result(res);
}

SEXP HArray_clone__impl(SEXP self__) {
    SEXP res = HArray_clone(self__);
    return handle_result(res);
}

SEXP HArray_collect__impl(SEXP self__) {
    SEXP res = HArray_collect(self__);
    return handle_result(res);
}

SEXP HArray_dtype__impl(SEXP self__) {
    SEXP res = HArray_dtype(self__);
    return handle_result(res);
}

SEXP HArray_is_shared__impl(SEXP self__) {
    SEXP res = HArray_is_shared(self__);
    return handle_result(res);
}

SEXP HArray_mem_adress__impl(SEXP self__) {
    SEXP res = HArray_mem_adress(self__);
    return handle_result(res);
}

SEXP HAudioOp_nchannels__impl(SEXP harray) {
    SEXP res = HAudioOp_nchannels(harray);
    return handle_result(res);
}

SEXP HAudioOp_nframes__impl(SEXP harray) {
    SEXP res = HAudioOp_nframes(harray);
    return handle_result(res);
}

SEXP HAudioOp_db_to_amplitude__impl(SEXP harray, SEXP reference, SEXP power) {
    SEXP res = HAudioOp_db_to_amplitude(harray, reference, power);
    return handle_result(res);
}

SEXP HAudioOp_to_mono__impl(SEXP harray) {
    SEXP res = HAudioOp_to_mono(harray);
    return handle_result(res);
}

SEXP HAudioSink_new__impl(void) {
    SEXP res = HAudioSink_new();
    return handle_result(res);
}

SEXP HAudioSink_append_from_harray__impl(SEXP self__, SEXP harray, SEXP sr) {
    SEXP res = HAudioSink_append_from_harray(self__, harray, sr);
    return handle_result(res);
}

SEXP HAudioSink_append_from_file__impl(SEXP self__, SEXP fpath) {
    SEXP res = HAudioSink_append_from_file(self__, fpath);
    return handle_result(res);
}

SEXP HAudioSink_play__impl(SEXP self__) {
    SEXP res = HAudioSink_play(self__);
    return handle_result(res);
}

SEXP HAudioSink_stop__impl(SEXP self__) {
    SEXP res = HAudioSink_stop(self__);
    return handle_result(res);
}

SEXP HAudioSink_pause__impl(SEXP self__) {
    SEXP res = HAudioSink_pause(self__);
    return handle_result(res);
}

SEXP HAudioSink_is_paused__impl(SEXP self__) {
    SEXP res = HAudioSink_is_paused(self__);
    return handle_result(res);
}

SEXP HAudioSink_volume__impl(SEXP self__) {
    SEXP res = HAudioSink_volume(self__);
    return handle_result(res);
}

SEXP HAudioSink_set_volume__impl(SEXP self__, SEXP value) {
    SEXP res = HAudioSink_set_volume(self__, value);
    return handle_result(res);
}

SEXP HAudioSink_speed__impl(SEXP self__) {
    SEXP res = HAudioSink_speed(self__);
    return handle_result(res);
}

SEXP HAudioSink_set_speed__impl(SEXP self__, SEXP value) {
    SEXP res = HAudioSink_set_speed(self__, value);
    return handle_result(res);
}

SEXP HAudioSink_sleep_until_end__impl(SEXP self__) {
    SEXP res = HAudioSink_sleep_until_end(self__);
    return handle_result(res);
}

SEXP HAudioSink_len__impl(SEXP self__) {
    SEXP res = HAudioSink_len(self__);
    return handle_result(res);
}

SEXP HAudioSink_is_empty__impl(SEXP self__) {
    SEXP res = HAudioSink_is_empty(self__);
    return handle_result(res);
}

SEXP HAudioSink_clear__impl(SEXP self__) {
    SEXP res = HAudioSink_clear(self__);
    return handle_result(res);
}

SEXP HAudioSink_skip_one__impl(SEXP self__) {
    SEXP res = HAudioSink_skip_one(self__);
    return handle_result(res);
}

SEXP HAudioSink_audio_output_devices__impl(void) {
    SEXP res = HAudioSink_audio_output_devices();
    return handle_result(res);
}

SEXP HAudioSink_audio_default_device__impl(void) {
    SEXP res = HAudioSink_audio_default_device();
    return handle_result(res);
}

SEXP HAudioSink_audio_supported_configs__impl(void) {
    SEXP res = HAudioSink_audio_supported_configs();
    return handle_result(res);
}

SEXP HDataType_print__impl(SEXP self__) {
    SEXP res = HDataType_print(self__);
    return handle_result(res);
}

SEXP HDataType_eq__impl(SEXP self__, SEXP other) {
    SEXP res = HDataType_eq(self__, other);
    return handle_result(res);
}

SEXP HDataType_ne__impl(SEXP self__, SEXP other) {
    SEXP res = HDataType_ne(self__, other);
    return handle_result(res);
}

SEXP HDecodedAudio_harray__impl(SEXP self__) {
    SEXP res = HDecodedAudio_harray(self__);
    return handle_result(res);
}

SEXP HDecodedAudio_sr__impl(SEXP self__) {
    SEXP res = HDecodedAudio_sr(self__);
    return handle_result(res);
}

SEXP HDecoderStream_stream__impl(SEXP self__) {
    SEXP res = HDecoderStream_stream(self__);
    return handle_result(res);
}

SEXP HFft_fft__impl(SEXP harray) {
    SEXP res = HFft_fft(harray);
    return handle_result(res);
}

SEXP HFft_fft_mut__impl(SEXP harray) {
    SEXP res = HFft_fft_mut(harray);
    return handle_result(res);
}

SEXP HFft_fft_real_mut__impl(SEXP harray) {
    SEXP res = HFft_fft_real_mut(harray);
    return handle_result(res);
}

SEXP HFile_decode__impl(SEXP fpath, SEXP dtype) {
    SEXP res = HFile_decode(fpath, dtype);
    return handle_result(res);
}

SEXP HFile_decode_stream__impl(SEXP fpath, SEXP frames, SEXP dtype) {
    SEXP res = HFile_decode_stream(fpath, frames, dtype);
    return handle_result(res);
}

SEXP HFile_metadata__impl(SEXP fpath, SEXP metadata_type) {
    SEXP res = HFile_metadata(fpath, metadata_type);
    return handle_result(res);
}

SEXP HFile_params__impl(SEXP fpath) {
    SEXP res = HFile_params(fpath);
    return handle_result(res);
}

SEXP HFile_verify__impl(SEXP fpath) {
    SEXP res = HFile_verify(fpath);
    return handle_result(res);
}

SEXP HMetadataType_print__impl(SEXP self__) {
    SEXP res = HMetadataType_print(self__);
    return handle_result(res);
}

SEXP HMetadataType_eq__impl(SEXP self__, SEXP other) {
    SEXP res = HMetadataType_eq(self__, other);
    return handle_result(res);
}

SEXP HMetadataType_ne__impl(SEXP self__, SEXP other) {
    SEXP res = HMetadataType_ne(self__, other);
    return handle_result(res);
}

SEXP HPolynomialDegree_print__impl(SEXP self__) {
    SEXP res = HPolynomialDegree_print(self__);
    return handle_result(res);
}

SEXP HPolynomialDegree_eq__impl(SEXP self__, SEXP other) {
    SEXP res = HPolynomialDegree_eq(self__, other);
    return handle_result(res);
}

SEXP HPolynomialDegree_ne__impl(SEXP self__, SEXP other) {
    SEXP res = HPolynomialDegree_ne(self__, other);
    return handle_result(res);
}

SEXP HResampler_new_fft__impl(SEXP sr_in, SEXP sr_out, SEXP chunk_size, SEXP sub_chunks, SEXP nchannels, SEXP res_type, SEXP dtype) {
    SEXP res = HResampler_new_fft(sr_in, sr_out, chunk_size, sub_chunks, nchannels, res_type, dtype);
    return handle_result(res);
}

SEXP HResampler_new_sinc__impl(SEXP resample_ratio, SEXP max_resample_ratio_relative, SEXP parameters, SEXP chunk_size, SEXP nchannels, SEXP res_type, SEXP dtype) {
    SEXP res = HResampler_new_sinc(resample_ratio, max_resample_ratio_relative, parameters, chunk_size, nchannels, res_type, dtype);
    return handle_result(res);
}

SEXP HResampler_new_fast__impl(SEXP resample_ratio, SEXP max_resample_ratio_relative, SEXP pol_deg, SEXP chunk_size, SEXP nchannels, SEXP res_type, SEXP dtype) {
    SEXP res = HResampler_new_fast(resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nchannels, res_type, dtype);
    return handle_result(res);
}

SEXP HResampler_process__impl(SEXP self__, SEXP harray) {
    SEXP res = HResampler_process(self__, harray);
    return handle_result(res);
}

SEXP HResampler_set_resample_ratio__impl(SEXP self__, SEXP new_ratio, SEXP ramp) {
    SEXP res = HResampler_set_resample_ratio(self__, new_ratio, ramp);
    return handle_result(res);
}

SEXP HResampler_set_resample_ratio_relative__impl(SEXP self__, SEXP rel_ratio, SEXP ramp) {
    SEXP res = HResampler_set_resample_ratio_relative(self__, rel_ratio, ramp);
    return handle_result(res);
}

SEXP HResampler_reset__impl(SEXP self__) {
    SEXP res = HResampler_reset(self__);
    return handle_result(res);
}

SEXP HResampler_res_type__impl(SEXP self__) {
    SEXP res = HResampler_res_type(self__);
    return handle_result(res);
}

SEXP HResampler_dtype__impl(SEXP self__) {
    SEXP res = HResampler_dtype(self__);
    return handle_result(res);
}

SEXP HResampler_print__impl(SEXP self__) {
    SEXP res = HResampler_print(self__);
    return handle_result(res);
}

SEXP HResamplerType_print__impl(SEXP self__) {
    SEXP res = HResamplerType_print(self__);
    return handle_result(res);
}

SEXP HResamplerType_eq__impl(SEXP self__, SEXP other) {
    SEXP res = HResamplerType_eq(self__, other);
    return handle_result(res);
}

SEXP HResamplerType_ne__impl(SEXP self__, SEXP other) {
    SEXP res = HResamplerType_ne(self__, other);
    return handle_result(res);
}

SEXP HSincInterpolationParameters_new__impl(SEXP sinc_len, SEXP f_cutoff, SEXP oversampling_factor, SEXP interpolation, SEXP window) {
    SEXP res = HSincInterpolationParameters_new(sinc_len, f_cutoff, oversampling_factor, interpolation, window);
    return handle_result(res);
}

SEXP HSincInterpolationParameters_print__impl(SEXP self__) {
    SEXP res = HSincInterpolationParameters_print(self__);
    return handle_result(res);
}

SEXP HWindow_barthann__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = HWindow_barthann(npoints, sym, dtype);
    return handle_result(res);
}

SEXP HWindow_bartlett__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = HWindow_bartlett(npoints, sym, dtype);
    return handle_result(res);
}

SEXP HWindow_blackman__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = HWindow_blackman(npoints, sym, dtype);
    return handle_result(res);
}

SEXP HWindow_blackmanharris__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = HWindow_blackmanharris(npoints, sym, dtype);
    return handle_result(res);
}

SEXP HWindow_bohman__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = HWindow_bohman(npoints, sym, dtype);
    return handle_result(res);
}

SEXP HWindow_boxcar__impl(SEXP npoints, SEXP dtype) {
    SEXP res = HWindow_boxcar(npoints, dtype);
    return handle_result(res);
}

SEXP HWindow_cosine__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = HWindow_cosine(npoints, sym, dtype);
    return handle_result(res);
}

SEXP HWindow_hann__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = HWindow_hann(npoints, sym, dtype);
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {

    {"HArray_new_from_values__impl", (DL_FUNC) &HArray_new_from_values__impl, 2},
    {"HArray_len__impl", (DL_FUNC) &HArray_len__impl, 1},
    {"HArray_shape__impl", (DL_FUNC) &HArray_shape__impl, 1},
    {"HArray_ndim__impl", (DL_FUNC) &HArray_ndim__impl, 1},
    {"HArray_slice__impl", (DL_FUNC) &HArray_slice__impl, 2},
    {"HArray_print__impl", (DL_FUNC) &HArray_print__impl, 1},
    {"HArray_eq__impl", (DL_FUNC) &HArray_eq__impl, 2},
    {"HArray_ne__impl", (DL_FUNC) &HArray_ne__impl, 2},
    {"HArray_clone__impl", (DL_FUNC) &HArray_clone__impl, 1},
    {"HArray_collect__impl", (DL_FUNC) &HArray_collect__impl, 1},
    {"HArray_dtype__impl", (DL_FUNC) &HArray_dtype__impl, 1},
    {"HArray_is_shared__impl", (DL_FUNC) &HArray_is_shared__impl, 1},
    {"HArray_mem_adress__impl", (DL_FUNC) &HArray_mem_adress__impl, 1},
    {"HAudioOp_nchannels__impl", (DL_FUNC) &HAudioOp_nchannels__impl, 1},
    {"HAudioOp_nframes__impl", (DL_FUNC) &HAudioOp_nframes__impl, 1},
    {"HAudioOp_db_to_amplitude__impl", (DL_FUNC) &HAudioOp_db_to_amplitude__impl, 3},
    {"HAudioOp_to_mono__impl", (DL_FUNC) &HAudioOp_to_mono__impl, 1},
    {"HAudioSink_new__impl", (DL_FUNC) &HAudioSink_new__impl, 0},
    {"HAudioSink_append_from_harray__impl", (DL_FUNC) &HAudioSink_append_from_harray__impl, 3},
    {"HAudioSink_append_from_file__impl", (DL_FUNC) &HAudioSink_append_from_file__impl, 2},
    {"HAudioSink_play__impl", (DL_FUNC) &HAudioSink_play__impl, 1},
    {"HAudioSink_stop__impl", (DL_FUNC) &HAudioSink_stop__impl, 1},
    {"HAudioSink_pause__impl", (DL_FUNC) &HAudioSink_pause__impl, 1},
    {"HAudioSink_is_paused__impl", (DL_FUNC) &HAudioSink_is_paused__impl, 1},
    {"HAudioSink_volume__impl", (DL_FUNC) &HAudioSink_volume__impl, 1},
    {"HAudioSink_set_volume__impl", (DL_FUNC) &HAudioSink_set_volume__impl, 2},
    {"HAudioSink_speed__impl", (DL_FUNC) &HAudioSink_speed__impl, 1},
    {"HAudioSink_set_speed__impl", (DL_FUNC) &HAudioSink_set_speed__impl, 2},
    {"HAudioSink_sleep_until_end__impl", (DL_FUNC) &HAudioSink_sleep_until_end__impl, 1},
    {"HAudioSink_len__impl", (DL_FUNC) &HAudioSink_len__impl, 1},
    {"HAudioSink_is_empty__impl", (DL_FUNC) &HAudioSink_is_empty__impl, 1},
    {"HAudioSink_clear__impl", (DL_FUNC) &HAudioSink_clear__impl, 1},
    {"HAudioSink_skip_one__impl", (DL_FUNC) &HAudioSink_skip_one__impl, 1},
    {"HAudioSink_audio_output_devices__impl", (DL_FUNC) &HAudioSink_audio_output_devices__impl, 0},
    {"HAudioSink_audio_default_device__impl", (DL_FUNC) &HAudioSink_audio_default_device__impl, 0},
    {"HAudioSink_audio_supported_configs__impl", (DL_FUNC) &HAudioSink_audio_supported_configs__impl, 0},
    {"HDataType_print__impl", (DL_FUNC) &HDataType_print__impl, 1},
    {"HDataType_eq__impl", (DL_FUNC) &HDataType_eq__impl, 2},
    {"HDataType_ne__impl", (DL_FUNC) &HDataType_ne__impl, 2},
    {"HDecodedAudio_harray__impl", (DL_FUNC) &HDecodedAudio_harray__impl, 1},
    {"HDecodedAudio_sr__impl", (DL_FUNC) &HDecodedAudio_sr__impl, 1},
    {"HDecoderStream_stream__impl", (DL_FUNC) &HDecoderStream_stream__impl, 1},
    {"HFft_fft__impl", (DL_FUNC) &HFft_fft__impl, 1},
    {"HFft_fft_mut__impl", (DL_FUNC) &HFft_fft_mut__impl, 1},
    {"HFft_fft_real_mut__impl", (DL_FUNC) &HFft_fft_real_mut__impl, 1},
    {"HFile_decode__impl", (DL_FUNC) &HFile_decode__impl, 2},
    {"HFile_decode_stream__impl", (DL_FUNC) &HFile_decode_stream__impl, 3},
    {"HFile_metadata__impl", (DL_FUNC) &HFile_metadata__impl, 2},
    {"HFile_params__impl", (DL_FUNC) &HFile_params__impl, 1},
    {"HFile_verify__impl", (DL_FUNC) &HFile_verify__impl, 1},
    {"HMetadataType_print__impl", (DL_FUNC) &HMetadataType_print__impl, 1},
    {"HMetadataType_eq__impl", (DL_FUNC) &HMetadataType_eq__impl, 2},
    {"HMetadataType_ne__impl", (DL_FUNC) &HMetadataType_ne__impl, 2},
    {"HPolynomialDegree_print__impl", (DL_FUNC) &HPolynomialDegree_print__impl, 1},
    {"HPolynomialDegree_eq__impl", (DL_FUNC) &HPolynomialDegree_eq__impl, 2},
    {"HPolynomialDegree_ne__impl", (DL_FUNC) &HPolynomialDegree_ne__impl, 2},
    {"HResampler_new_fft__impl", (DL_FUNC) &HResampler_new_fft__impl, 7},
    {"HResampler_new_sinc__impl", (DL_FUNC) &HResampler_new_sinc__impl, 7},
    {"HResampler_new_fast__impl", (DL_FUNC) &HResampler_new_fast__impl, 7},
    {"HResampler_process__impl", (DL_FUNC) &HResampler_process__impl, 2},
    {"HResampler_set_resample_ratio__impl", (DL_FUNC) &HResampler_set_resample_ratio__impl, 3},
    {"HResampler_set_resample_ratio_relative__impl", (DL_FUNC) &HResampler_set_resample_ratio_relative__impl, 3},
    {"HResampler_reset__impl", (DL_FUNC) &HResampler_reset__impl, 1},
    {"HResampler_res_type__impl", (DL_FUNC) &HResampler_res_type__impl, 1},
    {"HResampler_dtype__impl", (DL_FUNC) &HResampler_dtype__impl, 1},
    {"HResampler_print__impl", (DL_FUNC) &HResampler_print__impl, 1},
    {"HResamplerType_print__impl", (DL_FUNC) &HResamplerType_print__impl, 1},
    {"HResamplerType_eq__impl", (DL_FUNC) &HResamplerType_eq__impl, 2},
    {"HResamplerType_ne__impl", (DL_FUNC) &HResamplerType_ne__impl, 2},
    {"HSincInterpolationParameters_new__impl", (DL_FUNC) &HSincInterpolationParameters_new__impl, 5},
    {"HSincInterpolationParameters_print__impl", (DL_FUNC) &HSincInterpolationParameters_print__impl, 1},
    {"HWindow_barthann__impl", (DL_FUNC) &HWindow_barthann__impl, 3},
    {"HWindow_bartlett__impl", (DL_FUNC) &HWindow_bartlett__impl, 3},
    {"HWindow_blackman__impl", (DL_FUNC) &HWindow_blackman__impl, 3},
    {"HWindow_blackmanharris__impl", (DL_FUNC) &HWindow_blackmanharris__impl, 3},
    {"HWindow_bohman__impl", (DL_FUNC) &HWindow_bohman__impl, 3},
    {"HWindow_boxcar__impl", (DL_FUNC) &HWindow_boxcar__impl, 2},
    {"HWindow_cosine__impl", (DL_FUNC) &HWindow_cosine__impl, 3},
    {"HWindow_hann__impl", (DL_FUNC) &HWindow_hann__impl, 3},
    {NULL, NULL, 0}
};

void R_init_harmonium(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
