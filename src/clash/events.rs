use specs::prelude::*;
use specs_derive::Component;

use super::PostAnimationEffect;

#[derive(Copy, Clone)]
pub enum EventKind {
    Bolt(Entity),
    Melee(Entity),
    AnimationComplete(Entity, PostAnimationEffect),
}

type EventCallback = fn(ecs: &mut World, kind: EventKind, target: &Entity) -> ();

#[derive(Component)]
pub struct EventComponent {
    pub on_event: Vec<EventCallback>,
}

impl EventComponent {
    pub fn init() -> EventComponent {
        EventComponent { on_event: vec![] }
    }
}

pub trait EventCoordinator {
    fn fire_event(&mut self, kind: EventKind, target: &Entity);
    fn subscribe(&mut self, callback: EventCallback);
}

impl EventCoordinator for World {
    fn fire_event(&mut self, kind: EventKind, target: &Entity) {
        let events = self.read_resource::<EventComponent>().on_event.clone();
        for handler in events.iter() {
            handler(self, kind, target);
        }
    }

    fn subscribe(&mut self, callback: EventCallback) {
        self.write_resource::<EventComponent>().on_event.push(callback);
    }
}
