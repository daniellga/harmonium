t = seq(0, 1, length.out = 44100+1)[-(44100+1)] # 44100 hz sr with 1 sec
wave64 = sin(1000*2*pi*t) # 1000 hz sin wave 
wave64 = array(c(wave64, wave64), c(length(wave64), 2))

# analyze results

blackman_harris <- function(npoints) {
  x <- seq(0, length.out = npoints)
  
  0.35875 - 0.48829*cos(2*pi*x/npoints) + 0.14128*cos(4*pi*x/npoints) - 0.01168*cos(6*pi*x/npoints)
}
blackman_harris(3)

plot_spec <- function(indata, window = TRUE) {
  wf = indata$data
  fs = indata$sr 
  print(sum(wf))
  npoints = length(wf)
  divfact = npoints/2
  if(window) {
    wind = blackman_harris(npoints)
    wf = wf*wind*wind 
    divfact = sum(wind)/2
  }
  print(npoints)
  t = seq(0, npoints/fs, length.out = npoints+1)[-(npoints+1)]
  f = seq(0, fs/2, length.out = floor(npoints/2))
  valfft = stats::fft(wf)
  cut = valfft[1:floor(npoints/2)]
  ampl = 20*log10(abs(cut)/divfact)
  phase = (180/pi)*Arg(cut)
  
  dev.new()
  plot(f, ampl)
  dev.new()
  plot(wf)
  dev.new()
  plot(diff(wf, differences = 5))
}

indata = list(data = wave64[, 1], sr = 44100)
plot_spec(indata)