test_that(
  "hfft works",
  {
    check_hfft_1d = function() {
      r = as.double(sample(100, 30, replace = TRUE))
      i = as.double(sample(100, 30, replace = TRUE))
      x = as.array(complex(real=r, imaginary=i))

      expect_equal(stats::fft(x), HFft$fft(HArray$new_from_values(x, HDataType$Complex64))$collect())
      expect_equal(stats::fft(x, inverse = TRUE), HFft$ifft(HArray$new_from_values(x, HDataType$Complex64))$collect())

      x = as.array(as.double(sample(100, 30, replace = TRUE)))
      expect_equal(stats::fft(x), HFft$fft(HArray$new_from_values(x, HDataType$Float64))$collect())
      expect_error(HFft$ifft(HArray$new_from_values(x, HDataType$Float64))$collect())
    }

    check_hfft_2d = function() {
      r = as.double(sample(100, 30, replace = TRUE))
      i = as.double(sample(100, 30, replace = TRUE))
      x = complex(real=r, imaginary=i)
      x = matrix(x, ncol = 10)

      expect_equal(stats::mvfft(x), HFft$fft(HArray$new_from_values(x, HDataType$Complex64))$collect())
      expect_equal(stats::mvfft(x, inverse = TRUE), HFft$ifft(HArray$new_from_values(x, HDataType$Complex64))$collect())

      x = as.double(sample(100, 30, replace = TRUE))
      x = matrix(x, ncol = 10)
      expect_equal(stats::mvfft(x), HFft$fft(HArray$new_from_values(x, HDataType$Float64))$collect())
      expect_error(HFft$ifft(HArray$new_from_values(x, HDataType$Float64))$collect())
    }

    check_hfft_1d()
    check_hfft_2d()
  }
)
