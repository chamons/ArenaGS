use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{wrap_progression, CharacterWeaponKind, ProgressionState, SkillNodeStatus, SkillTree, SkillTreeNode};
use crate::conductor::{Scene, StageDirection};

pub struct CharacterScene {
    interacted: bool,
    progression: ProgressionState,
    text_renderer: Rc<TextRenderer>,
    tree: SkillTree,
    icons: IconCache,
    ui: IconCache,
}

fn get_tree(kind: &CharacterWeaponKind) -> Vec<SkillTreeNode> {
    match kind {
        CharacterWeaponKind::Gunslinger => crate::clash::content::gunslinger::get_skill_tree(),
    }
}

impl CharacterScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>, progression: ProgressionState) -> BoxResult<CharacterScene> {
        let tree = SkillTree::init(&get_tree(&progression.weapon));
        let tree_icons = tree.icons();
        Ok(CharacterScene {
            tree,
            interacted: false,
            progression,
            text_renderer: Rc::clone(text_renderer),
            icons: IconCache::init(&render_context_holder.borrow(), IconLoader::init_icons(), &tree_icons[..])?,
            ui: IconCache::init(&render_context_holder.borrow(), IconLoader::init_ui(), &["skill_tree_frame.png"])?,
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

    fn render(&mut self, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        let node_frame = self.ui.get("skill_tree_frame.png");
        for (node, status) in self.tree.all(&self.progression) {
            let (color, show_border) = match status {
                SkillNodeStatus::Selected => (None, true),
                SkillNodeStatus::Available => (Some(Color::RGBA(12, 12, 12, 48)), false),
                SkillNodeStatus::Unavailable => (Some(Color::RGBA(12, 12, 12, 218)), false),
            };

            if show_border {
                canvas.copy(
                    node_frame,
                    None,
                    SDLRect::new(node.position.x as i32 - 2, node.position.y as i32 - 2, 48 + 4, 48 + 4),
                )?;
            }

            let node_rect = SDLRect::new(node.position.x as i32, node.position.y as i32, 48, 48);
            canvas.copy(self.icons.get(&node.image.unwrap()), None, node_rect)?;

            if let Some(color) = color {
                canvas.set_draw_color(color);
                canvas.fill_rect(node_rect)?;
            }
        }

        self.text_renderer.render_text("Character", 50, 50, canvas, FontSize::Large, FontColor::White)?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        if self.interacted {
            StageDirection::NewRound(wrap_progression(&self.progression))
        } else {
            StageDirection::Continue
        }
    }
}
