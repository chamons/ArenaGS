use std::fmt;

// The game rules and logic for the games
mod character;
pub use character::Character;

mod skills;
pub use skills::{get_target_for_skill, invoke_skill, TargetType};

mod components;
pub use components::{create_world, CharacterInfoComponent, FieldComponent, PlayerComponent, PositionComponent};

mod map;
pub use map::{element_at_location, MapHitTestResult};

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub const fn init(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
