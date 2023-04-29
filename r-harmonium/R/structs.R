#' print a HArray
#'
#' @param x HArray
#'
#' @return self
#' @export
#'
print.HArray = function(x) {
  cat(class(x), sep = "\n")
  cat(paste0("dtype: ", x$data_type()), sep = "\n")
  x$print()
}

#' @export
"==.HArray" <- function(e1,e2) e1$eq(e2)
#' @export
"!=.HArray" <- function(e1,e2) e1$ne(e2)

#' print a HMatrix
#'
#' @param x HMatrix
#'
#' @return self
#' @export
#'
print.HMatrix = function(x) {
  cat(class(x), sep = "\n")
  cat(paste0("dtype: ", x$data_type()), sep = "\n")
  x$print()
}

#' @export
"==.HMatrix" <- function(e1,e2) e1$eq(e2)
#' @export
"!=.HMatrix" <- function(e1,e2) e1$ne(e2)

#' print a HAudio
#'
#' @param x HAudio
#'
#' @return self
#' @export
#'
print.HAudio = function(x) {
  cat(class(x), sep = "\n")
  cat(paste0("dtype: ", x$data_type()), sep = "\n")
  x$print()
}

#' @export
"==.HAudio" <- function(e1,e2) e1$eq(e2)
#' @export
"!=.HAudio" <- function(e1,e2) e1$ne(e2)

#' print a HDataType
#'
#' @param x HDataType
#'
#' @return self
#' @export
#'
print.HDataType = function(x) {
  x$print()
}

#' @export
"==.HDataType" <- function(e1,e2) e1$eq(e2)
#' @export
"!=.HDataType" <- function(e1,e2) e1$ne(e2)
