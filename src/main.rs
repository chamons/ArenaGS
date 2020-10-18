#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]
#![allow(clippy::len_zero)]

use std::{cell::RefCell, rc::Rc};

use leak::Leak;

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
use after_image::{FontContext, RenderContext, RenderContextHolder, TextRenderer};

mod conductor;
use conductor::{Director, Storyteller};

mod arena;
mod clash;
mod intermission;

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

    #[cfg(feature = "crash_reporting")]
    let _guard = sentry::init(include_str!("../lib/sentry.key"));

    let render_context = Rc::new(RefCell::new(RenderContext::initialize()?));
    let mut director = Director::init(get_storyteller(&render_context)?);
    director.run(render_context)?;

    Ok(())
}

#[allow(clippy::needless_return)]
fn get_storyteller(render_context: &RenderContextHolder) -> BoxResult<Box<dyn Storyteller>> {
    // See text_renderer.rs for details on this hack
    let font_context = Box::from(FontContext::initialize()?).leak();
    let text_renderer = Rc::new(TextRenderer::init(&font_context)?);

    #[cfg(feature = "image_tester")]
    {
        return Ok(Box::new(arena::ImageTesterStoryteller::init(&render_context, &text_renderer)?));
    }
    #[cfg(not(feature = "image_tester"))]
    {
        return Ok(Box::new(arena::arena_storyteller::ArenaStoryteller::init(&render_context, &text_renderer)));
    }
}
