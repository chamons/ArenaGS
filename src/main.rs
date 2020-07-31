#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]

// Disable annoying black terminal
//#![windows_subsystem = "windows"]

#[macro_use]
extern crate derive_is_enum_variant;

#[cfg(debug_assertions)]
use std::panic;

mod atlas;
#[cfg(debug_assertions)]
use atlas::{on_crash, BoxResult};

mod after_image;
use after_image::{FontContext, RenderContext, TextRenderer};

mod conductor;
use conductor::Director;

mod clash;

mod arena;
use arena::BattleScene;

pub fn main() -> BoxResult<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    #[cfg(debug_assertions)]
    {
        let default_hook = std::panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            on_crash(&panic_info);
            default_hook(&panic_info);
        }));
    }

    let mut render_context = RenderContext::initialize()?;
    let font_context = FontContext::initialize()?;
    let text_renderer = TextRenderer::init(&font_context)?;

    let scene = Box::new(BattleScene::init(&render_context, &text_renderer)?);
    let mut director = Director::init(scene);
    director.run(&mut render_context)?;

    Ok(())
}
