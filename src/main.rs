// Disable annoying black terminal
#![windows_subsystem = "windows"]

mod after_image;
use after_image::{load_image, BoxResult, RenderContext};

mod conductor;
use conductor::{Director, EventStatus, Scene};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
struct Character {
    position: Point,
    sprite: Rect,
}

struct BattleScene {
    texture: sdl2::render::Texture,
}

impl BattleScene {
    pub fn init(render_context: &mut RenderContext) -> BoxResult<BattleScene> {
        let texture = load_image(r#"images\battle\set1\1\1_1_idle1 (1).png"#, &render_context)?;
        Ok(BattleScene { texture })
    }
}

impl Scene for BattleScene {
    fn handle_event(&self, event: &sdl2::event::Event) -> EventStatus {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return EventStatus::Quit,
            _ => {}
        }
        EventStatus::Continue
    }

    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 128, 255)));
        canvas.clear();

        let (width, height) = canvas.output_size()?;
        let sprite = Rect::new(0, 0, 96, 96);
        let screen_position = Point::new(0, 0) + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
        canvas.copy(&self.texture, sprite, screen_rect)?;

        canvas.present();

        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn open_url(url: &str) -> bool {
    if let Ok(mut child) = std::process::Command::new("cmd.exe").arg("/C").arg("start").arg("").arg(&url).spawn() {
        std::thread::sleep(std::time::Duration::new(1, 0));
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}

pub fn main() -> Result<(), String> {
    std::env::set_var("RUST_BACKTRACE", "1");
    panic::set_hook(Box::new(|panic_info| {
        let mut debug_spew = String::new();
        if let Some(location) = panic_info.location() {
            debug_spew.push_str(&format!("{} Line: {}\n", location.file(), location.line())[..]);
        }
        let payload = panic_info.payload();
        if let Some(s) = payload.downcast_ref::<&str>() {
            debug_spew.push_str(s);
        } else if let Some(s) = payload.downcast_ref::<String>() {
            debug_spew.push_str(s);
        }

        let _ = fs::write("debug.txt", debug_spew);

        #[cfg(target_os = "windows")]
        open_url("debug.txt");
    }));

    let mut render_context = RenderContext::initialize()?;

    let scene = BattleScene::init(&mut render_context).unwrap();

    let scene = Box::new(scene);

    let mut director = Director::init(scene);
    director.run(&mut render_context).unwrap();

    Ok(())
}
