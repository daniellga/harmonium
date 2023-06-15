## Harmonium: Audio analysis and IO in R

Harmonium is an audio interface inspired by Python's [librosa](https://github.com/librosa/librosa).

- Cross-platform audio IO
- Decode and get audio metadata
- Asynchronous and Synchronous resampling
- Arrow interface with zero copy integration

To learn more, read the [documentation](https://daniellga.github.io/harmonium/).

## R Setup

### Windows

Download [Rustup](https://www.rust-lang.org/tools/install).

Windows users have to add both gnu targets below:
`rustup target add x86_64-pc-windows-gnu`
`rustup target add i686-pc-windows-gnu`


In R, install harmonium:
`remotes::install_github("daniellga/harmonium/r-harmonium")`

### Linux

Download [Rustup](https://www.rust-lang.org/tools/install).

On Linux, the ALSA development files are required. These are provided as part of the libasound2-dev package on Debian and Ubuntu distributions and alsa-lib-devel on Fedora.

In R, install harmonium:
`remotes::install_github("daniellga/harmonium/r-harmonium")`

### Mac

Download [Rustup](https://www.rust-lang.org/tools/install).

In R, install harmonium:
`remotes::install_github("daniellga/harmonium/r-harmonium")`
