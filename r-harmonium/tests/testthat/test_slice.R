test_that(
  "slice works",
  {
    check_slice = function(arr, dtype, dtype_result) {
      harray = HArray$new_from_values(arr, dtype)
      
      slice = harray$slice(list(c(0L, 2L, 1L), c(0L, 2L, 1L)))
      expect_equal(slice$shape(), c(2, 2))
      expect_equal(harray$mem_adress(), slice$mem_adress())
      
      # Out-of-bounds check.
      expect_error(harray$slice(list(c(0L, 5L, 1L), c(0L, 2L, 1L))))
      # Step = 0 check.
      expect_error(harray$slice(list(c(0L, 2L, 0L), c(0L, 2L, 1L))))
    }
    
    arr = array(c(1,2,3,4,5,6,7,8,9,10,11,12), c(3, 4))
    check_slice(arr, HDataType$Float32, HDataType$Float32)
    check_slice(arr, HDataType$Float64, HDataType$Float64)
    
    arr = array(c(1+2i,3+4i,5-6i,7+8i,9-10i,10+11i,11-12i,12+13i,14+15i,16+17i,18+19i, 20+21i), c(3, 4))
    check_slice(arr, HDataType$Complex32, HDataType$Complex32)
    check_slice(arr, HDataType$Complex64, HDataType$Complex64)
  }
)