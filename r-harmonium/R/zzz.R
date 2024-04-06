.onLoad <- function(libname, pkgname){
  # Assure only one instance of HDataType's external pointer is created for each enum variant.
  lockEnvironment(HDataType, bindings = TRUE)

  # Assure only one instance of HMetadataType's external pointer is created for each enum variant.
  # HMetadataType$all = HMetadataType$all()
  # HMetadataType$text = HMetadataType$text()
  # HMetadataType$visual = HMetadataType$visual()
  lockEnvironment(HMetadataType, bindings = TRUE)

  # Assure only one instance of HResamplerType's external pointer is created for each enum variant.
  # HResamplerType$fft_fixed_in = HResamplerType$fft_fixed_in()
  # HResamplerType$fft_fixed_in_out = HResamplerType$fft_fixed_in_out()
  # HResamplerType$fft_fixed_out = HResamplerType$fft_fixed_out()
  # HResamplerType$sinc_fixed_in = HResamplerType$sinc_fixed_in()
  # HResamplerType$sinc_fixed_out = HResamplerType$sinc_fixed_out()
  # HResamplerType$fast_fixed_in = HResamplerType$fast_fixed_in()
  # HResamplerType$fast_fixed_out = HResamplerType$fast_fixed_out()
  lockEnvironment(HResamplerType, bindings = TRUE)

  # Assure only one instance of HPolynomialDegree's external pointer is created for each enum variant.
  # HPolynomialDegree$septic = HPolynomialDegree$septic()
  # HPolynomialDegree$quintic = HPolynomialDegree$quintic()
  # HPolynomialDegree$cubic = HPolynomialDegree$cubic()
  # HPolynomialDegree$linear = HPolynomialDegree$linear()
  # HPolynomialDegree$nearest = HPolynomialDegree$nearest()
  lockEnvironment(HPolynomialDegree, bindings = TRUE)
  
  lockEnvironment(HArray, bindings = TRUE)
  lockEnvironment(HAudioSink, bindings = TRUE)
  lockEnvironment(HWindow, bindings = TRUE)
  lockEnvironment(HFile, bindings = TRUE)
  lockEnvironment(HResampler, bindings = TRUE)
  lockEnvironment(HFft, bindings = TRUE)
  lockEnvironment(HDecoderStream, bindings = TRUE)
}

.onAttach <- function(libname, pkgname) {
  packageStartupMessage(paste0("harmonium ",packageVersion("harmonium"),", see harmonium::hdocs() for documentation."))
}
