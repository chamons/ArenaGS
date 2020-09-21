use std::cmp;

use serde::{Deserialize, Serialize};

pub const LOG_COUNT: usize = 9;

#[derive(Deserialize, Serialize, Clone)]
pub struct Log {
    logs: Vec<String>,
    pub index: usize,
}

impl Log {
    pub fn init() -> Log {
        Log { logs: vec![], index: 0 }
    }

    pub fn get(&self, index: usize, length: usize) -> &[String] {
        if length == 0 || index >= self.logs.len() {
            return &[];
        }
        // Take as many items as you can before hitting the end
        let length = cmp::min(index + length, self.logs.len()) - index;
        &self.logs[index..index + length]
    }

    pub fn add(&mut self, entry: &str) {
        self.logs.push(entry.to_string());
        self.index = self.clamp_index(self.logs.len() as i64 - LOG_COUNT as i64);
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
    fn normal_get() {
        let log = Log {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            index: 0,
        };
        let output = log.get(0, 5);
        assert_eq!(output.len(), 3);
        let output = log.get(2, 1);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "3".to_string());
        let output = log.get(1, 2);
        assert_eq!(output.len(), 2);
        assert_eq!(output[1], "3".to_string());
    }

    #[test]
    fn get_zero() {
        let log = Log {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            index: 0,
        };
        let output = log.get(2, 0);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn out_of_range() {
        let log = Log {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            index: 0,
        };
        let output = log.get(4, 1);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn get_too_far() {
        let log = Log {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            index: 0,
        };
        let output = log.get(2, 3);
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn add_can_bump_index() {
        let mut log = Log::init();
        for _ in 0..LOG_COUNT {
            log.add("Test");
            assert_eq!(log.index, 0);
        }
        log.add("Test");
        assert_eq!(log.index, 1);
        log.index = 0;
        log.add("Test");
        assert_eq!(log.index, 2);
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
