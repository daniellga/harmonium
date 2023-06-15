test_that(
  "Conversion to arrow works.",
  {
    skip_if_not_installed("nanoarrow")
    skip_if_not_installed("arrow")
    
    check_arrow_array_conversion = function(values, dtype) {
      harray = HArray$new_from_values(values, dtype)
      arrow_array = arrow::as_arrow_array(harray)
      harray2 = HArray$new_from_arrow(arrow_array, dtype)
      expect_true(harray == harray2)
    }

    check_arrow_matrix_conversion = function(values, dtype) {
      ncols = ncol(values)
      hmatrix = HMatrix$new_from_values(values, dtype)
      arrow_array = arrow::as_arrow_array(hmatrix)
      hmatrix2 = HMatrix$new_from_arrow(arrow_array, ncols, dtype)
      expect_true(hmatrix == hmatrix2)
    }


    check_arrow_array_conversion(c(1,2,3), HDataType$float32)
    check_arrow_array_conversion(c(1,2,3), HDataType$float64)
    check_arrow_array_conversion(c(1+2i,3+4i,5+6i), HDataType$complex32)
    check_arrow_array_conversion(c(1+2i,3+4i,5+6i), HDataType$complex64)
    check_arrow_matrix_conversion(matrix(c(1,2,3,4), ncol = 2), HDataType$float32)
    check_arrow_matrix_conversion(matrix(c(1,2,3,4), ncol = 2), HDataType$float64)
    check_arrow_matrix_conversion(matrix(c(1+2i,3+4i,5+6i,7+8i), ncol = 2), HDataType$complex32)
    check_arrow_matrix_conversion(matrix(c(1+2i,3+4i,5+6i,7+8i), ncol = 2), HDataType$complex64)
  }
)
