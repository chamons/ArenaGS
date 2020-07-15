// After Image provides general image and sprite processing
// - render_context provides glue to load in SDL
// - image_loader simplifies loading images relative to exe
// - Background and various sprites render different image sheet formats to screen

pub type RenderCanvas = sdl2::render::Canvas<sdl2::video::Window>;

mod image_loader;
mod render_context;
mod sprites;
mod text_renderer;

pub use image_loader::load_image;
pub use render_context::{FontContext, RenderContext};
pub use sprites::{Background, CharacterAnimationState, DetailedCharacter, LargeEnemy, Sprite, SpriteFolderDescription};
pub use text_renderer::{FontSize, TextRenderer};
