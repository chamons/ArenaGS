use std::collections::HashMap;
use std::convert::*;

use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::{EventCoordinator, EventKind, StatusComponent, TickTimer};
use crate::atlas::EasyMutECS;

#[derive(Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Debug, FromPrimitive, IntoPrimitive, is_enum_variant)]
#[repr(u32)]
pub enum StatusKind {
    Burning, // Retriggers as long as temperature is high enough
    Frozen,
    Magnum,
    Ignite,
    Cyclone,
    StaticCharge,
    Aimed,
    Armored,
    Flying,
    Regen,
    RegenTick,
    // When you add more here, consider if it should be shown in character_overlay.rs
    #[cfg(test)]
    TestStatus,
    #[cfg(test)]
    TestTrait,
}

impl StatusKind {
    fn description(&self) -> String {
        match self {
            StatusKind::StaticCharge => "Charge with Static".to_string(),
            _ => format!("{:?}", self),
        }
    }

    #[allow(clippy::match_like_matches_macro)]
    pub fn should_display(&self) -> bool {
        match self {
            StatusKind::Flying | StatusKind::RegenTick => false,
            _ => true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum StatusType {
    Status(TickTimer),
    Trait,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StatusStore {
    store: HashMap<StatusKind, StatusType>,
}

#[allow(dead_code)]
impl StatusStore {
    pub fn init() -> StatusStore {
        StatusStore { store: HashMap::new() }
    }

    pub fn add_status_to(ecs: &mut World, entity: Entity, kind: StatusKind, length: i32) {
        ecs.write_storage::<StatusComponent>().grab_mut(entity).status.add_status(kind, length);
        ecs.raise_event(EventKind::StatusAdded(kind), Some(entity));
    }

    pub fn add_trait_to(ecs: &mut World, entity: Entity, kind: StatusKind) {
        ecs.write_storage::<StatusComponent>().grab_mut(entity).status.add_trait(kind);
        ecs.raise_event(EventKind::StatusAdded(kind), Some(entity));
    }

    pub fn remove_status_from(ecs: &mut World, entity: Entity, kind: StatusKind) {
        ecs.write_storage::<StatusComponent>().grab_mut(entity).status.remove_status(kind);
        ecs.raise_event(EventKind::StatusRemoved(kind), Some(entity));
    }

    pub fn remove_trait_from(ecs: &mut World, entity: Entity, kind: StatusKind) {
        ecs.write_storage::<StatusComponent>().grab_mut(entity).status.remove_trait(kind);
        ecs.raise_event(EventKind::StatusRemoved(kind), Some(entity));
    }

    pub fn remove_trait_if_found_from(ecs: &mut World, entity: Entity, kind: StatusKind) {
        if ecs.write_storage::<StatusComponent>().grab_mut(entity).status.remove_trait_if_found(kind) {
            ecs.raise_event(EventKind::StatusRemoved(kind), Some(entity));
        }
    }

    // Returns true when swapping from false -> true
    pub fn toggle_trait_from(ecs: &mut World, entity: Entity, kind: StatusKind, state: bool) -> bool {
        let delta = ecs.write_storage::<StatusComponent>().grab_mut(entity).status.toggle_trait(kind, state);
        if let Some(delta) = delta {
            if delta {
                ecs.raise_event(EventKind::StatusAdded(kind), Some(entity));
                true
            } else {
                ecs.raise_event(EventKind::StatusRemoved(kind), Some(entity));
                false
            }
        } else {
            false
        }
    }

    fn add_status(&mut self, kind: StatusKind, length: i32) {
        self.store
            .entry(kind)
            .and_modify(|e| match e {
                StatusType::Status(timer) => timer.extend_to_duration(timer.duration()),
                StatusType::Trait => panic!("Status insert of {:?} but already in as a trait?", kind),
            })
            .or_insert_with(|| StatusType::Status(TickTimer::init_with_duration(length)));
    }

    fn add_trait(&mut self, kind: StatusKind) {
        self.store
            .entry(kind)
            .and_modify(|e| match e {
                StatusType::Status(_) => panic!("Status insert of {:?} but already as a status?", kind),
                StatusType::Trait => {}
            })
            .or_insert_with(|| StatusType::Trait);
    }

    fn remove_status(&mut self, kind: StatusKind) {
        match self.store.get(&kind) {
            Some(StatusType::Status(_)) => {}
            None => panic!("Status remove of status {:?} but not found?", kind),
            Some(StatusType::Trait) => panic!("Status removal of status {:?} but already as a trait?", kind),
        };
        self.store.remove(&kind);
    }

    fn remove_trait(&mut self, kind: StatusKind) {
        match self.store.get(&kind) {
            Some(StatusType::Status(_)) => panic!("Status removal of trait {:?} but already as a status?", kind),
            None => panic!("Status remove of trait {:?} but not found?", kind),
            Some(StatusType::Trait) => {}
        };
        self.store.remove(&kind);
    }

    fn remove_trait_if_found(&mut self, kind: StatusKind) -> bool {
        if self.has(kind) {
            self.remove_trait(kind);
            true
        } else {
            false
        }
    }

    fn toggle_trait(&mut self, kind: StatusKind, state: bool) -> Option<bool> {
        match self.store.get(&kind) {
            Some(StatusType::Status(_)) => panic!("Status toggle of trait {:?} but already as a status?", kind),
            _ => {}
        };

        if state {
            if !self.has(kind) {
                self.add_trait(kind);
                return Some(true);
            }
        } else {
            if self.has(kind) {
                self.remove_trait(kind);
                return Some(false);
            }
        }
        None
    }

    // Need notification or return list to event
    fn apply_ticks(&mut self, ticks: i32) -> Vec<StatusKind> {
        let mut remove = vec![];

        for (k, v) in self.store.iter_mut() {
            match v {
                StatusType::Status(timer) => {
                    if timer.apply_ticks(ticks) {
                        remove.push(*k);
                    }
                }
                StatusType::Trait => {}
            }
        }
        for r in &remove {
            self.store.remove(r);
        }
        remove
    }

    pub fn has(&self, kind: StatusKind) -> bool {
        self.store.contains_key(&kind)
    }

    pub fn duration(&self, kind: StatusKind) -> Option<i32> {
        match self.store.get(&kind) {
            Some(StatusType::Status(timer)) => Some(timer.duration()),
            Some(StatusType::Trait) => None,
            None => None,
        }
    }

    pub fn get_all(&self) -> Vec<StatusKind> {
        let mut names: Vec<u32> = self.store.keys().map(|x| (*x).into()).collect();
        names.sort_unstable();
        names.iter().map(|x| StatusKind::from_u32(*x).unwrap()).collect()
    }

    pub fn get_all_display_status(&self) -> Vec<String> {
        self.store
            .iter()
            .filter(|(k, _)| k.should_display())
            .filter_map(|(k, v)| match v {
                StatusType::Status(_) => Some(k.description()),
                StatusType::Trait => None,
            })
            .collect()
    }
}

pub fn status_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Tick(ticks) => {
            let removed = {
                if let Some(status) = ecs.write_storage::<StatusComponent>().get_mut(target.unwrap()) {
                    status.status.apply_ticks(ticks)
                } else {
                    vec![]
                }
            };
            for r in removed {
                ecs.raise_event(EventKind::StatusExpired(r), target);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn add_status() {
        let mut store = StatusStore::init();
        assert!(!store.has(StatusKind::TestStatus));
        store.add_status(StatusKind::TestStatus, 200);
        assert!(store.has(StatusKind::TestStatus));
        assert_eq!(200, store.duration(StatusKind::TestStatus).unwrap());
    }

    #[test]
    fn add_duplicate_status() {
        let mut store = StatusStore::init();
        assert!(!store.has(StatusKind::TestStatus));
        store.add_status(StatusKind::TestStatus, 300);
        store.add_status(StatusKind::TestStatus, 200);
        assert!(store.has(StatusKind::TestStatus));
        assert_eq!(300, store.duration(StatusKind::TestStatus).unwrap());
    }

    #[test]
    fn add_trait() {
        let mut store = StatusStore::init();
        assert!(!store.has(StatusKind::TestTrait));
        store.add_trait(StatusKind::TestTrait);
        assert!(store.has(StatusKind::TestTrait));
    }

    #[test]
    fn add_duplicate_trait() {
        let mut store = StatusStore::init();
        store.add_trait(StatusKind::TestTrait);
        store.add_trait(StatusKind::TestTrait);
        assert!(store.has(StatusKind::TestTrait));
    }

    #[should_panic]
    #[test]
    fn add_trait_existing_status() {
        let mut store = StatusStore::init();
        store.add_status(StatusKind::TestTrait, 100);
        store.add_trait(StatusKind::TestTrait);
    }

    #[should_panic]
    #[test]
    fn add_status_existing_trait() {
        let mut store = StatusStore::init();
        store.add_trait(StatusKind::TestTrait);
        store.add_status(StatusKind::TestTrait, 100);
    }

    #[test]
    fn remove_trait() {
        let mut store = StatusStore::init();
        store.add_trait(StatusKind::TestTrait);
        assert!(store.has(StatusKind::TestTrait));
        store.remove_trait(StatusKind::TestTrait);
        assert!(!store.has(StatusKind::TestTrait));
    }

    #[should_panic]
    #[test]
    fn remove_trait_but_was_status() {
        let mut store = StatusStore::init();
        store.add_status(StatusKind::TestTrait, 100);
        store.remove_trait(StatusKind::TestTrait);
    }

    #[test]
    #[should_panic]
    fn remove_non_existant_trait() {
        let mut store = StatusStore::init();
        store.remove_trait(StatusKind::TestTrait);
    }

    #[test]
    fn tick_statuses() {
        let mut store = StatusStore::init();
        store.add_trait(StatusKind::TestTrait);
        store.add_status(StatusKind::TestStatus, 100);
        store.apply_ticks(100);
        assert!(!store.has(StatusKind::TestStatus));
        assert!(store.has(StatusKind::TestTrait));
    }

    #[test]
    fn get_all_names() {
        let mut store = StatusStore::init();
        store.add_trait(StatusKind::TestTrait);
        store.add_status(StatusKind::TestStatus, 100);
        let all = store.get_all();
        assert_eq!(StatusKind::TestStatus, all[0]);
        assert_eq!(StatusKind::TestTrait, all[1]);
    }

    #[test]
    fn status_integration_with_ecs() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let entity = find_at(&ecs, 2, 2);
        ecs.add_status(entity, StatusKind::TestStatus, 100);
        assert!(ecs.has_status(entity, StatusKind::TestStatus));
        add_ticks(&mut ecs, 100);
        assert!(!ecs.has_status(entity, StatusKind::TestStatus));
    }

    #[test]
    fn toggle_inactive_false_trait() {
        let mut store = StatusStore::init();
        store.toggle_trait(StatusKind::TestTrait, false);
        assert!(!store.has(StatusKind::TestTrait));
    }

    #[test]
    fn toggle_inactive_true_trait() {
        let mut store = StatusStore::init();
        store.toggle_trait(StatusKind::TestTrait, true);
        assert!(store.has(StatusKind::TestTrait));
    }

    #[test]
    fn toggle_active_false_trait() {
        let mut store = StatusStore::init();
        store.add_trait(StatusKind::TestTrait);
        store.toggle_trait(StatusKind::TestTrait, false);
        assert!(!store.has(StatusKind::TestTrait));
    }

    #[test]
    fn toggle_active_true_trait() {
        let mut store = StatusStore::init();
        store.add_trait(StatusKind::TestTrait);
        store.toggle_trait(StatusKind::TestTrait, true);
        assert!(store.has(StatusKind::TestTrait));
    }

    #[test]
    #[should_panic]
    fn toggle_status() {
        let mut store = StatusStore::init();
        store.add_status(StatusKind::TestStatus, 100);
        store.toggle_trait(StatusKind::TestStatus, false);
    }

    #[test]
    fn remove_trait_found() {
        let mut store = StatusStore::init();
        store.add_trait(StatusKind::TestTrait);
        store.remove_trait_if_found(StatusKind::TestTrait);
        assert_eq!(false, store.has(StatusKind::TestTrait));
    }

    #[test]
    fn remove_trait_missing() {
        let mut store = StatusStore::init();
        store.remove_trait_if_found(StatusKind::TestTrait);
        assert_eq!(false, store.has(StatusKind::TestTrait));
    }

    #[test]
    fn remove_status_found() {
        let mut store = StatusStore::init();
        store.add_status(StatusKind::TestStatus, 100);
        store.remove_status(StatusKind::TestStatus);
        assert_eq!(false, store.has(StatusKind::TestTrait));
    }

    #[test]
    #[should_panic]
    fn remove_status_missing() {
        let mut store = StatusStore::init();
        store.remove_status(StatusKind::TestStatus);
    }

    #[test]
    #[should_panic]
    fn remove_status_wrong_kind() {
        let mut store = StatusStore::init();
        store.add_trait(StatusKind::TestStatus);
        store.remove_status(StatusKind::TestStatus);
    }

    fn test_event(ecs: &mut World, kind: EventKind, _target: Option<Entity>) {
        match kind {
            EventKind::StatusAdded(status_kind) => ecs.increment_test_data(format!("Added {:?}", status_kind)),
            EventKind::StatusExpired(status_kind) => ecs.increment_test_data(format!("Expired {:?}", status_kind)),
            EventKind::StatusRemoved(status_kind) => ecs.increment_test_data(format!("Removed {:?}", status_kind)),
            _ => {}
        };
    }

    #[test]
    fn events() {
        let mut ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_at(&ecs, 2, 2);
        ecs.subscribe(test_event);
        ecs.add_status(player, StatusKind::Aimed, 200);
        assert_eq!(1, ecs.get_test_data("Added Aimed"));
        ecs.add_trait(player, StatusKind::Armored);
        assert_eq!(1, ecs.get_test_data("Added Armored"));
        ecs.remove_status(player, StatusKind::Aimed);
        assert_eq!(1, ecs.get_test_data("Removed Aimed"));
        StatusStore::toggle_trait_from(&mut ecs, player, StatusKind::Frozen, true);
        StatusStore::toggle_trait_from(&mut ecs, player, StatusKind::Frozen, true);
        assert_eq!(1, ecs.get_test_data("Added Frozen"));
        StatusStore::toggle_trait_from(&mut ecs, player, StatusKind::Frozen, false);
        assert_eq!(1, ecs.get_test_data("Removed Frozen"));
        StatusStore::remove_trait_if_found_from(&mut ecs, player, StatusKind::Armored);
        assert_eq!(1, ecs.get_test_data("Removed Armored"));
        ecs.add_trait(player, StatusKind::Magnum);
        StatusStore::remove_trait_if_found_from(&mut ecs, player, StatusKind::Magnum);
        assert_eq!(1, ecs.get_test_data("Removed Magnum"));
    }
}
