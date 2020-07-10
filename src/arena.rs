// The star of the show, the arena itself
// The scene and UX
mod animation;
mod battle_scene;
mod render;
mod sprite_loader;

use animation::AnimationComponent;
pub use battle_scene::BattleScene;
use render::{RenderComponent, RenderOrder, SpriteKinds};
use sprite_loader::SpriteLoader;
