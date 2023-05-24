# R runtime options.
H_ENV_VARS = c("H_FMT_MAX_LEN", "H_FMT_MAX_COLS", "H_FMT_MAX_ROWS")

# Requirement functions.
req_fun = function(x) {
  is.integer(x) && length(x)==1 && !is.na(x) && x>0
}

HConfig <- new.env()

# Keep a list of initial values.
h_env_vars_ini = as.list(Sys.getenv(H_ENV_VARS))
h_fmt_float_ini = get_float_fmt()

### HConfig
### Manage configuration variables related to harmonium. \
###
### #### Environment Variables
###
### * `H_FMT_MAX_LEN` \
### An `integer`. Default = `10L`. \
### Maximum length to print in an `HArray`. \
### * `H_FMT_MAX_COLS` \
### An `integer`. Default = `10L`. \
### Maximum number of cols to print in an `HMatrix`. \
### * `H_FMT_MAX_ROWS` \
### An `integer`. Default = `10L`. \
### Maximum number of rows to print in an `HMatrix`. \
###
### #### Other Variables
###
### These variables are set directly, not via an environment variable. \
###
### * `H_FMT_FLOAT` \
### A `string`. One of ["full", "mixed"]. Default = `mixed`. \
### Controls how floating point numbers are displayed. \
###
### # Methods
###

### HConfig
### ## get
###
### `get() -> list`
###
### Retrieves the current settings. \
###
### #### Returns
###
### A `list`. \
###
### #### Examples
###
### ```r
### HConfig$get()
### ```
###
### _________
###
HConfig$get = function() {
  l = as.list(Sys.getenv(H_ENV_VARS))
  l$H_FMT_FLOAT = get_float_fmt()
  l
}

### HConfig
### ## set_table_max_len
###
### `set_table_max_len(value: integer)`
###
### Sets the `H_FMT_MAX_LEN` environment variable to `value`. \
### Will result in an error if not passing `value`'s requirements. \
###
### #### Arguments
###
### * `value` \
### An `integer`. The new value to be set. \
###
### #### Examples
###
### ```r
### HConfig$set_table_max_len(20L)
###
### HConfig$set_table_max_len(20) # Error! Value must be an integer.
### ```
###
### _________
###
HConfig$set_table_max_len = function(value) {
  if(!req_fun(value)) stop('"value" must be an integer.')

  Sys.setenv("H_FMT_MAX_LEN" = value)
}

### HConfig
### ## set_table_max_cols
###
### `set_table_max_cols(value: integer)`
###
### Sets the `H_FMT_MAX_COLS` environment variable to `value`. \
### Will result in an error if not passing `value`'s requirements. \
###
### #### Arguments
###
### * `value` \
### An `integer`. The new value to be set. \
###
### #### Examples
###
### ```r
### HConfig$set_table_max_cols(20L)
###
### HConfig$set_table_max_cols(20) # Error! Value must be an integer.
### ```
###
### _________
###
HConfig$set_table_max_cols = function(value) {
  if(!req_fun(value)) stop('"value" must be an integer.')

  Sys.setenv("H_FMT_MAX_COLS" = value)
}

### HConfig
### ## set_table_max_rows
###
### `set_table_max_rows(value: integer)`
###
### Sets the `H_FMT_MAX_ROWS` environment variable to `value`. \
### Will result in an error if not passing `value`'s requirements. \
###
### #### Arguments
###
### * `value` \
### An `integer`. The new value to be set. \
###
### #### Examples
###
### ```r
### HConfig$set_table_max_rows(20L)
###
### HConfig$set_table_max_rows(20) # Error! Value must be an integer.
### ```
###
### _________
###
HConfig$set_table_max_rows = function(value) {
  if(!req_fun(value)) stop('"value" must be an integer.')

  Sys.setenv("H_FMT_MAX_ROWS" = value)
}

### HConfig
### ## set_float_fmt
###
### `set_float_fmt(value: string)`
###
### Sets the `H_FMT_FLOAT` variable to `value`. \
### Will result in an error if not passing `value`'s requirements. \
###
### #### Arguments
###
### * `value` \
### A `string`. The new value to be set. \
### Must be one of ["full", "mixed"]. \
###
### #### Examples
###
### ```r
### HConfig$set_float_fmt("full")
###
### HConfig$set_float_fmt("whatever") # Error! Value must be one of ["full", "mixed"].
### ```
###
### _________
###
HConfig$set_float_fmt = function(value) {
  set_float_fmt(value)
}

### HConfig
### ## set_default
###
### `set_default()`
###
### Sets to the default configuration. \
###
### #### Examples
###
### ```r
### HConfig$set_default()
### ```
###
### _________
###
HConfig$set_default = function() {
  Sys.unsetenv(H_ENV_VARS)
  set_float_fmt("mixed")
}

### HConfig
### ## set_initial
###
### `set_initial()`
###
### Sets to the initial settings, from before the package was loaded. \
###
### #### Examples
###
### ```r
### HConfig$set_initial()
### ```
###
### _________
###
HConfig$set_initial = function() {
  do.call(Sys.setenv, h_env_vars_ini)
  set_float_fmt(h_fmt_float_ini)
}

