rextendr::register_extendr()
devtools::load_all(".", export_all = FALSE)
devtools::test()
devtools::check(document = FALSE, cran = FALSE, args = c("--no-manual", "--no-build-vignettes", "--no-codoc", "--no-examples", "--no-tests"))


# another test
haudiosink = HAudioSink$new()
haudio = HAudio$new_from_file("../testfiles/gs-16b-2c-44100hz.flac", dtype = HDataType$float64)
resampler_type = HResamplerType$fft_fixed_in
sr_in = haudio$sr()
sr_out = 22050L
dtype = HDataType$float64
resampler = HResampler$new_fft(sr_in = sr_in, sr_out = sr_out, nbr_channels = 2L, chunk_size = 1024L, sub_chunks = 2L , resampler_type = resampler_type, dtype = dtype)
resampler$process(haudio, sr_out = sr_out)




haudio = HAudio$new_from_values(matrix(0, nrow = 1024, ncol = 2), 44100, dtype = HDataType$float64)
hparams = HInterpolationParams$new(256, 0.95, 256, "linear", "blackmanharris2")
res = HResampler$new_sinc(48000 / 44100, 2, hparams, 1024, 2, HResamplerType$sinc_fixed_in, HDataType$float64)
res$resampler_type()
res$data_type()
res$process(haudio, sr_out = 48000)
haudio$len() == 1948
haudio$sr() == 48000






s = HResampler$new_fft(haudio$sr(), 22050, 1000, 2, haudio$nchannels(), HResamplerType$fft_fixed_in, HDataType$float32)


a = HFile$metadata_from_file("../testfiles/gs-16b-1c-44100hz.flac", HMetadataType$text)
a = HFile$metadata_from_file("../testfiles/gs-16b-1c-44100hz.wav", HMetadataType$text)

haudio = HAudio$new_from_file("../testfiles/gs-16b-1c-44100hz.flac", dtype = HDataType$float64)
resampler_type = resampler_type = HResamplerType$fft_fixed_in
sr_in = haudio$sr()
sr_out = 22050L
dtype = HDataType$float64
resampler = HResampler$new_fft(sr_in = sr_in, sr_out = sr_out, nbr_channels = 2L, chunk_size = 1024L, sub_chunks = 2L , resampler_type = resampler_type, dtype = dtype)
hresampler$process(haudio, sr_out = sr_out)




haudio = HAudio$new_from_file("../testfiles/gs-16b-1c-44100hz.flac", dtype = HDataType$float64)
b = HAudio$new_from_file("../testfiles/gs-16b-1c-44100hz.wav", dtype = HDataType$float64)

values = matrix(as.numeric(c(1:1000000)), ncol = 1000)
hmatrix = HMatrix$new_from_values(values, HDataType$float32())
hmatrix2 = HMatrix$new_from_values(values, HDataType$float32())
bench::mark(
  hmatrix$mean_cols(),
  hmatrix2$mean_cols2()
)

values = matrix(as.numeric(c(1:1000000)), ncol = 1000)
hmatrix = HMatrix$new_from_values(values, HDataType$float32())
hmatrix2 = HMatrix$new_from_values(values, HDataType$float32())
bench::mark(
  hmatrix2$mean_cols2(),
  hmatrix$mean_cols()
)





values = c(1+1i,2-2i,3+3i,4-5i,5+6i,6-7i,7+8i,8-9i,9+10i,10-11i,11+12i,12-13i,13+14i,14-15i,15-16i,16+17i,17-18i,18-19i)
values = c(1+1i,2-2i,3+3i,4-5i,5+6i,6-7i,7+8i,8-9i,9+10i,10-11i)
dtype = DataType$complex32()
harray = HArray$new_from_values(values, dtype)
harray

values = matrix(as.complex(c(1,2,3,4,5,6,7,8,9,10,11,12)), 11, 12)
dtype = DataType$complex32()
hmatrix = HMatrix$new_from_values(values, dtype)
hmatrix

values = as.double(c())


# test metadata
text_metadata_from_file("../harmonium/testfiles/gs-16b-2c-44100hz.flac")[1] == "[key: title, std_key: TrackTitle, value: Galway]"

# test get_params_from_file
all.equal(get_params_from_file("../harmonium/testfiles/gs-16b-2c-44100hz.wav"), c(44100, 698194, 2, 15.832063492))

# test verify
verify_file("../harmonium/testfiles/gs-16b-2c-44100hz.wav") == "NotSupported"

# test print
a = HFloat32ArrayR$new_from_values(c(1,NA,3,4,5))
a = HFloat32ArrayR$new_from_values(as.double(1:12))
a = HComplex32ArrayR$new_from_values(c(NA,3 + 4i,5-6i,7+8i,9-10i))
a = HComplex32ArrayR$new_from_values(complex(rnorm(10), rnorm(10)))

b = HFloat32ArrayR$new_from_values(c(as.double(1:12)))
b = b$as_hmatrix(3)
b = HFloat32ArrayR$new_from_values(c(as.double(1:48)))
b = b$as_hmatrix(6)
b = HFloat32ArrayR$new_from_values(12)
b = b$as_hmatrix(1)
b = HComplex32ArrayR$new_from_values(c(NA,3 + 4i,5-6i,7+8i,9-10i,10+11i,11-12i, 12+13i))
b = b$as_hmatrix(2)
b = HComplex32ArrayR$new_from_values(complex(rnorm(48), rnorm(48)))
b = b$as_hmatrix(8)
b = HComplex32ArrayR$new_from_values(c(12+13i))
b = b$as_hmatrix(1)

#other tests
values = c(1,2,3,4,5,6,7,8,9,10,11,12)
a = HFloat32ArrayR$new_from_values(values)
a
a == a
a != a
identical(a$clone(), a)
a$clone()==a
a$clone()$mem_adress() == a$mem_adress()
identical(a$copy(), a)
a$copy()==a
a$copy()$mem_adress() == a$mem_adress()
identical(a$collect(), values)

# test hmatrix
values = c(1,2,3,4,5,6,7,8,9,10,11,12)
b = HFloat64ArrayR$new_from_values(values)
b = b$as_hmatrix(3)
b
b==b
b!=b
identical(b$collect(), matrix(values, ncol = 3))
identical(b$copy(), b)
b$clone()==b
b$clone()$mem_adress() == b$mem_adress()
identical(b$copy(), b)
b$copy()==b
b$copy()$mem_adress() == b$mem_adress()

# test hmatrix
values = matrix(c(1+2i,3+4i,5+6i,7+8i,9+10i,11+12i), ncol = 2)
a = HComplex64MatrixR$new_from_values(values)
b = a$as_harray()
class(b) == "HComplex64ArrayR"
a$mem_adress() == b$mem_adress()

# test haudio
a = HFloat64AudioR$new_from_file("../harmonium/testfiles/gs-16b-2c-44100hz.wav")
class(a)
a$sr()
a$nchannels()
a$nframes()
a$duration()
a==a
a!=a
a
b = a$as_hmatrix()
class(b) == "HFloat64MatrixR"
a$mem_adress() == b$mem_adress()
a$mem_adress() == a$clone()$mem_adress()
a$mem_adress() == a$copy()$mem_adress()
a==a$clone()
a==a$copy()

# test fft
values = c(1+2i,2+3i,3+4i,4+5i,5+6i,6+7i,7+8i,8+9i,9+10i,10+11i,11+12i,12+13i)
a = HComplex64ArrayR$new_from_values(values)
b = a$as_hmatrix(3)
identical(b$collect(), matrix(values, ncol = 3))
b
b == b
identical(values, b$collect())
b$fft()

# test fft
values = c(1,2,3,4,5,6,7,8,9,10,11,12)
a = HFloat64ArrayR$new_from_values(values)
b = a$as_hmatrix(3)
identical(b$collect(), matrix(values, ncol = 3))
b
b == b
identical(values, b$collect())
b$fft()

