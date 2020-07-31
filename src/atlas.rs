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

mod easy_path;
pub use easy_path::EasyPath;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
