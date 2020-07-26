use specs::prelude::*;
use specs_derive::Component;

use sdl2::pixels::Color;

use super::{LogComponent, PositionComponent};
use crate::clash::Character;

#[derive(Component)]
pub struct PlayerComponent {}

impl PlayerComponent {
    pub fn init() -> PlayerComponent {
        PlayerComponent {}
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
pub struct FrameComponent {
    pub current_frame: u64,
}
impl FrameComponent {
    pub fn init() -> FrameComponent {
        FrameComponent { current_frame: 0 }
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

#[derive(Component)]
pub struct SkillsComponent {
    pub skills: Vec<&'static str>,
}

impl SkillsComponent {
    pub fn init(skills: &[&'static str]) -> SkillsComponent {
        SkillsComponent { skills: skills.to_vec() }
    }
}

pub fn create_world() -> World {
    let mut ecs = World::new();
    ecs.register::<PositionComponent>();
    ecs.register::<FieldComponent>();
    ecs.register::<PlayerComponent>();
    ecs.register::<CharacterInfoComponent>();
    ecs.register::<super::MapComponent>();
    ecs.register::<super::AnimationComponent>();
    ecs.register::<super::FrameComponent>();
    ecs.register::<super::TimeComponent>();
    ecs.register::<super::LogComponent>();
    ecs.register::<super::SkillsComponent>();

    ecs.insert(FrameComponent::init());
    ecs.insert(LogComponent::init());
    ecs
}
