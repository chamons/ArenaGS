// The star of the show, the arena itself
// The scene and UX
mod animations;
mod battle_actions;
pub use battle_actions::BattleActionRequest;
mod battle_animations;

mod components;
use components::*;

mod sprite_loader;
mod views;

use animations::*;
use sprite_loader::SpriteLoader;

#[cfg(test)]
pub use animations::force_complete_animations;
#[cfg(test)]
pub use components::add_ui_extension;

mod saveload;

pub mod arena_storyteller;
pub mod battle_scene;
pub mod death_scene;
pub mod round_fade_scene;

#[cfg(feature = "self_play")]
pub mod self_play;

#[cfg(feature = "image_tester")]
pub use views::image_tester::*;
