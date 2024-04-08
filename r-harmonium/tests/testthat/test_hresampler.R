test_that(
  "Resample works.",
  {
    # SincFixedIn. Test example from rubato repo.
    arr = matrix(0, nrow = 512, ncol = 2)
    harray = HArray$new_from_values(arr, dtype = HDataType$Float64)
    hparams = HSincInterpolationParameters$new(256L, 0.95, 256L, "linear", "blackmanharris2")

    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512L, HAudioOp$nchannels(harray), HResamplerType$SincFixedIn, HDataType$Float32)
    expect_error(res$process(harray))

    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512L, HAudioOp$nchannels(harray), HResamplerType$SincFixedIn, HDataType$Float64)
    expect_true(res$res_type() == HResamplerType$SincFixedIn)
    expect_true(res$dtype() == HDataType$Float64)
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
    harray = HArray$new_from_values(arr, dtype = HDataType$Float64)
    hparams = HSincInterpolationParameters$new(256, 0.95, 256, "linear", "blackmanharris2")
    
    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512L, HAudioOp$nchannels(harray), HResamplerType$SincFixedOut, HDataType$Float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512L, HAudioOp$nchannels(harray), HResamplerType$SincFixedOut, HDataType$Float64)
    expect_true(res$res_type() == HResamplerType$SincFixedOut)
    expect_true(res$dtype() == HDataType$Float64)
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
    harray = HArray$new_from_values(arr, dtype = HDataType$Float64)
    
    res = HResampler$new_fft(44100L, 48000L, 1024L, 2L, 2L, HResamplerType$FftFixedIn, HDataType$Float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_fft(44100L, 48000L, 1024L, 2L, 2L, HResamplerType$FftFixedIn, HDataType$Float64)
    expect_true(res$res_type() == HResamplerType$FftFixedIn)
    expect_true(res$dtype() == HDataType$Float64)
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
    harray = HArray$new_from_values(arr, dtype = HDataType$Float64)
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$FftFixedInOut, HDataType$Float32)
    expect_error(res$process(harray))
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$FftFixedInOut, HDataType$Float64)
    expect_true(res$res_type() == HResamplerType$FftFixedInOut)
    expect_true(res$dtype() == HDataType$Float64)
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
    harray = HArray$new_from_values(arr, dtype = HDataType$Float64)
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$FftFixedOut, HDataType$Float32)
    expect_error(res$process(harray))
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$FftFixedOut, HDataType$Float64)
    expect_true(res$res_type() == HResamplerType$FftFixedOut)
    expect_true(res$dtype() == HDataType$Float64)
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
    harray = HArray$new_from_values(arr, dtype = HDataType$Float64)
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$Linear, 512L, HAudioOp$nchannels(harray), HResamplerType$FastFixedIn, HDataType$Float32)
    expect_error(res$process(harray))
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$Linear, 512L, HAudioOp$nchannels(harray), HResamplerType$FastFixedIn, HDataType$Float64)
    expect_true(res$res_type() == HResamplerType$FastFixedIn)
    expect_true(res$dtype() == HDataType$Float64)
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
    harray = HArray$new_from_values(arr, dtype = HDataType$Float64)
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$Linear, 512L, HAudioOp$nchannels(harray), HResamplerType$FastFixedOut, HDataType$Float32)
    expect_error(res$process(harray))
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$Linear, 512L, HAudioOp$nchannels(harray), HResamplerType$FastFixedOut, HDataType$Float64)
    expect_true(res$res_type() == HResamplerType$FastFixedOut)
    expect_true(res$dtype() == HDataType$Float64)
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
