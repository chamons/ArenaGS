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
use crate::clash::{EquipmentItem, EquipmentResource, ProgressionComponent, RewardsComponent};
use crate::conductor::{Scene, StageDirection};
use crate::enclose;
use crate::props::{Button, ButtonDelegate, ButtonEnabledState, View};

pub struct RewardScene {
    text_renderer: Rc<TextRenderer>,
    ecs: World,
    reward: RewardsComponent,
    ui: Rc<IconCache>,
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
            reward.cards.iter().map(|c| equipment.get(&c)).collect()
        };

        let icons: Vec<&String> = items.iter().flat_map(|i| &i.image).collect();
        let icons = Rc::new(IconCache::init(&render_context, IconLoader::init_icons(), &icons[..])?);
        let ui = Rc::new(IconCache::init(
            &render_context,
            IconLoader::init_ui(),
            &[
                "card_frame.png",
                "card_frame_large.png",
                "card_frame_large_selection.png",
                "button_frame_full_selection.png",
                "reward_background.png",
            ],
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
                    CardView::init(SDLPoint::new(left + card_delta * i as i32, 200), text_renderer, &ui, &icons, s, true, false)
                        .expect("Unable to load equipment card")
                })
                .collect()
        };

        let chosen = Rc::new(RefCell::new(false));
        let selection = Rc::new(RefCell::new(None));
        let accept_button = Button::text(
            SDLPoint::new(780, 585),
            "Accept",
            &render_context,
            text_renderer,
            true,
            true,
            ButtonDelegate::init()
                .enabled(Box::new(enclose! { (selection) move ||
                    if selection.borrow().is_some() {
                        ButtonEnabledState::Shown
                    } else {
                        ButtonEnabledState::Hide
                    }
                }))
                .handler(Box::new(enclose! { (chosen) move || *chosen.borrow_mut() = true })),
        )?;

        let cash_out_button = Button::text(
            SDLPoint::new(475, 625),
            &format!("Pass (+{} Influence)", reward.cashout_influence),
            &render_context,
            text_renderer,
            true,
            true,
            ButtonDelegate::init().handler(Box::new(enclose! { (selection) move || *selection.borrow_mut() = Some(3)})),
        )?;
        Ok(RewardScene {
            text_renderer: Rc::clone(text_renderer),
            ui,
            ecs,
            reward,
            cards,
            accept_button,
            cash_out_button,
            selection: Rc::clone(&selection),
            chosen: Rc::clone(&chosen),
        })
    }

    fn apply_selection(&mut self) {
        let selection = self.selection.borrow().unwrap();
        let progression = &mut self.ecs.write_resource::<ProgressionComponent>();
        progression.state.influence += self.reward.influence;
        if selection < 3 {
            progression.state.items.insert(self.cards[selection as usize].equipment.name.to_string());
        } else {
            progression.state.influence += self.reward.cashout_influence;
        }
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
        canvas.copy(self.ui.get("reward_background.png"), None, None)?;

        if let Some(selection) = *self.selection.borrow() {
            let (mut selection_frame, image) = if selection < 3 {
                (self.cards[selection as usize].frame, "card_frame_large_selection.png")
            } else {
                (self.cash_out_button.frame, "button_frame_full_selection.png")
            };
            selection_frame.offset(-2, -2);
            selection_frame.set_width(selection_frame.width() + 4);
            selection_frame.set_height(selection_frame.height() + 4);
            canvas.copy(self.ui.get(image), None, selection_frame)?;
        }

        for c in &self.cards {
            c.render(&self.ecs, canvas, frame)?;
        }

        let mut current_influence = self.reward.influence;
        if let Some(selection) = *self.selection.borrow() {
            if selection == 3 {
                current_influence += self.reward.cashout_influence;
            }
        }
        self.text_renderer.render_text(
            &format!("Influence Reward: {}", current_influence),
            780,
            550,
            canvas,
            FontSize::Bold,
            FontColor::White,
        )?;

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
            self.apply_selection();
            StageDirection::ShowCharacter(std::mem::replace(&mut self.ecs, World::new()))
        } else {
            StageDirection::Continue
        }
    }
}
