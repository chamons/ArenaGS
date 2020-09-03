use specs::prelude::*;

use super::super::wait;

pub fn take_action(ecs: &mut World, enemy: &Entity, phase: u32) {
    wait(ecs, *enemy);
}
