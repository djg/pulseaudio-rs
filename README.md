# pulseaudio-rs
PulseAudio wrappers used by Cubeb's Rust PulseAudio backend.

This version supports the Rust version of
[cubeb](https://github.com/kinetiknz/cubeb) PulseAudio backend.  If
you're looking for a more general wrapping for PulseAudio see
[libpulse-binding](https://crates.io/crates/libpulse-binding)

To enable dynamic loading of the system `libpulse.so`, set Cargo
feature `dlopen`.
