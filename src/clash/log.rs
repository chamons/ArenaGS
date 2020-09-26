use serde::{Deserialize, Serialize};

pub const LOG_COUNT: usize = 9;

#[derive(Deserialize, Serialize, Clone)]
pub struct Log {
    pub logs: Vec<String>,
}

impl Log {
    pub fn init() -> Log {
        Log { logs: vec![] }
    }

    fn add(&mut self, entry: &str) {
        self.logs.push(entry.to_string());
    }

    #[cfg(test)]
    pub fn contains_count(&self, value: &str) -> usize {
        self.logs.iter().filter(|x| x.contains(value)).count()
    }
}

use super::{EventCoordinator, EventKind, LogComponent, LogDirection};
use specs::prelude::*;

pub trait Logger {
    fn log<T: AsRef<str>>(&mut self, message: T);
}

impl Logger for World {
    fn log<T: AsRef<str>>(&mut self, message: T) {
        {
            let log = &mut self.write_resource::<LogComponent>().log;
            log.add(message.as_ref());
        }
        self.raise_event(EventKind::LogScrolled(LogDirection::SnapToEnd), None);
    }
}
