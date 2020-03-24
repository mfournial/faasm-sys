#!/bin/bash

set -e

NAME=simple_link
INSTALL_DIR=/usr/local/code/faasm/wasm/rust/$NAME

mkdir -p $INSTALL_DIR

# Depending on whether we are in the Rust-Faasm workspace or not
if [[ -d target ]]; then
    cp target/wasm32-unknown-unknown/debug/faasm-sys.wasm $INSTALL_DIR/function.wasm
else
    cp ../target/wasm32-unknown-unknown/debug/faasm-sys.wasm $INSTALL_DIR/function.wasm
fi

# Faasm toolchain as usual, could use inv too
codegen_func rust $NAME
simple_runner rust $NAME
