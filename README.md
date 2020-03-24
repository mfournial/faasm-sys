[![!travis-ci](https://travis-ci.com/mfournial/faasm-sys.svg?branch=master)](https://travis-ci.com/github/mfournial/faasm-sys/)
# Faasm-sys

Rust bindings for [Faasm](https://github.com/lsds/Faasm). 

## Build

### Normal mode

Running the following will:
1. Download faasm libs and store them in `vendor`
2. Include the `bindings.rs` file in `vendor` in the library
3. Build the Wasm library

```bash
cargo build --target wasm32-unknown-unknown
```

### Local development mode

If you set the `FAASM_SYS_DEV` environment variable before building, then the build script will fetch libraries according to the [Faasm local development](https://github.com/lsds/Faasm/blob/master/docs/local_dev.md) defaults. You can then run the test script as well:

```bash
env FAASM_SYS_DEV=1 cargo build --target wasm32-unknown-unknown
./bin/test.sh
```

Note that when used as a submodule of [rust-faasm](https://github.com/mfournial/rust-faasm), 
the `target` directory is directed one level up.
