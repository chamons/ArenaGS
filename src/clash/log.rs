use std::cmp;

use serde::{Deserialize, Serialize};

pub const LOG_COUNT: usize = 9;

#[derive(Deserialize, Serialize, Clone)]
pub struct Log {
    pub logs: Vec<String>,
    pub index: usize,
}

impl Log {
    pub fn init() -> Log {
        Log { logs: vec![], index: 0 }
    }

    pub fn add(&mut self, entry: &str) {
        self.logs.push(entry.to_string());
        self.index = self.logs.len() - 1;
    }

    pub fn scroll_back(&mut self) {
        self.index = self.clamp_index(self.index as i64 - LOG_COUNT as i64);
    }

    pub fn scroll_forward(&mut self) {
        self.index = self.clamp_index(self.index as i64 + LOG_COUNT as i64);
    }

    fn clamp_index(&self, index: i64) -> usize {
        cmp::min(cmp::max(index, 0) as usize, self.logs.len() - 1)
    }

    #[cfg(test)]
    pub fn contains_count(&self, value: &str) -> usize {
        self.logs.iter().filter(|x| x.contains(value)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_can_bump_index() {
        let mut log = Log::init();
        for i in 0..LOG_COUNT {
            log.add("Test");
            assert_eq!(log.index, i);
        }
        log.add("Test");
        assert_eq!(log.index, LOG_COUNT);
        log.index = 0;
        log.add("Test");
        assert_eq!(log.index, LOG_COUNT + 1);
    }

    #[test]
    fn scroll_forward() {
        let mut log = Log::init();
        for _ in 0..15 {
            log.add("Test");
        }
        log.index = 0;
        log.scroll_forward();
        assert_eq!(log.index, LOG_COUNT);

        for _ in 0..5 {
            log.scroll_forward();
        }
        assert_eq!(log.index, log.logs.len() - 1);
    }

    #[test]
    fn scroll_back() {
        let mut log = Log::init();
        for _ in 0..15 {
            log.add("Test");
        }
        log.index = 14;
        log.scroll_back();
        assert_eq!(log.index, 14 - LOG_COUNT);

        for _ in 0..5 {
            log.scroll_back();
        }
        assert_eq!(log.index, 0);
    }
}
