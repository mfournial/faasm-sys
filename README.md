# Faasm-sys

Rust bindings for [Faasm](https://github.com/lsds/Faasm). 

## Build

First we need the latest Faasm sysroot, this can be downloaded with:

```bash
./bin/download.sh
```

You can then run the build with:

```bash
cargo build --target wasm32-unknown-unknown
```

## Running the test function

To build and run the test function you need a Faasm 
[local development](https://github.com/lsds/Faasm/blob/master/docs/local_dev.md) set up,
and can use the script:

```bash
./bin/test.sh
```

Note that when used as a submodule of [rust-faasm](https://github.com/mfournial/rust-faasm), 
the `target` directory is directed one level up.

## `assert_eq` 

Although panics do not look pretty when they happen in Wasm, `main` uses the `assert_eq` macro for
example to check the value of an expression.

```rust
assert_eq!(omp::get_num_threads(), 1);
```
