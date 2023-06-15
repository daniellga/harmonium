test_that(
  "Resample works.",
  {
    # SincFixedIn. Test example from rubato repo.
    data = matrix(0, nrow = 1024, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
    hparams = HSincInterpolationParams$new(256, 0.95, 256, "linear", "blackmanharris2")

    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 1024, 2, HResamplerType$sinc_fixed_in, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))

    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 1024, 2, HResamplerType$sinc_fixed_in, HDataType$float64)
    expect_true(res$resampler_type() == HResamplerType$sinc_fixed_in)
    expect_true(res$dtype() == HDataType$float64)
    res$process(haudio, sr_out = 48000)

    expect_equal(haudio$len(), 1948)
    expect_equal(haudio$sr(), 48000)
    
    # SincFixedOut.
    data = matrix(0, nrow = 1024, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
    hparams = HSincInterpolationParams$new(256, 0.95, 256, "linear", "blackmanharris2")
    
    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512, 2, HResamplerType$sinc_fixed_out, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_sinc(48000 / 44100, 2, hparams, 512, 2, HResamplerType$sinc_fixed_out, HDataType$float64)
    expect_true(res$resampler_type() == HResamplerType$sinc_fixed_out)
    expect_true(res$dtype() == HDataType$float64)
    res$process(haudio, sr_out = 48000)
    
    expect_equal(haudio$len(), 1024)
    expect_equal(haudio$sr(), 48000)
    
    # FftFixedIn
    data = matrix(0, nrow = 1024, ncol = 2)
    haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
    res = HResampler$new_fft(44100L, 48000L, 1024L, 2L, 2L, HResamplerType$fft_fixed_in, HDataType$float32)
    expect_error(res$process(haudio, sr_out = 48000))
    
    res = HResampler$new_fft(44100L, 48000L, 1024L, 2L, 2L, HResamplerType$fft_fixed_in, HDataType$float64)
    expect_true(res$resampler_type() == HResamplerType$fft_fixed_in)
    expect_true(res$dtype() == HDataType$float64)
    res$process(haudio, sr_out = 48000)
    
    expect_equal(haudio$len(), 1280)
    expect_equal(haudio$sr(), 48000)
  }
)
