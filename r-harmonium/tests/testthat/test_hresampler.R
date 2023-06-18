test_that(
  "Resample works.",
  {
    # SincFixedIn. Test example from rubato repo.
    data = matrix(0, nrow = 512, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
    hparams = HSincInterpolationParams$new(256, 0.95, 256, "linear", "blackmanharris2")

    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512, 2, HResamplerType$sinc_fixed_in, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))

    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512, 2, HResamplerType$sinc_fixed_in, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$sinc_fixed_in)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(haudio, sr_out = 48000))

    expect_equal(haudio$sr(), 48000)
    
    # SincFixedOut.
    data = matrix(0, nrow = 1024, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
    hparams = HSincInterpolationParams$new(256, 0.95, 256, "linear", "blackmanharris2")
    
    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512, 2, HResamplerType$sinc_fixed_out, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512, 2, HResamplerType$sinc_fixed_out, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$sinc_fixed_out)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(haudio, sr_out = 48000))
    
    expect_equal(haudio$sr(), 48000)
    
    # FftFixedIn
    data = matrix(0, nrow = 1024, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
    res = HResampler$new_fft(44100L, 48000L, 1024L, 2L, 2L, HResamplerType$fft_fixed_in, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_fft(44100L, 48000L, 1024L, 2L, 2L, HResamplerType$fft_fixed_in, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fft_fixed_in)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(haudio, sr_out = 48000))
    
    expect_equal(haudio$sr(), 48000)
    
    # FftFixedInOut
    data = matrix(0, nrow = 1024, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$fft_fixed_in_out, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$fft_fixed_in_out, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fft_fixed_in_out)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(haudio, sr_out = 48000))
    
    expect_equal(haudio$sr(), 48000)
    
    # FftFixedOut
    data = matrix(0, nrow = 1024, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$fft_fixed_out, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_fft(44100L, 48000L, 512L, 2L, 2L, HResamplerType$fft_fixed_out, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fft_fixed_out)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(haudio, sr_out = 48000))
    
    expect_equal(haudio$sr(), 48000)
    
    # FastFixedIn. Test example from rubato repo.
    data = matrix(0, nrow = 512, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$linear, 512, 2, HResamplerType$fast_fixed_in, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$linear, 512, 2, HResamplerType$fast_fixed_in, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fast_fixed_in)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(haudio, sr_out = 48000))
    
    # FastFixedOut. Test example from rubato repo.
    data = matrix(0, nrow = 512, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)

    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$linear, 512, 2, HResamplerType$fast_fixed_out, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_fast(48000 / 44100, 2, HPolynomialDegree$linear, 512, 2, HResamplerType$fast_fixed_out, HDataType$float64)
    expect_true(res$res_type() == HResamplerType$fast_fixed_out)
    expect_true(res$dtype() == HDataType$float64)
    expect_no_error(res$process(haudio, sr_out = 48000))
    
  }
)
