use specs::prelude::*;
use specs_derive::Component;

use crate::clash::Point;

#[derive(PartialEq)]
pub struct FPoint {
    pub x: f32,
    pub y: f32,
}

impl FPoint {
    pub const fn init(x: f32, y: f32) -> FPoint {
        FPoint { x, y }
    }
}

enum Animation {
    Position { start: Point, end: Point },
}

#[derive(Component)]
pub struct AnimationComponent {
    animation: Animation,
    beginning: u64,
    ending: u64,
}

impl AnimationComponent {
    pub fn movement(start_point: Point, end_point: Point, beginning: u64, ending: u64) -> AnimationComponent {
        AnimationComponent {
            animation: Animation::Position {
                start: start_point,
                end: end_point,
            },
            beginning,
            ending,
        }
    }

    pub fn is_complete(&self, current: u64) -> bool {
        current > self.ending
    }

    pub fn current_position(&self, current: u64) -> Option<FPoint> {
        match &self.animation {
            Animation::Position { start, end } => {
                if self.is_complete(current) {
                    Some(FPoint::init(end.x as f32, end.y as f32))
                } else {
                    let delta = (current - self.beginning) as f64 / (self.ending - self.beginning) as f64;
                    let x = lerp(start.x as f32, end.x as f32, delta);
                    let y = lerp(start.y as f32, end.y as f32, delta);
                    Some(FPoint::init(x, y))
                }
            }
        }
    }
}

fn lerp(start: f32, end: f32, t: f64) -> f32 {
    (start as f64 * (1.0f64 - t) + end as f64 * t) as f32
}
