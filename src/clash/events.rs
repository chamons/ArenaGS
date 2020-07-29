use specs::prelude::*;
use specs_derive::Component;

use super::{PostAnimationEffect, WeaponKind};
use crate::atlas::Point;

pub enum EventKind {
    Bolt(Entity),
    Melee(Entity),
    AnimationComplete(Entity, PostAnimationEffect),
}

type EventCallback = fn(ecs: &mut World, kind: EventKind, target: &Entity) -> ();

#[derive(Component)]
pub struct EventComponent {
    pub on_event: Option<EventCallback>,
}

impl EventComponent {
    pub fn init() -> EventComponent {
        EventComponent { on_event: None }
    }
}

pub trait EventCoordinator {
    fn fire_event(&mut self, kind: EventKind, target: &Entity);
    fn subscribe(&mut self, callback: EventCallback);
}

impl EventCoordinator for World {
    fn fire_event(&mut self, kind: EventKind, target: &Entity) {
        let event = self.read_resource::<EventComponent>().on_event;
        if let Some(event) = event {
            event(self, kind, target);
        }
    }

    fn subscribe(&mut self, callback: EventCallback) {
        self.write_resource::<EventComponent>().on_event = Some(callback);
    }
}
