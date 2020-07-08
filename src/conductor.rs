// Conductor is an event loop and scene transition system
// - Director processes event loop and dispatches to the current Scene trait object for input/render

mod director;
mod scene;

pub use director::{Director, EventStatus};
pub use scene::Scene;
