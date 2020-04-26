use std::fs;
use std::panic;

#[cfg(target_os = "windows")]
fn open_url(url: &str) -> bool {
    if let Ok(mut child) = std::process::Command::new("cmd.exe").arg("/C").arg("code").arg("").arg(&url).spawn() {
        std::thread::sleep(std::time::Duration::new(1, 0));
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}

pub fn on_crash(panic_info: &panic::PanicInfo) {
    let mut debug_spew = String::new();
    if let Some(location) = panic_info.location() {
        debug_spew.push_str(&format!("{} Line: {}\n", location.file(), location.line())[..]);
    }
    let payload = panic_info.payload();
    if let Some(s) = payload.downcast_ref::<&str>() {
        debug_spew.push_str(s);
    } else if let Some(s) = payload.downcast_ref::<String>() {
        debug_spew.push_str(s);
    }

    let _ = fs::write("debug.txt", debug_spew);

    #[cfg(target_os = "windows")]
    open_url("debug.txt");
}
