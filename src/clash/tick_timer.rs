use serde::{Deserialize, Serialize};

use super::BASE_ACTION_COST;

// Track how long since last proc, since ticks can be offset of BASE_ACTION_COST
#[derive(Serialize, Deserialize, Clone)]
pub struct TickTimer {
    ticks: i32,
}

impl TickTimer {
    pub fn init() -> TickTimer {
        TickTimer { ticks: 0 }
    }

    pub fn apply_ticks(&mut self, ticks_to_add: i32) -> bool {
        self.ticks += ticks_to_add;
        // We don't handle double wrap around
        assert!(self.ticks < (BASE_ACTION_COST * 2));
        if self.ticks >= BASE_ACTION_COST {
            self.ticks -= BASE_ACTION_COST;
            true
        } else {
            false
        }
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
}
