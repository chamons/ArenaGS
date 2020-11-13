use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::enclose;
use crate::props::{Button, ButtonDelegate, InfoBarView, SkillBarView, View};

pub struct NextBattleView {
    continue_button: Button,
    skillbar: SkillBarView,
    infobar: InfoBarView,
}

impl NextBattleView {
    pub fn init(render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, next_fight: &Rc<RefCell<bool>>) -> BoxResult<NextBattleView> {
        let continue_button = Button::text(
            SDLPoint::new(800, 650),
            "Next Fight",
            render_context,
            text_renderer,
            ButtonDelegate::init().handler(Box::new(enclose! { (next_fight) move |_| *next_fight.borrow_mut() = true })),
        )?;
        let skillbar = SkillBarView::init(render_context, &World::new(), SDLPoint::new(137, 700), Rc::clone(&text_renderer))?;
        let infobar = InfoBarView::init(SDLPoint::new(780, 20), render_context, Rc::clone(&text_renderer))?;
        Ok(NextBattleView {
            continue_button,
            skillbar,
            infobar,
        })
    }
}

impl View for NextBattleView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.continue_button.render(ecs, canvas, frame)?;

        self.skillbar.render(ecs, canvas, frame)?;
        self.infobar.render(ecs, canvas, frame)?;

        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        self.continue_button.handle_mouse_click(ecs, x, y, button);
    }

    fn handle_mouse_move(&mut self, ecs: &World, x: i32, y: i32, state: MouseState) {
        self.continue_button.handle_mouse_move(ecs, x, y, state);
    }
}
