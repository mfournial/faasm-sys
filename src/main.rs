use faasm_sys::*;

// ---------------------------
// Simple test function
// ---------------------------

// Replace println macro in Wasm
#[cfg(target_arch = "wasm32")]
use faasm_sys::println;

fn main() {
    println!("faasm-sys works");
}
