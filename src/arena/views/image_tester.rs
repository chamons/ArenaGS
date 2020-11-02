use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::view_components::*;
use super::View;
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::conductor::{EventStatus, Scene, StageDirection, Storyteller};

pub struct ImageTesterScene {
    view: Box<dyn View>,
    ecs: World,
}

impl ImageTesterScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>) -> BoxResult<ImageTesterScene> {
        let render_context = render_context_holder.borrow();
        Ok(ImageTesterScene {
            ecs: World::new(),
            view: Box::new(TabView::init(
                SDLPoint::new(20, 20),
                &render_context,
                text_renderer,
                vec![
                    TabInfo::init(
                        "First",
                        Box::from(Frame::init(SDLPoint::new(60, 60), &render_context, FrameKind::InfoBar)?),
                        |_| true,
                    ),
                    TabInfo::init(
                        "Second",
                        Box::from(Frame::init(SDLPoint::new(60, 60), &render_context, FrameKind::Log)?),
                        |_| true,
                    ),
                    TabInfo::init(
                        "Third",
                        Box::from(Button::text(
                            SDLPoint::new(60, 60),
                            "Click me!",
                            &render_context,
                            text_renderer,
                            None,
                            Some(Box::new(|| {
                                println!("Button Pressed!");
                                None
                            })),
                        )?),
                        |_| true,
                    ),
                ],
            )?),
        })
    }
}

impl Scene for ImageTesterScene {
    fn handle_mouse_click(&mut self, x: i32, y: i32, button: Option<MouseButton>) {
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

    fn ask_stage_direction(&mut self) -> StageDirection {
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
