use bevy_ecs::schedule::SystemStage;

pub fn gameplay_schedule() -> SystemStage {
    // All systems must be marked #[no_mangle] for hot reloading to work
    SystemStage::single_threaded()
}
