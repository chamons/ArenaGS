use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;

use crate::props::HitTestResult;

pub struct TextHitTester {
    entries: Vec<(SDLRect, HitTestResult)>,
}

impl TextHitTester {
    pub fn init() -> TextHitTester {
        TextHitTester { entries: vec![] }
    }

    pub fn add(&mut self, rect: SDLRect, result: HitTestResult) {
        self.entries.push((rect, result));
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn hit_test(&self, x: i32, y: i32) -> Option<HitTestResult> {
        let point = SDLPoint::new(x, y);
        for e in &self.entries {
            if e.0.contains_point(point) {
                return Some(e.1.clone());
            }
        }
        None
    }
}
