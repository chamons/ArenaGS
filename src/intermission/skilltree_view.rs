use std::rc::Rc;

use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{CharacterWeaponKind, ProgressionState, SkillNodeStatus, SkillTree, SkillTreeNode};
use crate::props::{HitTestResult, View};

pub struct SkillTreeView {
    position: SDLPoint,
    icons: IconCache,
    ui: IconCache,
    tree: SkillTree,
    text_renderer: Rc<TextRenderer>,
}

const SKILL_NODE_SIZE: u32 = 48;
const SKILL_BORDER: u32 = 2;

fn get_tree(kind: &CharacterWeaponKind) -> Vec<SkillTreeNode> {
    match kind {
        CharacterWeaponKind::Gunslinger => crate::clash::content::gunslinger::get_skill_tree(),
    }
}

impl SkillTreeView {
    pub fn init(
        position: SDLPoint,
        render_context: &RenderContext,
        text_renderer: &Rc<TextRenderer>,
        progression: &ProgressionState,
    ) -> BoxResult<SkillTreeView> {
        let tree = SkillTree::init(&get_tree(&progression.weapon));
        let tree_icons = tree.icons();

        Ok(SkillTreeView {
            position,
            icons: IconCache::init(&render_context, IconLoader::init_icons(), &tree_icons[..])?,
            ui: IconCache::init(
                &render_context,
                IconLoader::init_ui(),
                &["skill_tree_frame.png", "skill_tree_frame_inactive.png"],
            )?,
            tree,
            text_renderer: Rc::clone(&text_renderer),
        })
    }

    fn find_node_at(&self, ecs: &World, x: i32, y: i32) -> Option<SkillTreeNode> {
        let progression = ecs.read_resource::<ProgressionState>();
        self.tree
            .all(&progression)
            .iter()
            .filter_map(|(node, _)| {
                let position = SDLRect::new(
                    self.position.x() + node.position.x as i32,
                    self.position.x() + node.position.y as i32,
                    SKILL_NODE_SIZE,
                    SKILL_NODE_SIZE,
                );
                if position.contains_point(SDLPoint::new(x, y)) {
                    Some(node)
                } else {
                    None
                }
            })
            .next()
            .cloned()
    }
}

impl View for SkillTreeView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        let mut dependencies = vec![];

        let node_frame = self.ui.get("skill_tree_frame.png");
        let node_frame_inactive = self.ui.get("skill_tree_frame_inactive.png");
        let progression = ecs.read_resource::<ProgressionState>();

        self.text_renderer.render_text(
            &format!("Experience: {}", progression.experience),
            770,
            70,
            canvas,
            FontSize::Large,
            FontColor::Brown,
        )?;

        let all = self.tree.all(&progression);
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
                    self.position.x() + node.position.x as i32 - SKILL_BORDER as i32,
                    self.position.y() + node.position.y as i32 - SKILL_BORDER as i32,
                    SKILL_NODE_SIZE + (SKILL_BORDER * 2),
                    SKILL_NODE_SIZE + (SKILL_BORDER * 2),
                ),
            )?;

            let node_rect = SDLRect::new(
                self.position.x() + node.position.x as i32,
                self.position.y() + node.position.y as i32,
                SKILL_NODE_SIZE,
                SKILL_NODE_SIZE,
            );
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

            if progression.skills.contains(&d.0) {
                canvas.set_draw_color(Color::from((218, 218, 218)));
            } else {
                canvas.set_draw_color(Color::from((45, 45, 45)));
            }

            let left = SDLPoint::new(
                self.position.x() + (left.position.x + SKILL_NODE_SIZE + SKILL_BORDER) as i32,
                self.position.y() + (left.position.y + (SKILL_NODE_SIZE / 2)) as i32,
            );
            let right = SDLPoint::new(
                self.position.x() + (right.position.x - SKILL_BORDER - 1) as i32,
                self.position.y() + (right.position.y + (SKILL_NODE_SIZE / 2)) as i32,
            );

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

        Ok(())
    }

    fn handle_mouse(&mut self, ecs: &World, x: i32, y: i32, button: Option<MouseButton>) {
        if let Some(button) = button {
            if button == MouseButton::Left {
                if let Some(hit) = self.find_node_at(ecs, x, y) {
                    let mut progression = ecs.write_resource::<ProgressionState>();
                    self.tree.select(&mut progression, &hit.name);
                }
            }
        }
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        if let Some(hit) = self.find_node_at(ecs, x, y) {
            Some(HitTestResult::Skill(hit.name))
        } else {
            None
        }
    }
}
