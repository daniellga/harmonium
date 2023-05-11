#' print a HArray
#'
#' @param x HArray
#'
#' @return self
#' @export
#'
print.HArray = function(x) {
  cat(class(x), sep = "\n")
  x$dtype()$print()
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
  x$dtype()$print()
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
  x$dtype()$print()
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

#' print a HMetadataType
#'
#' @param x HMetadataType
#'
#' @return self
#' @export
#'
print.HMetadataType = function(x) {
  x$print()
}

#' @export
"==.HMetadataType" <- function(e1,e2) e1$eq(e2)
#' @export
"!=.HMetadataType" <- function(e1,e2) e1$ne(e2)

#' print a HResampler
#'
#' @param x HResampler
#'
#' @return self
#' @export
#'
print.HResampler = function(x) {
  x$print()
}

#' print a HResamplerType
#'
#' @param x HResamplerType
#'
#' @return self
#' @export
#'
print.HResamplerType = function(x) {
  x$print()
}

#' @export
"==.HResamplerType" <- function(e1,e2) e1$eq(e2)
#' @export
"!=.HResamplerType" <- function(e1,e2) e1$ne(e2)

