// The star of the show, the arena itself
// The scene and UX
mod battle_scene;
mod components;
mod icon_loader;
mod render;
mod sprite_loader;
mod views;

use components::{tick_animations, AnimationComponent, CharacterInfoComponent, FieldComponent, PlayerComponent, PositionComponent};
use icon_loader::IconLoader;
use render::{RenderComponent, RenderOrder, SpriteKinds};
use sprite_loader::SpriteLoader;

pub use battle_scene::BattleScene;
