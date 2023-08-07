library(bench)
library(torch)
library(ggplot2)
rextendr::register_extendr()
devtools::load_all(".", export_all = FALSE)

# fft_matrix with complexes
results <- bench::press(
  n = seq(30, 400000, 30000),
  {
    r = as.double(sample(100, n, replace = TRUE))
    i = as.double(sample(100, n, replace = TRUE))
    x = complex(real=r, imaginary=i)
    x = matrix(x, ncol = 10)
    mark(
      torch = as_array(torch::torch_fft_fft(torch_tensor(x, dtype = torch_cfloat64()), dim = 1)),
      harmonium = HFft$fft(HArray$new_from_values(x, HDataType$complex64))$collect(),
      harmonium_mut = {
        harray = HArray$new_from_values(x, HDataType$complex64)
        HFft$fft_mut(harray)
        harray$collect()
        },
      base_r = stats::mvfft(x),
      iterations = 50,
      check = TRUE
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))

# fft_matrix with floats
results <- bench::press(
  n = seq(30, 400000, 30000),
  {
    x = as.double(sample(100, n, replace = TRUE))
    x = matrix(x, ncol = 10)
    mark(
      torch = as_array(torch::torch_fft_fft(torch_tensor(x, dtype = torch_float64()), dim = 1)),
      harmonium = HFft$fft(HArray$new_from_values(x, HDataType$float64))$collect(),
      base_r = stats::mvfft(x),
      iterations = 50,
      check = TRUE
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))
