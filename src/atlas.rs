// Atlas holds up the world with low level functionality and types
// - crash provides super basic crash reporting (open file with log)
// - paths provides current exe location (for resource loading)
// - BoxResults types
mod crash;
mod paths;

pub use crash::on_crash;
pub use paths::get_exe_folder;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait Logger {
    fn log(&mut self, message: &str);
}
