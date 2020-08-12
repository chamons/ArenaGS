// The star of the show, the arena itself
// The scene and UX
mod animations;
mod battle_actions;
mod battle_animations;
mod battle_scene;

mod components;
use components::*;

mod icon_loader;
mod sprite_loader;
mod views;

use animations::*;
use icon_loader::IconLoader;
use sprite_loader::SpriteLoader;

pub use battle_scene::BattleScene;

#[cfg(test)]
pub use components::add_ui_extension;

mod saveload;
mod spawner;
