.onLoad <- function(libname, pkgname){
  lockEnvironment(HDataType, bindings = TRUE)
  lockEnvironment(HMetadataType, bindings = TRUE)
  lockEnvironment(HResamplerType, bindings = TRUE)
  lockEnvironment(HWindowType, bindings = TRUE)
  lockEnvironment(HInterpolationType, bindings = TRUE)
  lockEnvironment(HPolynomialDegree, bindings = TRUE)
  lockEnvironment(HArray, bindings = TRUE)
  lockEnvironment(HArrayAudio, bindings = TRUE)
  lockEnvironment(HAudioSink, bindings = TRUE)
  lockEnvironment(HWindow, bindings = TRUE)
  lockEnvironment(HFile, bindings = TRUE)
  lockEnvironment(HResampler, bindings = TRUE)
  lockEnvironment(HFft, bindings = TRUE)
  lockEnvironment(HStft, bindings = TRUE)
  lockEnvironment(HDecoderStream, bindings = TRUE)
}

.onAttach <- function(libname, pkgname) {
  packageStartupMessage(paste0("harmonium ",packageVersion("harmonium"),". Use harmonium::hdocs() for documentation."))
}
