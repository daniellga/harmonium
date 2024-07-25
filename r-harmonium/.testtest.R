#pkgbuild::clean_dll()
devtools::load_all(".", export_all = FALSE)
devtools::test()
#devtools::check(document = FALSE, cran = FALSE, args = c("--no-manual", "--no-build-vignettes", "--no-codoc", "--no-examples", "--no-tests"))

library(torch)
v <- c(
  complex(real = 1, imaginary = 2),
  complex(real = 3, imaginary = 4),
  complex(real = 5, imaginary = 6),
  complex(real = 7, imaginary = 8),
  complex(real = 9, imaginary = 10),
  complex(real = 11, imaginary = 12)
)

complex_tensor = torch_tensor(v)
a = torch_stft(
    input = complex_tensor,
    n_fft = 5,
    hop_length = 2,
    win_length = 3,
    window = torch_tensor(c(1,2,3)),
    center = FALSE,
    onesided = FALSE,
    return_complex = TRUE
)
t(as_array(a))