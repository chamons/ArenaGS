use std::fmt;

// The game rules and logic for the games
mod character;
pub use character::Character;

mod skills;
use skills::invoke_skill;
pub use skills::{get_skill, SkillInfo, TargetType};

mod components;
pub use components::{create_world, CharacterInfoComponent, FieldComponent, FrameComponent, PlayerComponent, SkillsComponent};

mod map;
pub use map::{element_at_location, Map, MapComponent, MapHitTestResult, MapTile, MAX_MAP_TILES};

mod physics;
use physics::{can_move_character, complete_move, move_character, point_in_direction, wait};

mod position_component;
pub use position_component::PositionComponent;

mod animation;
pub use animation::*;

mod time;
pub use time::*;

mod actions;
pub use actions::*;

mod log;
pub use log::*;

mod ai;
pub use ai::take_enemy_action;

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
