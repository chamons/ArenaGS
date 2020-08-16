#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]

use std::{cell::RefCell, rc::Rc};

use leak::Leak;

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
use arena::ArenaStoryteller;

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

    let render_context = Rc::new(RefCell::new(RenderContext::initialize()?));
    // See text_renderer.rs for details on this hack
    let font_context = Box::from(FontContext::initialize()?).leak();
    let text_renderer = Rc::new(TextRenderer::init(&font_context)?);

    let storyteller = Box::new(ArenaStoryteller::init(&render_context, &text_renderer));
    let mut director = Director::init(storyteller);
    director.run(render_context)?;

    Ok(())
}
