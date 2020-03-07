use faasm_sys::{omp};

fn main() {
    assert_eq!(omp::get_thread_num(), 0);
    assert_eq!(omp::get_num_threads(), 1);
}
