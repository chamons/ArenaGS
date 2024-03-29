use bevy_ecs::schedule::SystemStage;

#[cfg(not(feature = "hotreload"))]
use super::{end_movement_animation, end_sprite_animation, start_movement_animations, start_sprite_animations};

#[cfg(feature = "hotreload")]
use systems_hot::*;

#[cfg(feature = "hotreload")]
#[hot_lib_reloader::hot_module(dylib = "arenalib")]
mod systems_hot {
    use crate::core::Appearance;
    use crate::ui::{
        Animation, MovementAnimationComplete, MovementAnimationEvent, PostMovementAction, SpriteAnimateActionCompleteEvent, SpriteAnimateActionEvent,
    };

    use bevy_ecs::prelude::*;
    hot_functions_from_file!("src/ui/animation.rs");
}

pub fn create_ui_schedule() -> SystemStage {
    // All systems must be marked #[no_mangle] for hot reloading to work
    SystemStage::single_threaded()
        .with_system(start_sprite_animations)
        .with_system(end_sprite_animation)
        .with_system(start_movement_animations)
        .with_system(end_movement_animation)
        .with_system(super::clear_event_buffers) // This is is fine never to hot reload, it's data centric
}
