# Faasm-sys

Rust bindings for [Faasm](https://github.com/lsds/faasm). At the moment an experimental POC.

## Run 

### Requirements

* Faasm libs at traditional locations (`/usr/local/faasm/llvm-sysroot/lib/`)
* `codegen_func` and `simple_runner` compiled
* Recent Rust compiler installed via `rustup`, prefer nightly.
* The Wasm target of interest Rust libraries:
```
rustup target add wasm32-unknown-unknown [--toolchain nightly]
```
There is 2 others targets for Wasm (`wasm32-wasi`, `wasm32-unknown-emscripten`) that might be useful
in the future.

### Compile and run

Building is done through cargo, running can be done, after the codegen, by the simple runner.
```bash
cargo build --target wasm32-unknown-unknown
NAME=simple_link
INSTALL_DIR=/usr/local/code/faasm/wasm/rust/$NAME
mkdir -p $INSTALL_DIR
cp target/wasm32-unknown-unknown/debug/faasm-sys.wasm $INSTALL_DIR/function.wasm
codegen rust $NAME
simple_runner rust $NAME
```

Although panics do not look pretty when they happen in Wasm, `main` uses the `assert_eq` macro for
example to check the value of an expression since print is not supported.
```rust
assert_eq!(omp::get_num_threads(), 1);
```
