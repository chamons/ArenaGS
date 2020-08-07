use std::cmp;
use std::collections::BTreeMap;

use ordered_float::*;
use specs::prelude::*;
use specs_derive::Component;

use super::SkillResourceComponent;
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

pub const EXHAUSTION_PER_100_TICKS: f64 = 5.0;
pub const EXHAUSTION_COST_PER_MOVE: f64 = 5.0;
pub const FOCUS_PER_100_TICKS: f64 = 0.1;

pub fn add_ticks(ecs: &mut World, ticks_to_add: i32) {
    let mut times = ecs.write_storage::<TimeComponent>();
    let mut skills = ecs.write_storage::<SkillResourceComponent>();
    for (time, skill) in (&mut times, (&mut skills).maybe()).join() {
        time.ticks += ticks_to_add;
        if let Some(skill) = skill {
            add_ticks_for_skill(skill, ticks_to_add);
        }
    }
}

fn add_ticks_for_skill(skill: &mut SkillResourceComponent, ticks_to_add: i32) {
    let exhaustion_to_remove = EXHAUSTION_PER_100_TICKS as f64 * (ticks_to_add as f64 / 100.0);

    let focus_to_add = FOCUS_PER_100_TICKS as f64 * (ticks_to_add as f64 / 100.0);
    // Ordering f64 is hard _tm_
    skill.exhaustion = *cmp::max(NotNan::new(0.0).unwrap(), NotNan::new(skill.exhaustion - exhaustion_to_remove).unwrap());
    skill.focus = *cmp::min(NotNan::new(skill.max_focus).unwrap(), NotNan::new(skill.focus + focus_to_add).unwrap());
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
    assert!(times.grab(*element).ticks >= 0);
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::super::{create_test_state, create_world, find_all_entities, find_at, find_at_time, find_first_entity, SkillResourceComponent};
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

    #[test]
    fn add_ticks_reduces_exhaustion() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        ecs.write_storage::<SkillResourceComponent>().grab_mut(player).exhaustion = 50.0;
        // This works as long as there is no rounding, as 20 *(5 *.5) = 50.0
        for _ in 0..20 {
            add_ticks(&mut ecs, 50);
        }

        {
            let skills = ecs.read_storage::<SkillResourceComponent>();
            assert_approx_eq!(skills.grab(player).exhaustion, 0.0);
        }

        // Keep going, make sure it doesn't drop below zero
        for _ in 0..10 {
            add_ticks(&mut ecs, 100);
        }

        {
            let skills = ecs.read_storage::<SkillResourceComponent>();
            assert_approx_eq!(skills.grab(player).exhaustion, 0.0);
        }
    }

    #[test]
    fn add_ticks_increases_focus() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        ecs.write_storage::<SkillResourceComponent>().grab_mut(player).focus = 0.0;
        ecs.write_storage::<SkillResourceComponent>().grab_mut(player).max_focus = 1.0;
        for _ in 0..20 {
            add_ticks(&mut ecs, 50);
        }

        {
            let skills = ecs.read_storage::<SkillResourceComponent>();
            assert_approx_eq!(skills.grab(player).focus, 1.0);
        }

        // Keep going, make sure it doesn't go above max
        for _ in 0..10 {
            add_ticks(&mut ecs, 100);
        }

        {
            let skills = ecs.read_storage::<SkillResourceComponent>();
            assert_approx_eq!(skills.grab(player).focus, 1.0);
        }
    }
}
