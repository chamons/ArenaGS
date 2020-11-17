use std::cell::RefCell;
use std::rc::Rc;
use std::slice;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::equipment_view::EquipmentView;
use super::merchant_view::MerchantView;
use super::next_battle_view::NextBattleView;
use super::profession_tree::ProfessionTreeView;
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::conductor::{Scene, StageDirection};
use crate::props::{HelpPopup, TabInfo, TabView, View};

pub struct CharacterScene {
    next_fight: Rc<RefCell<bool>>,
    tab: Box<dyn View>,
    ecs: World,
    help: HelpPopup,
}

impl CharacterScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>, ecs: World) -> BoxResult<CharacterScene> {
        let render_context = &render_context_holder.borrow();
        let next_fight = Rc::new(RefCell::new(false));

        Ok(CharacterScene {
            tab: Box::from(TabView::init(
                SDLPoint::new(0, 0),
                render_context,
                text_renderer,
                vec![
                    TabInfo::init("Profession", Box::new(ProfessionTreeView::init(render_context, text_renderer, &ecs)?)),
                    TabInfo::init("Equipment", Box::new(EquipmentView::init(render_context, text_renderer, &ecs)?)),
                    TabInfo::init("Merchant", Box::new(MerchantView::init(render_context, text_renderer, &ecs)?)),
                    TabInfo::init("Next Battle", Box::new(NextBattleView::init(render_context, text_renderer, &ecs, &next_fight)?)),
                ],
            )?),
            help: HelpPopup::init(&ecs, &render_context, Rc::clone(&text_renderer))?,
            ecs,
            next_fight,
        })
    }
}

impl Scene for CharacterScene {
    fn handle_key(&mut self, keycode: Keycode, keymod: Mod) {
        self.help.handle_key(&self.ecs, keycode, keymod);
    }

    fn handle_mouse_click(&mut self, x: i32, y: i32, button: Option<MouseButton>) {
        // So a bit of weirdness here - the world state works for everything EXCEPT
        // the next battle "preview world", since that will have unique skills
        // we may not have had last battle (the current snapshot)
        // So ask our current tab view for custom_help_context and use it if provided
        if self.help.handle_mouse_event(&mut self.ecs, x, y, button, slice::from_ref(&self.tab)) {
            return;
        }

        self.tab.handle_mouse_click(&mut self.ecs, x, y, button);
    }

    fn handle_mouse_move(&mut self, x: i32, y: i32, state: MouseState) {
        self.help.handle_mouse_move(&self.ecs, x, y, state);
        self.tab.handle_mouse_move(&self.ecs, x, y, state);
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        self.tab.render(&self.ecs, canvas, frame)?;
        self.help.render(&self.ecs, canvas, frame)?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&mut self) -> StageDirection {
        if *self.next_fight.borrow() {
            StageDirection::NewRound(std::mem::replace(&mut self.ecs, World::new()))
        } else {
            StageDirection::Continue
        }
    }
}
