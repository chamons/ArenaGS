use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::card_view::{CardView, CARD_WIDTH};
use super::reward_scene::{get_reward, icons_for_items};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{gambler, EquipmentItem, EquipmentResource, RewardsComponent};
use crate::enclose;
use crate::props::{Button, ButtonDelegate, ButtonEnabledState, HitTestResult, View};

pub struct MerchantView {
    text_renderer: Rc<TextRenderer>,
    reward: RewardsComponent,
    ui: Rc<IconCache>,
    cards: Vec<CardView>,
    accept_button: Button,
    selection: Rc<RefCell<Option<u32>>>,
}

impl MerchantView {
    pub fn init(render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, ecs: &World) -> BoxResult<MerchantView> {
        let reward = get_reward(ecs);
        let mut items = gambler::get_merchant_items(ecs);
        let icons = icons_for_items(render_context, &items)?;

        let ui = Rc::new(IconCache::init(
            &render_context,
            IconLoader::init_ui(),
            &["card_frame_large.png", "card_frame_large_selection.png", "button_frame_full_selection.png"],
        )?);

        let cards = {
            items
                .drain(..)
                .enumerate()
                .map(|(i, s)| {
                    CardView::init(
                        SDLPoint::new(100 + ((i % 4) * 200) as i32, 100 + ((i / 4) * 275) as i32),
                        text_renderer,
                        &ui,
                        &icons,
                        s,
                        true,
                        false,
                    )
                    .expect("Unable to load merchant card")
                })
                .collect()
        };

        let selection = Rc::new(RefCell::new(None));
        let accept_button = Button::text(
            SDLPoint::new(780, 585),
            "Purchase",
            &render_context,
            text_renderer,
            true,
            true,
            ButtonDelegate::init().handler(Box::new(move |_| {})).enabled(Box::new(
                enclose! { (selection) move || if selection.borrow().is_some() { ButtonEnabledState::Shown } else { ButtonEnabledState::Ghosted} },
            )),
        )?;

        Ok(MerchantView {
            text_renderer: Rc::clone(text_renderer),
            ui,
            reward,
            cards,
            accept_button,
            selection: Rc::clone(&selection),
        })
    }
}

impl View for MerchantView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        for c in &self.cards {
            c.render(ecs, canvas, frame)?;
        }

        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>) {}

    fn handle_mouse_move(&mut self, ecs: &World, x: i32, y: i32, state: MouseState) {}

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        None
    }
}
