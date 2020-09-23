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

    pub fn add(&mut self, entry: &str) {
        self.logs.push(entry.to_string());
    }

    #[cfg(test)]
    pub fn contains_count(&self, value: &str) -> usize {
        self.logs.iter().filter(|x| x.contains(value)).count()
    }
}
