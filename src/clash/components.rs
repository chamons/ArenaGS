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
use crate::atlas::prelude::*;
use crate::clash::{AmmoKind, AttackKind, BehaviorKind, Damage, Defenses, FieldEffect, FieldKind, Map, StatusKind, StatusStore, Temperature};

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

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct PlayerAlly {}

impl PlayerAlly {
    pub fn init() -> PlayerAlly {
        PlayerAlly {}
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct IsCharacterComponent {}

impl IsCharacterComponent {
    pub fn init() -> IsCharacterComponent {
        IsCharacterComponent {}
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct NamedComponent {
    pub name: String,
}

impl NamedComponent {
    pub fn init(name: &str) -> NamedComponent {
        NamedComponent { name: name.to_string() }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct DefenseComponent {
    pub defenses: Defenses,
}

impl DefenseComponent {
    pub fn init(defenses: Defenses) -> DefenseComponent {
        DefenseComponent { defenses }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct SkillPowerComponent {
    pub skill_power: u32,
}

impl SkillPowerComponent {
    pub fn init(skill_power: u32) -> SkillPowerComponent {
        SkillPowerComponent { skill_power }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct TemperatureComponent {
    pub temperature: Temperature,
}

impl TemperatureComponent {
    pub fn init(temperature: Temperature) -> TemperatureComponent {
        TemperatureComponent { temperature }
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

#[allow(clippy::type_complexity)]
type SimpleColor = (u8, u8, u8, u8);

#[derive(Component, ConvertSaveload, Clone)]
pub struct FieldComponent {
    pub fields: Vec<(Option<Point>, SimpleColor)>,
}

#[allow(dead_code)]
impl FieldComponent {
    pub fn init() -> FieldComponent {
        FieldComponent { fields: vec![] }
    }

    pub fn init_single(r: u8, g: u8, b: u8) -> FieldComponent {
        FieldComponent {
            fields: vec![(None, (r, g, b, 140))],
        }
    }

    pub fn init_group(fields: Vec<(Option<Point>, SimpleColor)>) -> FieldComponent {
        FieldComponent { fields }
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
    pub cooldown: HashMap<String, u32>,
    pub exhaustion: f64,
    pub focus: f64,
    pub max_focus: f64,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct AttackComponent {
    pub damage: Damage,
    pub target: Point,
    pub kind: AttackKind,
    pub source: Option<Point>,
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

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct PlayerDeadComponent {}

impl PlayerDeadComponent {
    pub fn init() -> PlayerDeadComponent {
        PlayerDeadComponent {}
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct StatusComponent {
    pub status: StatusStore,
}

impl StatusComponent {
    pub fn init() -> StatusComponent {
        StatusComponent { status: StatusStore::init() }
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct OrbComponent {
    pub path: Vec<Point>,
    pub speed: u32,
    pub duration: u32,
    pub name: String,
}

impl OrbComponent {
    pub fn init(path: Vec<Point>, speed: u32, duration: u32, name: &str) -> OrbComponent {
        OrbComponent {
            path,
            speed,
            duration,
            name: name.to_string(),
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct FlightComponent {
    pub takeoff_point: SizedPoint,
}

impl FlightComponent {
    pub fn init(takeoff_point: SizedPoint) -> FlightComponent {
        FlightComponent { takeoff_point }
    }
}

#[cfg(test)]
#[derive(PartialEq, Eq, Component, ConvertSaveload, Clone)]
pub struct TestComponent {
    pub data: HashMap<String, u32>,
}

#[cfg(test)]
impl TestComponent {
    pub fn init() -> TestComponent {
        TestComponent { data: HashMap::new() }
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SkipRenderComponent {}

impl SkipRenderComponent {
    pub fn init() -> SkipRenderComponent {
        SkipRenderComponent {}
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct FieldCastComponent {
    pub effect: FieldEffect,
    pub name: String,
    pub kind: FieldKind,
    pub target: SizedPoint,
    pub is_from_player: bool,
}

impl FieldCastComponent {
    pub fn init(effect: FieldEffect, name: &str, kind: FieldKind, target: SizedPoint, is_from_player: bool) -> FieldCastComponent {
        FieldCastComponent {
            effect,
            name: name.to_string(),
            kind,
            target,
            is_from_player,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct DurationComponent {
    pub duration: u32,
}

impl DurationComponent {
    pub fn init(duration: u32) -> DurationComponent {
        DurationComponent { duration }
    }
}

use super::content::gunslinger::GunslingerAmmo;
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct GunslingerComponent {
    pub ammo_types: Vec<GunslingerAmmo>,
    pub weapon_skills: Vec<String>,
}

impl GunslingerComponent {
    pub fn init(ammo_types: &[GunslingerAmmo], weapon_skills: &[String]) -> GunslingerComponent {
        GunslingerComponent {
            ammo_types: ammo_types.to_vec(),
            weapon_skills: weapon_skills.to_vec(),
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct RewardsComponent {
    pub cards: Vec<String>,
    pub influence: u32,
    pub cashout_influence: u32,
}

impl RewardsComponent {
    pub fn init(influence: u32, cards: Vec<String>, cashout_influence: u32) -> RewardsComponent {
        assert!(3 == cards.len());
        RewardsComponent {
            cards,
            influence,
            cashout_influence,
        }
    }
}

#[cfg(test)]
pub trait TestInfo {
    fn get_test_data(&self, name: &str) -> u32;
    fn set_test_data(&self, name: String, value: u32);
    fn increment_test_data(&self, name: String);
}

#[cfg(test)]
impl TestInfo for World {
    fn get_test_data(&self, name: &str) -> u32 {
        *self.read_resource::<TestComponent>().data.get(&name.to_string()).unwrap()
    }
    fn set_test_data(&self, name: String, value: u32) {
        self.write_resource::<TestComponent>().data.insert(name, value);
    }
    fn increment_test_data(&self, name: String) {
        *self.write_resource::<TestComponent>().data.entry(name).or_insert(0) += 1;
    }
}

pub fn create_world() -> World {
    let mut ecs = World::new();
    ecs.register::<PositionComponent>();
    ecs.register::<FieldComponent>();
    ecs.register::<PlayerComponent>();
    ecs.register::<IsCharacterComponent>();
    ecs.register::<TemperatureComponent>();
    ecs.register::<SkillPowerComponent>();
    ecs.register::<DefenseComponent>();
    ecs.register::<NamedComponent>();
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
    ecs.register::<PlayerDeadComponent>();
    ecs.register::<SimpleMarker<ToSerialize>>();
    ecs.register::<StatusComponent>();
    ecs.register::<OrbComponent>();
    ecs.register::<FlightComponent>();
    ecs.register::<SkipRenderComponent>();
    ecs.register::<FieldCastComponent>();
    ecs.register::<DurationComponent>();
    ecs.register::<GunslingerComponent>();
    ecs.register::<RewardsComponent>();
    ecs.register::<PlayerAlly>();
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
    ecs.subscribe(super::combat::orb_event);
    ecs.subscribe(super::combat::cone_event);
    ecs.subscribe(super::defenses::defense_event);
    ecs.subscribe(super::skills::tick_event);
    ecs.subscribe(super::temperature::temp_event);
    ecs.subscribe(super::status::status_event);
    ecs.subscribe(super::flying::flying_event);
    ecs.subscribe(super::damage::regen_event);

    #[cfg(test)]
    {
        ecs.insert(TestComponent::init());
        // Normally done by BattleScene in UI case
        crate::arena::add_ui_extension(&mut ecs);
    }

    ecs
}

pub trait ShortInfo {
    fn get_position(&self, entity: Entity) -> SizedPoint;
    fn get_defenses(&self, entity: Entity) -> Defenses;
    fn get_temperature(&self, entity: Entity) -> Temperature;
    fn get_name(&self, entity: Entity) -> Option<String>;
}

impl ShortInfo for World {
    fn get_position(&self, entity: Entity) -> SizedPoint {
        self.read_storage::<PositionComponent>().grab(entity).position
    }
    fn get_defenses(&self, entity: Entity) -> Defenses {
        self.read_storage::<DefenseComponent>().grab(entity).defenses.clone()
    }
    fn get_temperature(&self, entity: Entity) -> Temperature {
        self.read_storage::<TemperatureComponent>().grab(entity).temperature.clone()
    }
    fn get_name(&self, entity: Entity) -> Option<String> {
        if let Some(named) = self.read_storage::<NamedComponent>().get(entity) {
            Some(named.name.to_string())
        } else {
            None
        }
    }
}

pub trait StatusInfo {
    fn has_status(&self, entity: Entity, kind: StatusKind) -> bool;
}

impl StatusInfo for World {
    fn has_status(&self, entity: Entity, kind: StatusKind) -> bool {
        self.read_storage::<StatusComponent>().grab(entity).status.has(kind)
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

pub trait StatusApplier {
    fn add_status(&mut self, entity: Entity, kind: StatusKind, length: i32);
    fn remove_status(&mut self, entity: Entity, kind: StatusKind);
    fn add_trait(&mut self, entity: Entity, kind: StatusKind);
}
impl StatusApplier for World {
    fn add_status(&mut self, entity: Entity, kind: StatusKind, length: i32) {
        StatusStore::add_status_to(self, entity, kind, length);
    }
    fn remove_status(&mut self, entity: Entity, kind: StatusKind) {
        StatusStore::remove_status_from(self, entity, kind);
    }
    fn add_trait(&mut self, entity: Entity, kind: StatusKind) {
        StatusStore::add_trait_to(self, entity, kind);
    }
}
