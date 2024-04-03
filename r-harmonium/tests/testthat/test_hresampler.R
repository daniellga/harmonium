test_that(
  "Resample works.",
  {
    # SincFixedIn. Test example from rubato repo.
    arr = matrix(0, nrow = 512, ncol = 2)
    harray = HArray$new_from_values(arr, dtype = HDataType$float64)
    hparams = HSincInterpolationParams$new(256, 0.95, 256, "linear", "blackmanharris2")

    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512L, HAudioOp$nchannels(harray), HResamplerType$sinc_fixed_in, HDataType$float32)
    expect_error(res$process(harray))

    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512L, HAudioOp$nchannels(harray), HResamplerType$sinc_fixed_in, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$sinc_fixed_in)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(harray))
    
    expect_no_error(res$set_resample_ratio(1, FALSE))
    expect_no_error(res$set_resample_ratio(2, FALSE))
    expect_no_error(res$set_resample_ratio(1, TRUE))
    expect_no_error(res$set_resample_ratio(2, TRUE))
    expect_no_error(res$set_resample_ratio_relative(0.5, FALSE))
    expect_no_error(res$set_resample_ratio_relative(2, FALSE))
    expect_no_error(res$set_resample_ratio_relative(0.5, TRUE))
    expect_no_error(res$set_resample_ratio_relative(2, TRUE))
    
    # SincFixedOut.
    arr = matrix(0, nrow = 1024, ncol = 2)
    harray = HArray$new_from_values(arr, dtype = HDataType$float64)
    hparams = HSincInterpolationParams$new(256, 0.95, 256, "linear", "blackmanharris2")
    
    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512L, HAudioOp$nchannels(harray), HResamplerType$sinc_fixed_out, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512L, HAudioOp$nchannels(harray), HResamplerType$sinc_fixed_out, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$sinc_fixed_out)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(harray))
    
    expect_no_error(res$set_resample_ratio(1, FALSE))
    expect_no_error(res$set_resample_ratio(2, FALSE))
    expect_no_error(res$set_resample_ratio(1, TRUE))
    expect_no_error(res$set_resample_ratio(2, TRUE))
    expect_no_error(res$set_resample_ratio_relative(0.5, FALSE))
    expect_no_error(res$set_resample_ratio_relative(2, FALSE))
    expect_no_error(res$set_resample_ratio_relative(0.5, TRUE))
    expect_no_error(res$set_resample_ratio_relative(2, TRUE))
    
    # FftFixedIn
    arr = matrix(0, nrow = 1024, ncol = 2)
    harray = HArray$new_from_values(arr, dtype = HDataType$float64)
    
    res = HResampler$new_fft(44100L, 48000L, 1024L, 2L, 2L, HResamplerType$fft_fixed_in, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_fft(44100L, 48000L, 1024L, 2L, 2L, HResamplerType$fft_fixed_in, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fft_fixed_in)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(harray))
    
    expect_error(res$set_resample_ratio(1, FALSE))
    expect_error(res$set_resample_ratio(2, FALSE))
    expect_error(res$set_resample_ratio(1, TRUE))
    expect_error(res$set_resample_ratio(2, TRUE))
    expect_error(res$set_resample_ratio_relative(0.5, FALSE))
    expect_error(res$set_resample_ratio_relative(2, FALSE))
    expect_error(res$set_resample_ratio_relative(0.5, TRUE))
    expect_error(res$set_resample_ratio_relative(2, TRUE))
    
    # FftFixedInOut
    arr = matrix(0, nrow = 1024, ncol = 2)
    harray = HArray$new_from_values(arr, dtype = HDataType$float64)
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$fft_fixed_in_out, HDataType$float32)
    expect_error(res$process(harray))
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$fft_fixed_in_out, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fft_fixed_in_out)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(harray))
    
    expect_error(res$set_resample_ratio(1, FALSE))
    expect_error(res$set_resample_ratio(2, FALSE))
    expect_error(res$set_resample_ratio(1, TRUE))
    expect_error(res$set_resample_ratio(2, TRUE))
    expect_error(res$set_resample_ratio_relative(0.5, FALSE))
    expect_error(res$set_resample_ratio_relative(2, FALSE))
    expect_error(res$set_resample_ratio_relative(0.5, TRUE))
    expect_error(res$set_resample_ratio_relative(2, TRUE))
    
    # FftFixedOut
    arr = matrix(0, nrow = 1024, ncol = 2)
    harray = HArray$new_from_values(arr, dtype = HDataType$float64)
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$fft_fixed_out, HDataType$float32)
    expect_error(res$process(harray))
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$fft_fixed_out, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fft_fixed_out)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(harray))
    
    expect_error(res$set_resample_ratio(1, FALSE))
    expect_error(res$set_resample_ratio(2, FALSE))
    expect_error(res$set_resample_ratio(1, TRUE))
    expect_error(res$set_resample_ratio(2, TRUE))
    expect_error(res$set_resample_ratio_relative(0.5, FALSE))
    expect_error(res$set_resample_ratio_relative(2, FALSE))
    expect_error(res$set_resample_ratio_relative(0.5, TRUE))
    expect_error(res$set_resample_ratio_relative(2, TRUE))
    
    # FastFixedIn. Test example from rubato repo.
    arr = matrix(0, nrow = 1024, ncol = 2)
    harray = HArray$new_from_values(arr, dtype = HDataType$float64)
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$linear, 512L, HAudioOp$nchannels(harray), HResamplerType$fast_fixed_in, HDataType$float32)
    expect_error(res$process(harray))
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$linear, 512L, HAudioOp$nchannels(harray), HResamplerType$fast_fixed_in, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fast_fixed_in)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(harray))
    
    expect_no_error(res$set_resample_ratio(1, FALSE))
    expect_no_error(res$set_resample_ratio(2, FALSE))
    expect_no_error(res$set_resample_ratio(1, TRUE))
    expect_no_error(res$set_resample_ratio(2, TRUE))
    expect_no_error(res$set_resample_ratio_relative(0.5, FALSE))
    expect_no_error(res$set_resample_ratio_relative(2, FALSE))
    expect_no_error(res$set_resample_ratio_relative(0.5, TRUE))
    expect_no_error(res$set_resample_ratio_relative(2, TRUE))
    
    # FastFixedOut. Test example from rubato repo.
    arr = matrix(0, nrow = 512, ncol = 2)
    harray = HArray$new_from_values(arr, dtype = HDataType$float64)
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$linear, 512L, HAudioOp$nchannels(harray), HResamplerType$fast_fixed_out, HDataType$float32)
    expect_error(res$process(harray))
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$linear, 512L, HAudioOp$nchannels(harray), HResamplerType$fast_fixed_out, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fast_fixed_out)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(harray))
    
    expect_no_error(res$set_resample_ratio(1, FALSE))
    expect_no_error(res$set_resample_ratio(2, FALSE))
    expect_no_error(res$set_resample_ratio(1, TRUE))
    expect_no_error(res$set_resample_ratio(2, TRUE))
    expect_no_error(res$set_resample_ratio_relative(0.5, FALSE))
    expect_no_error(res$set_resample_ratio_relative(2, FALSE))
    expect_no_error(res$set_resample_ratio_relative(0.5, TRUE))
    expect_no_error(res$set_resample_ratio_relative(2, TRUE))
  }
)
