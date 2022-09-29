use bevy_ecs::schedule::SystemStage;

#[cfg(not(feature = "hotreload"))]
use super::{process_new_messages, set_message_index};

#[cfg(feature = "hotreload")]
use systems_hot::*;

#[cfg(feature = "hotreload")]
#[hot_lib_reloader::hot_module(dylib = "arenalib")]
mod systems_hot {
    use crate::core::{Log, NewMessageEvent, ScrollMessageEvent};
    use bevy_ecs::prelude::*;
    hot_functions_from_file!("src/core/log.rs");
}

pub fn gameplay_schedule() -> SystemStage {
    // All systems must be marked #[no_mangle] for hot reloading to work
    SystemStage::single_threaded()
        .with_system(process_new_messages)
        .with_system(set_message_index)
        .with_system(super::clear_event_buffers) // This is is fine never to hot reload, it's data centric
}
