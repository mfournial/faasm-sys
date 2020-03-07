fn main() {

    // Faasm default lib location
    println!("cargo:rustc-link-search=/usr/local/faasm/llvm-sysroot/lib");
    println!("cargo:rustc-link-lib=static=faasm");
    println!("cargo:rustc-link-lib=static=faasmp");
}
