
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


SEXP savvy_HArray_new_from_values__impl(SEXP c_arg__arr, SEXP c_arg__dtype) {
    SEXP res = savvy_HArray_new_from_values__ffi(c_arg__arr, c_arg__dtype);
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

SEXP savvy_HArray_slice__impl(SEXP self__, SEXP c_arg__range) {
    SEXP res = savvy_HArray_slice__ffi(self__, c_arg__range);
    return handle_result(res);
}

SEXP savvy_HArray_print__impl(SEXP self__) {
    SEXP res = savvy_HArray_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_eq__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HArray_eq__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HArray_ne__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HArray_ne__ffi(self__, c_arg__other);
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

SEXP savvy_HArray_mem_adress__impl(SEXP self__) {
    SEXP res = savvy_HArray_mem_adress__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_is_standard_layout__impl(SEXP self__) {
    SEXP res = savvy_HArray_is_standard_layout__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_is_unique__impl(SEXP self__) {
    SEXP res = savvy_HArray_is_unique__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArray_invalidate__impl(SEXP self__) {
    SEXP res = savvy_HArray_invalidate__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HArrayAudio_nchannels__impl(SEXP c_arg__harray) {
    SEXP res = savvy_HArrayAudio_nchannels__ffi(c_arg__harray);
    return handle_result(res);
}

SEXP savvy_HArrayAudio_nframes__impl(SEXP c_arg__harray) {
    SEXP res = savvy_HArrayAudio_nframes__ffi(c_arg__harray);
    return handle_result(res);
}

SEXP savvy_HArrayAudio_db_to_amplitude__impl(SEXP c_arg__harray, SEXP c_arg__reference, SEXP c_arg__power) {
    SEXP res = savvy_HArrayAudio_db_to_amplitude__ffi(c_arg__harray, c_arg__reference, c_arg__power);
    return handle_result(res);
}

SEXP savvy_HArrayAudio_to_mono__impl(SEXP c_arg__harray) {
    SEXP res = savvy_HArrayAudio_to_mono__ffi(c_arg__harray);
    return handle_result(res);
}

SEXP savvy_HAudioSink_new__impl(void) {
    SEXP res = savvy_HAudioSink_new__ffi();
    return handle_result(res);
}

SEXP savvy_HAudioSink_append_from_harray__impl(SEXP self__, SEXP c_arg__harray, SEXP c_arg__sr) {
    SEXP res = savvy_HAudioSink_append_from_harray__ffi(self__, c_arg__harray, c_arg__sr);
    return handle_result(res);
}

SEXP savvy_HAudioSink_append_from_file__impl(SEXP self__, SEXP c_arg__fpath) {
    SEXP res = savvy_HAudioSink_append_from_file__ffi(self__, c_arg__fpath);
    return handle_result(res);
}

SEXP savvy_HAudioSink_audio_default_device__impl(void) {
    SEXP res = savvy_HAudioSink_audio_default_device__ffi();
    return handle_result(res);
}

SEXP savvy_HAudioSink_audio_output_devices__impl(void) {
    SEXP res = savvy_HAudioSink_audio_output_devices__ffi();
    return handle_result(res);
}

SEXP savvy_HAudioSink_audio_supported_configs__impl(void) {
    SEXP res = savvy_HAudioSink_audio_supported_configs__ffi();
    return handle_result(res);
}

SEXP savvy_HAudioSink_clear__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_clear__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_get_pos__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_get_pos__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_is_empty__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_is_empty__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_is_paused__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_is_paused__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_len__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_len__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_pause__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_pause__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_play__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_play__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_set_speed__impl(SEXP self__, SEXP c_arg__value) {
    SEXP res = savvy_HAudioSink_set_speed__ffi(self__, c_arg__value);
    return handle_result(res);
}

SEXP savvy_HAudioSink_set_volume__impl(SEXP self__, SEXP c_arg__value) {
    SEXP res = savvy_HAudioSink_set_volume__ffi(self__, c_arg__value);
    return handle_result(res);
}

SEXP savvy_HAudioSink_skip_one__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_skip_one__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_sleep_until_end__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_sleep_until_end__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_speed__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_speed__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_stop__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_stop__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_try_seek__impl(SEXP self__, SEXP c_arg__pos) {
    SEXP res = savvy_HAudioSink_try_seek__ffi(self__, c_arg__pos);
    return handle_result(res);
}

SEXP savvy_HAudioSink_volume__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_volume__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HAudioSink_invalidate__impl(SEXP self__) {
    SEXP res = savvy_HAudioSink_invalidate__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HDataType_print__impl(SEXP self__) {
    SEXP res = savvy_HDataType_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HDataType_eq__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HDataType_eq__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HDataType_ne__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HDataType_ne__ffi(self__, c_arg__other);
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

SEXP savvy_HFft_new_forward__impl(SEXP c_arg__length, SEXP c_arg__dtype) {
    SEXP res = savvy_HFft_new_forward__ffi(c_arg__length, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HFft_new_inverse__impl(SEXP c_arg__length, SEXP c_arg__dtype) {
    SEXP res = savvy_HFft_new_inverse__ffi(c_arg__length, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HFft_new_real_forward__impl(SEXP c_arg__length, SEXP c_arg__dtype) {
    SEXP res = savvy_HFft_new_real_forward__ffi(c_arg__length, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HFft_new_real_inverse__impl(SEXP c_arg__length, SEXP c_arg__dtype) {
    SEXP res = savvy_HFft_new_real_inverse__ffi(c_arg__length, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HFft_process__impl(SEXP self__, SEXP c_arg__harray) {
    SEXP res = savvy_HFft_process__ffi(self__, c_arg__harray);
    return handle_result(res);
}

SEXP savvy_HFft_dtype__impl(SEXP self__) {
    SEXP res = savvy_HFft_dtype__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HFft_print__impl(SEXP self__) {
    SEXP res = savvy_HFft_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HFft_clone__impl(SEXP self__) {
    SEXP res = savvy_HFft_clone__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HFft_is_unique__impl(SEXP self__) {
    SEXP res = savvy_HFft_is_unique__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HFft_invalidate__impl(SEXP self__) {
    SEXP res = savvy_HFft_invalidate__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HFile_decode__impl(SEXP c_arg__fpath, SEXP c_arg__dtype) {
    SEXP res = savvy_HFile_decode__ffi(c_arg__fpath, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HFile_decode_stream__impl(SEXP c_arg__fpath, SEXP c_arg__frames, SEXP c_arg__dtype) {
    SEXP res = savvy_HFile_decode_stream__ffi(c_arg__fpath, c_arg__frames, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HFile_metadata__impl(SEXP c_arg__fpath, SEXP c_arg__metadata_type) {
    SEXP res = savvy_HFile_metadata__ffi(c_arg__fpath, c_arg__metadata_type);
    return handle_result(res);
}

SEXP savvy_HFile_params__impl(SEXP c_arg__fpath) {
    SEXP res = savvy_HFile_params__ffi(c_arg__fpath);
    return handle_result(res);
}

SEXP savvy_HFile_verify__impl(SEXP c_arg__fpath) {
    SEXP res = savvy_HFile_verify__ffi(c_arg__fpath);
    return handle_result(res);
}

SEXP savvy_HInterpolationType_print__impl(SEXP self__) {
    SEXP res = savvy_HInterpolationType_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HInterpolationType_eq__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HInterpolationType_eq__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HInterpolationType_ne__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HInterpolationType_ne__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HMetadataType_print__impl(SEXP self__) {
    SEXP res = savvy_HMetadataType_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HMetadataType_eq__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HMetadataType_eq__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HMetadataType_ne__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HMetadataType_ne__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HPolynomialDegree_print__impl(SEXP self__) {
    SEXP res = savvy_HPolynomialDegree_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HPolynomialDegree_eq__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HPolynomialDegree_eq__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HPolynomialDegree_ne__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HPolynomialDegree_ne__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HResampler_new_fft__impl(SEXP c_arg__sr_in, SEXP c_arg__sr_out, SEXP c_arg__chunk_size, SEXP c_arg__sub_chunks, SEXP c_arg__nchannels, SEXP c_arg__res_type, SEXP c_arg__dtype) {
    SEXP res = savvy_HResampler_new_fft__ffi(c_arg__sr_in, c_arg__sr_out, c_arg__chunk_size, c_arg__sub_chunks, c_arg__nchannels, c_arg__res_type, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HResampler_new_sinc__impl(SEXP c_arg__resample_ratio, SEXP c_arg__max_resample_ratio_relative, SEXP c_arg__parameters, SEXP c_arg__chunk_size, SEXP c_arg__nchannels, SEXP c_arg__res_type, SEXP c_arg__dtype) {
    SEXP res = savvy_HResampler_new_sinc__ffi(c_arg__resample_ratio, c_arg__max_resample_ratio_relative, c_arg__parameters, c_arg__chunk_size, c_arg__nchannels, c_arg__res_type, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HResampler_new_fast__impl(SEXP c_arg__resample_ratio, SEXP c_arg__max_resample_ratio_relative, SEXP c_arg__pol_deg, SEXP c_arg__chunk_size, SEXP c_arg__nchannels, SEXP c_arg__res_type, SEXP c_arg__dtype) {
    SEXP res = savvy_HResampler_new_fast__ffi(c_arg__resample_ratio, c_arg__max_resample_ratio_relative, c_arg__pol_deg, c_arg__chunk_size, c_arg__nchannels, c_arg__res_type, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HResampler_process__impl(SEXP self__, SEXP c_arg__harray) {
    SEXP res = savvy_HResampler_process__ffi(self__, c_arg__harray);
    return handle_result(res);
}

SEXP savvy_HResampler_set_resample_ratio__impl(SEXP self__, SEXP c_arg__new_ratio, SEXP c_arg__ramp) {
    SEXP res = savvy_HResampler_set_resample_ratio__ffi(self__, c_arg__new_ratio, c_arg__ramp);
    return handle_result(res);
}

SEXP savvy_HResampler_set_resample_ratio_relative__impl(SEXP self__, SEXP c_arg__rel_ratio, SEXP c_arg__ramp) {
    SEXP res = savvy_HResampler_set_resample_ratio_relative__ffi(self__, c_arg__rel_ratio, c_arg__ramp);
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

SEXP savvy_HResamplerType_eq__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HResamplerType_eq__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HResamplerType_ne__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HResamplerType_ne__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HSincInterpolationParameters_new__impl(SEXP c_arg__sinc_len, SEXP c_arg__f_cutoff, SEXP c_arg__oversampling_factor, SEXP c_arg__interpolation, SEXP c_arg__window) {
    SEXP res = savvy_HSincInterpolationParameters_new__ffi(c_arg__sinc_len, c_arg__f_cutoff, c_arg__oversampling_factor, c_arg__interpolation, c_arg__window);
    return handle_result(res);
}

SEXP savvy_HSincInterpolationParameters_print__impl(SEXP self__) {
    SEXP res = savvy_HSincInterpolationParameters_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HStft_new_forward__impl(SEXP c_arg__length, SEXP c_arg__dtype) {
    SEXP res = savvy_HStft_new_forward__ffi(c_arg__length, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HStft_new_real_forward__impl(SEXP c_arg__length, SEXP c_arg__dtype) {
    SEXP res = savvy_HStft_new_real_forward__ffi(c_arg__length, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HStft_process__impl(SEXP self__, SEXP c_arg__harray, SEXP c_arg__hop_length, SEXP c_arg__window_length, SEXP c_arg__window) {
    SEXP res = savvy_HStft_process__ffi(self__, c_arg__harray, c_arg__hop_length, c_arg__window_length, c_arg__window);
    return handle_result(res);
}

SEXP savvy_HStft_dtype__impl(SEXP self__) {
    SEXP res = savvy_HStft_dtype__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HStft_print__impl(SEXP self__) {
    SEXP res = savvy_HStft_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HStft_clone__impl(SEXP self__) {
    SEXP res = savvy_HStft_clone__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HStft_is_unique__impl(SEXP self__) {
    SEXP res = savvy_HStft_is_unique__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HStft_invalidate__impl(SEXP self__) {
    SEXP res = savvy_HStft_invalidate__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HWindow_barthann__impl(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype) {
    SEXP res = savvy_HWindow_barthann__ffi(c_arg__npoints, c_arg__sym, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_bartlett__impl(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype) {
    SEXP res = savvy_HWindow_bartlett__ffi(c_arg__npoints, c_arg__sym, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_blackman__impl(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype) {
    SEXP res = savvy_HWindow_blackman__ffi(c_arg__npoints, c_arg__sym, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_blackmanharris__impl(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype) {
    SEXP res = savvy_HWindow_blackmanharris__ffi(c_arg__npoints, c_arg__sym, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_bohman__impl(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype) {
    SEXP res = savvy_HWindow_bohman__ffi(c_arg__npoints, c_arg__sym, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_boxcar__impl(SEXP c_arg__npoints, SEXP c_arg__dtype) {
    SEXP res = savvy_HWindow_boxcar__ffi(c_arg__npoints, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_cosine__impl(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype) {
    SEXP res = savvy_HWindow_cosine__ffi(c_arg__npoints, c_arg__sym, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HWindow_hann__impl(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype) {
    SEXP res = savvy_HWindow_hann__ffi(c_arg__npoints, c_arg__sym, c_arg__dtype);
    return handle_result(res);
}

SEXP savvy_HWindowType_print__impl(SEXP self__) {
    SEXP res = savvy_HWindowType_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_HWindowType_eq__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HWindowType_eq__ffi(self__, c_arg__other);
    return handle_result(res);
}

SEXP savvy_HWindowType_ne__impl(SEXP self__, SEXP c_arg__other) {
    SEXP res = savvy_HWindowType_ne__ffi(self__, c_arg__other);
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
    {"savvy_HArray_mem_adress__impl", (DL_FUNC) &savvy_HArray_mem_adress__impl, 1},
    {"savvy_HArray_is_standard_layout__impl", (DL_FUNC) &savvy_HArray_is_standard_layout__impl, 1},
    {"savvy_HArray_is_unique__impl", (DL_FUNC) &savvy_HArray_is_unique__impl, 1},
    {"savvy_HArray_invalidate__impl", (DL_FUNC) &savvy_HArray_invalidate__impl, 1},
    {"savvy_HArrayAudio_nchannels__impl", (DL_FUNC) &savvy_HArrayAudio_nchannels__impl, 1},
    {"savvy_HArrayAudio_nframes__impl", (DL_FUNC) &savvy_HArrayAudio_nframes__impl, 1},
    {"savvy_HArrayAudio_db_to_amplitude__impl", (DL_FUNC) &savvy_HArrayAudio_db_to_amplitude__impl, 3},
    {"savvy_HArrayAudio_to_mono__impl", (DL_FUNC) &savvy_HArrayAudio_to_mono__impl, 1},
    {"savvy_HAudioSink_new__impl", (DL_FUNC) &savvy_HAudioSink_new__impl, 0},
    {"savvy_HAudioSink_append_from_harray__impl", (DL_FUNC) &savvy_HAudioSink_append_from_harray__impl, 3},
    {"savvy_HAudioSink_append_from_file__impl", (DL_FUNC) &savvy_HAudioSink_append_from_file__impl, 2},
    {"savvy_HAudioSink_audio_default_device__impl", (DL_FUNC) &savvy_HAudioSink_audio_default_device__impl, 0},
    {"savvy_HAudioSink_audio_output_devices__impl", (DL_FUNC) &savvy_HAudioSink_audio_output_devices__impl, 0},
    {"savvy_HAudioSink_audio_supported_configs__impl", (DL_FUNC) &savvy_HAudioSink_audio_supported_configs__impl, 0},
    {"savvy_HAudioSink_clear__impl", (DL_FUNC) &savvy_HAudioSink_clear__impl, 1},
    {"savvy_HAudioSink_get_pos__impl", (DL_FUNC) &savvy_HAudioSink_get_pos__impl, 1},
    {"savvy_HAudioSink_is_empty__impl", (DL_FUNC) &savvy_HAudioSink_is_empty__impl, 1},
    {"savvy_HAudioSink_is_paused__impl", (DL_FUNC) &savvy_HAudioSink_is_paused__impl, 1},
    {"savvy_HAudioSink_len__impl", (DL_FUNC) &savvy_HAudioSink_len__impl, 1},
    {"savvy_HAudioSink_pause__impl", (DL_FUNC) &savvy_HAudioSink_pause__impl, 1},
    {"savvy_HAudioSink_play__impl", (DL_FUNC) &savvy_HAudioSink_play__impl, 1},
    {"savvy_HAudioSink_set_speed__impl", (DL_FUNC) &savvy_HAudioSink_set_speed__impl, 2},
    {"savvy_HAudioSink_set_volume__impl", (DL_FUNC) &savvy_HAudioSink_set_volume__impl, 2},
    {"savvy_HAudioSink_skip_one__impl", (DL_FUNC) &savvy_HAudioSink_skip_one__impl, 1},
    {"savvy_HAudioSink_sleep_until_end__impl", (DL_FUNC) &savvy_HAudioSink_sleep_until_end__impl, 1},
    {"savvy_HAudioSink_speed__impl", (DL_FUNC) &savvy_HAudioSink_speed__impl, 1},
    {"savvy_HAudioSink_stop__impl", (DL_FUNC) &savvy_HAudioSink_stop__impl, 1},
    {"savvy_HAudioSink_try_seek__impl", (DL_FUNC) &savvy_HAudioSink_try_seek__impl, 2},
    {"savvy_HAudioSink_volume__impl", (DL_FUNC) &savvy_HAudioSink_volume__impl, 1},
    {"savvy_HAudioSink_invalidate__impl", (DL_FUNC) &savvy_HAudioSink_invalidate__impl, 1},
    {"savvy_HDataType_print__impl", (DL_FUNC) &savvy_HDataType_print__impl, 1},
    {"savvy_HDataType_eq__impl", (DL_FUNC) &savvy_HDataType_eq__impl, 2},
    {"savvy_HDataType_ne__impl", (DL_FUNC) &savvy_HDataType_ne__impl, 2},
    {"savvy_HDecodedAudio_harray__impl", (DL_FUNC) &savvy_HDecodedAudio_harray__impl, 1},
    {"savvy_HDecodedAudio_sr__impl", (DL_FUNC) &savvy_HDecodedAudio_sr__impl, 1},
    {"savvy_HDecodedAudio_invalidate__impl", (DL_FUNC) &savvy_HDecodedAudio_invalidate__impl, 1},
    {"savvy_HDecoderStream_stream__impl", (DL_FUNC) &savvy_HDecoderStream_stream__impl, 1},
    {"savvy_HFft_new_forward__impl", (DL_FUNC) &savvy_HFft_new_forward__impl, 2},
    {"savvy_HFft_new_inverse__impl", (DL_FUNC) &savvy_HFft_new_inverse__impl, 2},
    {"savvy_HFft_new_real_forward__impl", (DL_FUNC) &savvy_HFft_new_real_forward__impl, 2},
    {"savvy_HFft_new_real_inverse__impl", (DL_FUNC) &savvy_HFft_new_real_inverse__impl, 2},
    {"savvy_HFft_process__impl", (DL_FUNC) &savvy_HFft_process__impl, 2},
    {"savvy_HFft_dtype__impl", (DL_FUNC) &savvy_HFft_dtype__impl, 1},
    {"savvy_HFft_print__impl", (DL_FUNC) &savvy_HFft_print__impl, 1},
    {"savvy_HFft_clone__impl", (DL_FUNC) &savvy_HFft_clone__impl, 1},
    {"savvy_HFft_is_unique__impl", (DL_FUNC) &savvy_HFft_is_unique__impl, 1},
    {"savvy_HFft_invalidate__impl", (DL_FUNC) &savvy_HFft_invalidate__impl, 1},
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
    {"savvy_HStft_new_forward__impl", (DL_FUNC) &savvy_HStft_new_forward__impl, 2},
    {"savvy_HStft_new_real_forward__impl", (DL_FUNC) &savvy_HStft_new_real_forward__impl, 2},
    {"savvy_HStft_process__impl", (DL_FUNC) &savvy_HStft_process__impl, 5},
    {"savvy_HStft_dtype__impl", (DL_FUNC) &savvy_HStft_dtype__impl, 1},
    {"savvy_HStft_print__impl", (DL_FUNC) &savvy_HStft_print__impl, 1},
    {"savvy_HStft_clone__impl", (DL_FUNC) &savvy_HStft_clone__impl, 1},
    {"savvy_HStft_is_unique__impl", (DL_FUNC) &savvy_HStft_is_unique__impl, 1},
    {"savvy_HStft_invalidate__impl", (DL_FUNC) &savvy_HStft_invalidate__impl, 1},
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

    // Functions for initialzation, if any.

}
