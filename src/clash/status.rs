use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::TickTimer;

#[derive(Serialize, Deserialize, Clone)]
pub enum StatusKind {
    Status(TickTimer),
    Trait,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StatusStore {
    store: HashMap<String, StatusKind>,
}

impl StatusStore {
    pub fn init() -> StatusStore {
        StatusStore { store: HashMap::new() }
    }

    pub fn add_status(&mut self, name: &str, length: i32) {
        self.store
            .entry(name.to_string())
            .and_modify(|e| match e {
                StatusKind::Status(timer) => timer.extend_to_duration(timer.duration()),
                StatusKind::Trait => panic!("Status insert of {} but already in as a trait?", name),
            })
            .or_insert(StatusKind::Status(TickTimer::init_with_duration(length)));
    }

    pub fn add_trait(&mut self, name: &str) {
        self.store
            .entry(name.to_string())
            .and_modify(|e| match e {
                StatusKind::Status(_) => panic!("Status insert of {} but already as a status?", name),
                StatusKind::Trait => {}
            })
            .or_insert(StatusKind::Trait);

        self.store.insert(name.to_string(), StatusKind::Trait);
    }

    pub fn remote_trait(&mut self, name: &str) {
        match self.store.get(name) {
            Some(StatusKind::Status(_)) => panic!("Status removal of trait {} but already as a status?", name),
            None => panic!("Status remove of trait {} but not found?", name),
            Some(StatusKind::Trait) => {}
        };
        self.store.remove(name);
    }

    pub fn has(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }

    pub fn duration(&self, name: &str) -> Option<i32> {
        match self.store.get(name) {
            Some(StatusKind::Status(timer)) => Some(timer.duration()),
            Some(StatusKind::Trait) => None,
            None => None,
        }
    }

    pub fn apply_ticks(&mut self, ticks: i32) {
        let mut remove = vec![];

        for (k, v) in self.store.iter_mut() {
            match v {
                StatusKind::Status(timer) => {
                    if timer.apply_ticks(ticks) {
                        remove.push(k.to_string());
                    }
                },
                StatusKind::Trait => {}
            }
        }
        
        for r in remove {
            self.store.remove(&r);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn add_status() {
        let mut store = StatusStore::init();
        assert!(!store.has("TestStatus"));
        store.add_status("TestStatus", 200);
        assert!(store.has("TestStatus"));
        assert_eq!(200, store.duration("TestStatus").unwrap());
    }

    #[test]
    fn add_duplicate_status() {
        let mut store = StatusStore::init();
        assert!(!store.has("TestStatus"));
        store.add_status("TestStatus", 300);
        store.add_status("TestStatus", 200);
        assert!(store.has("TestStatus"));
        assert_eq!(300, store.duration("TestStatus").unwrap());
    }

    #[test]
    fn add_trait() {
        let mut store = StatusStore::init();
        assert!(!store.has("TestTrait"));
        store.add_trait("TestTrait");
        assert!(store.has("TestTrait"));
    }

    #[test]
    fn add_duplicate_trait() {
        let mut store = StatusStore::init();
        store.add_trait("TestTrait");
        store.add_trait("TestTrait");
        assert!(store.has("TestTrait"));
    }

    #[should_panic]
    #[test]
    fn add_trait_existing_status() {
        let mut store = StatusStore::init();
        store.add_status("TestTrait", 100);
        store.add_trait("TestTrait");
    }

    #[should_panic]
    #[test]
    fn add_status_existing_trait() {
        let mut store = StatusStore::init();
        store.add_trait("TestTrait");
        store.add_status("TestTrait", 100);
    }

    #[test]
    fn remove_trait() {
        let mut store = StatusStore::init();
        store.add_trait("TestTrait");
        assert!(store.has("TestTrait"));
        store.remote_trait("TestTrait");
        assert!(!store.has("TestTrait"));
    }

    #[should_panic]
    #[test]
    fn remove_trait_but_was_status() {
        let mut store = StatusStore::init();
        store.add_status("TestTrait", 100);
        store.remote_trait("TestTrait");
    }

    #[test]
    #[should_panic]
    fn remove_non_existant_trait() {
        let mut store = StatusStore::init();
        store.remote_trait("TestTrait");
    }

    #[test]
    fn tick_statuses() {
        let mut store = StatusStore::init();
        store.add_trait("TestTrait");
        store.add_status("TestStatus", 100);
        store.apply_ticks(100);
        assert!(!store.has("TestStatus"));
        assert!(store.has("TestTrait"));
    }

    #[test]
    fn status_integration_with_ecs() {}
}
