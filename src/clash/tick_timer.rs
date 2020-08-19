use std::cmp;

use serde::{Deserialize, Serialize};

use super::BASE_ACTION_COST;

// Track how long since last proc, since ticks can be offset of BASE_ACTION_COST
#[derive(Serialize, Deserialize, Clone)]
pub struct TickTimer {
    ticks: i32,
    duration: i32,
}

impl TickTimer {
    pub fn init() -> TickTimer {
        TickTimer {
            ticks: 0,
            duration: BASE_ACTION_COST,
        }
    }

    pub fn init_with_duration(duration: i32) -> TickTimer {
        TickTimer { ticks: 0, duration }
    }

    pub fn apply_ticks(&mut self, ticks_to_add: i32) -> bool {
        self.ticks += ticks_to_add;
        // We don't handle double wrap around
        assert!(self.ticks < (self.duration * 2));
        if self.ticks >= self.duration {
            self.ticks -= self.duration;
            true
        } else {
            false
        }
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn extend_to_duration(&mut self, duration: i32) {
        self.duration = cmp::max(self.duration(), duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_ticks() {
        let mut timer = TickTimer::init();
        assert!(timer.apply_ticks(100));
        assert!(timer.apply_ticks(100));
        assert!(timer.apply_ticks(100));
    }

    #[test]
    fn multiple_small_ticks() {
        let mut timer = TickTimer::init();
        assert_eq!(false, timer.apply_ticks(50));
        assert_eq!(true, timer.apply_ticks(50));
        assert_eq!(false, timer.apply_ticks(30));
        assert_eq!(false, timer.apply_ticks(30));
        assert_eq!(true, timer.apply_ticks(40));
    }

    #[test]
    #[should_panic]
    fn double_wrap_asserts() {
        let mut timer = TickTimer::init();
        timer.apply_ticks(200);
    }

    #[test]
    fn custom_duration() {
        let mut timer = TickTimer::init_with_duration(150);
        assert_eq!(false, timer.apply_ticks(50));
        assert_eq!(false, timer.apply_ticks(50));
        assert_eq!(false, timer.apply_ticks(30));
        assert_eq!(true, timer.apply_ticks(30));
    }

    #[test]
    fn extend_duration() {
        let mut timer = TickTimer::init();
        assert_eq!(100, timer.duration());
        timer.extend_to_duration(50);
        assert_eq!(100, timer.duration());
        timer.extend_to_duration(150);
        assert_eq!(150, timer.duration());
    }
}
