// include automatically generated bindings from C include files
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// OpenMP bindings, manually written for initial testing and for print function
extern "C" {

    // Printing hack
    #[cfg(target_arch = "wasm32")]
    pub fn __println(line: *const i8);

    #[cfg(target_arch = "wasm32")]
    pub fn omp_get_num_threads() -> i32;
    #[cfg(target_arch = "wasm32")]
    pub fn omp_get_thread_num() -> i32;
}


// Replaces Rust's original println when imported, works with Wasm
#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! println {
    () => ({
        unsafe { __println(r"".as_ptr() as *const i8); }
    });
    ($($args:tt)*) => ({
        let s = format!($($args)*);
        let mut byte_arr = Vec::from(s.as_bytes());
        byte_arr.push('\0' as u8);
        unsafe { __println(std::ffi::CStr::from_bytes_with_nul(&byte_arr).unwrap().as_ptr() as *const i8); }
    })
}
