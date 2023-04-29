# conversions test

values = matrix(c(1,2,3,4,5,6), 2, 3)
dtype = HDataType$float32()
sr = 44100

haudio = HAudio$new_from_values(values, sr, dtype)
mem_adress = haudio$mem_adress()
hmatrix = haudio$as_hmatrix()
expect_true(hmatrix$eq_inner_haudio(haudio))
expect_true(haudio$eq_inner_hmatrix(hmatrix))
haudio2 = hmatrix$as_haudio(sr)
expect_equal(haudio2, haudio)
expect_true(haudio2$eq_inner(haudio))
harray = hmatrix$as_harray()
expect_true(harray$eq_inner_haudio(haudio))
expect_true(haudio$eq_inner_harray(harray))
hmatrix2 = harray$as_hmatrix(3)
expect_equal(hmatrix, hmatrix2)
expect_true(hmatrix$eq_inner(hmatrix2))

expect_equal(mem_adress, hmatrix$mem_adress())
expect_equal(mem_adress, hmatrix2$mem_adress())
expect_equal(mem_adress, haudio2$mem_adress())
expect_equal(mem_adress, harray$mem_adress())

