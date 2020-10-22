use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
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
            ui: IconCache::init(
                &render_context_holder.borrow(),
                IconLoader::init_ui(),
                &["skill_tree_frame.png", "skill_tree_frame_inactive.png"],
            )?,
        })
    }
}

const SKILL_NODE_SIZE: u32 = 48;
const SKILL_BORDER: u32 = 2;

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

        let mut dependencies = vec![];

        let node_frame = self.ui.get("skill_tree_frame.png");
        let node_frame_inactive = self.ui.get("skill_tree_frame_inactive.png");
        let all = self.tree.all(&self.progression);
        for (node, status) in &all {
            let border = match status {
                SkillNodeStatus::Selected => node_frame,
                SkillNodeStatus::Available => node_frame_inactive,
                SkillNodeStatus::Unavailable => node_frame_inactive,
            };
            canvas.copy(
                border,
                None,
                SDLRect::new(
                    node.position.x as i32 - SKILL_BORDER as i32,
                    node.position.y as i32 - SKILL_BORDER as i32,
                    SKILL_NODE_SIZE + (SKILL_BORDER * 2),
                    SKILL_NODE_SIZE + (SKILL_BORDER * 2),
                ),
            )?;

            let node_rect = SDLRect::new(node.position.x as i32, node.position.y as i32, SKILL_NODE_SIZE, SKILL_NODE_SIZE);
            canvas.copy(self.icons.get(&node.image.as_ref().unwrap()), None, node_rect)?;

            if let Some(color) = match status {
                SkillNodeStatus::Selected => None,
                SkillNodeStatus::Available | SkillNodeStatus::Unavailable => Some(Color::RGBA(12, 12, 12, 156)),
            } {
                canvas.set_draw_color(color);
                canvas.fill_rect(node_rect)?;
            }

            for d in &node.dependencies {
                dependencies.push((d.to_string(), node.name.to_string()));
            }
        }

        for d in dependencies {
            let left = all.iter().map(|a| &a.0).find(|a| a.name == d.0).unwrap();
            let right = all.iter().map(|a| &a.0).find(|a| a.name == d.1).unwrap();

            if self.progression.skills.contains(&d.0) {
                canvas.set_draw_color(Color::from((218, 218, 218)));
            } else {
                canvas.set_draw_color(Color::from((45, 45, 45)));
            }

            let left = SDLPoint::new(
                (left.position.x + SKILL_NODE_SIZE + SKILL_BORDER) as i32,
                (left.position.y + (SKILL_NODE_SIZE / 2)) as i32,
            );
            let right = SDLPoint::new((right.position.x - SKILL_BORDER) as i32, (right.position.y + (SKILL_NODE_SIZE / 2)) as i32);

            // Draw a straight line
            if left.y == right.y {
                canvas.draw_line(left, right)?;
            } else {
                // Draw half distance over
                let mid_top = SDLPoint::new((left.x() + right.x()) / 2, left.y());
                canvas.draw_line(left, mid_top)?;

                // // Then down the y distance
                let mid_bottom = SDLPoint::new((left.x() + right.x()) / 2, right.y());
                canvas.draw_line(mid_top, mid_bottom)?;

                // Then rest of the way over
                canvas.draw_line(mid_bottom, right)?;
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
