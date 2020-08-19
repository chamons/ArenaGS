use specs::prelude::*;

use super::{CharacterInfoComponent, EventKind, StatusComponent, StatusKind};
use crate::atlas::EasyECS;

pub fn effect_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Tick(ticks) => {
            if ecs.read_storage::<StatusComponent>().has(target.unwrap()) {
                process_effects(ecs, target.unwrap(), ticks)
            }
        }
        _ => {}
    }
}

pub fn process_effects(ecs: &mut World, target: Entity, _ticks: i32) {
    let effects = ecs.read_storage::<StatusComponent>().grab(target).status.get_all();
    for effect in effects {
        match effect {
            StatusKind::Burning => {}
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::atlas::EasyMutECS;

    #[test]
    fn burning_triggers_burn_status() {}

    #[test]
    fn burn_damages_and_retriggers_if_burning() {}

    #[test]
    fn burn_damages_and_stops_if_not() {}
}
