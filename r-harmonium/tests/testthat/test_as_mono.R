test_that(
  "as_mono works.",
  {
    # f32
    values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 3)
    haudio = HAudio$new_from_values(values, 22000, HDataType$float32)
    mem_adress_before = haudio$mem_adress()
    haudio$as_mono()
    mem_adress_after = haudio$mem_adress()
    expect_false(mem_adress_before == mem_adress_after)
    expect_equal(haudio$collect(), as.matrix(rowMeans(values)))

    # testing with more than one reference to the same haudio
    rm(haudio)
    gc()
    haudio = HAudio$new_from_values(values, 22000, HDataType$float32)
    haudio2 = haudio$clone()
    haudio$as_mono()
    expect_equal(haudio2$collect(), values)
    expect_equal(haudio$collect(), as.matrix(rowMeans(values)))


    # f64
    values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 3)
    haudio = HAudio$new_from_values(values, 22000, HDataType$float64)
    mem_adress_before = haudio$mem_adress()
    haudio$as_mono()
    mem_adress_after = haudio$mem_adress()
    expect_false(mem_adress_before == mem_adress_after)
    expect_equal(haudio$collect(), as.matrix(rowMeans(values)))

    # testing with more than one reference to the same haudio
    rm(haudio, haudio2)
    gc()
    haudio = HAudio$new_from_values(values, 22000, HDataType$float64)
    haudio2 = haudio$clone()
    haudio$as_mono()
    expect_equal(haudio2$collect(), values)
    expect_equal(haudio$collect(), as.matrix(rowMeans(values)))
  }
)
