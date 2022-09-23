use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Frame {
    pub current: u64,
}

impl Frame {
    pub fn zero() -> Self {
        Frame { current: 0 }
    }
}
