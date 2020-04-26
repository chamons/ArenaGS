mod image_loader;
mod render_context;
mod sprite;

pub use image_loader::load_image;
pub use render_context::RenderContext;
pub use sprite::{CharacterAnimationState, DetailedCharacterSprite, SpriteDeepFolderDescription};

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
