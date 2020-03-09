# Faasm-sys

Rust bindings for [Faasm](https://github.com/lsds/faasm). At the moment an experimental POC.

## Run 

### Requirements

* Faasm libs at traditional locations (`/usr/local/faasm/llvm-sysroot/lib/`)
* `codegen_func` and `simple_runner` compiled on Faasm branch `mfournial:add-rust` and in the `PATH`.
* Recent Rust compiler installed via `rustup`, prefer _nightly_.
* The Wasm target of interest Rust libraries installed:
```
rustup target add wasm32-unknown-unknown [--toolchain nightly]
```
There is 2 others targets for Wasm (`wasm32-wasi`, `wasm32-unknown-emscripten`) that might be useful
in the future.

### Compile and run

Building the Wasm binary is done with cargo.

Cargo's build directory is called `target`. :warning: if this code is used as a submodule in the
[rust-faasm](https://github.com/mfournial/rust-faasm) workspace, then the `target` directory is
directed one level up, which

```bash
cargo build --target wasm32-unknown-unknown
NAME=simple_link
INSTALL_DIR=/usr/local/code/faasm/wasm/rust/$NAME
mkdir -p $INSTALL_DIR

# Depending on whether we are in the Rust-Faasm workspace or not
if [ -d target ]
then
    cp target/wasm32-unknown-unknown/debug/faasm-sys.wasm $INSTALL_DIR/function.wasm
else
    cp ../target/wasm32-unknown-unknown/debug/faasm-sys.wasm $INSTALL_DIR/function.wasm
fi

# Faasm toolchain as usual, could use inv too
codegen_func rust $NAME
simple_runner rust $NAME
```

Although panics do not look pretty when they happen in Wasm, `main` uses the `assert_eq` macro for
example to check the value of an expression.
```rust
assert_eq!(omp::get_num_threads(), 1);
```
