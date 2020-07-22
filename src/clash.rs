use std::fmt;

// The game rules and logic for the games
mod character;
pub use character::Character;

mod skills;
pub use skills::{get_target_for_skill, invoke_skill, TargetType};

mod components;
pub use components::{create_world, CharacterInfoComponent, FieldComponent, PlayerComponent};

mod map;
pub use map::{element_at_location, Map, MapComponent, MapHitTestResult, MapTile, MAX_MAP_TILES};

mod physics;
pub use physics::move_character;

mod position_component;
pub use position_component::PositionComponent; 

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
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
