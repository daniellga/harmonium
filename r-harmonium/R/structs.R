print.HArray = function(x, ...) {
  cat(class(x), sep = "\n")
  x$dtype()$print()
  x$print()
}
"==.HArray" <- function(e1,e2) e1$eq(e2)
"!=.HArray" <- function(e1,e2) e1$ne(e2)

print.HDataType = function(x, ...) {
  x$print()
}
"==.HDataType" <- function(e1,e2) e1$eq(e2)
"!=.HDataType" <- function(e1,e2) e1$ne(e2)

print.HMetadataType = function(x, ...) {
  x$print()
}
"==.HMetadataType" <- function(e1,e2) e1$eq(e2)
"!=.HMetadataType" <- function(e1,e2) e1$ne(e2)

print.HResampler = function(x, ...) {
  x$print()
}

print.HSincInterpolationParams = function(x, ...) {
  x$print()
}

print.HResamplerType = function(x, ...) {
  x$print()
}
"==.HResamplerType" <- function(e1,e2) e1$eq(e2)
"!=.HResamplerType" <- function(e1,e2) e1$ne(e2)

print.HPolynomialDegree = function(x, ...) {
  x$print()
}
"==.HPolynomialDegree" <- function(e1,e2) e1$eq(e2)
"!=.HPolynomialDegree" <- function(e1,e2) e1$ne(e2)
