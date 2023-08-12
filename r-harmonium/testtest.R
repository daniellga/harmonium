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

