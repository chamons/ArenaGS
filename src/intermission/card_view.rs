use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::skilltree_view::SKILL_NODE_SIZE;
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{EquipmentItem, EquipmentKinds};
use crate::props::{render_text_layout, HitTestResult, RenderTextOptions, View};

pub struct CardView {
    pub frame: SDLRect,
    text_renderer: Rc<TextRenderer>,
    ui: Rc<IconCache>,
    icons: Rc<IconCache>,
    pub equipment: EquipmentItem,
    pub grabbed: Option<SDLPoint>,
    pub z_order: u32,
}

pub const CARD_WIDTH: u32 = 110;
pub const CARD_HEIGHT: u32 = 110;

impl CardView {
    pub fn init(
        position: SDLPoint,
        text_renderer: &Rc<TextRenderer>,
        ui: &Rc<IconCache>,
        icons: &Rc<IconCache>,
        equipment: EquipmentItem,
    ) -> BoxResult<CardView> {
        Ok(CardView {
            frame: SDLRect::new(position.x(), position.y(), CARD_WIDTH, CARD_HEIGHT),
            text_renderer: Rc::clone(&text_renderer),
            ui: Rc::clone(&ui),
            icons: Rc::clone(&icons),
            equipment,
            grabbed: None,
            z_order: 0,
        })
    }

    fn border_color(&self) -> Color {
        match self.equipment.kind {
            EquipmentKinds::Weapon => Color::RGB(127, 0, 0),
            EquipmentKinds::Armor => Color::RGB(0, 7, 150),
            EquipmentKinds::Accessory => Color::RGB(26, 86, 0),
            EquipmentKinds::Mastery => Color::RGB(111, 38, 193),
        }
    }
}

impl View for CardView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.copy(self.ui.get("card_frame.png"), None, self.frame)?;

        if let Some(image) = &self.equipment.image {
            let image_rect = SDLRect::new(
                (self.frame.x() + (CARD_WIDTH as i32 / 2) - (SKILL_NODE_SIZE as i32 / 4)) as i32,
                self.frame.y() + 20,
                SKILL_NODE_SIZE / 2,
                SKILL_NODE_SIZE / 2,
            );

            canvas.set_draw_color(self.border_color());
            canvas.fill_rect(SDLRect::new(
                image_rect.x() - 3,
                image_rect.y() - 3,
                image_rect.width() + 6,
                image_rect.height() + 6,
            ))?;

            canvas.copy(self.icons.get(&image), None, image_rect)?;
        }

        let layout = self.text_renderer.layout_text(
            &self.equipment.name,
            FontSize::Small,
            LayoutRequest::init(
                self.frame.x() as u32 + 14,
                self.frame.y() as u32 + SKILL_NODE_SIZE / 2 + 20 + 10,
                CARD_WIDTH - 30,
                0,
            ),
        )?;

        render_text_layout(
            &layout,
            canvas,
            &self.text_renderer,
            RenderTextOptions::init(FontColor::Brown).with_centered(Some(CARD_WIDTH - 28)),
            |_, _| {},
        )?;
        Ok(())
    }

    fn handle_mouse_click(&mut self, _ecs: &World, x: i32, y: i32, button: Option<MouseButton>) {
        if let Some(button) = button {
            if button == MouseButton::Left {
                if self.frame.contains_point(SDLPoint::new(x, y)) {
                    self.grabbed = Some(SDLPoint::new(x - self.frame.x(), y - self.frame.y()));
                    return;
                }
            }
        }
    }

    fn handle_mouse_move(&mut self, _ecs: &World, x: i32, y: i32, state: MouseState) {
        if let Some(origin) = self.grabbed {
            if state.left() {
                self.frame = SDLRect::new(x - origin.x(), y - origin.y(), CARD_WIDTH, CARD_HEIGHT);
            } else {
                self.grabbed = None;
            }
        }
    }

    fn hit_test(&self, _ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        if self.frame.contains_point(SDLPoint::new(x, y)) {
            Some(HitTestResult::Skill(self.equipment.name.clone()))
        } else {
            None
        }
    }
}
