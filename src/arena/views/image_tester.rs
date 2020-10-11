use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::View;
use crate::after_image::{IconLoader, RenderCanvas, RenderContextHolder, TextRenderer};
use crate::atlas::prelude::*;
use crate::conductor::{EventStatus, Scene, StageDirection, Storyteller};

pub struct ImageTesterScene {
    view: Box<dyn View>,
    ecs: World,
}

impl ImageTesterScene {
    pub fn init(render_context_holder: &RenderContextHolder, _text_renderer: &Rc<TextRenderer>) -> BoxResult<ImageTesterScene> {
        Ok(ImageTesterScene {
            ecs: World::new(),
            view: Box::new(super::view_components::Button::init(
                SDLRect::new(20, 20, 60, 60),
                IconLoader::init_icons().get(&render_context_holder.borrow(), "b_07_a.png")?,
                || Some(super::HitTestResult::Tile(Point::init(0, 0))),
            )?),
        })
    }
}

impl Scene for ImageTesterScene {
    fn handle_key(&mut self, _keycode: Keycode, _keymod: Mod) {}

    fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) {
        if let Some(button) = button {
            if button == MouseButton::Left {
                let hit = self.view.hit_test(&self.ecs, x, y);
                println!("{:?}", hit);
            }
        }
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 128, 255)));
        canvas.clear();

        self.view.render(&self.ecs, canvas, frame)?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        StageDirection::Continue
    }
}

pub struct ImageTesterStoryteller {
    render_context: RenderContextHolder,
    text_renderer: Rc<TextRenderer>,
}

impl ImageTesterStoryteller {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>) -> BoxResult<ImageTesterStoryteller> {
        Ok(ImageTesterStoryteller {
            render_context: Rc::clone(render_context_holder),
            text_renderer: Rc::clone(&text_renderer),
        })
    }
}

impl Storyteller for ImageTesterStoryteller {
    fn follow_stage_direction(&self, _direction: StageDirection, _render_context: &RenderContextHolder) -> EventStatus {
        EventStatus::Continue
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        Box::new(ImageTesterScene::init(&self.render_context, &self.text_renderer).unwrap())
    }
}
