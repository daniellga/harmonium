library(bench)
library(torch)
library(ggplot2)
devtools::load_all(".", export_all = FALSE)

# fft
results <- bench::press(
  n = seq.int(30, 400000, 30000L),
  {
    r = as.double(sample(100, n, replace = TRUE))
    i = as.double(sample(100, n, replace = TRUE))
    ncol = 10
    x = complex(real=r, imaginary=i)
    x = matrix(x, ncol = ncol)
    hfft = HFft$new_forward(as.integer(n/ncol), HDataType$Complex64)
    mark(
      torch = as_array(torch::torch_fft_fft(torch_tensor(x, dtype = torch_cfloat64()), dim = 1)),
      harmonium_fft = {
        harray = HArray$new_from_values(x, HDataType$Complex64)
        hfft$process(harray)
        harray$collect()
      },
      base_r = stats::mvfft(x),
      iterations = 50,
      check = FALSE
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))

# real fft
results <- bench::press(
  n = seq(30, 400000, 30000),
  {
    x = as.double(sample(100, n, replace = TRUE))
    x = matrix(x, ncol = ncol)
    ncol = 10
    hfft = HFft$new_real_forward(as.integer(n/ncol), HDataType$Float64)
    mark(
      torch = as_array(torch::torch_fft_rfft(torch_tensor(x, dtype = torch_float64()), dim = 1)),
      harmonium_fft = {
        harray = HArray$new_from_values(x, HDataType$Float64)
        hfft$process(harray)
        harray$collect()
      },
      iterations = 50,
      check = TRUE
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))

# stft 1d with complexes
results <- bench::press(
  n = seq.int(3000, 150000, 5000),
  {
    r = as.double(sample(100, n, replace = TRUE))
    i = as.double(sample(100, n, replace = TRUE))
    window = as.numeric(1:80)
    x = as.array(complex(real=r, imaginary=i))
    n_fft = 200L
    hop_length = 100L
    win_length = 80L
    hstft = HStft$new_forward(n_fft, HDataType$Complex64)
    mark(
      torch = as_array(torch::torch_stft(
        input = torch_tensor(x, dtype = torch_cfloat64()),
        n_fft = n_fft,
        hop_length = hop_length,
        win_length = win_length,
        window = window,
        center = FALSE,
        return_complex = TRUE
        )),
      harmonium_stft = {
        harray = HArray$new_from_values(x, HDataType$Complex64)
        harray_window = HArray$new_from_values(as.array(window), HDataType$Float64)
        hstft$process(harray, hop_length, win_length, harray_window)
        harray$collect()
      },
      iterations = 50,
      check = function(a, b) { all.equal(a, b, tolerance = 1.0e-06) }
    )
  }
)
ggplot(results) + geom_point(aes(x = n, y = median, color = as.character(expression)))
