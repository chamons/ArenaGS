// Atlas holds up the world with low level functionality and types
// - crash provides super basic crash reporting (open file with log)
// - paths provides current exe location (for resource loading)
// - Basic Point and BoxResults types
mod crash;
mod paths;

pub use crash::on_crash;
pub use paths::get_exe_folder;

#[derive(Hash, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub const fn init(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
