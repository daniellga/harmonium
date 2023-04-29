#' HArray and HMatrix to nanoarrow and arrow
#' @description Conversion via native apache arrow array. Requires ´nanoarrow´ package.
#' @name nanoarrow
#' @param x an HArray.
#' @keywords nanoarrow_interface.
#' @return A nanoarrow array.
#' @details
#'
#' The following functions enable conversion to `nanoarrow` and `arrow`.
#'
#' @examples
#' library(nanoarrow)
#' harray = HArray$new_from_values(c(1,2,3), HDataType$float32())
#' nanoarrow_array = as_nanoarrow_array(harray)
#' v = as.vector(nanoarrow_array)
#' print(v)
#'
as_nanoarrow_array.HArray = function(x) {
  array = nanoarrow::nanoarrow_allocate_array()
  schema = nanoarrow::nanoarrow_allocate_schema()
  x$to_c_arrow(nanoarrow::nanoarrow_pointer_addr_chr(array), nanoarrow::nanoarrow_pointer_addr_chr(schema))
  nanoarrow::nanoarrow_array_set_schema(array, schema, validate = FALSE)
  array
}

#' @rdname nanoarrow
#' @return An R vector of zero size describing the target into which the array should be materialized.
#' @examples
#' infer_nanoarrow_ptype(harray)
#'
infer_nanoarrow_ptype.HArray = function(x) {
  nanoarrow::infer_nanoarrow_ptype(as_nanoarrow_array.HArray(x))
}

#' @rdname nanoarrow
#' @return An arrow array.
#' @examples
#' arrow::as_arrow_array(harray)
#' @exportS3Method arrow::as_arrow_array
#'
as_arrow_array.HArray = function(x) {
  arrow::as_arrow_array(as_nanoarrow_array.HArray(x))
}

#' @rdname nanoarrow
#' @param x an HArray.
#' @keywords nanoarrow_interface.
#' @return A nanoarrow array.
#' @examples
#' library(nanoarrow)
#' values = matrix(c(1,2,3,4), ncol = 2)
#' hmatrix = HMatrix$new_from_values(values, HDataType$float32())
#' nanoarrow_array = as_nanoarrow_array(hmatrix)
#' v = as.vector(nanoarrow_array)
#' print(v)
as_nanoarrow_array.HMatrix = function(x) {
  array = nanoarrow::nanoarrow_allocate_array()
  schema = nanoarrow::nanoarrow_allocate_schema()
  x$to_c_arrow(nanoarrow::nanoarrow_pointer_addr_chr(array), nanoarrow::nanoarrow_pointer_addr_chr(schema))
  nanoarrow::nanoarrow_array_set_schema(array, schema, validate = FALSE)
  array
}

#' @rdname nanoarrow
#' @return An R vector of zero size describing the target into which the array should be materialized.
#' @examples
#' infer_nanoarrow_ptype(hmatrix)
#'
infer_nanoarrow_ptype.HMatrix = function(x) {
  nanoarrow::infer_nanoarrow_ptype(as_nanoarrow_array.HMatrix(x))
}

#' @rdname nanoarrow
#' @return An arrow array.
#' @examples
#' arrow::as_arrow_array(hmatrix)
#' @exportS3Method arrow::as_arrow_array
#'
as_arrow_array.HMatrix = function(x) {
  arrow::as_arrow_array(as_nanoarrow_array.HMatrix(x))
}
