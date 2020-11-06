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
use crate::props::View;

pub struct RewardScene {
    interacted: bool,
    text_renderer: Rc<TextRenderer>,
    ecs: World,
    reward: RewardsComponent,
    cards: Vec<CardView>,
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
                    CardView::init(SDLPoint::new(left + card_delta * i as i32, 300), text_renderer, &ui, &icons, s, true)
                        .expect("Unable to load equipment card")
                })
                .collect()
        };
        Ok(RewardScene {
            interacted: false,
            text_renderer: Rc::clone(text_renderer),
            ecs,
            reward,
            cards,
        })
    }
}

impl Scene for RewardScene {
    fn handle_key(&mut self, _keycode: Keycode, _keymod: Mod) {}

    fn handle_mouse_click(&mut self, _x: i32, _y: i32, button: Option<MouseButton>) {
        if button.is_some() {
            self.interacted = true;
        }
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        self.text_renderer.render_text("Reward", 50, 50, canvas, FontSize::Large, FontColor::White)?;

        for c in &self.cards {
            c.render(&self.ecs, canvas, frame)?;
        }

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&mut self) -> StageDirection {
        if self.interacted {
            StageDirection::ShowCharacter(std::mem::replace(&mut self.ecs, World::new()))
        } else {
            StageDirection::Continue
        }
    }
}
