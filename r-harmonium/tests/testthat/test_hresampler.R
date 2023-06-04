# test_that(
#   "Resample works.",
#   {
#     # test example from rubato repo.
#     data = matrix(0, nrow = 1024, ncol = 2)
#     haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64)
#     hparams = HInterpolationParams$new(256, 0.95, 256, "linear", "blackmanharris2")
# 
#     res = HResampler$new_sinc(48000 / 44100, 2, hparams, 1024, 2, HResamplerType$sinc_fixed_in, HDataType$float32)
#     expect_error(res$process(haudio, sr_out = 48000))
# 
#     res = HResampler$new_sinc(48000 / 44100, 2, hparams, 1024, 2, HResamplerType$sinc_fixed_in, HDataType$float64)
#     expect_true(res$resampler_type() == HResamplerType$sinc_fixed_in)
#     expect_true(res$dtype() == HDataType$float64)
#     res$process(haudio, sr_out = 48000)
# 
#     expect_equal(haudio$len(), 1948)
#     expect_equal(haudio$sr(), 48000)
#   }
# )
