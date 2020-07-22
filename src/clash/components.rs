use specs::prelude::*;
use specs_derive::Component;

use sdl2::pixels::Color;

use crate::clash::{Character, Point};

#[derive(Component)]
pub struct PlayerComponent {}

#[derive(Hash, PartialEq, Eq, Component)]
pub struct PositionComponent {
    pub position: Point,
}

impl PositionComponent {
    pub const fn init(x: u32, y: u32) -> PositionComponent {
        PositionComponent { position: Point::init(x, y) }
    }

    pub fn x(&self) -> u32 {
        self.position.x
    }
    pub fn y(&self) -> u32 {
        self.position.y
    }
}

#[derive(Component)]
pub struct CharacterInfoComponent {
    pub character: Character,
}

impl CharacterInfoComponent {
    pub const fn init(character: Character) -> CharacterInfoComponent {
        CharacterInfoComponent { character }
    }
}

#[derive(Component)]
pub struct FieldComponent {
    pub color: Color,
}

impl FieldComponent {
    pub fn init(r: u8, g: u8, b: u8) -> FieldComponent {
        FieldComponent {
            color: Color::from((r, g, b, 140)),
        }
    }
}

pub fn create_world() -> World {
    let mut ecs = World::new();
    ecs.register::<PositionComponent>();
    ecs.register::<FieldComponent>();
    ecs.register::<PlayerComponent>();
    ecs.register::<CharacterInfoComponent>();
    ecs.register::<super::MapComponent>();
    ecs
}
