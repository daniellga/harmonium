test_that(
  "harray works",
  {
    check_harray = function(values, dtype, dtype_result) {
      harray = HArray$new_from_values(values, dtype)

      expect_true(harray$dtype() == dtype_result)
      expect_equal(harray$len(), length(values))
      expect_equal(harray$collect(), values)
      expect_true(is.character(harray$mem_adress()))

      harray_cloned = harray$clone()
      expect_true(harray==harray_cloned)
      expect_false(harray!=harray_cloned)
      expect_true(harray$mem_adress() == harray_cloned$mem_adress())
      expect_false(identical(harray, harray_cloned))

      harray_new = HArray$new_from_values(values, dtype)
      expect_true(harray == harray_new)
      expect_false(harray$mem_adress() == harray_new$mem_adress())
    }

    values = c(1,2,3,4,5,6,7,8,9,10,11,12)
    check_harray(values, HDataType$float32, HDataType$float32)
    check_harray(values, HDataType$float64, HDataType$float64)

    values = c(1+2i,3+4i,5-6i,7+8i,9-10i,10+11i,11-12i,12+13i)
    check_harray(values, HDataType$complex32, HDataType$complex32)
    check_harray(values, HDataType$complex64, HDataType$complex64)
  }
)
