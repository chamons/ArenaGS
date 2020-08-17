use specs::prelude::*;
use specs_derive::Component;

use super::DamageKind;

#[derive(Copy, Clone, is_enum_variant)]
pub enum MoveState {
    BeginAnimation,
    CompleteAnimation,
    Complete(u32),
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum BoltState {
    BeginCastAnimation,
    CompleteCastAnimation,
    BeginFlyingAnimation,
    CompleteFlyingAnimation,
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum MeleeState {
    BeginAnimation,
    CompleteAnimation,
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
    CompleteAnimation,
}

#[derive(Copy, Clone)]
pub enum EventKind {
    Move(MoveState),
    Bolt(BoltState),
    Melee(MeleeState),
    Field(FieldState),
    Explode(ExplodeState),
    Tick(i32),
    Damage(u32, DamageKind),
}

type EventCallback = fn(ecs: &mut World, kind: EventKind, target: Option<Entity>) -> ();

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
