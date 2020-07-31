// Atlas holds up the world with low level functionality and types
// - crash provides super basic crash reporting (open file with log)
// - paths provides current exe location (for resource loading)
// - BoxResults types
mod crash;
#[cfg(debug_assertions)]
pub use crash::on_crash;

mod paths;
pub use paths::get_exe_folder;

mod point;
pub use point::{Point, SizedPoint};

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait EasyPath {
    fn stringify(&self) -> &str;
    fn stringify_owned(&self) -> String;
}

impl EasyPath for std::path::Path {
    fn stringify(&self) -> &str {
        self.to_str().unwrap()
    }
    fn stringify_owned(&self) -> String {
        self.stringify().to_string()
    }
}

impl EasyPath for std::path::PathBuf {
    fn stringify(&self) -> &str {
        self.to_str().unwrap()
    }
    fn stringify_owned(&self) -> String {
        self.stringify().to_string()
    }
}
