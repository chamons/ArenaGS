use std::cell::RefCell;
use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::card_view::{CardView, CARD_WIDTH};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{EquipmentItem, EquipmentResource, RewardsComponent};
use crate::conductor::{Scene, StageDirection};
use crate::enclose;
use crate::props::{Button, View};

pub struct RewardScene {
    text_renderer: Rc<TextRenderer>,
    ecs: World,
    reward: RewardsComponent,
    cards: Vec<CardView>,
    accept_button: Button,
    cash_out_button: Button,
    selection: Rc<RefCell<Option<u32>>>,
    chosen: Rc<RefCell<bool>>,
}

impl RewardScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>, ecs: World) -> BoxResult<RewardScene> {
        let render_context = render_context_holder.borrow();

        let reward = {
            let rewards = ecs.read_storage::<RewardsComponent>();
            (&rewards).join().next().unwrap().clone()
        };
        let mut items: Vec<EquipmentItem> = {
            let equipment = &ecs.read_resource::<EquipmentResource>();
            reward.cards.iter().map(|c| equipment.get(&c).clone()).collect()
        };

        let icons: Vec<&String> = items.iter().flat_map(|i| &i.image).collect();
        let icons = Rc::new(IconCache::init(&render_context, IconLoader::init_icons(), &icons[..])?);
        let ui = Rc::new(IconCache::init(
            &render_context,
            IconLoader::init_ui(),
            &["card_frame.png", "card_frame_large.png"],
        )?);

        const REWARD_MID: i32 = 1024 / 2;
        const REWARD_GAP: i32 = 200;
        let left = REWARD_MID - ((3 * CARD_WIDTH as i32) / 2) - REWARD_GAP;
        let card_delta = CARD_WIDTH as i32 + REWARD_GAP;
        let cards = {
            items
                .drain(..)
                .enumerate()
                .map(|(i, s)| {
                    CardView::init(SDLPoint::new(left + card_delta * i as i32, 300), text_renderer, &ui, &icons, s, true, false)
                        .expect("Unable to load equipment card")
                })
                .collect()
        };

        let chosen = Rc::new(RefCell::new(false));
        let selection = Rc::new(RefCell::new(None));
        let accept_button = Button::text(
            SDLPoint::new(875, 585),
            "Accept",
            &render_context,
            text_renderer,
            true,
            true,
            Some(Box::new(enclose! { (selection) move |_| selection.borrow().is_some() })),
            Some(Box::new(enclose! { (chosen) move || *chosen.borrow_mut() = true })),
        )?;
        let cash_out_button = Button::text(
            SDLPoint::new(475, 625),
            "Pass (+10 Influence)",
            &render_context,
            text_renderer,
            true,
            true,
            None,
            Some(Box::new(enclose! { (selection, chosen) move || {
                *selection.borrow_mut() = Some(3);
                *chosen.borrow_mut() = true;
            }})),
        )?;
        Ok(RewardScene {
            text_renderer: Rc::clone(text_renderer),
            ecs,
            reward,
            cards,
            accept_button,
            cash_out_button,
            selection: Rc::clone(&selection),
            chosen: Rc::clone(&chosen),
        })
    }
}

impl Scene for RewardScene {
    fn handle_key(&mut self, _keycode: Keycode, _keymod: Mod) {}

    fn handle_mouse_click(&mut self, x: i32, y: i32, button: Option<MouseButton>) {
        for (i, c) in &mut self.cards.iter_mut().enumerate() {
            c.handle_mouse_click(&self.ecs, x, y, button);
            if c.grabbed.is_some() {
                c.grabbed = None;
                *self.selection.borrow_mut() = Some(i as u32);
            }
        }
        self.accept_button.handle_mouse_click(&self.ecs, x, y, button);
        self.cash_out_button.handle_mouse_click(&self.ecs, x, y, button);
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        self.text_renderer.render_text("Reward", 50, 50, canvas, FontSize::Large, FontColor::White)?;

        for c in &self.cards {
            c.render(&self.ecs, canvas, frame)?;
        }

        self.accept_button.render(&self.ecs, canvas, frame)?;
        self.cash_out_button.render(&self.ecs, canvas, frame)?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&mut self) -> StageDirection {
        if *self.chosen.borrow() {
            StageDirection::ShowCharacter(std::mem::replace(&mut self.ecs, World::new()))
        } else {
            StageDirection::Continue
        }
    }
}
