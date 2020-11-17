use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{CharacterWeaponKind, EquipmentResource, ProgressionComponent, SkillNodeStatus, SkillTree, SkillTreeNode};
use crate::enclose;
use crate::props::{Button, ButtonDelegate, ButtonEnabledState, HitTestResult, View};

pub struct ProfessionTreeView {
    icons: IconCache,
    ui: IconCache,
    tree: Rc<SkillTree>,
    text_renderer: Rc<TextRenderer>,
    selection: Rc<RefCell<Option<String>>>,
    accept_button: Button,
}

pub const SKILL_NODE_SIZE: u32 = 48;
pub const SKILL_BORDER: u32 = 2;

pub fn get_tree(ecs: &World) -> Vec<SkillTreeNode> {
    let equipment = &ecs.read_resource::<EquipmentResource>();
    match ecs.read_resource::<ProgressionComponent>().state.weapon {
        CharacterWeaponKind::Gunslinger => crate::clash::content::gunslinger::get_skill_tree(equipment),
    }
}

pub fn get_tree_icons(render_context: &RenderContext, tree: &SkillTree) -> BoxResult<IconCache> {
    let tree_icons = tree.icons();
    Ok(IconCache::init(&render_context, IconLoader::init_icons(), &tree_icons[..])?)
}

impl ProfessionTreeView {
    pub fn init(render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, ecs: &World) -> BoxResult<ProfessionTreeView> {
        let tree = Rc::new(SkillTree::init(&get_tree(ecs)));

        let selection: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let accept_button = Button::text(
            SDLPoint::new(800, 650),
            "Purchase",
            &render_context,
            text_renderer,
            ButtonDelegate::init()
                .enabled(Box::new(enclose! { (selection, tree) move |ecs| {
                    let selection = selection.borrow_mut();
                    if let Some(selection) = (*selection).as_ref() {
                        if ProfessionTreeView::can_apply_selection(ecs, &tree, &selection) {
                            ButtonEnabledState::Shown
                        }
                        else {
                            ButtonEnabledState::Ghosted
                        }
                    }
                    else  {
                        ButtonEnabledState::Hide
                    }
                }}))
                .handler(Box::new(enclose! { (selection, tree) move |ecs| {
                    let mut selection = selection.borrow_mut();
                    if ProfessionTreeView::can_apply_selection(ecs, &tree, selection.as_ref().unwrap()) {
                        ProfessionTreeView::apply_selection(ecs, &tree, selection.as_ref().unwrap());
                        *selection = None;
                    }
                }})),
        )?;

        Ok(ProfessionTreeView {
            icons: get_tree_icons(render_context, &tree)?,
            ui: IconCache::init(
                &render_context,
                IconLoader::init_ui(),
                &["skill_tree_frame.png", "skill_tree_frame_selected.png", "skill_tree_frame_inactive.png"],
            )?,
            tree,
            text_renderer: Rc::clone(&text_renderer),
            selection,
            accept_button,
        })
    }

    fn find_node_at(&self, ecs: &World, x: i32, y: i32) -> Option<SkillTreeNode> {
        let progression = &(*ecs.read_resource::<ProgressionComponent>()).state;
        self.tree
            .all(&progression)
            .iter()
            .filter_map(|(node, _)| {
                let position = SDLRect::new(node.position.x as i32, node.position.y as i32, SKILL_NODE_SIZE, SKILL_NODE_SIZE);
                if position.contains_point(SDLPoint::new(x, y)) {
                    Some(node)
                } else {
                    None
                }
            })
            .next()
            .cloned()
    }

    fn can_apply_selection(ecs: &World, tree: &SkillTree, hit: &str) -> bool {
        let progression = &ecs.read_resource::<ProgressionComponent>().state;
        tree.can_select(&progression, &hit)
    }

    fn apply_selection(ecs: &World, tree: &SkillTree, hit: &str) {
        let mut progression = &mut ecs.write_resource::<ProgressionComponent>().state;
        tree.select(&mut progression, &hit);
    }
}

impl View for ProfessionTreeView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let mut dependencies = vec![];

        // Scope progression so we can re-borrow in accept button render
        {
            let selection = self.selection.borrow().clone();
            let node_frame = self.ui.get("skill_tree_frame.png");
            let node_frame_inactive = self.ui.get("skill_tree_frame_inactive.png");
            let node_frame_selected = self.ui.get("skill_tree_frame_selected.png");

            let progression = &(*ecs.read_resource::<ProgressionComponent>()).state;

            self.text_renderer.render_text(
                &format!("Influence: {}", progression.influence),
                800,
                585,
                canvas,
                FontSize::Large,
                FontColor::Brown,
            )?;

            let all = self.tree.all(&progression);
            for (node, status) in &all {
                let border = match status {
                    SkillNodeStatus::Selected => node_frame,
                    SkillNodeStatus::Available => {
                        if selection == Some(node.name()) {
                            node_frame_selected
                        } else {
                            node_frame_inactive
                        }
                    }
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
                canvas.copy(self.icons.get(&node.image().as_ref().unwrap()), None, node_rect)?;

                if let Some(color) = match status {
                    SkillNodeStatus::Selected => None,
                    SkillNodeStatus::Available | SkillNodeStatus::Unavailable => Some(Color::RGBA(12, 12, 12, 156)),
                } {
                    canvas.set_draw_color(color);
                    canvas.fill_rect(node_rect)?;
                }

                for d in &node.dependencies {
                    dependencies.push((d.to_string(), node.name().to_string()));
                }
            }

            for d in dependencies {
                let left = all.iter().map(|a| &a.0).find(|a| a.name() == d.0).unwrap();
                let right = all.iter().map(|a| &a.0).find(|a| a.name() == d.1).unwrap();

                if progression.items.contains(&d.0) {
                    canvas.set_draw_color(Color::from((218, 218, 218)));
                } else {
                    canvas.set_draw_color(Color::from((45, 45, 45)));
                }

                let left = SDLPoint::new(
                    (left.position.x + SKILL_NODE_SIZE + SKILL_BORDER) as i32,
                    (left.position.y + (SKILL_NODE_SIZE / 2)) as i32,
                );
                let right = SDLPoint::new((right.position.x - SKILL_BORDER - 1) as i32, (right.position.y + (SKILL_NODE_SIZE / 2)) as i32);

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

            if let Some(selection) = selection {
                self.text_renderer.render_text_centered(
                    &format!("Cost: {}", self.tree.cost(&selection)),
                    800,
                    625,
                    145,
                    canvas,
                    FontSize::Bold,
                    FontColor::Brown,
                )?;
            }
        }

        self.accept_button.render(&ecs, canvas, frame)?;

        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        if let Some(button) = button {
            if button == MouseButton::Left {
                if let Some(hit) = self.find_node_at(ecs, x, y) {
                    *self.selection.borrow_mut() = Some(hit.name());
                }
            }
        }

        self.accept_button.handle_mouse_click(ecs, x, y, button);
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        if let Some(hit) = self.find_node_at(ecs, x, y) {
            Some(HitTestResult::Skill(hit.name()))
        } else {
            None
        }
    }
}
