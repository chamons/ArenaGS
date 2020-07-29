use specs::prelude::*;
use specs_derive::Component;

use super::WeaponKind;

pub enum EventType {
    Bolt(Entity),
    Melee(Entity, WeaponKind),
}

type EventCallback = fn(ecs: &mut World, kind: EventType, target: &Entity) -> ();

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
    fn fire_event(&mut self, kind: EventType, target: &Entity);
    fn subscribe(&mut self, callback: EventCallback);
}

impl EventCoordinator for World {
    fn fire_event(&mut self, kind: EventType, target: &Entity) {
        let event = self.read_resource::<EventComponent>().on_event;
        if let Some(event) = event {
            event(self, kind, target);
        }
    }

    fn subscribe(&mut self, callback: EventCallback) {
        self.write_resource::<EventComponent>().on_event = Some(callback);
    }
}
