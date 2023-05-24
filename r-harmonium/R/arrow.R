# Conversion via native apache arrow array. Requires ´nanoarrow´ package.
# The following functions enable conversion to `nanoarrow` and `arrow`.

# Conversion from HArray to nanoarrow array.
# Internal function.
as_nanoarrow_array.HArray = function(x) {
  array = nanoarrow::nanoarrow_allocate_array()
  schema = nanoarrow::nanoarrow_allocate_schema()
  x$to_c_arrow(nanoarrow::nanoarrow_pointer_addr_chr(array), nanoarrow::nanoarrow_pointer_addr_chr(schema))
  nanoarrow::nanoarrow_array_set_schema(array, schema, validate = FALSE)
  array
}

# Conversion from HMatrix to nanoarrow array.
# Internal function.
as_nanoarrow_array.HMatrix = function(x) {
  array = nanoarrow::nanoarrow_allocate_array()
  schema = nanoarrow::nanoarrow_allocate_schema()
  x$to_c_arrow(nanoarrow::nanoarrow_pointer_addr_chr(array), nanoarrow::nanoarrow_pointer_addr_chr(schema))
  nanoarrow::nanoarrow_array_set_schema(array, schema, validate = FALSE)
  array
}

# Infer the nanoarrow array type.
# Internal function.
infer_nanoarrow_ptype.HArray = function(x) {
  nanoarrow::infer_nanoarrow_ptype(as_nanoarrow_array.HArray(x))
}

# Infer the nanoarrow array type.
# Internal function.
infer_nanoarrow_ptype.HMatrix = function(x) {
  nanoarrow::infer_nanoarrow_ptype(as_nanoarrow_array.HMatrix(x))
}

# Conversion to arrow array.
as_arrow_array.HArray = function(x) {
  arrow::as_arrow_array(as_nanoarrow_array.HArray(x))
}

# Conversion to arrow array.
as_arrow_array.HMatrix = function(x) {
  arrow::as_arrow_array(as_nanoarrow_array.HMatrix(x))
}

