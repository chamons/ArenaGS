// The star of the show, the arena itself
// The scene and UX
mod animation;
mod battle_scene;
mod render;
mod sprite_loader;

use animation::{Animation, AnimationComponent};
use render::{RenderComponent, RenderOrder, SpriteKinds};
use sprite_loader::SpriteLoader;

pub use battle_scene::BattleScene;
