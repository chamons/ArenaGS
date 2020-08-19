use std::collections::HashMap;
use std::convert::*;

use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::{EventKind, StatusComponent, TickTimer};

#[derive(Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StatusKind {
    Burning,
    Frozen,
    FireAmmo,
    IceAmmo,

    #[cfg(test)]
    TestStatus,
    #[cfg(test)]
    TestTrait,
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

    pub fn add_status(&mut self, kind: StatusKind, length: i32) {
        self.store
            .entry(kind)
            .and_modify(|e| match e {
                StatusType::Status(timer) => timer.extend_to_duration(timer.duration()),
                StatusType::Trait => panic!("Status insert of {:?} but already in as a trait?", kind),
            })
            .or_insert_with(|| StatusType::Status(TickTimer::init_with_duration(length)));
    }

    pub fn add_trait(&mut self, kind: StatusKind) {
        self.store
            .entry(kind)
            .and_modify(|e| match e {
                StatusType::Status(_) => panic!("Status insert of {:?} but already as a status?", kind),
                StatusType::Trait => {}
            })
            .or_insert_with(|| StatusType::Trait);
    }

    pub fn remove_trait(&mut self, kind: StatusKind) {
        match self.store.get(&kind) {
            Some(StatusType::Status(_)) => panic!("Status removal of trait {:?} but already as a status?", kind),
            None => panic!("Status remove of trait {:?} but not found?", kind),
            Some(StatusType::Trait) => {}
        };
        self.store.remove(&kind);
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

    // Need notification or return list to event
    pub fn apply_ticks(&mut self, ticks: i32) -> Vec<StatusKind> {
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

    pub fn get_all(&self) -> Vec<StatusKind> {
        let mut names: Vec<u32> = self.store.keys().map(|x| (*x).into()).collect();
        names.sort_by(|a, b| a.cmp(b));
        names.iter().map(|x| StatusKind::from_u32(*x).unwrap()).collect()
    }
}

pub fn status_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Tick(ticks) => {
            if let Some(status) = ecs.write_storage::<StatusComponent>().get_mut(target.unwrap()) {
                let removed = status.status.apply_ticks(ticks);
                for r in removed {
                    //ecs.raise_event(EventKind::StatusExpired(r))
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::atlas::EasyMutECS;

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
        ecs.write_storage::<StatusComponent>()
            .grab_mut(entity)
            .status
            .add_status(StatusKind::TestStatus, 100);
        assert!(ecs.has_status(&entity, StatusKind::TestStatus));
        add_ticks(&mut ecs, 100);
        assert!(!ecs.has_status(&entity, StatusKind::TestStatus));
    }
}
