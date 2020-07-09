// After Image provides general image and sprite processing
// - render_context provides glue to load in SDL
// - image_loader simplifies loading images relative to exe
// - Background and various sprites render different image sheet formats to screen

mod image_loader;
mod render_context;
mod sprites;

pub use image_loader::load_image;
pub use render_context::RenderContext;
pub use sprites::{Background, CharacterAnimationState, DetailedCharacter, LargeEnemy, Sprite, SpriteFolderDescription};
