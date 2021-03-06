use std::env;
use std::fs;
use std::path::Path;

// "Share" code with the actual project
#[path = "src/atlas/paths.rs"]
mod easy_path;
use easy_path::EasyPath;

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
            copy_all_with_extension(&path, Path::new(&dest).join(path.file_name().unwrap()).stringify(), extension)?;
        } else if let Some(file_name) = path.file_name() {
            if let Some(file_extension) = path.extension() {
                if file_extension.stringify().to_ascii_lowercase() == extension || extension == "*" {
                    let dest_file = Path::new(&dest).join(file_name);
                    //println!("{}", format!("cargo:rerun-if-changed={}", path.stringify()));

                    if !dest_file.exists() {
                        if !created_folder {
                            //print(format!("Creating {}", dest));
                            fs::create_dir_all(dest).expect("Unable to create output dir");
                            created_folder = true;
                        }
                        // Joys, no way to do this easily: https://github.com/rust-lang/cargo/issues/5305
                        //print(format!("Copy to {}", dest_file.stringify()));
                        fs::copy(path, dest_file)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").expect("No OUT_DIR set?");
    // ../../.. to get to out of the crate specific folder
    let dest_dir = Path::new(&out_dir).join("..").join("..").join("..");
    let dest_test_dir = Path::new(&out_dir).join("..").join("..").join("..").join("deps");

    let platform = env::var("CARGO_CFG_TARGET_OS").expect("No Target OS?");
    if let "windows" = platform.as_str() {
        let lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("lib").join("win");

        copy_all_with_extension(&lib_dir, &dest_dir.stringify(), "dll").expect("Unable to copy native libraries");

        // Thanks https://github.com/rust-lang/cargo/issues/4044
        copy_all_with_extension(&lib_dir, &dest_test_dir.stringify(), "dll").expect("Unable to copy native libraries");

        println!("{}", format!("cargo:rustc-link-search={}", lib_dir.stringify()));

        let mut res = winres::WindowsResource::new();
        res.set_manifest_file(lib_dir.join("arena_gunpowder_and_sorcery.manifest").stringify());
        res.set_icon(lib_dir.join("arena_gunpowder_and_sorcery.ico").stringify());

        res.compile().expect("Unable to run windows resource compiler");
    }

    for (folder, extension) in &[("images", "png"), ("maps", "*"), ("fonts", "*"), ("icons", "png"), ("ui", "png")] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("ArenaGS-Data").join(folder);
        if path.exists() {
            copy_all_with_extension(&path, &dest_dir.join(folder).stringify(), extension).unwrap_or_else(|_| panic!(format!("Unable to copy {}", folder)));
        }
    }
}
