use bevy_ecs::schedule::SystemStage;

// TODO - When we have a schedule here, clone ui/schedule.rs hot reload setup
pub fn gameplay_schedule() -> SystemStage {
    // All systems must be marked #[no_mangle] for hot reloading to work
    SystemStage::single_threaded()
}
