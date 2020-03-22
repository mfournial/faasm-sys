use faasm_sys::*;

// Replace println macro in Wasm
#[cfg(target_arch = "wasm32")]
use faasm_sys::println;

// Test OpenMP linkage since it holds the print function for now
fn main() {
    println!("Ignore OpenMP tests for native code");
}
