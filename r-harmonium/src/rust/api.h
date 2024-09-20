

// methods and associated functions for HArray
SEXP savvy_HArray_new_from_values__ffi(SEXP c_arg__arr, SEXP c_arg__dtype);
SEXP savvy_HArray_len__ffi(SEXP self__);
SEXP savvy_HArray_shape__ffi(SEXP self__);
SEXP savvy_HArray_ndim__ffi(SEXP self__);
SEXP savvy_HArray_slice__ffi(SEXP self__, SEXP c_arg__range);
SEXP savvy_HArray_print__ffi(SEXP self__);
SEXP savvy_HArray_eq__ffi(SEXP self__, SEXP c_arg__other);
SEXP savvy_HArray_ne__ffi(SEXP self__, SEXP c_arg__other);
SEXP savvy_HArray_clone__ffi(SEXP self__);
SEXP savvy_HArray_collect__ffi(SEXP self__);
SEXP savvy_HArray_dtype__ffi(SEXP self__);
SEXP savvy_HArray_mem_adress__ffi(SEXP self__);
SEXP savvy_HArray_is_standard_layout__ffi(SEXP self__);
SEXP savvy_HArray_is_unique__ffi(SEXP self__);
SEXP savvy_HArray_invalidate__ffi(SEXP self__);

// methods and associated functions for HArrayAudio
SEXP savvy_HArrayAudio_nchannels__ffi(SEXP c_arg__harray);
SEXP savvy_HArrayAudio_nframes__ffi(SEXP c_arg__harray);
SEXP savvy_HArrayAudio_db_to_amplitude__ffi(SEXP c_arg__harray, SEXP c_arg__reference, SEXP c_arg__power);
SEXP savvy_HArrayAudio_to_mono__ffi(SEXP c_arg__harray);

// methods and associated functions for HAudioSink
SEXP savvy_HAudioSink_new__ffi(void);
SEXP savvy_HAudioSink_append_from_harray__ffi(SEXP self__, SEXP c_arg__harray, SEXP c_arg__sr);
SEXP savvy_HAudioSink_append_from_file__ffi(SEXP self__, SEXP c_arg__fpath);
SEXP savvy_HAudioSink_audio_default_device__ffi(void);
SEXP savvy_HAudioSink_audio_output_devices__ffi(void);
SEXP savvy_HAudioSink_audio_supported_configs__ffi(void);
SEXP savvy_HAudioSink_clear__ffi(SEXP self__);
SEXP savvy_HAudioSink_get_pos__ffi(SEXP self__);
SEXP savvy_HAudioSink_is_empty__ffi(SEXP self__);
SEXP savvy_HAudioSink_is_paused__ffi(SEXP self__);
SEXP savvy_HAudioSink_len__ffi(SEXP self__);
SEXP savvy_HAudioSink_pause__ffi(SEXP self__);
SEXP savvy_HAudioSink_play__ffi(SEXP self__);
SEXP savvy_HAudioSink_set_speed__ffi(SEXP self__, SEXP c_arg__value);
SEXP savvy_HAudioSink_set_volume__ffi(SEXP self__, SEXP c_arg__value);
SEXP savvy_HAudioSink_skip_one__ffi(SEXP self__);
SEXP savvy_HAudioSink_sleep_until_end__ffi(SEXP self__);
SEXP savvy_HAudioSink_speed__ffi(SEXP self__);
SEXP savvy_HAudioSink_stop__ffi(SEXP self__);
SEXP savvy_HAudioSink_try_seek__ffi(SEXP self__, SEXP c_arg__pos);
SEXP savvy_HAudioSink_volume__ffi(SEXP self__);
SEXP savvy_HAudioSink_invalidate__ffi(SEXP self__);

// methods and associated functions for HDataType
SEXP savvy_HDataType_print__ffi(SEXP self__);
SEXP savvy_HDataType_eq__ffi(SEXP self__, SEXP c_arg__other);
SEXP savvy_HDataType_ne__ffi(SEXP self__, SEXP c_arg__other);

// methods and associated functions for HDecodedAudio
SEXP savvy_HDecodedAudio_harray__ffi(SEXP self__);
SEXP savvy_HDecodedAudio_sr__ffi(SEXP self__);
SEXP savvy_HDecodedAudio_invalidate__ffi(SEXP self__);

// methods and associated functions for HDecoderStream
SEXP savvy_HDecoderStream_stream__ffi(SEXP self__);

// methods and associated functions for HFft
SEXP savvy_HFft_new_forward__ffi(SEXP c_arg__length, SEXP c_arg__dtype);
SEXP savvy_HFft_new_inverse__ffi(SEXP c_arg__length, SEXP c_arg__dtype);
SEXP savvy_HFft_new_real_forward__ffi(SEXP c_arg__length, SEXP c_arg__dtype);
SEXP savvy_HFft_new_real_inverse__ffi(SEXP c_arg__length, SEXP c_arg__dtype);
SEXP savvy_HFft_process__ffi(SEXP self__, SEXP c_arg__harray);
SEXP savvy_HFft_dtype__ffi(SEXP self__);
SEXP savvy_HFft_print__ffi(SEXP self__);
SEXP savvy_HFft_clone__ffi(SEXP self__);
SEXP savvy_HFft_is_unique__ffi(SEXP self__);
SEXP savvy_HFft_invalidate__ffi(SEXP self__);

// methods and associated functions for HFile
SEXP savvy_HFile_decode__ffi(SEXP c_arg__fpath, SEXP c_arg__dtype);
SEXP savvy_HFile_decode_stream__ffi(SEXP c_arg__fpath, SEXP c_arg__frames, SEXP c_arg__dtype);
SEXP savvy_HFile_metadata__ffi(SEXP c_arg__fpath, SEXP c_arg__metadata_type);
SEXP savvy_HFile_params__ffi(SEXP c_arg__fpath);
SEXP savvy_HFile_verify__ffi(SEXP c_arg__fpath);

// methods and associated functions for HInterpolationType
SEXP savvy_HInterpolationType_print__ffi(SEXP self__);
SEXP savvy_HInterpolationType_eq__ffi(SEXP self__, SEXP c_arg__other);
SEXP savvy_HInterpolationType_ne__ffi(SEXP self__, SEXP c_arg__other);

// methods and associated functions for HMetadataType
SEXP savvy_HMetadataType_print__ffi(SEXP self__);
SEXP savvy_HMetadataType_eq__ffi(SEXP self__, SEXP c_arg__other);
SEXP savvy_HMetadataType_ne__ffi(SEXP self__, SEXP c_arg__other);

// methods and associated functions for HPolynomialDegree
SEXP savvy_HPolynomialDegree_print__ffi(SEXP self__);
SEXP savvy_HPolynomialDegree_eq__ffi(SEXP self__, SEXP c_arg__other);
SEXP savvy_HPolynomialDegree_ne__ffi(SEXP self__, SEXP c_arg__other);

// methods and associated functions for HResampler
SEXP savvy_HResampler_new_fft__ffi(SEXP c_arg__sr_in, SEXP c_arg__sr_out, SEXP c_arg__chunk_size, SEXP c_arg__sub_chunks, SEXP c_arg__nchannels, SEXP c_arg__res_type, SEXP c_arg__dtype);
SEXP savvy_HResampler_new_sinc__ffi(SEXP c_arg__resample_ratio, SEXP c_arg__max_resample_ratio_relative, SEXP c_arg__parameters, SEXP c_arg__chunk_size, SEXP c_arg__nchannels, SEXP c_arg__res_type, SEXP c_arg__dtype);
SEXP savvy_HResampler_new_fast__ffi(SEXP c_arg__resample_ratio, SEXP c_arg__max_resample_ratio_relative, SEXP c_arg__pol_deg, SEXP c_arg__chunk_size, SEXP c_arg__nchannels, SEXP c_arg__res_type, SEXP c_arg__dtype);
SEXP savvy_HResampler_process__ffi(SEXP self__, SEXP c_arg__harray);
SEXP savvy_HResampler_set_resample_ratio__ffi(SEXP self__, SEXP c_arg__new_ratio, SEXP c_arg__ramp);
SEXP savvy_HResampler_set_resample_ratio_relative__ffi(SEXP self__, SEXP c_arg__rel_ratio, SEXP c_arg__ramp);
SEXP savvy_HResampler_reset__ffi(SEXP self__);
SEXP savvy_HResampler_res_type__ffi(SEXP self__);
SEXP savvy_HResampler_dtype__ffi(SEXP self__);
SEXP savvy_HResampler_print__ffi(SEXP self__);

// methods and associated functions for HResamplerType
SEXP savvy_HResamplerType_print__ffi(SEXP self__);
SEXP savvy_HResamplerType_eq__ffi(SEXP self__, SEXP c_arg__other);
SEXP savvy_HResamplerType_ne__ffi(SEXP self__, SEXP c_arg__other);

// methods and associated functions for HSincInterpolationParameters
SEXP savvy_HSincInterpolationParameters_new__ffi(SEXP c_arg__sinc_len, SEXP c_arg__f_cutoff, SEXP c_arg__oversampling_factor, SEXP c_arg__interpolation, SEXP c_arg__window);
SEXP savvy_HSincInterpolationParameters_print__ffi(SEXP self__);

// methods and associated functions for HStft
SEXP savvy_HStft_new_forward__ffi(SEXP c_arg__length, SEXP c_arg__dtype);
SEXP savvy_HStft_new_real_forward__ffi(SEXP c_arg__length, SEXP c_arg__dtype);
SEXP savvy_HStft_process__ffi(SEXP self__, SEXP c_arg__harray, SEXP c_arg__hop_length, SEXP c_arg__window_length, SEXP c_arg__window);
SEXP savvy_HStft_dtype__ffi(SEXP self__);
SEXP savvy_HStft_print__ffi(SEXP self__);
SEXP savvy_HStft_clone__ffi(SEXP self__);
SEXP savvy_HStft_is_unique__ffi(SEXP self__);
SEXP savvy_HStft_invalidate__ffi(SEXP self__);

// methods and associated functions for HWindow
SEXP savvy_HWindow_barthann__ffi(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype);
SEXP savvy_HWindow_bartlett__ffi(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype);
SEXP savvy_HWindow_blackman__ffi(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype);
SEXP savvy_HWindow_blackmanharris__ffi(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype);
SEXP savvy_HWindow_bohman__ffi(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype);
SEXP savvy_HWindow_boxcar__ffi(SEXP c_arg__npoints, SEXP c_arg__dtype);
SEXP savvy_HWindow_cosine__ffi(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype);
SEXP savvy_HWindow_hann__ffi(SEXP c_arg__npoints, SEXP c_arg__sym, SEXP c_arg__dtype);

// methods and associated functions for HWindowType
SEXP savvy_HWindowType_print__ffi(SEXP self__);
SEXP savvy_HWindowType_eq__ffi(SEXP self__, SEXP c_arg__other);
SEXP savvy_HWindowType_ne__ffi(SEXP self__, SEXP c_arg__other);