#pkgbuild::clean_dll()
devtools::load_all(".", export_all = FALSE)
devtools::test()
devtools::check(document = FALSE, cran = FALSE, args = c("--no-manual", "--no-build-vignettes", "--no-codoc", "--no-examples", "--no-tests"))

library(torch)
v <- c(1,2,3,4,5,6)

complex_tensor = torch_tensor(v)
a = torch_stft(
    input = complex_tensor,
    n_fft = 5,
    hop_length = 2,
    win_length = 3,
    window = c(1.,2.,3.),
    center = FALSE,
    onesided = TRUE,
    return_complex = TRUE
)
t(as_array(a))
