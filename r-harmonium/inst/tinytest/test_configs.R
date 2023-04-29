# configs test

expect_true(is.list(hconfig_get()))
expect_true(length(hconfig_get()) > 0)
expect_error(hconfig_set_h_fmt_max_rows(10))
hconfig_set_h_fmt_max_rows(15L)
expect_equal(hconfig_get()[["H_FMT_MAX_ROWS"]], "15")
hconfig_set_default()
expect_true(is.list(hconfig_get()))
expect_true(length(hconfig_get()) > 0)
expect_equal(hconfig_get()[["H_FMT_MAX_ROWS"]], "")
hconfig_set_initial()
