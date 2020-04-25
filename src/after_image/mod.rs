mod image_loader;
mod render_context;

pub use image_loader::load_image;
pub use render_context::RenderContext;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
