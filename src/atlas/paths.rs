use std::path::MAIN_SEPARATOR;

pub fn get_exe_folder() -> String {
    let exe = std::env::current_exe().unwrap();
    let exe_path = exe.to_str().unwrap();
    let mut bits: Vec<&str> = exe_path.split(MAIN_SEPARATOR).collect();
    bits.pop();
    bits.join(&MAIN_SEPARATOR.to_string())
}
