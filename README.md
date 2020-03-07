# Faasm-sys

Rust bindings for Faasm. At the moment an experimental POC.

## Run 

### Requirements

* Faasm libs at traditional locations (`/usr/local/faasm/llvm-sysroot/lib/`)
* `codegen_func` and `simple_runner` compiled
* Recent Rust compiler installed via `rustup`, prefer nightly.
* The Wasm target of interest Rust libraries:
```
rustup target add wasm32-unknown-unknown [--toolchain nightly]
```
There is 2 others targets for Wasm (`wasm32-wasi`, `wasm32-unknown-emscripten) that might be useful
in the future.

### Compile and run

Building is done through cargo, running for now can be done after the codegen by the simple runner.
```
cargo build --target wasm32-unknown-unknown
NAME=simple_link
INSTALL_DIR=/usr/local/code/faasm/wasm/rust/$NAME
mkdir -p $INSTALL_DIR
cp target/wasm32-unknown-unknown/debug/faasm-sys.wasm $INSTALL_DIR/function.wasm
codegen rust $NAME
simple_runner rust $NAME
```

Although panics are not pretty when they happen in Wasm, you can use the `assert_eq` macro for
example to check the value of an expression until print is supported.
```rust
assert_eq!(omp::get_num_threads(), 1);
```
