use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Frame {
    current: u64,
}

impl Frame {
    pub fn zero() -> Self {
        Frame { current: 0 }
    }
}

pub fn update_frame_count(mut time: ResMut<Frame>) {
    time.current += 1;
}
