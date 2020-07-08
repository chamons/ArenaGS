// The game rules and logic for the games
use specs::prelude::*;
use specs_derive::Component;

#[derive(Hash, PartialEq, Eq, Component)]
pub struct PositionComponent {
    pub x: u32,
    pub y: u32,
}

impl PositionComponent {
    pub const fn init(x: u32, y: u32) -> PositionComponent {
        PositionComponent { x, y }
    }
}
