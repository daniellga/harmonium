.onLoad <- function(libname, pkgname){
  # Assure only one instance of HDataType's external pointer is created for each enum variant.
  HDataType$float32 = HDataType$float32()
  HDataType$float64 = HDataType$float64()
  HDataType$complex32 = HDataType$complex32()
  HDataType$complex64 = HDataType$complex64()
  lockEnvironment(HDataType, bindings = TRUE)

  # Assure only one instance of HMetadataType's external pointer is created for each enum variant.
  HMetadataType$all = HMetadataType$all()
  HMetadataType$text = HMetadataType$text()
  HMetadataType$visual = HMetadataType$visual()
  lockEnvironment(HMetadataType, bindings = TRUE)

  # Assure only one instance of HResamplerType's external pointer is created for each enum variant.
  HResamplerType$fft_fixed_in = HResamplerType$fft_fixed_in()
  HResamplerType$fft_fixed_in_out = HResamplerType$fft_fixed_in_out()
  HResamplerType$fft_fixed_out = HResamplerType$fft_fixed_out()
  HResamplerType$sinc_fixed_in = HResamplerType$sinc_fixed_in()
  HResamplerType$sinc_fixed_out = HResamplerType$sinc_fixed_out()
  lockEnvironment(HResamplerType, bindings = TRUE)

  # Lazy load hconfig. This is needed because HConfig uses rust functions, which are loaded after the scripts are run.
  env <- parent.env(environment())
  HConfig = generate_hconfig()
  assign("HConfig", HConfig, envir = env)
  lockEnvironment(HConfig, bindings = TRUE)
}

.onAttach <- function(libname, pkgname) {
  packageStartupMessage(paste0("harmonium ",packageVersion("harmonium"),", see harmonium::hdocs() for documentation."))
}
