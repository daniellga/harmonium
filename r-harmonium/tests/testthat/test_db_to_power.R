test_that(
  "db_to_power works.",
  {
    check_db_to_power = function(dtype, input, result) {
      hmatrix = HMatrix$new_from_values(input, dtype)
      initial_mem = hmatrix$mem_adress()
      hmatrix$db_to_power(1)
      final_mem = hmatrix$mem_adress()
      expect_true(initial_mem == final_mem)
      expect_true(all.equal(hmatrix$collect(), result, tolerance = 1e-4))
      
      hmatrix = HMatrix$new_from_values(input, dtype)
      hmatrix_clone = hmatrix$clone()
      initial_mem = hmatrix$mem_adress()
      hmatrix$db_to_power(1)
      final_mem = hmatrix$mem_adress()
      expect_true(initial_mem != final_mem)
      expect_true(all.equal(hmatrix$collect(), result, tolerance = 1e-4))
    }
    
    input = matrix(c(1,2,3,4,5,6,7,8), 4, 2)
    result = matrix(c(1.258925, 1.584893, 1.995262, 2.511886, 3.162278, 3.981072, 5.011872, 6.309574), 4, 2)
    check_db_to_power(HDataType$float32, input, result)
    check_db_to_power(HDataType$float64, input, result)
    
    input = matrix(c(1+2i,3+4i,5+6i,7+8i), 2, 2)
    result = matrix(c(1.12777415 + 0.55948071i, 1.20712802 + 1.58868299i, 0.59488038 + 3.10581991i, -1.34296574 + 4.82859269i), 2, 2)
    check_db_to_power(HDataType$complex32, input, result)
    check_db_to_power(HDataType$complex64, input, result)
  }
)


