// Disable annoying black terminal
#![windows_subsystem = "windows"]

mod after_image;

use after_image::{load_image, RenderContext};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

#[derive(Debug)]
struct Character {
    position: Point,
    sprite: Rect,
}

fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture, character: &Character) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = character.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, character.sprite.width(), character.sprite.height());
    canvas.copy(texture, character.sprite, screen_rect)?;

    canvas.present();

    Ok(())
}

pub fn main() -> Result<(), String> {
    let mut render_context = RenderContext::initialize()?;

    let texture = load_image(r#"\images\battle\set1\1\1_1_idle1 (1).png"#, &render_context).unwrap();

    let character = Character {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 96, 96),
    };

    let mut i = 0;
    'running: loop {
        // Handle events
        for event in render_context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        //TODO: Update

        // Render
        i = (i + 1) % 255;
        render(&mut render_context.canvas, Color::RGB(i, 64, 255 - i), &texture, &character)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
