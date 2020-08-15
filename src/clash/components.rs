use std::collections::HashMap;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use specs_derive::*;

use super::EventCoordinator;
use super::Log;
use crate::atlas::{EasyECS, Point, SizedPoint, ToSerialize};
use crate::clash::{AmmoKind, AttackInfo, BehaviorKind, CharacterInfo, Map};

#[derive(Hash, PartialEq, Eq, Component, ConvertSaveload, Clone)]
pub struct TimeComponent {
    pub ticks: i32,
}

impl TimeComponent {
    pub fn init(ticks: i32) -> TimeComponent {
        TimeComponent { ticks }
    }
}

#[derive(Hash, PartialEq, Eq, Component, ConvertSaveload, Clone)]
pub struct PositionComponent {
    pub position: SizedPoint,
}

impl PositionComponent {
    pub const fn init(position: SizedPoint) -> PositionComponent {
        PositionComponent { position }
    }

    pub fn move_to(&mut self, point: Point) {
        self.position = self.position.move_to(point);
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct PlayerComponent {}

impl PlayerComponent {
    pub fn init() -> PlayerComponent {
        PlayerComponent {}
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct CharacterInfoComponent {
    pub character: CharacterInfo,
}

impl CharacterInfoComponent {
    pub const fn init(character: CharacterInfo) -> CharacterInfoComponent {
        CharacterInfoComponent { character }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct FrameComponent {
    pub current_frame: u64,
}
impl FrameComponent {
    pub fn init() -> FrameComponent {
        FrameComponent { current_frame: 0 }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct FieldComponent {
    #[allow(clippy::type_complexity)]
    pub color: (u8, u8, u8, u8),
}

#[allow(dead_code)]
impl FieldComponent {
    pub fn init(r: u8, g: u8, b: u8) -> FieldComponent {
        FieldComponent { color: (r, g, b, 140) }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct SkillsComponent {
    pub skills: Vec<String>,
}

impl SkillsComponent {
    pub fn init(skills: &[&'static str]) -> SkillsComponent {
        SkillsComponent {
            skills: skills.iter().map(|x| x.to_string()).collect(),
        }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct LogComponent {
    pub log: Log,
}

impl LogComponent {
    pub fn init() -> LogComponent {
        LogComponent { log: Log::init() }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MapComponent {
    pub map: Map,
}

impl MapComponent {
    pub const fn init(map: Map) -> MapComponent {
        MapComponent { map }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MovementComponent {
    pub new_position: SizedPoint,
}

impl MovementComponent {
    pub fn init(new_position: SizedPoint) -> MovementComponent {
        MovementComponent { new_position }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct SkillResourceComponent {
    pub ammo: HashMap<AmmoKind, u32>,
    pub max: HashMap<AmmoKind, u32>,
    pub exhaustion: f64,
    pub focus: f64,
    pub max_focus: f64,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct AttackComponent {
    pub attack: AttackInfo,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct BehaviorComponent {
    pub behavior: BehaviorKind,
}

impl BehaviorComponent {
    pub fn init(behavior: BehaviorKind) -> BehaviorComponent {
        BehaviorComponent { behavior }
    }
}

#[derive(Component, Clone)] // NotConvertSaveload
pub struct RandomComponent {
    pub rand: StdRng,
}

impl RandomComponent {
    pub fn init() -> RandomComponent {
        RandomComponent { rand: StdRng::from_entropy() }
    }
}

pub fn create_world() -> World {
    let mut ecs = World::new();
    ecs.register::<PositionComponent>();
    ecs.register::<FieldComponent>();
    ecs.register::<PlayerComponent>();
    ecs.register::<CharacterInfoComponent>();
    ecs.register::<MapComponent>();
    ecs.register::<FrameComponent>();
    ecs.register::<TimeComponent>();
    ecs.register::<LogComponent>();
    ecs.register::<SkillsComponent>();
    ecs.register::<AttackComponent>();
    ecs.register::<MovementComponent>();
    ecs.register::<SkillResourceComponent>();
    ecs.register::<BehaviorComponent>();
    ecs.register::<RandomComponent>();
    ecs.register::<SimpleMarker<ToSerialize>>();
    // If you add additional components remember to update saveload.rs

    // This we do not serialized this as it contains function pointers
    ecs.register::<super::EventComponent>();
    ecs.insert(super::EventComponent::init());

    ecs.insert(RandomComponent::init());
    ecs.insert(FrameComponent::init());
    ecs.insert(LogComponent::init());
    ecs.insert(SimpleMarkerAllocator::<ToSerialize>::new());

    ecs.subscribe(super::physics::move_event);
    ecs.subscribe(super::combat::bolt_event);
    ecs.subscribe(super::combat::melee_event);
    ecs.subscribe(super::combat::field_event);
    ecs.subscribe(super::combat::explode_event);
    ecs.subscribe(super::defenses::defense_event);

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

pub trait Logger {
    fn log(&mut self, message: &str);
    fn log_scroll_forward(&mut self);
    fn log_scroll_back(&mut self);
}

impl Logger for World {
    fn log(&mut self, message: &str) {
        let log = &mut self.write_resource::<LogComponent>().log;
        log.add(message);
    }
    fn log_scroll_forward(&mut self) {
        let log = &mut self.write_resource::<LogComponent>().log;
        log.scroll_forward();
    }
    fn log_scroll_back(&mut self) {
        let log = &mut self.write_resource::<LogComponent>().log;
        log.scroll_back();
    }
}
