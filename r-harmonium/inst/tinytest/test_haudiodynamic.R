# haudiodynamic test
check_haudio = function(values, dtype, dtype_result) {
  haudio = HAudio$new_from_values(values, 22000, dtype)
  expect_equal(haudio$data_type(), dtype_result)
  expect_equal(haudio$len(), length(values))
  expect_equal(haudio$nchannels(), ncol(values))
  expect_equal(haudio$nframes(), nrow(values))
  expect_equal(haudio$collect(), values)
  expect_true(is.character(haudio$mem_adress()))

  haudio_cloned = haudio$clone()
  expect_true(haudio==haudio_cloned)
  expect_false(haudio!=haudio_cloned)
  expect_true(haudio$eq_inner(haudio_cloned))
  expect_false(identical(haudio, haudio_cloned))

  haudio_new = HAudio$new_from_values(values, 22000, dtype)
  expect_true(haudio==haudio_new)
  expect_false(haudio$eq_inner(haudio_new))
}

values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), 3, 4)
check_haudio(values, HDataType$float32(), "Float32")
check_haudio(values, HDataType$float64(), "Float64")

values = matrix(c(NA,3+4i,5-6i,7+8i,9-10i,10+11i,11-12i,12+13i), 4, 2)
expect_error(HAudio$new_from_values(values, sr = 22000, dtype = HDataType$complex32()))
