use specs::prelude::*;
use specs_derive::Component;

use super::{BoltKind, Damage, RolledDamage, SpawnKind, StatusKind};

#[derive(Copy, Clone, is_enum_variant)]
pub enum MoveState {
    BeginAnimation,
    CompleteAnimation,
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum PostMoveAction {
    None,
    Shoot(Damage, Option<u32>, BoltKind), // Shoot nearest target (if any)
    Attack,                               // Attack the exact target in the embedded AttackComponent
    CheckNewLocationDamage,
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum BoltState {
    BeginCastAnimation,
    CompleteCastAnimation,
    BeginFlyingAnimation,
    CompleteFlyingAnimation,
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum OrbState {
    BeginCastAnimation,
    CompleteCastAnimation,
    Created,
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum MeleeState {
    BeginAnimation,
    CompleteAnimation,
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum ConeState {
    BeginSwingAnimation,
    CompleteSwingAnimation,
    BeginHitAnimation,
    CompleteHitAnimation,
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum FieldState {
    BeginCastAnimation,
    CompleteCastAnimation,
    BeginFlyingAnimation,
    CompleteFlyingAnimation,
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum ExplodeState {
    BeginAnimation,
    BeginSecondaryExplosion,
    CompleteAnimation,
}

#[derive(Copy, Clone)]
pub enum LogDirection {
    Forward,
    Backwards,
    SnapToEnd,
}

#[derive(Copy, Clone)]
pub enum EventKind {
    Creation(SpawnKind),
    Move(MoveState, PostMoveAction),
    Bolt(BoltState),
    Orb(OrbState),
    Melee(MeleeState),
    Cone(ConeState),
    Field(FieldState),
    Explode(ExplodeState),
    SecondaryExplodeComplete,
    MoveComplete(u32),
    Tick(i32),
    Damage(RolledDamage),
    Healing(u32),
    StatusAdded(StatusKind),
    StatusRemoved(StatusKind),
    StatusExpired(StatusKind),
    LogScrolled(LogDirection),
}

pub type EventCallback = fn(ecs: &mut World, kind: EventKind, target: Option<Entity>) -> ();

#[derive(Component)] // NotConvertSaveload
pub struct EventComponent {
    pub on_event: Vec<EventCallback>,
}

impl EventComponent {
    pub fn init() -> EventComponent {
        EventComponent { on_event: vec![] }
    }
}

pub trait EventCoordinator {
    fn raise_event(&mut self, kind: EventKind, target: Option<Entity>);
    fn subscribe(&mut self, callback: EventCallback);
}

impl EventCoordinator for World {
    fn raise_event(&mut self, kind: EventKind, target: Option<Entity>) {
        let events = self.read_resource::<EventComponent>().on_event.clone();
        for handler in events.iter() {
            handler(self, kind, target);
        }
    }

    fn subscribe(&mut self, callback: EventCallback) {
        self.write_resource::<EventComponent>().on_event.push(callback);
    }
}
