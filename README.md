# lqr
C2RUST of liblqr

## Building
This package is meant to be built with a nightly version of rust (`1.91.0` - `1.92.X` so basically anything after this was merged https://github.com/rust-lang/rust/pull/144192) 
The `rust-toolchain` file is configured so that everything should just build using nightly, but if things fallback to mainline for whatever reason you should just be able to add the `+nightly` option to fix that (Eg. `cargo build` -> `cargo +nightly build)
