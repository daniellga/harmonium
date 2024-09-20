test_that(
  "stft works",
  {
    check_stft_1d = function() {
      v = c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i)
      dtype = HDataType$Complex32
      dtype_window = HDataType$Float32
      harray = HArray$new_from_values(as.array(v), dtype)
      hstft = HStft$new_forward(5L, dtype)
      harray_window = HArray$new_from_values(as.array(c(1,2,3)), dtype_window)
      hstft$process(harray, hop_length = 2L, window_length = 3L, window = harray_window)
      complex_values = c(
        20 + 20i,
        -15.56887 - 12.31967i,
        10.82618 - 2.93764i,
        -2.93764 + 10.82618i,
        -12.31967 - 15.56887i
      )

      result = matrix(complex_values, nrow = 5, ncol = 1)
      expect_equal(harray$collect(), result, tolerance = 1.0e-06)
    }

    check_stft_2d = function() {
      v = c(1+1i,2+2i,3+3i,4+4i,5+5i,6+6i)
      v = cbind(v, v)
      dtype = HDataType$Complex32
      dtype_window = HDataType$Float32
      harray = HArray$new_from_values(as.array(v), dtype)
      hstft = HStft$new_forward(5L, dtype)
      harray_window = HArray$new_from_values(as.array(c(1,2,3)), dtype_window)
      hstft$process(harray, hop_length = 2L, window_length = 3L, window = harray_window)
      complex_values = c(
        20 + 20i,
        -15.56887 - 12.31967i,
        10.82618 - 2.93764i,
        -2.93764 + 10.82618i,
        -12.31967 - 15.56887i
      )
      m = matrix(complex_values, nrow = 5, ncol = 1)
      result = array(c(m, m), dim = c(5, 1, 2))

      expect_equal(harray$collect(), result, tolerance = 1.0e-06)
    }

    check_rstft_1d = function() {
      v = c(1,2,3,4,5,6)
      dtype = HDataType$Float32
      dtype_window = HDataType$Float32
      harray = HArray$new_from_values(as.array(v), dtype)
      hstft = HStft$new_real_forward(5L, dtype)
      harray_window = HArray$new_from_values(as.array(c(1,2,3)), dtype_window)
      hstft$process(harray, hop_length = 2L, window_length = 3L, window = harray_window)
      complex_values = c(
        20 + 0i,
        -13.944272 + 1.6245984i,
        3.944272 - 6.88191i
      )

      result = matrix(complex_values, nrow = 3, ncol = 1)
      expect_equal(harray$collect(), result, tolerance = 1.0e-06)
    }

    check_stft_1d()
    check_stft_2d()
    check_rstft_1d()
  }
)
