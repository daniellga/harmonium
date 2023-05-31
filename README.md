## Harmonium: Audio analysis and IO in R

Harmonium is an audio interface inspired by Python's [librosa](https://github.com/librosa/librosa).

- Cross-platform audio IO
- Decode and get audio metadata
- Asynchronous and Synchronous resampling
- Arrow interface with zero copy integration

To learn more, read the [documentation](https://daniellga.github.io/harmonium/).

## Setup

### R

#### Windows

Windows users have add gnu target:
`rustup target add x86_64-pc-windows-gnu`

#### Linux

On Linux, the ALSA development files are required. These are provided as part of the libasound2-dev package on Debian and Ubuntu distributions and alsa-lib-devel on Fedora.
