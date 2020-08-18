use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::TickTimer;

#[derive(Serialize, Deserialize, Clone)]
pub enum StatusKind {
    Status(TickTimer),
    Trait,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StatusStore {}

impl StatusStore {
    pub fn init() -> StatusStore {
        StatusStore {}
    }

    pub fn add_status(&mut self, name: &str, length: u32) {}

    pub fn add_trait(&mut self, name: &str) {}

    pub fn remote_trait(&mut self, name: &str) {}

    pub fn has(&self, name: &str) -> bool {
        true
    }

    pub fn duration(&self, name: &str) -> Option<u32> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn add_status() {
        let mut store = StatusStore::init();
        store.add_status("TestStatus", 200);
        assert!(store.has("TestStatus"));
    }

    #[test]
    fn add_duplicate_status() {
        let store = StatusStore::init();
    }

    #[test]
    fn add_trait() {
        let store = StatusStore::init();
    }

    #[test]
    fn add_duplicate_trait() {
        let store = StatusStore::init();
    }

    #[test]
    fn remove_trait() {
        let store = StatusStore::init();
    }

    #[test]
    #[should_panic]
    fn remove_non_existant_trait() {
        let store = StatusStore::init();
    }

    #[test]
    fn tick_statuses() {}

    #[test]
    fn has_status() {}

    #[test]
    fn status_integration_with_ecs() {}
}
