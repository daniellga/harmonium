# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_harmonium_wrappers", use_symbols = TRUE, package_name = "harmonium")

#' @docType package
#' @usage NULL
#' @useDynLib harmonium, .registration = TRUE
NULL

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

HArray <- new.env(parent = emptyenv())

HArray$new_from_values <- function(arr, dtype) .Call(wrap__HArray__new_from_values, arr, dtype)

HArray$len <- function() .Call(wrap__HArray__len, self)

HArray$shape <- function() .Call(wrap__HArray__shape, self)

HArray$ndim <- function() .Call(wrap__HArray__ndim, self)

HArray$slice <- function(range) .Call(wrap__HArray__slice, self, range)

HArray$print <- function() invisible(.Call(wrap__HArray__print, self))

HArray$eq <- function(other) .Call(wrap__HArray__eq, self, other)

HArray$ne <- function(other) .Call(wrap__HArray__ne, self, other)

HArray$clone <- function() .Call(wrap__HArray__clone, self)

HArray$collect <- function() .Call(wrap__HArray__collect, self)

HArray$dtype <- function() .Call(wrap__HArray__dtype, self)

HArray$is_shared <- function() .Call(wrap__HArray__is_shared, self)

HArray$mem_adress <- function() .Call(wrap__HArray__mem_adress, self)

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

HAudioSink <- new.env(parent = emptyenv())

HAudioSink$new <- function() .Call(wrap__HAudioSink__new)

HAudioSink$append_from_harray <- function(harray, sr) invisible(.Call(wrap__HAudioSink__append_from_harray, self, harray, sr))

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

HResampler$new_fft <- function(sr_in, sr_out, chunk_size, sub_chunks, nchannels, res_type, dtype) .Call(wrap__HResampler__new_fft, sr_in, sr_out, chunk_size, sub_chunks, nchannels, res_type, dtype)

HResampler$new_sinc <- function(resample_ratio, max_resample_ratio_relative, parameters, chunk_size, nchannels, res_type, dtype) .Call(wrap__HResampler__new_sinc, resample_ratio, max_resample_ratio_relative, parameters, chunk_size, nchannels, res_type, dtype)

HResampler$new_fast <- function(resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nchannels, res_type, dtype) .Call(wrap__HResampler__new_fast, resample_ratio, max_resample_ratio_relative, pol_deg, chunk_size, nchannels, res_type, dtype)

HResampler$process <- function(harray) invisible(.Call(wrap__HResampler__process, self, harray))

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

HFft <- new.env(parent = emptyenv())

HFft$fft <- function(harray) .Call(wrap__HFft__fft, harray)

HFft$fft_mut <- function(harray) invisible(.Call(wrap__HFft__fft_mut, harray))

HFft$fft_real_mut <- function(harray) invisible(.Call(wrap__HFft__fft_real_mut, harray))

#' @export
`$.HFft` <- function (self, name) { func <- HFft[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HFft` <- `$.HFft`

HAudioOp <- new.env(parent = emptyenv())

HAudioOp$nchannels <- function(harray) .Call(wrap__HAudioOp__nchannels, harray)

HAudioOp$nframes <- function(harray) .Call(wrap__HAudioOp__nframes, harray)

HAudioOp$db_to_amplitude <- function(harray, reference, power) invisible(.Call(wrap__HAudioOp__db_to_amplitude, harray, reference, power))

HAudioOp$to_mono <- function(harray) invisible(.Call(wrap__HAudioOp__to_mono, harray))

#' @export
`$.HAudioOp` <- function (self, name) { func <- HAudioOp[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HAudioOp` <- `$.HAudioOp`

HFile <- new.env(parent = emptyenv())

HFile$decode <- function(fpath, dtype) .Call(wrap__HFile__decode, fpath, dtype)

HFile$decode_stream <- function(fpath, frames, dtype) .Call(wrap__HFile__decode_stream, fpath, frames, dtype)

HFile$metadata <- function(fpath, metadata_type) .Call(wrap__HFile__metadata, fpath, metadata_type)

HFile$params <- function(fpath) .Call(wrap__HFile__params, fpath)

HFile$verify <- function(fpath) .Call(wrap__HFile__verify, fpath)

#' @export
`$.HFile` <- function (self, name) { func <- HFile[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HFile` <- `$.HFile`

#' HDecoderStream
#' An iterator that decodes audio in streams. \
#'
#' # Methods
#'
HDecoderStream <- new.env(parent = emptyenv())

HDecoderStream$stream <- function() .Call(wrap__HDecoderStream__stream, self)

#' @export
`$.HDecoderStream` <- function (self, name) { func <- HDecoderStream[[name]]; environment(func) <- environment(); func }

#' @export
`[[.HDecoderStream` <- `$.HDecoderStream`


# nolint end
