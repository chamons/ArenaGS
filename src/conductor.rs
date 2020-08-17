// Conductor is an event loop and scene transition system
// - Director processes event loop and dispatches to the current Scene trait object for input/render

mod director;
pub use director::{Director, EventStatus};

mod scene;
pub use scene::Scene;

mod storyteller;
pub use storyteller::Storyteller;
