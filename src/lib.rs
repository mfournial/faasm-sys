pub mod omp {
    extern "C" {
        fn omp_get_num_threads() -> i32;
    }

    pub fn get_num_threads() -> i32 {
        unsafe { omp_get_num_threads() }
    }
}
