#[derive(Hash, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn init(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}
