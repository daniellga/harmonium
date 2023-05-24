print.HArray = function(x) {
  cat(class(x), sep = "\n")
  x$dtype()$print()
  x$print()
}

"==.HArray" <- function(e1,e2) e1$eq(e2)
"!=.HArray" <- function(e1,e2) e1$ne(e2)

print.HMatrix = function(x) {
  cat(class(x), sep = "\n")
  x$dtype()$print()
  x$print()
}

"==.HMatrix" <- function(e1,e2) e1$eq(e2)
"!=.HMatrix" <- function(e1,e2) e1$ne(e2)

print.HAudio = function(x) {
  cat(class(x), sep = "\n")
  x$dtype()$print()
  x$print()
}

"==.HAudio" <- function(e1,e2) e1$eq(e2)
"!=.HAudio" <- function(e1,e2) e1$ne(e2)

print.HDataType = function(x) {
  x$print()
}

"==.HDataType" <- function(e1,e2) e1$eq(e2)
"!=.HDataType" <- function(e1,e2) e1$ne(e2)

print.HMetadataType = function(x) {
  x$print()
}

"==.HMetadataType" <- function(e1,e2) e1$eq(e2)
"!=.HMetadataType" <- function(e1,e2) e1$ne(e2)

print.HResampler = function(x) {
  x$print()
}

print.HResamplerType = function(x) {
  x$print()
}

"==.HResamplerType" <- function(e1,e2) e1$eq(e2)
"!=.HResamplerType" <- function(e1,e2) e1$ne(e2)

