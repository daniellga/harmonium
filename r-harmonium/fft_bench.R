library(bench)
library(torch)
library(ggplot2)
rextendr::document()

# fft_arrays with complexes
results <- bench::press(
  n = seq(30, 400000, 30000),
  {
    r = as.double(sample(100, n, replace = TRUE))
    i = as.double(sample(100, n, replace = TRUE))
    x = complex(real=r, imaginary=i)
    mark(
      stats::fft(x),
      as_array(torch::torch_fft_fft(torch_tensor(x, dtype = torch_cfloat64()))),
      HComplex64ArrayR$new_from_values(x)$fft()$collect(),
      HArray$new_from_values(x, dtype = DataType$complex64())$fft()$collect(),
      iterations = 50,
      check = FALSE
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))

# fft_arrays with doubles
results <- bench::press(
  n = seq(30, 400000, 30000),
  {
    x = as.double(sample(100, n, replace = TRUE))
    mark(
      stats::fft(x),
      as_array(torch::torch_fft_fft(torch_tensor(x, dtype = torch_float64()))),
      HFloat64ArrayR$new_from_values(x)$fft()$collect(),
      HArray$new_from_values(x, dtype = DataType$float64())$fft()$collect(),
      iterations = 50,
      check = FALSE
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))

# fft_matrix with complexes
results <- bench::press(
  n = seq(30, 400000, 30000),
  {
    r = as.double(sample(100, n, replace = TRUE))
    i = as.double(sample(100, n, replace = TRUE))
    x = complex(real=r, imaginary=i)
    x = matrix(x, ncol = 10)
    mark(
      as_array(torch::torch_fft_fft(torch_tensor(x, dtype = torch_cfloat64()), dim = 1)),
      HComplex64MatrixR$new_from_values(x)$fft()$collect(),
      HMatrix$new_from_values(x, dtype = DataType$complex64())$fft()$collect(),
      stats::mvfft(x),
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
      as_array(torch::torch_fft_fft(torch_tensor(x, dtype = torch_float64()), dim = 1)),
      HFloat64MatrixR$new_from_values(x)$fft()$collect(),
      HMatrix$new_from_values(x, dtype = DataType$float64())$fft()$collect(),
      stats::mvfft(x),
      iterations = 50,
      check = TRUE
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))
