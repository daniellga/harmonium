library(bench)
library(torch)
library(ggplot2)
devtools::load_all(".", export_all = FALSE)

# fft_matrix with complexes
results <- bench::press(
  n = seq(30, 400000, 30000),
  {
    r = as.double(sample(100, n, replace = TRUE))
    i = as.double(sample(100, n, replace = TRUE))
    x = complex(real=r, imaginary=i)
    x = matrix(x, ncol = 10)
    fft_planner = HFftPlanner$new(HDataType$Complex64)
    mark(
      torch = as_array(torch::torch_fft_fft(torch_tensor(x, dtype = torch_cfloat64()), dim = 1)),
      harmonium = HFft$fft(HArray$new_from_values(x, HDataType$Complex64))$collect(),
      harmonium_mut = {
        harray = HArray$new_from_values(x, HDataType$Complex64)
        HFft$fft_mut(harray)
        harray$collect()
      },
      new_fft = {
        harray = HArray$new_from_values(x, HDataType$Complex64)
        fft_planner$fft(harray)
        harray$collect()
      },
      base_r = stats::mvfft(x),
      iterations = 50,
      check = FALSE
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))

# fft_real_matrix with floats
results <- bench::press(
  n = seq(30, 400000, 30000),
  {
    x = as.double(sample(100, n, replace = TRUE))
    x = matrix(x, ncol = 10)
    real_fft_planner = HRealFftPlanner$new(HDataType$Float64)
    mark(
      torch = as_array(torch::torch_fft_rfft(torch_tensor(x, dtype = torch_float64()), dim = 1)),
      harmonium = {
        harray = HArray$new_from_values(x, HDataType$Float64)
        HFft$rfft_mut(harray)
        harray$collect()
      },
      new_fft = {
        harray = HArray$new_from_values(x, HDataType$Float64)
        real_fft_planner$rfft(harray)
        harray$collect()
      },
      iterations = 50,
      check = TRUE
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))
