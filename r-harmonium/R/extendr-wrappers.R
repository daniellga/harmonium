# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_harmonium_wrappers", use_symbols = TRUE, package_name = "harmonium")

#' @docType package
#' @usage NULL
#' @useDynLib harmonium, .registration = TRUE
NULL

set_float_fmt <- function(fmt) invisible(.Call(wrap__set_float_fmt, fmt))

get_float_fmt <- function() .Call(wrap__get_float_fmt)

HPolynomialDegree <- new.env(parent = emptyenv())

HPolynomialDegree$septic <- function() .Call(wrap__HPolynomialDegree__septic)

HPolynomialDegree$quintic <- function() .Call(wrap__HPolynomialDegree__quintic)

HPolynomialDegree$cubic <- function() .Call(wrap__HPolynomialDegree__cubic)

HPolynomialDegree$linear <- function() .Call(wrap__HPolynomialDegree__linear)

HPolynomialDegree$nearest <- function() .Call(wrap__HPolynomialDegree__nearest)

HPolynomialDegree$print <- function() invisible(.Call(wrap__HPolynomialDegree__print, self))

HPolynomialDegree$eq <- function(other) .Call(wrap__HPolynomialDegree__eq, self, other)

HPolynomialDegree$ne <- function(other) .Call(wrap__HPolynomialDegree__ne, self, other)

#' @export
`$.HPolynomialDegree` <- function (self, name) { func <- HPolynomialDegree[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HPolynomialDegree` <- `$.HPolynomialDegree`

HError <- new.env(parent = emptyenv())

HError$to_error1 <- function() .Call(wrap__HError__to_error1)

HError$to_error2 <- function() .Call(wrap__HError__to_error2)

#' @export
`$.HError` <- function (self, name) { func <- HError[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HError` <- `$.HError`

HArray <- new.env(parent = emptyenv())

HArray$new_from_values <- function(values, dtype) .Call(wrap__HArray__new_from_values, values, dtype)

HArray$new_from_arrow <- function(values, dtype) .Call(wrap__HArray__new_from_arrow, values, dtype)

HArray$len <- function() .Call(wrap__HArray__len, self)

HArray$slice <- function(offset, length) invisible(.Call(wrap__HArray__slice, self, offset, length))

HArray$print <- function() invisible(.Call(wrap__HArray__print, self))

HArray$eq <- function(other) .Call(wrap__HArray__eq, self, other)

HArray$ne <- function(other) .Call(wrap__HArray__ne, self, other)

HArray$clone <- function() .Call(wrap__HArray__clone, self)

HArray$as_hmatrix <- function(ncols) .Call(wrap__HArray__as_hmatrix, self, ncols)

HArray$collect <- function() .Call(wrap__HArray__collect, self)

HArray$mem_adress <- function() .Call(wrap__HArray__mem_adress, self)

HArray$dtype <- function() .Call(wrap__HArray__dtype, self)

HArray$is_shared <- function() .Call(wrap__HArray__is_shared, self)

HArray$to_c_arrow <- function(array_ptr, schema_ptr) invisible(.Call(wrap__HArray__to_c_arrow, self, array_ptr, schema_ptr))

HArray$fft <- function() .Call(wrap__HArray__fft, self)

#' @export
`$.HArray` <- function (self, name) { func <- HArray[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HArray` <- `$.HArray`

HSincInterpolationParams <- new.env(parent = emptyenv())

HSincInterpolationParams$new <- function(sinc_len, f_cutoff, oversampling_factor, interpolation, window) .Call(wrap__HSincInterpolationParams__new, sinc_len, f_cutoff, oversampling_factor, interpolation, window)

HSincInterpolationParams$print <- function() invisible(.Call(wrap__HSincInterpolationParams__print, self))

#' @export
`$.HSincInterpolationParams` <- function (self, name) { func <- HSincInterpolationParams[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HSincInterpolationParams` <- `$.HSincInterpolationParams`

HAudio <- new.env(parent = emptyenv())

HAudio$new_from_file <- function(fpath, offset = NA_real_, duration = NA_real_, dtype) .Call(wrap__HAudio__new_from_file, fpath, offset, duration, dtype)

HAudio$new_from_values <- function(values, sr, dtype) .Call(wrap__HAudio__new_from_values, values, sr, dtype)

HAudio$len <- function() .Call(wrap__HAudio__len, self)

HAudio$nchannels <- function() .Call(wrap__HAudio__nchannels, self)

HAudio$nframes <- function() .Call(wrap__HAudio__nframes, self)

HAudio$print <- function() invisible(.Call(wrap__HAudio__print, self))

HAudio$as_hmatrix <- function() .Call(wrap__HAudio__as_hmatrix, self)

HAudio$eq <- function(other) .Call(wrap__HAudio__eq, self, other)

HAudio$ne <- function(other) .Call(wrap__HAudio__ne, self, other)

HAudio$clone <- function() .Call(wrap__HAudio__clone, self)

HAudio$collect <- function() .Call(wrap__HAudio__collect, self)

HAudio$sr <- function() .Call(wrap__HAudio__sr, self)

HAudio$mem_adress <- function() .Call(wrap__HAudio__mem_adress, self)

HAudio$dtype <- function() .Call(wrap__HAudio__dtype, self)

HAudio$as_mono <- function() invisible(.Call(wrap__HAudio__as_mono, self))

#' @export
`$.HAudio` <- function (self, name) { func <- HAudio[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HAudio` <- `$.HAudio`

HAudioSink <- new.env(parent = emptyenv())

HAudioSink$new <- function() .Call(wrap__HAudioSink__new)

HAudioSink$append_from_haudio <- function(haudio) invisible(.Call(wrap__HAudioSink__append_from_haudio, self, haudio))

HAudioSink$append_from_file <- function(fpath) invisible(.Call(wrap__HAudioSink__append_from_file, self, fpath))

HAudioSink$play <- function() invisible(.Call(wrap__HAudioSink__play, self))

HAudioSink$stop <- function() invisible(.Call(wrap__HAudioSink__stop, self))

HAudioSink$pause <- function() invisible(.Call(wrap__HAudioSink__pause, self))

HAudioSink$is_paused <- function() .Call(wrap__HAudioSink__is_paused, self)

HAudioSink$volume <- function() .Call(wrap__HAudioSink__volume, self)

HAudioSink$set_volume <- function(value) invisible(.Call(wrap__HAudioSink__set_volume, self, value))

HAudioSink$speed <- function() .Call(wrap__HAudioSink__speed, self)

HAudioSink$set_speed <- function(value) invisible(.Call(wrap__HAudioSink__set_speed, self, value))

HAudioSink$sleep_until_end <- function() invisible(.Call(wrap__HAudioSink__sleep_until_end, self))

HAudioSink$len <- function() .Call(wrap__HAudioSink__len, self)

HAudioSink$is_empty <- function() .Call(wrap__HAudioSink__is_empty, self)

HAudioSink$clear <- function() invisible(.Call(wrap__HAudioSink__clear, self))

HAudioSink$skip_one <- function() invisible(.Call(wrap__HAudioSink__skip_one, self))

HAudioSink$audio_output_devices <- function() .Call(wrap__HAudioSink__audio_output_devices)

HAudioSink$audio_default_device <- function() .Call(wrap__HAudioSink__audio_default_device)

HAudioSink$audio_supported_configs <- function() .Call(wrap__HAudioSink__audio_supported_configs)

#' @export
`$.HAudioSink` <- function (self, name) { func <- HAudioSink[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HAudioSink` <- `$.HAudioSink`

HFile <- new.env(parent = emptyenv())

HFile$metadata_from_file <- function(fpath, metadata_type) .Call(wrap__HFile__metadata_from_file, fpath, metadata_type)

HFile$get_params_from_file <- function(fpath) .Call(wrap__HFile__get_params_from_file, fpath)

HFile$verify_file <- function(fpath) .Call(wrap__HFile__verify_file, fpath)

#' @export
`$.HFile` <- function (self, name) { func <- HFile[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HFile` <- `$.HFile`

HDataType <- new.env(parent = emptyenv())

HDataType$float32 <- function() .Call(wrap__HDataType__float32)

HDataType$float64 <- function() .Call(wrap__HDataType__float64)

HDataType$complex32 <- function() .Call(wrap__HDataType__complex32)

HDataType$complex64 <- function() .Call(wrap__HDataType__complex64)

HDataType$print <- function() invisible(.Call(wrap__HDataType__print, self))

HDataType$eq <- function(other) .Call(wrap__HDataType__eq, self, other)

HDataType$ne <- function(other) .Call(wrap__HDataType__ne, self, other)

#' @export
`$.HDataType` <- function (self, name) { func <- HDataType[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HDataType` <- `$.HDataType`

HMatrix <- new.env(parent = emptyenv())

HMatrix$new_from_values <- function(values, dtype) .Call(wrap__HMatrix__new_from_values, values, dtype)

HMatrix$new_from_arrow <- function(values, ncols, dtype) .Call(wrap__HMatrix__new_from_arrow, values, ncols, dtype)

HMatrix$len <- function() .Call(wrap__HMatrix__len, self)

HMatrix$slice <- function(offset, length) invisible(.Call(wrap__HMatrix__slice, self, offset, length))

HMatrix$ncols <- function() .Call(wrap__HMatrix__ncols, self)

HMatrix$nrows <- function() .Call(wrap__HMatrix__nrows, self)

HMatrix$print <- function() invisible(.Call(wrap__HMatrix__print, self))

HMatrix$eq <- function(other) .Call(wrap__HMatrix__eq, self, other)

HMatrix$ne <- function(other) .Call(wrap__HMatrix__ne, self, other)

HMatrix$clone <- function() .Call(wrap__HMatrix__clone, self)

HMatrix$as_harray <- function() .Call(wrap__HMatrix__as_harray, self)

HMatrix$as_haudio <- function(sr) .Call(wrap__HMatrix__as_haudio, self, sr)

HMatrix$collect <- function() .Call(wrap__HMatrix__collect, self)

HMatrix$mem_adress <- function() .Call(wrap__HMatrix__mem_adress, self)

HMatrix$dtype <- function() .Call(wrap__HMatrix__dtype, self)

HMatrix$is_shared <- function() .Call(wrap__HMatrix__is_shared, self)

HMatrix$to_c_arrow <- function(array_ptr, schema_ptr) invisible(.Call(wrap__HMatrix__to_c_arrow, self, array_ptr, schema_ptr))

HMatrix$fft <- function() .Call(wrap__HMatrix__fft, self)

HMatrix$mean_cols <- function() invisible(.Call(wrap__HMatrix__mean_cols, self))

HMatrix$db_to_power <- function(reference) invisible(.Call(wrap__HMatrix__db_to_power, self, reference))

#' @export
`$.HMatrix` <- function (self, name) { func <- HMatrix[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HMatrix` <- `$.HMatrix`

HWindow <- new.env(parent = emptyenv())

HWindow$barthann <- function(npoints, sym, dtype) .Call(wrap__HWindow__barthann, npoints, sym, dtype)

HWindow$bartlett <- function(npoints, sym, dtype) .Call(wrap__HWindow__bartlett, npoints, sym, dtype)

HWindow$blackman <- function(npoints, sym, dtype) .Call(wrap__HWindow__blackman, npoints, sym, dtype)

HWindow$blackmanharris <- function(npoints, sym, dtype) .Call(wrap__HWindow__blackmanharris, npoints, sym, dtype)

HWindow$bohman <- function(npoints, sym, dtype) .Call(wrap__HWindow__bohman, npoints, sym, dtype)

HWindow$boxcar <- function(npoints, dtype) .Call(wrap__HWindow__boxcar, npoints, dtype)

HWindow$cosine <- function(npoints, sym, dtype) .Call(wrap__HWindow__cosine, npoints, sym, dtype)

HWindow$hann <- function(npoints, sym, dtype) .Call(wrap__HWindow__hann, npoints, sym, dtype)

#' @export
`$.HWindow` <- function (self, name) { func <- HWindow[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HWindow` <- `$.HWindow`

HMetadataType <- new.env(parent = emptyenv())

HMetadataType$all <- function() .Call(wrap__HMetadataType__all)

HMetadataType$text <- function() .Call(wrap__HMetadataType__text)

HMetadataType$visual <- function() .Call(wrap__HMetadataType__visual)

HMetadataType$print <- function() invisible(.Call(wrap__HMetadataType__print, self))

HMetadataType$eq <- function(other) .Call(wrap__HMetadataType__eq, self, other)

HMetadataType$ne <- function(other) .Call(wrap__HMetadataType__ne, self, other)

#' @export
`$.HMetadataType` <- function (self, name) { func <- HMetadataType[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HMetadataType` <- `$.HMetadataType`

HResampler <- new.env(parent = emptyenv())

HResampler$new_fft <- function(sr_in, sr_out, chunk_size, sub_chunks, nbr_channels, res_type, dtype) .Call(wrap__HResampler__new_fft, sr_in, sr_out, chunk_size, sub_chunks, nbr_channels, res_type, dtype)

HResampler$new_sinc <- function(resample_ratio, max_resample_ratio_relative, parameters, chunk_size, nbr_channels, res_type, dtype) .Call(wrap__HResampler__new_sinc, resample_ratio, max_resample_ratio_relative, parameters, chunk_size, nbr_channels, res_type, dtype)

HResampler$new_fast <- function(resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nbr_channels, res_type, dtype) .Call(wrap__HResampler__new_fast, resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nbr_channels, res_type, dtype)

HResampler$process <- function(haudio, sr_out) invisible(.Call(wrap__HResampler__process, self, haudio, sr_out))

HResampler$set_resample_ratio <- function(new_ratio, ramp) invisible(.Call(wrap__HResampler__set_resample_ratio, self, new_ratio, ramp))

HResampler$set_resample_ratio_relative <- function(rel_ratio, ramp) invisible(.Call(wrap__HResampler__set_resample_ratio_relative, self, rel_ratio, ramp))

HResampler$reset <- function() invisible(.Call(wrap__HResampler__reset, self))

HResampler$res_type <- function() .Call(wrap__HResampler__res_type, self)

HResampler$dtype <- function() .Call(wrap__HResampler__dtype, self)

HResampler$print <- function() invisible(.Call(wrap__HResampler__print, self))

#' @export
`$.HResampler` <- function (self, name) { func <- HResampler[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HResampler` <- `$.HResampler`

HResamplerType <- new.env(parent = emptyenv())

HResamplerType$fft_fixed_in <- function() .Call(wrap__HResamplerType__fft_fixed_in)

HResamplerType$fft_fixed_in_out <- function() .Call(wrap__HResamplerType__fft_fixed_in_out)

HResamplerType$fft_fixed_out <- function() .Call(wrap__HResamplerType__fft_fixed_out)

HResamplerType$sinc_fixed_in <- function() .Call(wrap__HResamplerType__sinc_fixed_in)

HResamplerType$sinc_fixed_out <- function() .Call(wrap__HResamplerType__sinc_fixed_out)

HResamplerType$fast_fixed_in <- function() .Call(wrap__HResamplerType__fast_fixed_in)

HResamplerType$fast_fixed_out <- function() .Call(wrap__HResamplerType__fast_fixed_out)

HResamplerType$print <- function() invisible(.Call(wrap__HResamplerType__print, self))

HResamplerType$eq <- function(other) .Call(wrap__HResamplerType__eq, self, other)

HResamplerType$ne <- function(other) .Call(wrap__HResamplerType__ne, self, other)

#' @export
`$.HResamplerType` <- function (self, name) { func <- HResamplerType[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HResamplerType` <- `$.HResamplerType`


# nolint end
