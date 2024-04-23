
#include <stdint.h>
#include <Rinternals.h>
#include <R_ext/Parse.h>

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


SEXP savvy_HArray_new_from_values__impl(SEXP arr, SEXP dtype) {
    SEXP res = savvy_HArray_new_from_values__ffi(arr, dtype);
    return handle_result(res);
}

SEXP savvy_HArray_len__impl(SEXP self__) {
    SEXP res = savvy_HArray_len__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_shape__impl(SEXP self__) {
    SEXP res = savvy_HArray_shape__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_ndim__impl(SEXP self__) {
    SEXP res = savvy_HArray_ndim__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_slice__impl(SEXP self__, SEXP range) {
    SEXP res = savvy_HArray_slice__ffi(self__, range);
    return handle_result(res);
}

SEXP savvy_HArray_print__impl(SEXP self__) {
    SEXP res = savvy_HArray_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_eq__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HArray_eq__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HArray_ne__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HArray_ne__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HArray_clone__impl(SEXP self__) {
    SEXP res = savvy_HArray_clone__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_collect__impl(SEXP self__) {
    SEXP res = savvy_HArray_collect__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_dtype__impl(SEXP self__) {
    SEXP res = savvy_HArray_dtype__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_is_shared__impl(SEXP self__) {
    SEXP res = savvy_HArray_is_shared__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_mem_adress__impl(SEXP self__) {
    SEXP res = savvy_HArray_mem_adress__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_invalidate__impl(SEXP self__) {
    SEXP res = savvy_HArray_invalidate__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioOp_nchannels__impl(SEXP harray) {
    SEXP res = savvy_HAudioOp_nchannels__ffi(harray);
    return handle_result(res);
}

SEXP savvy_HAudioOp_nframes__impl(SEXP harray) {
    SEXP res = savvy_HAudioOp_nframes__ffi(harray);
    return handle_result(res);
}

SEXP savvy_HAudioOp_db_to_amplitude__impl(SEXP harray, SEXP reference, SEXP power) {
    SEXP res = savvy_HAudioOp_db_to_amplitude__ffi(harray, reference, power);
    return handle_result(res);
}

SEXP savvy_HAudioOp_to_mono__impl(SEXP harray) {
    SEXP res = savvy_HAudioOp_to_mono__ffi(harray);
    return handle_result(res);
}

SEXP savvy_HAudioSink_new__impl(void) {
    SEXP res = savvy_HAudioSink_new__ffi();
    return handle_result(res);
}

SEXP savvy_HAudioSink_append_from_harray__impl(SEXP self__, SEXP harray, SEXP sr) {
    SEXP res = savvy_HAudioSink_append_from_harray__ffi(self__, harray, sr);
    return handle_result(res);
}

SEXP savvy_HAudioSink_append_from_file__impl(SEXP self__, SEXP fpath) {
    SEXP res = savvy_HAudioSink_append_from_file__ffi(self__, fpath);
    return handle_result(res);
}

SEXP savvy_HAudioSink_play__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_play__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_stop__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_stop__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_pause__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_pause__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_is_paused__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_is_paused__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_volume__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_volume__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_set_volume__impl(SEXP self__, SEXP value) {
    SEXP res = savvy_HAudioSink_set_volume__ffi(self__, value);
    return handle_result(res);
}

SEXP savvy_HAudioSink_speed__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_speed__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_set_speed__impl(SEXP self__, SEXP value) {
    SEXP res = savvy_HAudioSink_set_speed__ffi(self__, value);
    return handle_result(res);
}

SEXP savvy_HAudioSink_sleep_until_end__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_sleep_until_end__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_len__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_len__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_is_empty__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_is_empty__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_clear__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_clear__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_skip_one__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_skip_one__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_audio_output_devices__impl(void) {
    SEXP res = savvy_HAudioSink_audio_output_devices__ffi();
    return handle_result(res);
}

SEXP savvy_HAudioSink_audio_default_device__impl(void) {
    SEXP res = savvy_HAudioSink_audio_default_device__ffi();
    return handle_result(res);
}

SEXP savvy_HAudioSink_audio_supported_configs__impl(void) {
    SEXP res = savvy_HAudioSink_audio_supported_configs__ffi();
    return handle_result(res);
}

SEXP savvy_HDataType_print__impl(SEXP self__) {
    SEXP res = savvy_HDataType_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HDataType_eq__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HDataType_eq__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HDataType_ne__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HDataType_ne__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HDecodedAudio_harray__impl(SEXP self__) {
    SEXP res = savvy_HDecodedAudio_harray__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HDecodedAudio_sr__impl(SEXP self__) {
    SEXP res = savvy_HDecodedAudio_sr__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HDecodedAudio_invalidate__impl(SEXP self__) {
    SEXP res = savvy_HDecodedAudio_invalidate__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HDecoderStream_stream__impl(SEXP self__) {
    SEXP res = savvy_HDecoderStream_stream__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HFft_fft__impl(SEXP harray) {
    SEXP res = savvy_HFft_fft__ffi(harray);
    return handle_result(res);
}

SEXP savvy_HFft_fft_mut__impl(SEXP harray) {
    SEXP res = savvy_HFft_fft_mut__ffi(harray);
    return handle_result(res);
}

SEXP savvy_HFft_ifft__impl(SEXP harray) {
    SEXP res = savvy_HFft_ifft__ffi(harray);
    return handle_result(res);
}

SEXP savvy_HFft_ifft_mut__impl(SEXP harray) {
    SEXP res = savvy_HFft_ifft_mut__ffi(harray);
    return handle_result(res);
}

SEXP savvy_HFft_rfft_mut__impl(SEXP harray) {
    SEXP res = savvy_HFft_rfft_mut__ffi(harray);
    return handle_result(res);
}

SEXP savvy_HFft_irfft_mut__impl(SEXP harray, SEXP length) {
    SEXP res = savvy_HFft_irfft_mut__ffi(harray, length);
    return handle_result(res);
}

SEXP savvy_HFile_decode__impl(SEXP fpath, SEXP dtype) {
    SEXP res = savvy_HFile_decode__ffi(fpath, dtype);
    return handle_result(res);
}

SEXP savvy_HFile_decode_stream__impl(SEXP fpath, SEXP frames, SEXP dtype) {
    SEXP res = savvy_HFile_decode_stream__ffi(fpath, frames, dtype);
    return handle_result(res);
}

SEXP savvy_HFile_metadata__impl(SEXP fpath, SEXP metadata_type) {
    SEXP res = savvy_HFile_metadata__ffi(fpath, metadata_type);
    return handle_result(res);
}

SEXP savvy_HFile_params__impl(SEXP fpath) {
    SEXP res = savvy_HFile_params__ffi(fpath);
    return handle_result(res);
}

SEXP savvy_HFile_verify__impl(SEXP fpath) {
    SEXP res = savvy_HFile_verify__ffi(fpath);
    return handle_result(res);
}

SEXP savvy_HInterpolationType_print__impl(SEXP self__) {
    SEXP res = savvy_HInterpolationType_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HInterpolationType_eq__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HInterpolationType_eq__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HInterpolationType_ne__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HInterpolationType_ne__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HMetadataType_print__impl(SEXP self__) {
    SEXP res = savvy_HMetadataType_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HMetadataType_eq__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HMetadataType_eq__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HMetadataType_ne__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HMetadataType_ne__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HPolynomialDegree_print__impl(SEXP self__) {
    SEXP res = savvy_HPolynomialDegree_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HPolynomialDegree_eq__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HPolynomialDegree_eq__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HPolynomialDegree_ne__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HPolynomialDegree_ne__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HResampler_new_fft__impl(SEXP sr_in, SEXP sr_out, SEXP chunk_size, SEXP sub_chunks, SEXP nchannels, SEXP res_type, SEXP dtype) {
    SEXP res = savvy_HResampler_new_fft__ffi(sr_in, sr_out, chunk_size, sub_chunks, nchannels, res_type, dtype);
    return handle_result(res);
}

SEXP savvy_HResampler_new_sinc__impl(SEXP resample_ratio, SEXP max_resample_ratio_relative, SEXP parameters, SEXP chunk_size, SEXP nchannels, SEXP res_type, SEXP dtype) {
    SEXP res = savvy_HResampler_new_sinc__ffi(resample_ratio, max_resample_ratio_relative, parameters, chunk_size, nchannels, res_type, dtype);
    return handle_result(res);
}

SEXP savvy_HResampler_new_fast__impl(SEXP resample_ratio, SEXP max_resample_ratio_relative, SEXP pol_deg, SEXP chunk_size, SEXP nchannels, SEXP res_type, SEXP dtype) {
    SEXP res = savvy_HResampler_new_fast__ffi(resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nchannels, res_type, dtype);
    return handle_result(res);
}

SEXP savvy_HResampler_process__impl(SEXP self__, SEXP harray) {
    SEXP res = savvy_HResampler_process__ffi(self__, harray);
    return handle_result(res);
}

SEXP savvy_HResampler_set_resample_ratio__impl(SEXP self__, SEXP new_ratio, SEXP ramp) {
    SEXP res = savvy_HResampler_set_resample_ratio__ffi(self__, new_ratio, ramp);
    return handle_result(res);
}

SEXP savvy_HResampler_set_resample_ratio_relative__impl(SEXP self__, SEXP rel_ratio, SEXP ramp) {
    SEXP res = savvy_HResampler_set_resample_ratio_relative__ffi(self__, rel_ratio, ramp);
    return handle_result(res);
}

SEXP savvy_HResampler_reset__impl(SEXP self__) {
    SEXP res = savvy_HResampler_reset__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HResampler_res_type__impl(SEXP self__) {
    SEXP res = savvy_HResampler_res_type__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HResampler_dtype__impl(SEXP self__) {
    SEXP res = savvy_HResampler_dtype__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HResampler_print__impl(SEXP self__) {
    SEXP res = savvy_HResampler_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HResamplerType_print__impl(SEXP self__) {
    SEXP res = savvy_HResamplerType_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HResamplerType_eq__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HResamplerType_eq__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HResamplerType_ne__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HResamplerType_ne__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HSincInterpolationParameters_new__impl(SEXP sinc_len, SEXP f_cutoff, SEXP oversampling_factor, SEXP interpolation, SEXP window) {
    SEXP res = savvy_HSincInterpolationParameters_new__ffi(sinc_len, f_cutoff, oversampling_factor, interpolation, window);
    return handle_result(res);
}

SEXP savvy_HSincInterpolationParameters_print__impl(SEXP self__) {
    SEXP res = savvy_HSincInterpolationParameters_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HWindow_barthann__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = savvy_HWindow_barthann__ffi(npoints, sym, dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_bartlett__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = savvy_HWindow_bartlett__ffi(npoints, sym, dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_blackman__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = savvy_HWindow_blackman__ffi(npoints, sym, dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_blackmanharris__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = savvy_HWindow_blackmanharris__ffi(npoints, sym, dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_bohman__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = savvy_HWindow_bohman__ffi(npoints, sym, dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_boxcar__impl(SEXP npoints, SEXP dtype) {
    SEXP res = savvy_HWindow_boxcar__ffi(npoints, dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_cosine__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = savvy_HWindow_cosine__ffi(npoints, sym, dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_hann__impl(SEXP npoints, SEXP sym, SEXP dtype) {
    SEXP res = savvy_HWindow_hann__ffi(npoints, sym, dtype);
    return handle_result(res);
}

SEXP savvy_HWindowType_print__impl(SEXP self__) {
    SEXP res = savvy_HWindowType_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HWindowType_eq__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HWindowType_eq__ffi(self__, other);
    return handle_result(res);
}

SEXP savvy_HWindowType_ne__impl(SEXP self__, SEXP other) {
    SEXP res = savvy_HWindowType_ne__ffi(self__, other);
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {

    {"savvy_HArray_new_from_values__impl", (DL_FUNC) &savvy_HArray_new_from_values__impl, 2},
    {"savvy_HArray_len__impl", (DL_FUNC) &savvy_HArray_len__impl, 1},
    {"savvy_HArray_shape__impl", (DL_FUNC) &savvy_HArray_shape__impl, 1},
    {"savvy_HArray_ndim__impl", (DL_FUNC) &savvy_HArray_ndim__impl, 1},
    {"savvy_HArray_slice__impl", (DL_FUNC) &savvy_HArray_slice__impl, 2},
    {"savvy_HArray_print__impl", (DL_FUNC) &savvy_HArray_print__impl, 1},
    {"savvy_HArray_eq__impl", (DL_FUNC) &savvy_HArray_eq__impl, 2},
    {"savvy_HArray_ne__impl", (DL_FUNC) &savvy_HArray_ne__impl, 2},
    {"savvy_HArray_clone__impl", (DL_FUNC) &savvy_HArray_clone__impl, 1},
    {"savvy_HArray_collect__impl", (DL_FUNC) &savvy_HArray_collect__impl, 1},
    {"savvy_HArray_dtype__impl", (DL_FUNC) &savvy_HArray_dtype__impl, 1},
    {"savvy_HArray_is_shared__impl", (DL_FUNC) &savvy_HArray_is_shared__impl, 1},
    {"savvy_HArray_mem_adress__impl", (DL_FUNC) &savvy_HArray_mem_adress__impl, 1},
    {"savvy_HArray_invalidate__impl", (DL_FUNC) &savvy_HArray_invalidate__impl, 1},
    {"savvy_HAudioOp_nchannels__impl", (DL_FUNC) &savvy_HAudioOp_nchannels__impl, 1},
    {"savvy_HAudioOp_nframes__impl", (DL_FUNC) &savvy_HAudioOp_nframes__impl, 1},
    {"savvy_HAudioOp_db_to_amplitude__impl", (DL_FUNC) &savvy_HAudioOp_db_to_amplitude__impl, 3},
    {"savvy_HAudioOp_to_mono__impl", (DL_FUNC) &savvy_HAudioOp_to_mono__impl, 1},
    {"savvy_HAudioSink_new__impl", (DL_FUNC) &savvy_HAudioSink_new__impl, 0},
    {"savvy_HAudioSink_append_from_harray__impl", (DL_FUNC) &savvy_HAudioSink_append_from_harray__impl, 3},
    {"savvy_HAudioSink_append_from_file__impl", (DL_FUNC) &savvy_HAudioSink_append_from_file__impl, 2},
    {"savvy_HAudioSink_play__impl", (DL_FUNC) &savvy_HAudioSink_play__impl, 1},
    {"savvy_HAudioSink_stop__impl", (DL_FUNC) &savvy_HAudioSink_stop__impl, 1},
    {"savvy_HAudioSink_pause__impl", (DL_FUNC) &savvy_HAudioSink_pause__impl, 1},
    {"savvy_HAudioSink_is_paused__impl", (DL_FUNC) &savvy_HAudioSink_is_paused__impl, 1},
    {"savvy_HAudioSink_volume__impl", (DL_FUNC) &savvy_HAudioSink_volume__impl, 1},
    {"savvy_HAudioSink_set_volume__impl", (DL_FUNC) &savvy_HAudioSink_set_volume__impl, 2},
    {"savvy_HAudioSink_speed__impl", (DL_FUNC) &savvy_HAudioSink_speed__impl, 1},
    {"savvy_HAudioSink_set_speed__impl", (DL_FUNC) &savvy_HAudioSink_set_speed__impl, 2},
    {"savvy_HAudioSink_sleep_until_end__impl", (DL_FUNC) &savvy_HAudioSink_sleep_until_end__impl, 1},
    {"savvy_HAudioSink_len__impl", (DL_FUNC) &savvy_HAudioSink_len__impl, 1},
    {"savvy_HAudioSink_is_empty__impl", (DL_FUNC) &savvy_HAudioSink_is_empty__impl, 1},
    {"savvy_HAudioSink_clear__impl", (DL_FUNC) &savvy_HAudioSink_clear__impl, 1},
    {"savvy_HAudioSink_skip_one__impl", (DL_FUNC) &savvy_HAudioSink_skip_one__impl, 1},
    {"savvy_HAudioSink_audio_output_devices__impl", (DL_FUNC) &savvy_HAudioSink_audio_output_devices__impl, 0},
    {"savvy_HAudioSink_audio_default_device__impl", (DL_FUNC) &savvy_HAudioSink_audio_default_device__impl, 0},
    {"savvy_HAudioSink_audio_supported_configs__impl", (DL_FUNC) &savvy_HAudioSink_audio_supported_configs__impl, 0},
    {"savvy_HDataType_print__impl", (DL_FUNC) &savvy_HDataType_print__impl, 1},
    {"savvy_HDataType_eq__impl", (DL_FUNC) &savvy_HDataType_eq__impl, 2},
    {"savvy_HDataType_ne__impl", (DL_FUNC) &savvy_HDataType_ne__impl, 2},
    {"savvy_HDecodedAudio_harray__impl", (DL_FUNC) &savvy_HDecodedAudio_harray__impl, 1},
    {"savvy_HDecodedAudio_sr__impl", (DL_FUNC) &savvy_HDecodedAudio_sr__impl, 1},
    {"savvy_HDecodedAudio_invalidate__impl", (DL_FUNC) &savvy_HDecodedAudio_invalidate__impl, 1},
    {"savvy_HDecoderStream_stream__impl", (DL_FUNC) &savvy_HDecoderStream_stream__impl, 1},
    {"savvy_HFft_fft__impl", (DL_FUNC) &savvy_HFft_fft__impl, 1},
    {"savvy_HFft_fft_mut__impl", (DL_FUNC) &savvy_HFft_fft_mut__impl, 1},
    {"savvy_HFft_ifft__impl", (DL_FUNC) &savvy_HFft_ifft__impl, 1},
    {"savvy_HFft_ifft_mut__impl", (DL_FUNC) &savvy_HFft_ifft_mut__impl, 1},
    {"savvy_HFft_rfft_mut__impl", (DL_FUNC) &savvy_HFft_rfft_mut__impl, 1},
    {"savvy_HFft_irfft_mut__impl", (DL_FUNC) &savvy_HFft_irfft_mut__impl, 2},
    {"savvy_HFile_decode__impl", (DL_FUNC) &savvy_HFile_decode__impl, 2},
    {"savvy_HFile_decode_stream__impl", (DL_FUNC) &savvy_HFile_decode_stream__impl, 3},
    {"savvy_HFile_metadata__impl", (DL_FUNC) &savvy_HFile_metadata__impl, 2},
    {"savvy_HFile_params__impl", (DL_FUNC) &savvy_HFile_params__impl, 1},
    {"savvy_HFile_verify__impl", (DL_FUNC) &savvy_HFile_verify__impl, 1},
    {"savvy_HInterpolationType_print__impl", (DL_FUNC) &savvy_HInterpolationType_print__impl, 1},
    {"savvy_HInterpolationType_eq__impl", (DL_FUNC) &savvy_HInterpolationType_eq__impl, 2},
    {"savvy_HInterpolationType_ne__impl", (DL_FUNC) &savvy_HInterpolationType_ne__impl, 2},
    {"savvy_HMetadataType_print__impl", (DL_FUNC) &savvy_HMetadataType_print__impl, 1},
    {"savvy_HMetadataType_eq__impl", (DL_FUNC) &savvy_HMetadataType_eq__impl, 2},
    {"savvy_HMetadataType_ne__impl", (DL_FUNC) &savvy_HMetadataType_ne__impl, 2},
    {"savvy_HPolynomialDegree_print__impl", (DL_FUNC) &savvy_HPolynomialDegree_print__impl, 1},
    {"savvy_HPolynomialDegree_eq__impl", (DL_FUNC) &savvy_HPolynomialDegree_eq__impl, 2},
    {"savvy_HPolynomialDegree_ne__impl", (DL_FUNC) &savvy_HPolynomialDegree_ne__impl, 2},
    {"savvy_HResampler_new_fft__impl", (DL_FUNC) &savvy_HResampler_new_fft__impl, 7},
    {"savvy_HResampler_new_sinc__impl", (DL_FUNC) &savvy_HResampler_new_sinc__impl, 7},
    {"savvy_HResampler_new_fast__impl", (DL_FUNC) &savvy_HResampler_new_fast__impl, 7},
    {"savvy_HResampler_process__impl", (DL_FUNC) &savvy_HResampler_process__impl, 2},
    {"savvy_HResampler_set_resample_ratio__impl", (DL_FUNC) &savvy_HResampler_set_resample_ratio__impl, 3},
    {"savvy_HResampler_set_resample_ratio_relative__impl", (DL_FUNC) &savvy_HResampler_set_resample_ratio_relative__impl, 3},
    {"savvy_HResampler_reset__impl", (DL_FUNC) &savvy_HResampler_reset__impl, 1},
    {"savvy_HResampler_res_type__impl", (DL_FUNC) &savvy_HResampler_res_type__impl, 1},
    {"savvy_HResampler_dtype__impl", (DL_FUNC) &savvy_HResampler_dtype__impl, 1},
    {"savvy_HResampler_print__impl", (DL_FUNC) &savvy_HResampler_print__impl, 1},
    {"savvy_HResamplerType_print__impl", (DL_FUNC) &savvy_HResamplerType_print__impl, 1},
    {"savvy_HResamplerType_eq__impl", (DL_FUNC) &savvy_HResamplerType_eq__impl, 2},
    {"savvy_HResamplerType_ne__impl", (DL_FUNC) &savvy_HResamplerType_ne__impl, 2},
    {"savvy_HSincInterpolationParameters_new__impl", (DL_FUNC) &savvy_HSincInterpolationParameters_new__impl, 5},
    {"savvy_HSincInterpolationParameters_print__impl", (DL_FUNC) &savvy_HSincInterpolationParameters_print__impl, 1},
    {"savvy_HWindow_barthann__impl", (DL_FUNC) &savvy_HWindow_barthann__impl, 3},
    {"savvy_HWindow_bartlett__impl", (DL_FUNC) &savvy_HWindow_bartlett__impl, 3},
    {"savvy_HWindow_blackman__impl", (DL_FUNC) &savvy_HWindow_blackman__impl, 3},
    {"savvy_HWindow_blackmanharris__impl", (DL_FUNC) &savvy_HWindow_blackmanharris__impl, 3},
    {"savvy_HWindow_bohman__impl", (DL_FUNC) &savvy_HWindow_bohman__impl, 3},
    {"savvy_HWindow_boxcar__impl", (DL_FUNC) &savvy_HWindow_boxcar__impl, 2},
    {"savvy_HWindow_cosine__impl", (DL_FUNC) &savvy_HWindow_cosine__impl, 3},
    {"savvy_HWindow_hann__impl", (DL_FUNC) &savvy_HWindow_hann__impl, 3},
    {"savvy_HWindowType_print__impl", (DL_FUNC) &savvy_HWindowType_print__impl, 1},
    {"savvy_HWindowType_eq__impl", (DL_FUNC) &savvy_HWindowType_eq__impl, 2},
    {"savvy_HWindowType_ne__impl", (DL_FUNC) &savvy_HWindowType_ne__impl, 2},
    {NULL, NULL, 0}
};

void R_init_harmonium(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
