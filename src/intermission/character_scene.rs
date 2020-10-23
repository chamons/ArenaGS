use std::cell::RefCell;
use std::rc::Rc;
use std::slice;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::skilltree_view::SkillTreeView;
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{wrap_progression, ProgressionState};
use crate::conductor::{Scene, StageDirection};
use crate::props::{Button, EmptyView, HelpPopup, TabInfo, TabView, View};

pub struct CharacterScene {
    next_fight: Rc<RefCell<bool>>,
    tab: Box<dyn View>,
    continue_button: Button,
    ecs: World,
    help: HelpPopup,
}

impl CharacterScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>, progression: ProgressionState) -> BoxResult<CharacterScene> {
        let render_context = &render_context_holder.borrow();
        let next_fight = Rc::new(RefCell::new(false));

        Ok(CharacterScene {
            next_fight: Rc::clone(&next_fight),
            continue_button: Button::text(
                SDLPoint::new(800, 650),
                "Next Fight",
                render_context,
                text_renderer,
                true,
                true,
                None,
                Some(Box::new(move || *next_fight.borrow_mut() = true)),
            )?,
            tab: Box::from(TabView::init(
                SDLPoint::new(0, 0),
                render_context,
                text_renderer,
                vec![
                    TabInfo::init(
                        "Skill Tree",
                        Box::new(SkillTreeView::init(SDLPoint::new(10, 10), render_context, text_renderer, &progression)?),
                        |_| true,
                    ),
                    TabInfo::init("Equipment", Box::new(EmptyView::init()?), |_| true),
                    TabInfo::init("Store", Box::new(EmptyView::init()?), |_| true),
                ],
            )?),
            ecs: {
                let mut ecs = World::new();
                ecs.insert(progression);
                ecs
            },
            help: HelpPopup::init(&render_context, Rc::clone(&text_renderer))?,
        })
    }
}

impl Scene for CharacterScene {
    fn handle_key(&mut self, keycode: Keycode, keymod: Mod) {
        self.help.handle_key(&self.ecs, keycode, keymod);
    }

    fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) {
        if self.help.handle_mouse_event(&self.ecs, x, y, button, slice::from_ref(&self.tab)) {
            return;
        }

        self.tab.handle_mouse(&self.ecs, x, y, button);
        self.continue_button.handle_mouse(&self.ecs, x, y, button);
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        self.tab.render(&self.ecs, canvas, frame)?;
        self.continue_button.render(&self.ecs, canvas, frame)?;
        self.help.render(&self.ecs, canvas, frame)?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        if *self.next_fight.borrow() {
            StageDirection::NewRound(wrap_progression(&self.ecs.read_resource::<ProgressionState>()))
        } else {
            StageDirection::Continue
        }
    }
}
