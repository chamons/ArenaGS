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
    large: bool,
}

pub const CARD_WIDTH: u32 = 110;
pub const CARD_WIDTH_LARGE: u32 = 160;

pub const CARD_HEIGHT: u32 = 110;
pub const CARD_HEIGHT_LARGE: u32 = 220;

impl CardView {
    pub fn init(
        position: SDLPoint,
        text_renderer: &Rc<TextRenderer>,
        ui: &Rc<IconCache>,
        icons: &Rc<IconCache>,
        equipment: EquipmentItem,
        large: bool,
    ) -> BoxResult<CardView> {
        Ok(CardView {
            text_renderer: Rc::clone(&text_renderer),
            ui: Rc::clone(&ui),
            icons: Rc::clone(&icons),
            equipment,
            grabbed: None,
            z_order: 0,
            large,
            frame: SDLRect::new(
                position.x(),
                position.y(),
                if large { CARD_WIDTH_LARGE } else { CARD_WIDTH },
                if large { CARD_HEIGHT_LARGE } else { CARD_HEIGHT },
            ),
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
        if self.large {
            canvas.copy(self.ui.get("card_frame_large.png"), None, self.frame)?;
        } else {
            canvas.copy(self.ui.get("card_frame.png"), None, self.frame)?;
        }

        let card_width = if self.large { CARD_WIDTH_LARGE } else { CARD_WIDTH };
        let card_height = if self.large { CARD_HEIGHT_LARGE } else { CARD_HEIGHT };

        if let Some(image) = &self.equipment.image {
            let image_rect = SDLRect::new(
                (self.frame.x() + (card_width as i32 / 2) - (SKILL_NODE_SIZE as i32 / 4)) as i32,
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

        const CARD_TEXT_WIDTH_BORDER: u32 = 38;

        let layout = self.text_renderer.layout_text(
            &self.equipment.name,
            FontSize::Small,
            LayoutRequest::init(
                self.frame.x() as u32 + 18,
                self.frame.y() as u32 + SKILL_NODE_SIZE / 2 + 20 + 10,
                card_width - CARD_TEXT_WIDTH_BORDER,
                0,
            ),
        )?;

        render_text_layout(
            &layout,
            canvas,
            &self.text_renderer,
            RenderTextOptions::init(FontColor::Brown).with_centered(Some(card_width - CARD_TEXT_WIDTH_BORDER)),
            |_, _| {},
        )?;

        let rarity = match self.equipment.rarity {
            crate::clash::EquipmentRarity::Standard => " ",
            crate::clash::EquipmentRarity::Common => "C",
            crate::clash::EquipmentRarity::Uncommon => "U",
            crate::clash::EquipmentRarity::Rare => "R",
        };

        if self.large {
            let mut y = 50;

            for d in self.equipment.description() {
                let layout = self.text_renderer.layout_text(
                    &d,
                    FontSize::Tiny,
                    LayoutRequest::init(
                        self.frame.x() as u32 + 18,
                        self.frame.y() as u32 + SKILL_NODE_SIZE / 2 + 20 + y,
                        card_width - CARD_TEXT_WIDTH_BORDER,
                        0,
                    ),
                )?;

                render_text_layout(
                    &layout,
                    canvas,
                    &self.text_renderer,
                    RenderTextOptions::init(FontColor::Brown)
                        .with_centered(Some(card_width - CARD_TEXT_WIDTH_BORDER))
                        .with_font_size(FontSize::Tiny),
                    |_, _| {},
                )?;

                y += layout.line_count * 22;
            }

            self.text_renderer.render_text(
                rarity,
                self.frame.x() + card_width as i32 - 27,
                self.frame.y() + card_height as i32 - 28,
                canvas,
                FontSize::Tiny,
                FontColor::LightBrown,
            )?;
        }
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
