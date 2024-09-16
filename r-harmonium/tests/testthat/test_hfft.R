test_that(
  "hfft works",
  {
    check_fft_1d = function() {
      r = as.double(sample(100, 30, replace = TRUE))
      i = as.double(sample(100, 30, replace = TRUE))
      x = as.array(complex(real=r, imaginary=i))

      dtype = HDataType$Complex64
      harray = HArray$new_from_values(x, dtype)
      hfft = HFft$new_forward(30L, dtype)
      hfft$process(harray)
      expect_equal(stats::fft(x), harray$collect(), tolerance = 1.0e-06)
    }

    check_ifft_1d = function() {
      r = as.double(sample(100, 30, replace = TRUE))
      i = as.double(sample(100, 30, replace = TRUE))
      x = as.array(complex(real=r, imaginary=i))

      dtype = HDataType$Complex64
      harray = HArray$new_from_values(x, dtype)
      hfft = HFft$new_inverse(30L, dtype)
      hfft$process(harray)
      expect_equal(stats::fft(x, inverse = TRUE), harray$collect(), tolerance = 1.0e-06)
    }

    check_fft_2d = function() {
      r = as.double(sample(100, 30, replace = TRUE))
      i = as.double(sample(100, 30, replace = TRUE))
      x = complex(real=r, imaginary=i)
      x = matrix(x, ncol = 10)

      dtype = HDataType$Complex64
      harray = HArray$new_from_values(x, dtype)
      hfft = HFft$new_forward(as.integer(30/10), dtype)
      hfft$process(harray)
      expect_equal(stats::mvfft(x), harray$collect(), tolerance = 1.0e-06)
    }

    check_ifft_2d = function() {
      r = as.double(sample(100, 30, replace = TRUE))
      i = as.double(sample(100, 30, replace = TRUE))
      x = complex(real=r, imaginary=i)
      x = matrix(x, ncol = 10)

      dtype = HDataType$Complex64
      harray = HArray$new_from_values(x, dtype)
      hfft = HFft$new_inverse(as.integer(30/10), dtype)
      hfft$process(harray)
      expect_equal(stats::mvfft(x, inverse = TRUE), harray$collect(), tolerance = 1.0e-06)
    }

    check_rfft_1d = function() {
      x = as.array(as.double(1:6))
      dtype = HDataType$Float64
      harray = HArray$new_from_values(x, dtype)
      hfft = HFft$new_real_forward(as.integer(6), dtype)
      hfft$process(harray)
      result = as.array(c(21+0i, -3+5.196152i, -3+1.732051i, -3+0i))
      expect_equal(harray$collect(), result, tolerance = 1.0e-06)
    }

    check_rfft_2d = function() {
      x = as.double(1:12)
      x = matrix(x, ncol = 3)
      dtype = HDataType$Float64
      harray = HArray$new_from_values(x, dtype)
      hfft = HFft$new_real_forward(as.integer(4), dtype)
      hfft$process(harray)
      result = matrix(c(10+0i, -2+2i, -2+0i, 26+0i, -2+2i, -2+0i, 42+0i, -2+2i, -2+0i), ncol = 3)
      expect_equal(harray$collect(), result, tolerance = 1.0e-06)
    }

    check_fft_1d()
    check_fft_2d()
    check_ifft_1d()
    check_ifft_2d()
    check_rfft_1d()
    check_rfft_2d()
  }
)
