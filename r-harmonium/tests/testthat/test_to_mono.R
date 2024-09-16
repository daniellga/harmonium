test_that(
  "as_mono works.",
  {
    # f32
    values = c(1,2,3,4,5,6,7,8,9,10,11,12)
    arr = matrix(values, ncol = 3)
    harray = HArray$new_from_values(arr, HDataType$Float32)
    HArrayAudio$to_mono(harray)
    expect_equal(harray$collect(), as.array(rowMeans(arr)))

    # f64
    values = c(1,2,3,4,5,6,7,8,9,10,11,12)
    arr = matrix(values, ncol = 3)
    harray = HArray$new_from_values(arr, HDataType$Float64)
    HArrayAudio$to_mono(harray)
    expect_equal(harray$collect(), as.array(rowMeans(arr)))
  }
)
