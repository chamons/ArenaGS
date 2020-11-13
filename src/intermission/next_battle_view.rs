use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::new_game;
use crate::enclose;
use crate::props::{Button, ButtonDelegate, InfoBarView, SkillBarView, View};

pub struct NextBattleView {
    continue_button: Button,
    preview_world: RefCell<World>,
    regenerate_world: bool,
    skillbar: SkillBarView,
    infobar: InfoBarView,
}

impl NextBattleView {
    pub fn init(render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, ecs: &World, next_fight: &Rc<RefCell<bool>>) -> BoxResult<NextBattleView> {
        let continue_button = Button::text(
            SDLPoint::new(800, 650),
            "Next Fight",
            render_context,
            text_renderer,
            ButtonDelegate::init().handler(Box::new(enclose! { (next_fight) move |_| *next_fight.borrow_mut() = true })),
        )?;
        let preview_world = NextBattleView::generate_preview_world(ecs);
        let skillbar = SkillBarView::init(render_context, &preview_world, SDLPoint::new(137, 700), Rc::clone(&text_renderer), true)?;
        let infobar = InfoBarView::init(SDLPoint::new(750, 100), render_context, Rc::clone(&text_renderer), true)?;
        Ok(NextBattleView {
            continue_button,
            preview_world: RefCell::new(preview_world),
            regenerate_world: false,
            skillbar,
            infobar,
        })
    }

    fn generate_preview_world(ecs: &World) -> World {
        new_game::create_equipment_preview_battle(ecs)
    }
}

impl View for NextBattleView {
    fn render(&self, outside_world: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        // Do not use passed in world as we're using our own "preview" simulation for this tab
        // Unless we are regenerating the preview
        if self.regenerate_world {
            *self.preview_world.borrow_mut() = NextBattleView::generate_preview_world(outside_world);
        }
        let preview_world = self.preview_world.borrow();

        self.continue_button.render(&preview_world, canvas, frame)?;

        self.skillbar.render(&preview_world, canvas, frame)?;
        self.infobar.render(&preview_world, canvas, frame)?;

        Ok(())
    }

    fn handle_mouse_click(&mut self, _: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        let mut preview_world = self.preview_world.borrow_mut();
        self.continue_button.handle_mouse_click(&mut preview_world, x, y, button);
    }

    fn handle_mouse_move(&mut self, _: &World, x: i32, y: i32, state: MouseState) {
        let preview_world = self.preview_world.borrow();
        self.continue_button.handle_mouse_move(&preview_world, x, y, state);
    }

    fn on_tab_swap(&mut self) {
        self.regenerate_world = true;
    }
}
