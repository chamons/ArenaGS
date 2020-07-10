// The star of the show, the arena itself
// The scene and UX
mod animation;
mod battle_scene;
mod components;
mod field;
mod render;
mod sprite_loader;

use animation::{Animation, AnimationComponent};
use components::{CharacterInfoComponent, PlayerComponent, PositionComponent};
use field::FieldComponent;
use render::{RenderComponent, RenderOrder, SpriteKinds};
use sprite_loader::SpriteLoader;

pub use battle_scene::BattleScene;
