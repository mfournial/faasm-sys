#[cfg(target_arch = "wasm32")]
use faasm_sys::*;


// Test OpenMP linkage since it holds the print function for now
#[cfg(target_arch = "wasm32")]
fn main() {
    assert_eq!(unsafe { omp_get_thread_num() }, 0);
    assert_eq!(unsafe { omp_get_num_threads() }, 1);
}


#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Ignore OpenMP tests for native code");
}
