// The game rules and logic for the games
use specs::prelude::*;
use specs_derive::Component;

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
