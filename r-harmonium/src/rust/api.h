

// methods and associated functions for HArray
SEXP HArray_new_from_values(SEXP arr, SEXP dtype);
SEXP HArray_len(SEXP self__);
SEXP HArray_shape(SEXP self__);
SEXP HArray_ndim(SEXP self__);
SEXP HArray_slice(SEXP self__, SEXP range);
SEXP HArray_print(SEXP self__);
SEXP HArray_eq(SEXP self__, SEXP other);
SEXP HArray_ne(SEXP self__, SEXP other);
SEXP HArray_clone(SEXP self__);
SEXP HArray_collect(SEXP self__);
SEXP HArray_dtype(SEXP self__);
SEXP HArray_is_shared(SEXP self__);
SEXP HArray_mem_adress(SEXP self__);
SEXP HArray_invalidate(SEXP self__);

// methods and associated functions for HAudioOp
SEXP HAudioOp_nchannels(SEXP harray);
SEXP HAudioOp_nframes(SEXP harray);
SEXP HAudioOp_db_to_amplitude(SEXP harray, SEXP reference, SEXP power);
SEXP HAudioOp_to_mono(SEXP harray);

// methods and associated functions for HAudioSink
SEXP HAudioSink_new(void);
SEXP HAudioSink_append_from_harray(SEXP self__, SEXP harray, SEXP sr);
SEXP HAudioSink_append_from_file(SEXP self__, SEXP fpath);
SEXP HAudioSink_play(SEXP self__);
SEXP HAudioSink_stop(SEXP self__);
SEXP HAudioSink_pause(SEXP self__);
SEXP HAudioSink_is_paused(SEXP self__);
SEXP HAudioSink_volume(SEXP self__);
SEXP HAudioSink_set_volume(SEXP self__, SEXP value);
SEXP HAudioSink_speed(SEXP self__);
SEXP HAudioSink_set_speed(SEXP self__, SEXP value);
SEXP HAudioSink_sleep_until_end(SEXP self__);
SEXP HAudioSink_len(SEXP self__);
SEXP HAudioSink_is_empty(SEXP self__);
SEXP HAudioSink_clear(SEXP self__);
SEXP HAudioSink_skip_one(SEXP self__);
SEXP HAudioSink_audio_output_devices(void);
SEXP HAudioSink_audio_default_device(void);
SEXP HAudioSink_audio_supported_configs(void);

// methods and associated functions for HDataType
SEXP HDataType_print(SEXP self__);
SEXP HDataType_eq(SEXP self__, SEXP other);
SEXP HDataType_ne(SEXP self__, SEXP other);

// methods and associated functions for HDecodedAudio
SEXP HDecodedAudio_harray(SEXP self__);
SEXP HDecodedAudio_sr(SEXP self__);
SEXP HDecodedAudio_invalidate(SEXP self__);

// methods and associated functions for HDecoderStream
SEXP HDecoderStream_stream(SEXP self__);

// methods and associated functions for HFft
SEXP HFft_fft(SEXP harray);
SEXP HFft_fft_mut(SEXP harray);
SEXP HFft_fft_real_mut(SEXP harray);

// methods and associated functions for HFile
SEXP HFile_decode(SEXP fpath, SEXP dtype);
SEXP HFile_decode_stream(SEXP fpath, SEXP frames, SEXP dtype);
SEXP HFile_metadata(SEXP fpath, SEXP metadata_type);
SEXP HFile_params(SEXP fpath);
SEXP HFile_verify(SEXP fpath);

// methods and associated functions for HMetadataType
SEXP HMetadataType_print(SEXP self__);
SEXP HMetadataType_eq(SEXP self__, SEXP other);
SEXP HMetadataType_ne(SEXP self__, SEXP other);

// methods and associated functions for HPolynomialDegree
SEXP HPolynomialDegree_print(SEXP self__);
SEXP HPolynomialDegree_eq(SEXP self__, SEXP other);
SEXP HPolynomialDegree_ne(SEXP self__, SEXP other);

// methods and associated functions for HResampler
SEXP HResampler_new_fft(SEXP sr_in, SEXP sr_out, SEXP chunk_size, SEXP sub_chunks, SEXP nchannels, SEXP res_type, SEXP dtype);
SEXP HResampler_new_sinc(SEXP resample_ratio, SEXP max_resample_ratio_relative, SEXP parameters, SEXP chunk_size, SEXP nchannels, SEXP res_type, SEXP dtype);
SEXP HResampler_new_fast(SEXP resample_ratio, SEXP max_resample_ratio_relative, SEXP pol_deg, SEXP chunk_size, SEXP nchannels, SEXP res_type, SEXP dtype);
SEXP HResampler_process(SEXP self__, SEXP harray);
SEXP HResampler_set_resample_ratio(SEXP self__, SEXP new_ratio, SEXP ramp);
SEXP HResampler_set_resample_ratio_relative(SEXP self__, SEXP rel_ratio, SEXP ramp);
SEXP HResampler_reset(SEXP self__);
SEXP HResampler_res_type(SEXP self__);
SEXP HResampler_dtype(SEXP self__);
SEXP HResampler_print(SEXP self__);

// methods and associated functions for HResamplerType
SEXP HResamplerType_print(SEXP self__);
SEXP HResamplerType_eq(SEXP self__, SEXP other);
SEXP HResamplerType_ne(SEXP self__, SEXP other);

// methods and associated functions for HSincInterpolationParameters
SEXP HSincInterpolationParameters_new(SEXP sinc_len, SEXP f_cutoff, SEXP oversampling_factor, SEXP interpolation, SEXP window);
SEXP HSincInterpolationParameters_print(SEXP self__);

// methods and associated functions for HWindow
SEXP HWindow_barthann(SEXP npoints, SEXP sym, SEXP dtype);
SEXP HWindow_bartlett(SEXP npoints, SEXP sym, SEXP dtype);
SEXP HWindow_blackman(SEXP npoints, SEXP sym, SEXP dtype);
SEXP HWindow_blackmanharris(SEXP npoints, SEXP sym, SEXP dtype);
SEXP HWindow_bohman(SEXP npoints, SEXP sym, SEXP dtype);
SEXP HWindow_boxcar(SEXP npoints, SEXP dtype);
SEXP HWindow_cosine(SEXP npoints, SEXP sym, SEXP dtype);
SEXP HWindow_hann(SEXP npoints, SEXP sym, SEXP dtype);