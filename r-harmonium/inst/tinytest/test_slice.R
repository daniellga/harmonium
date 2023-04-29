# slice test
check_harray = function(values, dtype, offset, length, result) {
  harray = HArray$new_from_values(values, dtype)
  harray$slice(offset, length)
  expect_equal(harray$collect(), result)
}

# slice test
check_harray_shared = function(values, dtype, offset, length, result) {
  harray = HArray$new_from_values(values, dtype)
  expect_false(harray$is_shared())
  harray2 = harray$clone()
  expect_true(harray$is_shared())
  harray$slice(offset, length)
  expect_false(harray$is_shared())
  expect_equal(harray$collect(), result)
}

values = c(1,2,3,4,5,6,7,8,9,10,11,12)
result = c(4,5)
offset = 3
length = 2
check_harray(values, HDataType$float32(), offset, length, result)
check_harray(values, HDataType$float64(), offset, length, result)
check_harray_shared(values, HDataType$float32(), offset, length, result)
check_harray_shared(values, HDataType$float64(), offset, length, result)

values = c(1+2i,3+4i,5-6i,7+8i,9-10i,10+11i,11-12i,12+13i)
result = c(7+8i,9-10i)
offset = 3
length = 2
check_harray(values, HDataType$complex32(), offset, length, result)
check_harray(values, HDataType$complex64(), offset, length, result)
check_harray_shared(values, HDataType$complex32(), offset, length, result)
check_harray_shared(values, HDataType$complex64(), offset, length, result)

check_hmatrix = function(values, dtype, ncols, offset, length, result) {
  hmatrix = HArray$new_from_values(values, dtype)$as_hmatrix(ncols)
  hmatrix$slice(offset, length)
  expect_equal(hmatrix$collect(), result)
}

check_hmatrix_shared = function(values, dtype, ncols, offset, length, result) {
  hmatrix = HArray$new_from_values(values, dtype)$as_hmatrix(ncols)
  expect_false(hmatrix$is_shared())
  hmatrix2 = hmatrix$clone()
  expect_true(hmatrix$is_shared())
  hmatrix$slice(offset, length)
  expect_false(hmatrix$is_shared())
  expect_equal(hmatrix$collect(), result)
}

values = c(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18)
ncols = 3
result = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 2)
offset = 0
length = 2
check_hmatrix(values, HDataType$float32(), ncols, offset, length, result)
check_hmatrix(values, HDataType$float64(), ncols, offset, length, result)
check_hmatrix_shared(values, HDataType$float32(), ncols, offset, length, result)
check_hmatrix_shared(values, HDataType$float64(), ncols, offset, length, result)

values = c(1+2i,3+4i,5-6i,7+8i,9-10i,10+11i,11-12i,12+13i, 13+14i)
ncols = 3
result = matrix(c(1+2i,3+4i,5-6i,7+8i,9-10i,10+11i), ncol = 2)
offset = 0
length = 2
check_hmatrix(values, HDataType$complex32(), ncols, offset, length, result)
check_hmatrix(values, HDataType$complex64(), ncols, offset, length, result)
check_hmatrix_shared(values, HDataType$complex32(), ncols, offset, length, result)
check_hmatrix_shared(values, HDataType$complex64(), ncols, offset, length, result)
