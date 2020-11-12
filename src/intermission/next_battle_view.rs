use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::card_view::{CardView, CARD_WIDTH};
use super::reward_scene::{draw_selection_frame, get_reward, icons_for_items};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{gambler, EquipmentItem, EquipmentResource, RewardsComponent};
use crate::enclose;
use crate::props::{Button, ButtonDelegate, ButtonEnabledState, HitTestResult, View};

pub struct NextBattleView {
    continue_button: Button,
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
        Ok(NextBattleView { continue_button })
    }
}

impl View for NextBattleView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.continue_button.render(ecs, canvas, frame)?;

        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        self.continue_button.handle_mouse_click(ecs, x, y, button);
    }

    fn handle_mouse_move(&mut self, ecs: &World, x: i32, y: i32, state: MouseState) {
        self.continue_button.handle_mouse_move(ecs, x, y, state);
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        None
    }
}
