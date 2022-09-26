use std::env;
use std::path::Path;

#[allow(dead_code)]
fn print<S: Into<String>>(message: S) {
    println!("cargo:warning={}", message.into());
}

fn main() {
    let platform = env::var("CARGO_CFG_TARGET_OS").expect("No Target OS?");
    if let "windows" = platform.as_str() {
        let plat_dur = Path::new(env!("CARGO_MANIFEST_DIR")).join("platform").join("win");

        let mut res = winres::WindowsResource::new();
        res.set_manifest_file(plat_dur.join("arena_gunpowder_and_sorcery.manifest").to_str().unwrap());
        res.set_icon(plat_dur.join("arena_gunpowder_and_sorcery.ico").to_str().unwrap());

        res.compile().expect("Unable to run windows resource compiler");
    }
}
