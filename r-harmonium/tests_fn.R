load_check <- function(fileslist, ...) {
  results <- c()
  for (fname in fileslist) {
    harmonium <- harmonium.io::load(fname, ...)

    audiolibrosa <- librosa$load(fname, sr = NULL, dtype = np$float64, ...)[[1]]
    if (length(dim(audiolibrosa)) > 1) { # only needed to transpose 2d arrays
      audiolibrosa <- t(audiolibrosa)
    } else {
      audiolibrosa <- as.matrix(audiolibrosa)
    }

    test <- expect_identical(harmonium, audiolibrosa)
    cat(test, "\n")
    results <- c(results, test)

    test <- expect_inherits(harmonium, "matrix")
    cat(test, "\n")
    results <- c(results, test)

    test <- expect_identical(length(dim(harmonium)), 2L)
    cat(test, "\n")
    results <- c(results, test)
    cat("----------------------------", "\n")
  }

  cat("combined test result: ")
  return(all(results))
}
