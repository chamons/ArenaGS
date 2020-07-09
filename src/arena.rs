// The star of the show, the arena itself
// The scene and UX
mod battle_scene;
mod render;
mod sprite_loader;

pub use battle_scene::BattleScene;
use render::{RenderComponent, SpriteKinds};
use sprite_loader::SpriteLoader;
