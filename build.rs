use std::env;
use std::fs;
use std::path::Path;

fn print(message: &str) {
    println!("{}", format!("cargo:warning={}", message));
}

fn copy_all_with_extension(src: &str, dest: &str, extension: &str) -> Result<(), std::io::Error> {
    print(src);
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if let Some(file_extension) = path.extension() {
                if file_extension == extension {
                    let dest_file = Path::new(dest).join("..").join("..").join("..").join(file_name);
                    //println!("{}", format!("cargo:rerun-if-changed={}", path.to_str().unwrap()));

                    if !dest_file.exists() {
                        // Joys, no way to do this easily: https://github.com/rust-lang/cargo/issues/5305
                        fs::copy(path, dest_file)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let platform = env::var("CARGO_CFG_TARGET_OS").expect("No Target OS?");
    match platform.as_str() {
        "windows" => {
            let lib_dir = format!("{}\\lib\\win", env!("CARGO_MANIFEST_DIR"));

            let out_dir = env::var("OUT_DIR").unwrap();
            copy_all_with_extension(&lib_dir, &out_dir, "dll").expect("Unable to copy native libraries");

            println!("{}", format!("cargo:rustc-link-search={}", lib_dir));
        }
        o => panic!("unknown target os {:?}!", o),
    }
}
