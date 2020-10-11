// After Image provides general image and sprite processing
// - render_context provides glue to load in SDL
// - image_loader simplifies loading images relative to exe
// - Background and various sprites render different image sheet formats to screen

pub type RenderCanvas = sdl2::render::Canvas<sdl2::video::Window>;

mod font_cache;
mod icon_cache;
mod icon_loader;
mod image_loader;
mod render_context;
mod sprites;
mod text_layout;
mod text_renderer;

pub use font_cache::FontCache;
pub use icon_cache::IconCache;
pub use icon_loader::IconLoader;
pub use image_loader::load_image;
pub use render_context::{FontContext, RenderContext, RenderContextHolder};
pub use sprites::*;
pub use text_layout::*;
pub use text_renderer::*;

#[cfg(test)]
pub mod font_test_helpers;
pub mod prelude;
