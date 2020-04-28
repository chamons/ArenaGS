// Disable annoying black terminal
//#![windows_subsystem = "windows"]

use std::panic;

mod after_image;
use after_image::RenderContext;

mod conductor;
use conductor::Director;

mod atlas;
use atlas::on_crash;

mod arena;
use arena::BattleScene;

pub fn main() -> Result<(), String> {
    std::env::set_var("RUST_BACKTRACE", "1");

    #[cfg(debug_assertions)]
    panic::set_hook(Box::new(|panic_info| on_crash(&panic_info)));

    let mut render_context = RenderContext::initialize()?;

    let state = arena::BattleState::test_state();
    let scene = Box::new(BattleScene::init(&render_context, state).unwrap());
    let mut director = Director::init(scene);
    director.run(&mut render_context).unwrap();

    Ok(())
}
