test_that(
  "harray works",
  {
    check_harray = function(arr, dtype, dtype_result) {
      harray = HArray$new_from_values(arr, dtype)

      expect_true(harray$dtype() == dtype_result)
      expect_equal(harray$len(), length(arr))
      expect_equal(harray$shape(), c(ncol(arr), nrow(arr)))
      expect_equal(harray$ndim(), length(dim(arr)))
      expect_true(harray$eq(harray))
      expect_false(harray$ne(harray))
      expect_false(harray$is_shared())
      expect_equal(harray$collect(), arr)
    }

    arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3, 4))
    check_harray(arr, HDataType$float32, HDataType$float32)
    check_harray(arr, HDataType$float64, HDataType$float64)

    arr = array(c(1+2i,3+4i,5-6i,7+8i,9-10i,10+11i,11-12i,12+13i), c(2, 4))
    check_harray(arr, HDataType$complex32, HDataType$complex32)
    check_harray(arr, HDataType$complex64, HDataType$complex64)
  }
)
