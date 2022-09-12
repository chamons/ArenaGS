use std::env;
use std::path::Path;

#[allow(dead_code)]
fn print<S: Into<String>>(message: S) {
    println!("cargo:warning={}", message.into());
}

fn main() {
    let platform = env::var("CARGO_CFG_TARGET_OS").expect("No Target OS?");
    if let "windows" = platform.as_str() {
        let lib_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("lib")
            .join("win");

        let mut res = winres::WindowsResource::new();
        res.set_manifest_file(
            lib_dir
                .join("arena_gunpowder_and_sorcery.manifest")
                .to_str()
                .unwrap(),
        );
        res.set_icon(
            lib_dir
                .join("arena_gunpowder_and_sorcery.ico")
                .to_str()
                .unwrap(),
        );

        res.compile()
            .expect("Unable to run windows resource compiler");
    }
}
