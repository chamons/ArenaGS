use std::collections::BTreeMap;

use specs::prelude::*;

use super::{EventCoordinator, EventKind, TimeComponent};
use crate::atlas::{EasyECS, EasyMutECS};

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

pub const EXHAUSTION_PER_100_TICKS: f64 = 10.0;
pub const EXHAUSTION_COST_PER_MOVE: f64 = 5.0;
pub const FOCUS_PER_100_TICKS: f64 = 0.1;

pub fn add_ticks(ecs: &mut World, ticks_to_add: i32) {
    let mut elements = vec![];
    {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let mut times = ecs.write_storage::<TimeComponent>();
        for (entity, time) in (&entities, &mut times).join() {
            time.ticks += ticks_to_add;
            elements.push(entity);
        }
    }
    for e in elements {
        ecs.raise_event(EventKind::Tick(ticks_to_add), Some(e));
    }
}

pub fn wait_for_next(ecs: &mut World) -> Option<Entity> {
    if let Some(next) = get_next_actor(ecs) {
        let time = get_ticks(ecs, &next);
        if time < BASE_ACTION_COST {
            let missing = BASE_ACTION_COST - time;
            add_ticks(ecs, missing);
        }
        return Some(next);
    }
    None
}

pub fn get_ticks(ecs: &World, entity: &Entity) -> i32 {
    ecs.read_storage::<TimeComponent>().grab(*entity).ticks
}

pub fn spend_time(ecs: &mut World, element: &Entity, ticks_to_spend: i32) {
    let mut times = ecs.write_storage::<TimeComponent>();
    times.grab_mut(*element).ticks -= ticks_to_spend;
}

#[cfg(test)]
mod tests {
    use super::super::{create_test_state, create_world, find_all_entities, find_at_time, find_first_entity};
    use super::*;

    #[test]
    fn get_next_with_no_actors() {
        let ecs = create_world();
        assert_eq!(true, get_next_actor(&ecs).is_none());
    }

    #[test]
    fn get_next_two_disjoint() {
        let ecs = create_test_state().with_timed(0).with_timed(10).build();
        let next = get_next_actor(&ecs).unwrap();
        assert_eq!(10, get_ticks(&ecs, &next));
    }

    #[test]
    fn get_next_two_same() {
        let ecs = create_test_state().with_timed(10).with_timed(10).build();
        let all = find_all_entities(&ecs);
        let first = all[0];
        let second = all[1];
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
        let mut ecs = create_test_state().with_timed(10).build();
        let first = find_first_entity(&ecs);
        add_ticks(&mut ecs, 50);
        assert_eq!(60, get_ticks(&ecs, &first));
    }

    #[test]
    fn wait_for() {
        let mut ecs = create_test_state().with_timed(10).with_timed(20).build();
        let first = find_at_time(&ecs, 10);
        let second = find_at_time(&ecs, 20);

        let next = wait_for_next(&mut ecs).unwrap();
        assert_eq!(next, second);
        assert_eq!(90, get_ticks(&ecs, &first));
        assert_eq!(100, get_ticks(&ecs, &second));
    }

    #[test]
    fn wait_for_next_with_none() {
        let mut ecs = create_world();

        let next = wait_for_next(&mut ecs);
        assert_eq!(true, next.is_none());
    }

    #[test]
    fn spent() {
        let mut ecs = create_test_state().with_timed(110).build();
        let first = find_first_entity(&ecs);
        spend_time(&mut ecs, &first, BASE_ACTION_COST);
        assert_eq!(10, get_ticks(&ecs, &first));
    }
}
