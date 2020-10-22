use std::rc::Rc;

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
use crate::props::View;

pub struct CharacterScene {
    interacted: bool,
    text_renderer: Rc<TextRenderer>,
    tree: SkillTreeView,
    world: World,
}

impl CharacterScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>, progression: ProgressionState) -> BoxResult<CharacterScene> {
        Ok(CharacterScene {
            interacted: false,
            text_renderer: Rc::clone(text_renderer),
            tree: SkillTreeView::init(SDLPoint::new(10, 10), &render_context_holder.borrow(), &progression)?,
            world: {
                let mut world = World::new();
                world.insert(progression);
                world
            },
        })
    }
}

impl Scene for CharacterScene {
    fn handle_key(&mut self, _keycode: Keycode, _keymod: Mod) {}

    fn handle_mouse(&mut self, _x: i32, _y: i32, button: Option<MouseButton>) {
        if button.is_some() {
            self.interacted = true;
        }
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        self.text_renderer.render_text("Character", 50, 50, canvas, FontSize::Large, FontColor::White)?;

        self.tree.render(&self.world, canvas, frame)?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        if self.interacted {
            StageDirection::NewRound(wrap_progression(&self.world.read_resource::<ProgressionState>()))
        } else {
            StageDirection::Continue
        }
    }
}
