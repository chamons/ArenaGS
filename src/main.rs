#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]
#![allow(clippy::len_zero)]

use std::{cell::RefCell, rc::Rc};

use leak::Leak;

// Disable annoying black terminal
//#![windows_subsystem = "windows"]

#[macro_use]
extern crate derive_is_enum_variant;

#[macro_use]
extern crate bitflags;

#[cfg(debug_assertions)]
use std::panic;

mod atlas;
#[cfg(debug_assertions)]
use atlas::on_crash;
use atlas::BoxResult;

mod after_image;
use after_image::{FontContext, RenderContext, TextRenderer};

mod conductor;
use conductor::Director;

mod clash;

mod arena;

pub fn main() -> BoxResult<()> {
    #[cfg(feature = "profile_self_play")]
    {
        crate::arena::self_play::tests::self_play_10000_games();
        return Ok(());
    }

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

    let storyteller = Box::new(arena::arena_storyteller::ArenaStoryteller::init(&render_context, &text_renderer));
    let mut director = Director::init(storyteller);
    director.run(render_context)?;

    Ok(())
}
