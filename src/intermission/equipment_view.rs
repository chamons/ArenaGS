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

pub struct EquipmentView {
    position: SDLPoint,
    ui: IconCache,
}

impl EquipmentView {
    pub fn init(
        position: SDLPoint,
        render_context: &RenderContext,
        text_renderer: &Rc<TextRenderer>,
        progression: &ProgressionState,
    ) -> BoxResult<EquipmentView> {
        Ok(EquipmentView {
            position,
            ui: IconCache::init(&render_context, IconLoader::init_ui(), &["card_frame.png"])?,
        })
    }
}

impl View for EquipmentView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        Ok(())
    }

    fn handle_mouse(&mut self, ecs: &World, x: i32, y: i32, button: Option<MouseButton>) {
        if let Some(button) = button {
            if button == MouseButton::Left {}
        }
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        None
    }
}
