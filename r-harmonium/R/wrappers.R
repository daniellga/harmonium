#' @useDynLib harmonium, .registration = TRUE
#' @keywords internal
NULL

# Check class and extract the external pointer embedded in the environment
.savvy_extract_ptr <- function(e, class) {
  if(inherits(e, class)) {
    e$.ptr
  } else {
    msg <- paste0("Expected ", class, ", got ", class(e)[1])
    stop(msg, call. = FALSE)
  }
}



HArray <- new.env(parent = emptyenv())
HArray$new_from_values <- function(arr, dtype) {
  dtype <- .savvy_extract_ptr(dtype, "HDataType")
  .savvy_wrap_HArray(.Call(HArray_new_from_values__impl, arr, dtype))
}


.savvy_wrap_HArray <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$len <- HArray_len(ptr)
  e$shape <- HArray_shape(ptr)
  e$ndim <- HArray_ndim(ptr)
  e$slice <- HArray_slice(ptr)
  e$print <- HArray_print(ptr)
  e$eq <- HArray_eq(ptr)
  e$ne <- HArray_ne(ptr)
  e$clone <- HArray_clone(ptr)
  e$collect <- HArray_collect(ptr)
  e$dtype <- HArray_dtype(ptr)
  e$is_shared <- HArray_is_shared(ptr)
  e$mem_adress <- HArray_mem_adress(ptr)

  class(e) <- "HArray"
  e
}


HArray_len <- function(self) {
  function() {
  .Call(HArray_len__impl, self)
  }
}

HArray_shape <- function(self) {
  function() {
  .Call(HArray_shape__impl, self)
  }
}

HArray_ndim <- function(self) {
  function() {
  .Call(HArray_ndim__impl, self)
  }
}

HArray_slice <- function(self) {
  function(range) {
    .savvy_wrap_HArray(.Call(HArray_slice__impl, self, range))
  }
}

HArray_print <- function(self) {
  function() {
  invisible(.Call(HArray_print__impl, self))
  }
}

HArray_eq <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HArray")
.Call(HArray_eq__impl, self, other)
  }
}

HArray_ne <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HArray")
.Call(HArray_ne__impl, self, other)
  }
}

HArray_clone <- function(self) {
  function() {
    .savvy_wrap_HArray(.Call(HArray_clone__impl, self))
  }
}

HArray_collect <- function(self) {
  function() {
  .Call(HArray_collect__impl, self)
  }
}

HArray_dtype <- function(self) {
  function() {
    .savvy_wrap_HDataType(.Call(HArray_dtype__impl, self))
  }
}

HArray_is_shared <- function(self) {
  function() {
  .Call(HArray_is_shared__impl, self)
  }
}

HArray_mem_adress <- function(self) {
  function() {
  .Call(HArray_mem_adress__impl, self)
  }
}



HAudioOp <- new.env(parent = emptyenv())
HAudioOp$nchannels <- function(harray) {
  harray <- .savvy_extract_ptr(harray, "HArray")
.Call(HAudioOp_nchannels__impl, harray)
}

HAudioOp$nframes <- function(harray) {
  harray <- .savvy_extract_ptr(harray, "HArray")
.Call(HAudioOp_nframes__impl, harray)
}

HAudioOp$db_to_amplitude <- function(harray, reference, power) {
  harray <- .savvy_extract_ptr(harray, "HArray")
invisible(.Call(HAudioOp_db_to_amplitude__impl, harray, reference, power))
}

HAudioOp$to_mono <- function(harray) {
  harray <- .savvy_extract_ptr(harray, "HArray")
invisible(.Call(HAudioOp_to_mono__impl, harray))
}


.savvy_wrap_HAudioOp <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr


  class(e) <- "HAudioOp"
  e
}





HAudioSink <- new.env(parent = emptyenv())
HAudioSink$new <- function() {
  .savvy_wrap_HAudioSink(.Call(HAudioSink_new__impl))
}

HAudioSink$audio_output_devices <- function() {
.Call(HAudioSink_audio_output_devices__impl)
}

HAudioSink$audio_default_device <- function() {
.Call(HAudioSink_audio_default_device__impl)
}

HAudioSink$audio_supported_configs <- function() {
.Call(HAudioSink_audio_supported_configs__impl)
}


.savvy_wrap_HAudioSink <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$append_from_harray <- HAudioSink_append_from_harray(ptr)
  e$append_from_file <- HAudioSink_append_from_file(ptr)
  e$play <- HAudioSink_play(ptr)
  e$stop <- HAudioSink_stop(ptr)
  e$pause <- HAudioSink_pause(ptr)
  e$is_paused <- HAudioSink_is_paused(ptr)
  e$volume <- HAudioSink_volume(ptr)
  e$set_volume <- HAudioSink_set_volume(ptr)
  e$speed <- HAudioSink_speed(ptr)
  e$set_speed <- HAudioSink_set_speed(ptr)
  e$sleep_until_end <- HAudioSink_sleep_until_end(ptr)
  e$len <- HAudioSink_len(ptr)
  e$is_empty <- HAudioSink_is_empty(ptr)
  e$clear <- HAudioSink_clear(ptr)
  e$skip_one <- HAudioSink_skip_one(ptr)

  class(e) <- "HAudioSink"
  e
}


HAudioSink_append_from_harray <- function(self) {
  function(harray, sr) {
    harray <- .savvy_extract_ptr(harray, "HArray")
invisible(.Call(HAudioSink_append_from_harray__impl, self, harray, sr))
  }
}

HAudioSink_append_from_file <- function(self) {
  function(fpath) {
  invisible(.Call(HAudioSink_append_from_file__impl, self, fpath))
  }
}

HAudioSink_play <- function(self) {
  function() {
  invisible(.Call(HAudioSink_play__impl, self))
  }
}

HAudioSink_stop <- function(self) {
  function() {
  invisible(.Call(HAudioSink_stop__impl, self))
  }
}

HAudioSink_pause <- function(self) {
  function() {
  invisible(.Call(HAudioSink_pause__impl, self))
  }
}

HAudioSink_is_paused <- function(self) {
  function() {
  .Call(HAudioSink_is_paused__impl, self)
  }
}

HAudioSink_volume <- function(self) {
  function() {
  .Call(HAudioSink_volume__impl, self)
  }
}

HAudioSink_set_volume <- function(self) {
  function(value) {
  invisible(.Call(HAudioSink_set_volume__impl, self, value))
  }
}

HAudioSink_speed <- function(self) {
  function() {
  .Call(HAudioSink_speed__impl, self)
  }
}

HAudioSink_set_speed <- function(self) {
  function(value) {
  invisible(.Call(HAudioSink_set_speed__impl, self, value))
  }
}

HAudioSink_sleep_until_end <- function(self) {
  function() {
  invisible(.Call(HAudioSink_sleep_until_end__impl, self))
  }
}

HAudioSink_len <- function(self) {
  function() {
  .Call(HAudioSink_len__impl, self)
  }
}

HAudioSink_is_empty <- function(self) {
  function() {
  .Call(HAudioSink_is_empty__impl, self)
  }
}

HAudioSink_clear <- function(self) {
  function() {
  invisible(.Call(HAudioSink_clear__impl, self))
  }
}

HAudioSink_skip_one <- function(self) {
  function() {
  invisible(.Call(HAudioSink_skip_one__impl, self))
  }
}



HDataType <- new.env(parent = emptyenv())
HDataType$float32 <- function() {
  .savvy_wrap_HDataType(.Call(HDataType_float32__impl))
}

HDataType$float64 <- function() {
  .savvy_wrap_HDataType(.Call(HDataType_float64__impl))
}

HDataType$complex32 <- function() {
  .savvy_wrap_HDataType(.Call(HDataType_complex32__impl))
}

HDataType$complex64 <- function() {
  .savvy_wrap_HDataType(.Call(HDataType_complex64__impl))
}


.savvy_wrap_HDataType <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$print <- HDataType_print(ptr)
  e$eq <- HDataType_eq(ptr)
  e$ne <- HDataType_ne(ptr)

  class(e) <- "HDataType"
  e
}


HDataType_print <- function(self) {
  function() {
  invisible(.Call(HDataType_print__impl, self))
  }
}

HDataType_eq <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HDataType")
.Call(HDataType_eq__impl, self, other)
  }
}

HDataType_ne <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HDataType")
.Call(HDataType_ne__impl, self, other)
  }
}



HFft <- new.env(parent = emptyenv())
HFft$fft <- function(harray) {
  harray <- .savvy_extract_ptr(harray, "HArray")
  .savvy_wrap_HArray(.Call(HFft_fft__impl, harray))
}

HFft$fft_mut <- function(harray) {
  harray <- .savvy_extract_ptr(harray, "HArray")
invisible(.Call(HFft_fft_mut__impl, harray))
}

HFft$fft_real_mut <- function(harray) {
  harray <- .savvy_extract_ptr(harray, "HArray")
invisible(.Call(HFft_fft_real_mut__impl, harray))
}


.savvy_wrap_HFft <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr


  class(e) <- "HFft"
  e
}





HMetadataType <- new.env(parent = emptyenv())
HMetadataType$all <- function() {
  .savvy_wrap_HMetadataType(.Call(HMetadataType_all__impl))
}

HMetadataType$text <- function() {
  .savvy_wrap_HMetadataType(.Call(HMetadataType_text__impl))
}

HMetadataType$visual <- function() {
  .savvy_wrap_HMetadataType(.Call(HMetadataType_visual__impl))
}


.savvy_wrap_HMetadataType <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$print <- HMetadataType_print(ptr)
  e$eq <- HMetadataType_eq(ptr)
  e$ne <- HMetadataType_ne(ptr)

  class(e) <- "HMetadataType"
  e
}


HMetadataType_print <- function(self) {
  function() {
  invisible(.Call(HMetadataType_print__impl, self))
  }
}

HMetadataType_eq <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HMetadataType")
.Call(HMetadataType_eq__impl, self, other)
  }
}

HMetadataType_ne <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HMetadataType")
.Call(HMetadataType_ne__impl, self, other)
  }
}



HPolynomialDegree <- new.env(parent = emptyenv())
HPolynomialDegree$septic <- function() {
  .savvy_wrap_HPolynomialDegree(.Call(HPolynomialDegree_septic__impl))
}

HPolynomialDegree$quintic <- function() {
  .savvy_wrap_HPolynomialDegree(.Call(HPolynomialDegree_quintic__impl))
}

HPolynomialDegree$cubic <- function() {
  .savvy_wrap_HPolynomialDegree(.Call(HPolynomialDegree_cubic__impl))
}

HPolynomialDegree$linear <- function() {
  .savvy_wrap_HPolynomialDegree(.Call(HPolynomialDegree_linear__impl))
}

HPolynomialDegree$nearest <- function() {
  .savvy_wrap_HPolynomialDegree(.Call(HPolynomialDegree_nearest__impl))
}


.savvy_wrap_HPolynomialDegree <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$print <- HPolynomialDegree_print(ptr)
  e$eq <- HPolynomialDegree_eq(ptr)
  e$ne <- HPolynomialDegree_ne(ptr)

  class(e) <- "HPolynomialDegree"
  e
}


HPolynomialDegree_print <- function(self) {
  function() {
  invisible(.Call(HPolynomialDegree_print__impl, self))
  }
}

HPolynomialDegree_eq <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HPolynomialDegree")
.Call(HPolynomialDegree_eq__impl, self, other)
  }
}

HPolynomialDegree_ne <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HPolynomialDegree")
.Call(HPolynomialDegree_ne__impl, self, other)
  }
}



HResamplerType <- new.env(parent = emptyenv())
HResamplerType$fft_fixed_in <- function() {
  .savvy_wrap_HResamplerType(.Call(HResamplerType_fft_fixed_in__impl))
}

HResamplerType$fft_fixed_in_out <- function() {
  .savvy_wrap_HResamplerType(.Call(HResamplerType_fft_fixed_in_out__impl))
}

HResamplerType$fft_fixed_out <- function() {
  .savvy_wrap_HResamplerType(.Call(HResamplerType_fft_fixed_out__impl))
}

HResamplerType$sinc_fixed_in <- function() {
  .savvy_wrap_HResamplerType(.Call(HResamplerType_sinc_fixed_in__impl))
}

HResamplerType$sinc_fixed_out <- function() {
  .savvy_wrap_HResamplerType(.Call(HResamplerType_sinc_fixed_out__impl))
}

HResamplerType$fast_fixed_in <- function() {
  .savvy_wrap_HResamplerType(.Call(HResamplerType_fast_fixed_in__impl))
}

HResamplerType$fast_fixed_out <- function() {
  .savvy_wrap_HResamplerType(.Call(HResamplerType_fast_fixed_out__impl))
}


.savvy_wrap_HResamplerType <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$print <- HResamplerType_print(ptr)
  e$eq <- HResamplerType_eq(ptr)
  e$ne <- HResamplerType_ne(ptr)

  class(e) <- "HResamplerType"
  e
}


HResamplerType_print <- function(self) {
  function() {
  invisible(.Call(HResamplerType_print__impl, self))
  }
}

HResamplerType_eq <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HResamplerType")
.Call(HResamplerType_eq__impl, self, other)
  }
}

HResamplerType_ne <- function(self) {
  function(other) {
    other <- .savvy_extract_ptr(other, "HResamplerType")
.Call(HResamplerType_ne__impl, self, other)
  }
}



HSincInterpolationParameters <- new.env(parent = emptyenv())
HSincInterpolationParameters$new <- function(sinc_len, f_cutoff, oversampling_factor, interpolation, window) {
  .savvy_wrap_HSincInterpolationParameters(.Call(HSincInterpolationParameters_new__impl, sinc_len, f_cutoff, oversampling_factor, interpolation, window))
}


.savvy_wrap_HSincInterpolationParameters <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr
  e$print <- HSincInterpolationParameters_print(ptr)

  class(e) <- "HSincInterpolationParameters"
  e
}


HSincInterpolationParameters_print <- function(self) {
  function() {
  invisible(.Call(HSincInterpolationParameters_print__impl, self))
  }
}



HWindow <- new.env(parent = emptyenv())
HWindow$barthann <- function(npoints, sym, dtype) {
  dtype <- .savvy_extract_ptr(dtype, "HDataType")
  .savvy_wrap_HArray(.Call(HWindow_barthann__impl, npoints, sym, dtype))
}

HWindow$bartlett <- function(npoints, sym, dtype) {
  dtype <- .savvy_extract_ptr(dtype, "HDataType")
  .savvy_wrap_HArray(.Call(HWindow_bartlett__impl, npoints, sym, dtype))
}

HWindow$blackman <- function(npoints, sym, dtype) {
  dtype <- .savvy_extract_ptr(dtype, "HDataType")
  .savvy_wrap_HArray(.Call(HWindow_blackman__impl, npoints, sym, dtype))
}

HWindow$blackmanharris <- function(npoints, sym, dtype) {
  dtype <- .savvy_extract_ptr(dtype, "HDataType")
  .savvy_wrap_HArray(.Call(HWindow_blackmanharris__impl, npoints, sym, dtype))
}

HWindow$bohman <- function(npoints, sym, dtype) {
  dtype <- .savvy_extract_ptr(dtype, "HDataType")
  .savvy_wrap_HArray(.Call(HWindow_bohman__impl, npoints, sym, dtype))
}

HWindow$boxcar <- function(npoints, dtype) {
  dtype <- .savvy_extract_ptr(dtype, "HDataType")
  .savvy_wrap_HArray(.Call(HWindow_boxcar__impl, npoints, dtype))
}

HWindow$cosine <- function(npoints, sym, dtype) {
  dtype <- .savvy_extract_ptr(dtype, "HDataType")
  .savvy_wrap_HArray(.Call(HWindow_cosine__impl, npoints, sym, dtype))
}

HWindow$hann <- function(npoints, sym, dtype) {
  dtype <- .savvy_extract_ptr(dtype, "HDataType")
  .savvy_wrap_HArray(.Call(HWindow_hann__impl, npoints, sym, dtype))
}


.savvy_wrap_HWindow <- function(ptr) {
  e <- new.env(parent = emptyenv())
  e$.ptr <- ptr


  class(e) <- "HWindow"
  e
}




