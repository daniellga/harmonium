

// methods and associated functions for HArray
SEXP savvy_HArray_new_from_values__ffi(SEXP arr, SEXP dtype);
SEXP savvy_HArray_len__ffi(SEXP self__);
SEXP savvy_HArray_shape__ffi(SEXP self__);
SEXP savvy_HArray_ndim__ffi(SEXP self__);
SEXP savvy_HArray_slice__ffi(SEXP self__, SEXP range);
SEXP savvy_HArray_print__ffi(SEXP self__);
SEXP savvy_HArray_eq__ffi(SEXP self__, SEXP other);
SEXP savvy_HArray_ne__ffi(SEXP self__, SEXP other);
SEXP savvy_HArray_clone__ffi(SEXP self__);
SEXP savvy_HArray_collect__ffi(SEXP self__);
SEXP savvy_HArray_dtype__ffi(SEXP self__);
SEXP savvy_HArray_is_shared__ffi(SEXP self__);
SEXP savvy_HArray_mem_adress__ffi(SEXP self__);
SEXP savvy_HArray_is_standard_layout__ffi(SEXP self__);
SEXP savvy_HArray_invalidate__ffi(SEXP self__);

// methods and associated functions for HAudioOp
SEXP savvy_HAudioOp_nchannels__ffi(SEXP harray);
SEXP savvy_HAudioOp_nframes__ffi(SEXP harray);
SEXP savvy_HAudioOp_db_to_amplitude__ffi(SEXP harray, SEXP reference, SEXP power);
SEXP savvy_HAudioOp_to_mono__ffi(SEXP harray);

// methods and associated functions for HAudioSink
SEXP savvy_HAudioSink_new__ffi(void);
SEXP savvy_HAudioSink_append_from_harray__ffi(SEXP self__, SEXP harray, SEXP sr);
SEXP savvy_HAudioSink_append_from_file__ffi(SEXP self__, SEXP fpath);
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
SEXP savvy_HAudioSink_set_speed__ffi(SEXP self__, SEXP value);
SEXP savvy_HAudioSink_set_volume__ffi(SEXP self__, SEXP value);
SEXP savvy_HAudioSink_skip_one__ffi(SEXP self__);
SEXP savvy_HAudioSink_sleep_until_end__ffi(SEXP self__);
SEXP savvy_HAudioSink_speed__ffi(SEXP self__);
SEXP savvy_HAudioSink_stop__ffi(SEXP self__);
SEXP savvy_HAudioSink_try_seek__ffi(SEXP self__, SEXP pos);
SEXP savvy_HAudioSink_volume__ffi(SEXP self__);
SEXP savvy_HAudioSink_invalidate__ffi(SEXP self__);

// methods and associated functions for HDataType
SEXP savvy_HDataType_print__ffi(SEXP self__);
SEXP savvy_HDataType_eq__ffi(SEXP self__, SEXP other);
SEXP savvy_HDataType_ne__ffi(SEXP self__, SEXP other);

// methods and associated functions for HDecodedAudio
SEXP savvy_HDecodedAudio_harray__ffi(SEXP self__);
SEXP savvy_HDecodedAudio_sr__ffi(SEXP self__);
SEXP savvy_HDecodedAudio_invalidate__ffi(SEXP self__);

// methods and associated functions for HDecoderStream
SEXP savvy_HDecoderStream_stream__ffi(SEXP self__);

// methods and associated functions for HFftPlanner
SEXP savvy_HFftPlanner_new__ffi(SEXP dtype);
SEXP savvy_HFftPlanner_fft__ffi(SEXP self__, SEXP harray);
SEXP savvy_HFftPlanner_ifft__ffi(SEXP self__, SEXP harray);
SEXP savvy_HFftPlanner_dtype__ffi(SEXP self__);
SEXP savvy_HFftPlanner_print__ffi(SEXP self__);

// methods and associated functions for HFile
SEXP savvy_HFile_decode__ffi(SEXP fpath, SEXP dtype);
SEXP savvy_HFile_decode_stream__ffi(SEXP fpath, SEXP frames, SEXP dtype);
SEXP savvy_HFile_metadata__ffi(SEXP fpath, SEXP metadata_type);
SEXP savvy_HFile_params__ffi(SEXP fpath);
SEXP savvy_HFile_verify__ffi(SEXP fpath);

// methods and associated functions for HInterpolationType
SEXP savvy_HInterpolationType_print__ffi(SEXP self__);
SEXP savvy_HInterpolationType_eq__ffi(SEXP self__, SEXP other);
SEXP savvy_HInterpolationType_ne__ffi(SEXP self__, SEXP other);

// methods and associated functions for HMetadataType
SEXP savvy_HMetadataType_print__ffi(SEXP self__);
SEXP savvy_HMetadataType_eq__ffi(SEXP self__, SEXP other);
SEXP savvy_HMetadataType_ne__ffi(SEXP self__, SEXP other);

// methods and associated functions for HPolynomialDegree
SEXP savvy_HPolynomialDegree_print__ffi(SEXP self__);
SEXP savvy_HPolynomialDegree_eq__ffi(SEXP self__, SEXP other);
SEXP savvy_HPolynomialDegree_ne__ffi(SEXP self__, SEXP other);

// methods and associated functions for HRealFftPlanner
SEXP savvy_HRealFftPlanner_new__ffi(SEXP dtype);
SEXP savvy_HRealFftPlanner_rfft__ffi(SEXP self__, SEXP harray);
SEXP savvy_HRealFftPlanner_irfft__ffi(SEXP self__, SEXP harray, SEXP length);
SEXP savvy_HRealFftPlanner_dtype__ffi(SEXP self__);
SEXP savvy_HRealFftPlanner_print__ffi(SEXP self__);

// methods and associated functions for HResampler
SEXP savvy_HResampler_new_fft__ffi(SEXP sr_in, SEXP sr_out, SEXP chunk_size, SEXP sub_chunks, SEXP nchannels, SEXP res_type, SEXP dtype);
SEXP savvy_HResampler_new_sinc__ffi(SEXP resample_ratio, SEXP max_resample_ratio_relative, SEXP parameters, SEXP chunk_size, SEXP nchannels, SEXP res_type, SEXP dtype);
SEXP savvy_HResampler_new_fast__ffi(SEXP resample_ratio, SEXP max_resample_ratio_relative, SEXP pol_deg, SEXP chunk_size, SEXP nchannels, SEXP res_type, SEXP dtype);
SEXP savvy_HResampler_process__ffi(SEXP self__, SEXP harray);
SEXP savvy_HResampler_set_resample_ratio__ffi(SEXP self__, SEXP new_ratio, SEXP ramp);
SEXP savvy_HResampler_set_resample_ratio_relative__ffi(SEXP self__, SEXP rel_ratio, SEXP ramp);
SEXP savvy_HResampler_reset__ffi(SEXP self__);
SEXP savvy_HResampler_res_type__ffi(SEXP self__);
SEXP savvy_HResampler_dtype__ffi(SEXP self__);
SEXP savvy_HResampler_print__ffi(SEXP self__);

// methods and associated functions for HResamplerType
SEXP savvy_HResamplerType_print__ffi(SEXP self__);
SEXP savvy_HResamplerType_eq__ffi(SEXP self__, SEXP other);
SEXP savvy_HResamplerType_ne__ffi(SEXP self__, SEXP other);

// methods and associated functions for HSincInterpolationParameters
SEXP savvy_HSincInterpolationParameters_new__ffi(SEXP sinc_len, SEXP f_cutoff, SEXP oversampling_factor, SEXP interpolation, SEXP window);
SEXP savvy_HSincInterpolationParameters_print__ffi(SEXP self__);

// methods and associated functions for HWindow
SEXP savvy_HWindow_barthann__ffi(SEXP npoints, SEXP sym, SEXP dtype);
SEXP savvy_HWindow_bartlett__ffi(SEXP npoints, SEXP sym, SEXP dtype);
SEXP savvy_HWindow_blackman__ffi(SEXP npoints, SEXP sym, SEXP dtype);
SEXP savvy_HWindow_blackmanharris__ffi(SEXP npoints, SEXP sym, SEXP dtype);
SEXP savvy_HWindow_bohman__ffi(SEXP npoints, SEXP sym, SEXP dtype);
SEXP savvy_HWindow_boxcar__ffi(SEXP npoints, SEXP dtype);
SEXP savvy_HWindow_cosine__ffi(SEXP npoints, SEXP sym, SEXP dtype);
SEXP savvy_HWindow_hann__ffi(SEXP npoints, SEXP sym, SEXP dtype);

// methods and associated functions for HWindowType
SEXP savvy_HWindowType_print__ffi(SEXP self__);
SEXP savvy_HWindowType_eq__ffi(SEXP self__, SEXP other);
SEXP savvy_HWindowType_ne__ffi(SEXP self__, SEXP other);