use std::env;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
fn print<S: Into<String>>(message: S) {
    println!("{}", format!("cargo:warning={}", message.into()));
}

fn copy_all_with_extension(src: &Path, dest: &str, extension: &str) -> Result<(), std::io::Error> {
    let mut created_folder = false;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            copy_all_with_extension(&path, Path::new(&dest).join(path.file_name().unwrap()).to_str().unwrap(), extension)?;
        } else if let Some(file_name) = path.file_name() {
            if let Some(file_extension) = path.extension() {
                if file_extension.to_str().unwrap().to_ascii_lowercase() == extension || extension == "*" {
                    let dest_file = Path::new(&dest).join(file_name);
                    //println!("{}", format!("cargo:rerun-if-changed={}", path.to_str().unwrap()));

                    if !dest_file.exists() {
                        if !created_folder {
                            //print(format!("Creating {}", dest));
                            fs::create_dir_all(dest).expect("Unable to create output dir");
                            created_folder = true;
                        }
                        // Joys, no way to do this easily: https://github.com/rust-lang/cargo/issues/5305
                        //print(format!("Copy to {}", dest_file.to_str().unwrap()));
                        fs::copy(path, dest_file)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // ../../.. to get to out of the crate specific folder
    let dest_dir = Path::new(&out_dir).join("..").join("..").join("..");

    let platform = env::var("CARGO_CFG_TARGET_OS").expect("No Target OS?");
    if let "windows" = platform.as_str() {
        let lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("lib").join("win");

        copy_all_with_extension(&lib_dir, &dest_dir.to_str().unwrap(), "dll").expect("Unable to copy native libraries");

        println!("{}", format!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap()));
    }

    for (folder, extension) in &[("images", "png"), ("maps", "*"), ("fonts", "*"), ("icons", "png")] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("ArenaGS-Data").join(folder);
        if path.exists() {
            copy_all_with_extension(&path, &dest_dir.join(folder).to_str().unwrap(), extension)
                .unwrap_or_else(|_| panic!(format!("Unable to copy {}", folder)));
        }
    }
}
