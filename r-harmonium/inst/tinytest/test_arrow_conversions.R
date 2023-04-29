# arrow_conversions test

check_arrow_array_conversion = function(values, data_type) {
  harray = HArray$new_from_values(values, data_type)
  arrow_array = arrow::as_arrow_array(harray)
  harray2 = HArray$new_from_arrow(arrow_array, data_type)
  expect_equal(harray, harray2)
}

check_arrow_matrix_conversion = function(values, data_type) {
  ncols = ncol(values)
  hmatrix = HMatrix$new_from_values(values, data_type)
  arrow_array = arrow::as_arrow_array(hmatrix)
  hmatrix2 = HMatrix$new_from_arrow(arrow_array, data_type)
  expect_equal(hmatrix, hmatrix2)
}


check_arrow_array_conversion(c(1,2,3), HDataType$float32())
check_arrow_array_conversion(c(1,2,3), HDataType$float64())
check_arrow_array_conversion(c(1+2i,3+4i,5+6i), HDataType$complex32())
check_arrow_array_conversion(c(1+2i,3+4i,5+6i), HDataType$complex64())
check_arrow_matrix_conversion(matrix(c(1,2,3,4), ncol = 2), HDataType$float32())
check_arrow_matrix_conversion(matrix(c(1,2,3,4), ncol = 2), HDataType$float64())
check_arrow_matrix_conversion(matrix(c(1+2i,3+4i,5+6i,7+8i), ncol = 2), HDataType$complex32())
check_arrow_matrix_conversion(matrix(c(1+2i,3+4i,5+6i,7+8i), ncol = 2), HDataType$complex64())
