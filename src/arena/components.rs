use specs::prelude::*;
use specs_derive::Component;

use crate::clash::Character;

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

#[derive(Component)]
pub struct PlayerComponent {}

#[derive(Component)]
pub struct CharacterInfoComponent {
    pub character: Character,
}

impl CharacterInfoComponent {
    pub const fn init(character: Character) -> CharacterInfoComponent {
        CharacterInfoComponent { character }
    }
}
