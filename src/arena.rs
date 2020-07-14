// The star of the show, the arena itself
// The scene and UX
mod animation;
mod battle_scene;
mod components;
mod field;
mod icon_loader;
mod render;
mod sprite_loader;
mod views;

use animation::{tick_animations, AnimationComponent};
use components::{CharacterInfoComponent, PlayerComponent, PositionComponent};
use field::FieldComponent;
use icon_loader::IconLoader;
use render::{RenderComponent, RenderOrder, SpriteKinds};
use sprite_loader::SpriteLoader;

pub use battle_scene::BattleScene;
