use faasm_sys::*;

// Replace println macro in Wasm
#[cfg(target_arch = "wasm32")]
use faasm_sys::println;

fn main() {
    println!("Hello from Faasm");
}
