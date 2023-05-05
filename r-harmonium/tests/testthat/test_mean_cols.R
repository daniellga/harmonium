test_that(
  "mean_cols works.",
  {
    values = matrix(c(1,2,3,4,5,6,7,8,9,10,11,12), ncol = 3)
    hmatrix = HMatrix$new_from_values(values, HDataType$float32)
    mem_adress_before = hmatrix$mem_adress()
    hmatrix$mean_cols()
    mem_adress_after = hmatrix$mem_adress()
    expect_false(mem_adress_before == mem_adress_after)
    expect_equal(hmatrix$collect(), as.matrix(rowMeans(values)))

    # testing with more than one reference to the same hmatrix.
    rm(hmatrix)
    gc()
    hmatrix = HMatrix$new_from_values(values, HDataType$float32)
    expect_false(hmatrix$is_shared())
    hmatrix2 = hmatrix$clone()
    expect_true(hmatrix$is_shared())
    hmatrix$mean_cols()
    expect_equal(hmatrix2$collect(), values)
    expect_equal(hmatrix$collect(), as.matrix(rowMeans(values)))

    values = matrix(c(1+2i,3+4i,4+5i,5+6i,6+7i,7+8i,8+9i,9+10i,10+11i,11+12i), ncol = 2)
    hmatrix = HMatrix$new_from_values(values, HDataType$complex32)
    mem_adress_before = hmatrix$mem_adress()
    hmatrix$mean_cols()
    mem_adress_after = hmatrix$mem_adress()
    expect_false(mem_adress_before == mem_adress_after)
    expect_equal(hmatrix$collect(), as.matrix(rowMeans(values)))

    # testing with more than one reference to the same hmatrix.
    rm(hmatrix)
    gc()
    hmatrix = HMatrix$new_from_values(values, HDataType$complex32)
    expect_false(hmatrix$is_shared())
    hmatrix2 = hmatrix$clone()
    expect_true(hmatrix$is_shared())
    hmatrix$mean_cols()
    expect_equal(hmatrix2$collect(), values)
    expect_equal(hmatrix$collect(), as.matrix(rowMeans(values)))
  }
)
