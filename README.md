# dsp4sdr-rs

An SDR DSP library for Rust.

Currently includes:
* FIR filtering/decimation
* CIC filtering/decimation
* Downconversion by Fs/4

To build the tests and examples, `cargo test --release` and check 
`target/release/examples/`. Currently the examples are really more like 
benchmarks, but those aren't shipped in stable Rust at the moment.

An emphasis is placed on performance. Everything is implemented in a reasonably 
fast fashion. The filters can operate on tens or hundreds of millions of 
samples per second, even on a single core, for reasonable filter sizes and data 
widths.

Use alongside airspy-rs, rtlsdr-rs and hackrf-rs to act as SDR sources and 
sinks.
