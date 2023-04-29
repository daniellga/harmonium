# hmatrixdynamic test
check_hmatrix = function(values, dtype, dtype_result) {
  hmatrix = HMatrix$new_from_values(values, dtype)

  expect_equal(hmatrix$data_type(), dtype_result)
  expect_equal(hmatrix$len(), length(values))
  expect_equal(hmatrix$ncols(), ncol(values))
  expect_equal(hmatrix$nrows(), nrow(values))
  expect_equal(hmatrix$collect(), values)
  expect_true(is.character(hmatrix$mem_adress()))

  hmatrix_cloned = hmatrix$clone()
  expect_true(hmatrix==hmatrix_cloned)
  expect_false(hmatrix!=hmatrix_cloned)
  expect_true(hmatrix$eq_inner(hmatrix_cloned))
  expect_false(identical(hmatrix, hmatrix_cloned))

  hmatrix_new = HMatrix$new_from_values(values, dtype)
  expect_true(hmatrix==hmatrix_new)
  expect_false(hmatrix$eq_inner(hmatrix_new))
}

values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), 3, 4)
check_hmatrix(values, HDataType$float32(), "Float32")
check_hmatrix(values, HDataType$float64(), "Float64")

values = matrix(c(1+2i,3+4i,5-6i,7+8i,9-10i,10+11i,11-12i,12+13i), 4, 2)
check_hmatrix(values, HDataType$complex32(), "Complex32")
check_hmatrix(values, HDataType$complex64(), "Complex64")
