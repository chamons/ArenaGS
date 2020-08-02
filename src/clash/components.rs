use specs::prelude::*;
use specs_derive::Component;

use sdl2::pixels::Color;

use super::{EventCoordinator, LogComponent, PositionComponent};
use crate::atlas::{EasyECS, SizedPoint};
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

#[allow(dead_code)]
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
    ecs.register::<super::FrameComponent>();
    ecs.register::<super::TimeComponent>();
    ecs.register::<super::LogComponent>();
    ecs.register::<super::SkillsComponent>();
    ecs.register::<super::AttackComponent>();
    ecs.register::<super::EventComponent>();
    ecs.register::<super::MovementComponent>();
    ecs.register::<super::SkillResourceComponent>();

    ecs.insert(FrameComponent::init());
    ecs.insert(LogComponent::init());

    ecs.insert(super::EventComponent::init());
    ecs.subscribe(super::combat_on_event);
    ecs.subscribe(super::physics_on_event);

    #[cfg(test)]
    {
        crate::arena::add_ui_extension(&mut ecs);
    }

    ecs
}

pub trait Positions {
    fn get_position(&self, entity: &Entity) -> SizedPoint;
}

impl Positions for World {
    fn get_position(&self, entity: &Entity) -> SizedPoint {
        self.read_storage::<PositionComponent>().grab(*entity).position
    }
}

pub trait Framer {
    fn get_current_frame(&self) -> u64;
}

impl Framer for World {
    fn get_current_frame(&self) -> u64 {
        self.read_resource::<FrameComponent>().current_frame
    }
}
