use faasm_sys::{omp};

fn main() -> Result<(), i32> {
    Err(omp::get_num_threads())
}
