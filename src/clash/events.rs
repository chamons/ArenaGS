use specs::prelude::*;
use specs_derive::Component;

#[derive(Copy, Clone)]
pub enum EventKind {
    Move(),
    Bolt(),
    Melee(),
    Field(),

    #[cfg(test)]
    WaitForAnimations(),
}

type EventCallback = fn(ecs: &mut World, kind: EventKind, target: Option<Entity>) -> ();

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
