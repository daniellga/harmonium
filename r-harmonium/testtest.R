rextendr::register_extendr()
devtools::load_all(".", export_all = FALSE)
devtools::test()
devtools::check(document = FALSE, cran = FALSE, args = c("--no-manual", "--no-build-vignettes", "--no-codoc", "--no-examples", "--no-tests"))

arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3, 4))
a = HArray$new_from_values(arr, HDataType$float32)
HAudioOp$nchannels(a)
HAudioOp$nframes(a)
HAudioOp$db_to_power(a, 3)
HAudioOp$to_mono(a)

arr = array(as.numeric(0:1024), c(512, 2))
a = HArray$new_from_values(arr, HDataType$float32)
sr_in = 44100L
sr_out = 48000L
resample_ratio = sr_out / sr_in
max_resample_ratio_relative = 2
hparams = HSincInterpolationParams$new(256, 0.95, 256, "linear", "blackmanharris2")
chunk_size = 512L
nchannels = 2L
res_type = HResamplerType$sinc_fixed_in
dtype = HDataType$float32
res = HResampler$new_sinc(resample_ratio, max_resample_ratio_relative, hparams, chunk_size, nchannels, res_type, dtype)
res$process(a)

l = HFile$decode("testfiles/gs-16b-2c-44100hz.flac", HDataType$float32)


