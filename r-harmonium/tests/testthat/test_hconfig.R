test_that(
  "Configs for environment variables work.",
  {
    expect_true(is.list(HConfig$get()))
    expect_true(length(HConfig$get()) > 0)

    expect_equal(HConfig$get()[["H_FMT_MAX_LEN"]], "")
    expect_error(HConfig$set_table_max_len(10))
    HConfig$set_table_max_len(15L)
    expect_equal(HConfig$get()[["H_FMT_MAX_LEN"]], "15")

    expect_equal(HConfig$get()[["H_FMT_MAX_COLS"]], "")
    expect_error(HConfig$set_table_max_cols(10))
    HConfig$set_table_max_cols(15L)
    expect_equal(HConfig$get()[["H_FMT_MAX_COLS"]], "15")

    expect_equal(HConfig$get()[["H_FMT_MAX_ROWS"]], "")
    expect_error(HConfig$set_table_max_rows(10))
    HConfig$set_table_max_rows(15L)
    expect_equal(HConfig$get()[["H_FMT_MAX_ROWS"]], "15")

    expect_equal(HConfig$get()[["H_FMT_FLOAT"]], "mixed")
    expect_error(HConfig$set_float_fmt("whatever"))
    HConfig$set_float_fmt("full")
    expect_equal(HConfig$get()[["H_FMT_FLOAT"]], "full")

    HConfig$set_default()
    expect_true(is.list(HConfig$get()))
    expect_true(length(HConfig$get()) > 0)

    expect_equal(HConfig$get()[["H_FMT_MAX_LEN"]], "")
    expect_equal(HConfig$get()[["H_FMT_MAX_COLS"]], "")
    expect_equal(HConfig$get()[["H_FMT_MAX_ROWS"]], "")
    expect_equal(HConfig$get()[["H_FMT_FLOAT"]], "mixed")

    HConfig$set_initial()
  }
)
