use flate2::read::GzDecoder;
use std::{env::var, fs, io, io::prelude::*, io::ErrorKind::AlreadyExists, path::PathBuf};
use tar::Archive;

const FAASM_VENDOR_FOLDER: &str = "vendor/faasm";
const FAASM_RELEASE_BASE_URL: &str = "https://github.com/faasm/faasm/releases/download/v";
const FAASM_VERSION: &str = env!("CARGO_PKG_VERSION");

enum FaasmRelease {
    RuntimeRoot,
    Sysroot,
    Toolchain,
}

impl FaasmRelease {
    fn tar_filename(&self) -> String {
        match self {
            Self::RuntimeRoot => format!("faasm-runtime-root-{}.tar.gz", FAASM_VERSION),
            Self::Sysroot => format!("faasm-sysroot-{}.tar.gz", FAASM_VERSION),
            Self::Toolchain => format!("faasm-toolchain-{}.tar.gz", FAASM_VERSION),
        }
    }

    fn filename(&self) -> String {
        match self {
            Self::RuntimeRoot => format!("runtime-root-{}", FAASM_VERSION),
            Self::Sysroot => format!("sysroot-{}", FAASM_VERSION),
            Self::Toolchain => format!("toolchain-{}", FAASM_VERSION),
        }
    }

    fn file_path(&self) -> PathBuf {
        PathBuf::from(FAASM_VENDOR_FOLDER).join(self.filename())
    }

    fn url(&self) -> String {
        format!(
            "{}{}/{}",
            FAASM_RELEASE_BASE_URL,
            FAASM_VERSION,
            self.tar_filename()
        )
    }

    fn download_tar(&self) -> Result<(), Box<dyn std::error::Error>> {
        let out_path = self.file_path();
        if !out_path.exists() {
            // Downloads file
            let response = reqwest::blocking::get(&self.url())?;

            // Extacts tar.gz
            let tar = GzDecoder::new(response);
            let mut archive = Archive::new(tar);
            archive.unpack(out_path)?;
        }

        // We're done
        Ok(())
    }
}

// Generates a wrapper header pointing to the FaasmRelease sysroot
fn generate_header(root_dir: &str, sysroot_dir: &str) -> io::Result<String> {
    let header_file = format!("{}/wrapper-{}.h", root_dir, FAASM_VERSION);
    match fs::File::create(&header_file) {
        Ok(mut header_handler) => {
            let header_content = format!(
                "\
                #include \"{}/llvm-sysroot/include/faasm/host_interface.h\"\n\
                #include \"{}/llvm-sysroot/include/faasm/rfaasm.h\"\
            ",
                sysroot_dir, sysroot_dir
            );
            header_handler.write_all(&header_content.into_bytes())?;
            Ok(header_file)
        }
        Err(err) if err.kind() == AlreadyExists => Ok(header_file),
        Err(err) => Err(err),
    }
}

// TODO - implement bindings using bindgen. I spent several hours with it but
// can still only generate blank outputs using the library
fn generate_bindings(_wrapper: &str, output_file: &PathBuf) -> io::Result<()> {
    // Warn that dynamic binding generation is not implemented and needs to be ran
    // a-priori of build from the cmd line with bindgen.
    println!("cargo:warning=Using manually generated bindings");

    // Bindings file maintained manually in this repo
    let vendor_folder = PathBuf::from(FAASM_VENDOR_FOLDER);
    // Output file where we want to link things
    let manual_gen_file = vendor_folder.join("bindings.rs");

    // Location that can be included in the libray code
    fs::copy(manual_gen_file, output_file).unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Location of the binding file that will be included in the library
    let binding_file = PathBuf::from(var("OUT_DIR").unwrap()).join("bindings.rs");

    let target = var("TARGET").unwrap();
    if target == "wasm32-unknown-unknown" {
        // Determine if we want the dev environment or a simple installation
        let (library_path, header) = match var("FAASM_SYS_DEV") {
            Ok(_) => (
                "/usr/local/faasm/llvm-sysroot/lib".to_string(),
                format!("{}/wrapper.h", FAASM_VENDOR_FOLDER),
            ),
            Err(_) => {
                // Download the Sysroot if it doesn't exists
                FaasmRelease::Sysroot.download_tar()?;
                let library_path = std::env::current_dir()?
                    .join(FaasmRelease::Sysroot.file_path())
                    .join("llvm-sysroot/lib");

                // Paths can be invalid UTF-8 but we
                let library_dir = library_path.into_os_string().into_string().unwrap();

                // Generate a wrapper based on the library location
                let header = generate_header(FAASM_VENDOR_FOLDER, &FaasmRelease::Sysroot.filename())?;

                (library_dir, header)
            }
        };

        // Rerun if the wrapper is changed (more relevant for dev mode)
        println!("cargo:rerun-if-changed={}", header);

        // TODO - this only copies the manually generated bindings
        generate_bindings(&header, &binding_file)?;

        // Link libs from Faasm Sysroot
        println!("cargo:rustc-link-search={}", library_path);

        // Add libraries
        println!("cargo:rustc-link-lib=static=faasm");
    } else {
        unimplemented!("Link native Faasm libraries");
    }

    Ok(())
}
