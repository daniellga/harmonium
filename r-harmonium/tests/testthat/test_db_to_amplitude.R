test_that(
  "db_to_amplitude works.",
  {
    check_db_to_amplitude = function(dtype, input, result) {
      harray = HArray$new_from_values(input, dtype)
      HAudioOp$db_to_amplitude(harray, 1, 1)
      expect_true(all.equal(harray$collect(), result, tolerance = 1e-4))
    }
    
    input = matrix(c(1,2,3,4,5,6,7,8), 4, 2)
    result = matrix(c(1.258925, 1.584893, 1.995262, 2.511886, 3.162278, 3.981072, 5.011872, 6.309574), 4, 2)
    check_db_to_amplitude(HDataType$float32, input, result)
    check_db_to_amplitude(HDataType$float64, input, result)
  }
)


