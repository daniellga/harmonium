## Harmonium: Audio analysis and IO in R

Harmonium is an audio interface inspired by Python's [librosa](https://github.com/librosa/librosa).

- Cross-platform audio IO
- Decode audio
- Retrieve audio metadata
- Asynchronous and Synchronous resampling
- FFT

To learn more, read the [documentation](https://daniellga.github.io/harmonium/).

## R Setup

### Windows

Install [Rustup](https://www.rust-lang.org/tools/install).
Install [Rtools](https://cran.r-project.org/bin/windows/Rtools/).

Add the following gnu target: `rustup target add x86_64-pc-windows-gnu`.

In R, install harmonium:
`remotes::install_github("daniellga/harmonium/r-harmonium")`

### Linux

Install [Rustup](https://www.rust-lang.org/tools/install).

The ALSA development files are required. These are provided as part of the libasound2-dev package on Debian and Ubuntu distributions and alsa-lib-devel on Fedora.

In R, install harmonium:
`remotes::install_github("daniellga/harmonium/r-harmonium")`

### Mac

Install [Rustup](https://www.rust-lang.org/tools/install).

In R, install harmonium:
`remotes::install_github("daniellga/harmonium/r-harmonium")`
