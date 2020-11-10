use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::card_view::{CardView, CARD_WIDTH};
use super::reward_scene::{draw_selection_frame, get_reward, icons_for_items};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{gambler, sales, EquipmentItem, EquipmentRarity, EquipmentResource, ProgressionComponent, RewardsComponent};
use crate::enclose;
use crate::props::{Button, ButtonDelegate, ButtonEnabledState, HitTestResult, View};

pub struct MerchantView {
    text_renderer: Rc<TextRenderer>,
    ui: Rc<IconCache>,
    cards: Rc<RefCell<Vec<Option<CardView>>>>,
    accept_button: Button,
    selection: Rc<RefCell<Option<u32>>>,
}

impl MerchantView {
    pub fn init(render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, ecs: &World) -> BoxResult<MerchantView> {
        let mut items = gambler::get_merchant_items(ecs);
        let icons = icons_for_items(render_context, &items)?;

        let ui = Rc::new(IconCache::init(
            &render_context,
            IconLoader::init_ui(),
            &["card_frame_large.png", "card_frame_large_selection.png", "button_frame_full_selection.png"],
        )?);

        let cards: Rc<RefCell<Vec<Option<CardView>>>> = {
            Rc::new(RefCell::new(
                items
                    .drain(..)
                    .enumerate()
                    .map(|(i, s)| {
                        Some(
                            CardView::init(
                                SDLPoint::new(120 + ((i % 4) * 200) as i32, 90 + ((i / 4) * 275) as i32),
                                text_renderer,
                                &ui,
                                &icons,
                                s,
                                true,
                                false,
                            )
                            .expect("Unable to load merchant card"),
                        )
                    })
                    .collect(),
            ))
        };

        let selection = Rc::new(RefCell::new(None));
        let accept_button = Button::text(
            SDLPoint::new(800, 650),
            "Purchase",
            &render_context,
            text_renderer,
            true,
            true,
            ButtonDelegate::init()
                .handler(Box::new(move |_| {}))
                .enabled(Box::new(enclose! { (cards, selection) move |ecs| {
                    let cards = cards.borrow();

                    if let Some(selection_index) = *selection.borrow() {
                        let progression = &mut ecs.write_resource::<ProgressionComponent>();
                        let selection_equip = &cards[selection_index as usize].as_ref().unwrap().equipment;
                        if sales::can_purchase_selection (&progression, &selection_equip) {
                            return ButtonEnabledState::Shown;
                        }
                    }
                    ButtonEnabledState::Ghosted
                }}))
                .handler(Box::new(enclose! { (cards, selection) move |ecs| {
                    let mut cards = cards.borrow_mut();
                    let mut selection = selection.borrow_mut();
                    if let Some(selection_index) = *selection {
                        let mut progression = &mut ecs.write_resource::<ProgressionComponent>();
                        let selection_equip = &cards[selection_index as usize].as_ref().unwrap().equipment;
                        if sales::can_purchase_selection (&progression, &selection_equip) {
                            sales::purchase_selection(&mut progression, &selection_equip);
                            cards[selection_index as usize] = None;
                            *selection = None;
                        }
                    }
                }})),
        )?;

        Ok(MerchantView {
            text_renderer: Rc::clone(text_renderer),
            ui,
            cards: Rc::clone(&cards),
            accept_button,
            selection: Rc::clone(&selection),
        })
    }
}

impl View for MerchantView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let cards = self.cards.borrow();

        if let Some(selection) = *self.selection.borrow() {
            if let Some(card) = &cards[selection as usize] {
                draw_selection_frame(canvas, &self.ui, card.frame, "card_frame_large_selection.png")?;
            }
        }

        for c in cards.iter().flatten() {
            c.render(ecs, canvas, frame)?;

            self.text_renderer.render_text_centered(
                &format!("{} Influence", sales::selection_cost(&c.equipment)),
                c.frame.x(),
                c.frame.y() - 20,
                c.frame.width(),
                canvas,
                FontSize::Bold,
                FontColor::Brown,
            )?;
        }

        self.accept_button.render(&ecs, canvas, frame)?;

        let progression = &(*ecs.read_resource::<ProgressionComponent>()).state;
        self.text_renderer.render_text(
            &format!("Current Influence: {}", progression.influence),
            780,
            615,
            canvas,
            FontSize::Bold,
            FontColor::Brown,
        )?;

        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        for (i, c) in self.cards.borrow_mut().iter_mut().enumerate() {
            if let Some(c) = c {
                c.handle_mouse_click(ecs, x, y, button);
                if c.grabbed.is_some() {
                    c.grabbed = None;
                    *self.selection.borrow_mut() = Some(i as u32);
                }
            }
        }
        self.accept_button.handle_mouse_click(ecs, x, y, button);
    }

    fn handle_mouse_move(&mut self, ecs: &World, x: i32, y: i32, state: MouseState) {}

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        None
    }
}
