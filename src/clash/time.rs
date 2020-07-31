use std::collections::BTreeMap;

use specs::prelude::*;
use specs_derive::Component;

use crate::atlas::{EasyECS, EasyMutECS};

#[derive(Hash, PartialEq, Eq, Component)]
pub struct TimeComponent {
    pub ticks: i32,
}

impl TimeComponent {
    pub fn init(ticks: i32) -> TimeComponent {
        TimeComponent { ticks }
    }
}

pub const BASE_ACTION_COST: i32 = 100;
pub const MOVE_ACTION_COST: i32 = BASE_ACTION_COST;

pub fn get_next_actor(ecs: &World) -> Option<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let times = ecs.read_storage::<TimeComponent>();

    let mut time_map = BTreeMap::new();
    for (entity, time) in (&entities, &times).join() {
        time_map.insert(time.ticks, entity);
    }

    if let Some((_, entity)) = time_map.iter().last() {
        Some(*entity)
    } else {
        None
    }
}

pub fn add_ticks(ecs: &mut World, ticks_to_add: i32) {
    let mut times = ecs.write_storage::<TimeComponent>();
    for time in (&mut times).join() {
        time.ticks += ticks_to_add;
    }
}

pub fn wait_for_next(ecs: &mut World) -> Option<Entity> {
    if let Some(next) = get_next_actor(ecs) {
        let time = ecs.read_storage::<TimeComponent>().grab(next).ticks;
        if time < BASE_ACTION_COST {
            let missing = BASE_ACTION_COST - time;
            add_ticks(ecs, missing);
        }
        return Some(next);
    }
    None
}

pub fn spend_time(ecs: &mut World, element: &Entity, ticks_to_spend: i32) {
    let mut times = ecs.write_storage::<TimeComponent>();
    times.grab_mut(*element).ticks -= ticks_to_spend;
    assert!(times.grab(*element).ticks >= 0);
}

#[cfg(test)]
mod tests {
    use super::super::create_world;
    use super::*;

    fn create_timer(ecs: &mut World, ticks: i32) -> Entity {
        ecs.create_entity().with(TimeComponent::init(ticks)).build()
    }

    #[test]
    fn get_next_with_no_actors() {
        let ecs = create_world();
        assert_eq!(true, get_next_actor(&ecs).is_none());
    }

    #[test]
    fn get_next_two_disjoint() {
        let mut ecs = create_world();
        create_timer(&mut ecs, 0);
        create_timer(&mut ecs, 10);
        let next = get_next_actor(&ecs).unwrap();
        let times = ecs.read_storage::<TimeComponent>();
        let time = times.get(next).unwrap().ticks;
        assert_eq!(10, time);
    }

    #[test]
    fn get_next_two_same() {
        let mut ecs = create_world();
        let first = create_timer(&mut ecs, 10);
        let second = create_timer(&mut ecs, 10);
        let next = get_next_actor(&ecs).unwrap();
        assert_eq!(next, second);
        {
            let mut times = ecs.write_storage::<TimeComponent>();
            times.grab_mut(next).ticks = 0;
        }
        let next = get_next_actor(&ecs).unwrap();
        assert_eq!(next, first);
    }

    #[test]
    fn add_some_ticks() {
        let mut ecs = create_world();
        let first = create_timer(&mut ecs, 10);
        add_ticks(&mut ecs, 50);
        let times = ecs.read_storage::<TimeComponent>();
        assert_eq!(60, times.grab(first).ticks);
    }

    #[test]
    fn wait_for() {
        let mut ecs = create_world();
        let first = create_timer(&mut ecs, 10);
        let second = create_timer(&mut ecs, 20);

        let next = wait_for_next(&mut ecs).unwrap();
        assert_eq!(next, second);
        let times = ecs.read_storage::<TimeComponent>();
        assert_eq!(90, times.grab(first).ticks);
        assert_eq!(100, times.grab(second).ticks);
    }

    #[test]
    fn wait_for_next_with_none() {
        let mut ecs = create_world();

        let next = wait_for_next(&mut ecs);
        assert_eq!(true, next.is_none());
    }

    #[test]
    fn spent() {
        let mut ecs = create_world();
        let first = create_timer(&mut ecs, 110);
        spend_time(&mut ecs, &first, BASE_ACTION_COST);
        let times = ecs.read_storage::<TimeComponent>();
        assert_eq!(10, times.grab(first).ticks);
    }
}
