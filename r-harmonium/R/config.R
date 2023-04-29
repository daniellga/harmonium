# R runtime options.
HENV_VARS = c("H_FMT_MAX_COLS", "H_FMT_MAX_ROWS", "H_FMT_MAX_LEN")

# Requirement functions
is_integer = function(x) {
  is.integer(x) && length(x)==1 && !is.na(x) && x>0
}

# Keep a list of initial values.
hcfg_ini = as.list(Sys.getenv(HENV_VARS))

#' @rdname hconfig
#' @name hconfig
#' @description Manage environment variables related to harmoniumr.
#' @section Environment Variables:
#' H_FMT_MAX_COLS \[Integer\]. Default = 10.
#' Maximum number of cols to print in an HMatrix.
#'
NULL
#' @rdname hconfig
#' @name hconfig
#' @section Environment Variables:
#' H_FMT_MAX_ROWS \[Integer\]. Default = 10.
#' Maximum number of rows to print in an HMatrix.
#'
NULL

#' @rdname hconfig
#' @name hconfig
#' @section Environment Variables:
#' H_FMT_MAX_LEN \[Integer\]. Default = 10.
#' Maximum length to print in an HArray.
#'
NULL

#' hconfig
#' @rdname hconfig
#' @name hconfig
#'
#' @return hconfig_get() returns the current settings as a list
#' @examples
#' hconfig_get()
#' @export
#'
hconfig_get = function() {
  as.list(Sys.getenv(HENV_VARS))
}

#' @param option \[String]. Any option to modify.
#'
#' @rdname hconfig
#' @name hconfig_set_h_fmt_max_cols
#' @details Setting an option may be rejected if not passing the option's requirements.
#' @examples
#' hconfig_set_h_fmt_max_cols(20L)
#' @export
#'
hconfig_set_h_fmt_max_cols = function(value) {
  if(!is.integer(value)) stop('"value" must be an integer.')
  if(!is_integer(value)) stop(paste0(value, " doesn't satisfy the requirements."))

  Sys.setenv("H_FMT_MAX_COLS" = value)
}

#' @param option \[String]. Any option to modify.
#'
#' @rdname hconfig
#' @name hconfig_set_h_fmt_max_rows
#' @details Setting an option may be rejected if not passing the option's requirements.
#' @examples
#' hconfig_set_h_fmt_max_rows(20L)
#' @export
#'
hconfig_set_h_fmt_max_rows = function(value) {
  if(!is.integer(value)) stop('"value" must be an integer.')
  if(!is_integer(value)) stop(paste0(value, " doesn't satisfy the requirements."))

  Sys.setenv("H_FMT_MAX_ROWS" = value)
}

#' @param option \[String]. Any option to modify.
#'
#' @rdname hconfig
#' @name hconfig_set_h_fmt_max_len
#' @details Setting an option may be rejected if not passing the option's requirements.
#' @examples
#' hconfig_set_h_fmt_max_len(20L)
#' @export
#'
hconfig_set_h_fmt_max_len = function(value) {
  if(!is.integer(value)) stop('"value" must be an integer.')
  if(!is_integer(value)) stop(paste0(value, " doesn't satisfy the requirements."))

  Sys.setenv("H_FMT_MAX_LEN" = value)
}

#' @rdname hconfig
#' @name hconfig
#' @examples
#' hconfig_set_default()
#' @export
#'
hconfig_set_default = function() {
  Sys.unsetenv(HENV_VARS)
}

#' @rdname hconfig
#' @name hconfig
#' @examples
#' hconfig_set_initial()
#' @export
#'
# Use it to restart environment variables when package is unattached.
hconfig_set_initial = function() {
  do.call(Sys.setenv, hcfg_ini)
}
