use std::{env::var, fs::copy, path::PathBuf};

const FAASM_INCLUDE_WRAPPER: &str = "wrapper.h";
const FAASM_VENDOR_FOLDER: &str = "vendor/faasm";
const FAASM_VENDOR_LIBS_FOLDER: &str = "vendor/faasm-libs";

fn main() {
    let header = format!("{}/{}", FAASM_VENDOR_FOLDER, FAASM_INCLUDE_WRAPPER);

    println!("cargo:rerun-if-changed={}", header);

    let target = var("TARGET").unwrap();
    if target == "wasm32-unknown-unknown" {
        // Link libs from Faasm sysroot
        println!("cargo:rustc-link-search={}/llvm-sysroot/lib", FAASM_VENDOR_LIBS_FOLDER);

        // Add libraries
        println!("cargo:rustc-link-lib=static=faasm");
    } else {
        unimplemented!("Link native Faasm libraries");
    }

    // Included bindings were originally generated with the bindgen command line
    // This would be a more streamlined process
    let vendor_folder = PathBuf::from(FAASM_VENDOR_FOLDER);
    let manual_gen_file = vendor_folder.join("bindings.rs");

    // Location that can be included in the libray code
    let binding_file = PathBuf::from(var("OUT_DIR").unwrap()).join("bindings.rs");

    if manual_gen_file.exists() {
        copy(manual_gen_file, binding_file).unwrap();
    } else {
        unimplemented!("FIXME: Implement dynamic bindings generation");
        /*
        std::env::set_var("LLVM_CONFIG_PATH", "/usr/lib/llvm-6.0/bin/llvm-config");
        let bindings = bindgen::Builder::default()
            .header(header)
            // Only generate if wrapper was modified
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(binding_file)
            .expect("Couldn't write bindings!");
        */
    }
}
