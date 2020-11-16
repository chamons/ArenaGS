use std::cell::RefCell;
use std::rc::Rc;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::after_image::LayoutRequest;
use crate::atlas::prelude::*;
use crate::clash::{find_enemies, find_player, summarize_character};
use crate::props::{render_text_layout, Frame, FrameKind, HitTestResult, RenderTextOptions, TextHitTester, View, CARD_HEIGHT_LARGE, CARD_WIDTH_LARGE};

#[derive(is_enum_variant)]
enum InfoBarFormat {
    Sidebar(Frame),
    Card(Texture, SDLRect),
}

pub struct InfoBarView {
    position: SDLPoint,
    text: Rc<TextRenderer>,
    hit_tester: RefCell<TextHitTester>,
    format: InfoBarFormat,
}

impl InfoBarView {
    pub fn init(position: SDLPoint, render_context: &RenderContext, text: Rc<TextRenderer>, card_view: bool) -> BoxResult<InfoBarView> {
        let format = if !card_view {
            InfoBarFormat::Sidebar(Frame::init(
                SDLPoint::new(position.x() - 27, position.y() - 20),
                render_context,
                FrameKind::InfoBar,
            )?)
        } else {
            InfoBarFormat::Card(
                IconLoader::init_ui().get(render_context, "card_frame_large.png")?,
                SDLRect::new(position.x() - 27, position.y() - 20, CARD_WIDTH_LARGE, CARD_HEIGHT_LARGE),
            )
        };
        Ok(InfoBarView {
            position,
            text,
            format,
            hit_tester: RefCell::new(TextHitTester::init()),
        })
    }

    fn render_character_info(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let mut offset = 5;
        self.render_character(canvas, ecs, find_player(&ecs), &mut offset, false)?;
        offset += 40;

        if self.format.is_sidebar() {
            for e in find_enemies(&ecs) {
                self.small_text(canvas, "Enemy:", &mut offset)?;
                self.render_character(canvas, ecs, e, &mut offset, true)?;
                offset += 20;
            }
        }
        Ok(())
    }

    fn render_character(&self, canvas: &mut RenderCanvas, ecs: &World, entity: Entity, offset: &mut i32, show_status_effect: bool) -> BoxResult<()> {
        summarize_character(ecs, entity, show_status_effect, self.format.is_sidebar(), |t| {
            self.small_text(canvas, t, offset).unwrap()
        });
        Ok(())
    }

    const MAX_INFO_OFFSET: i32 = 480;
    fn small_text(&self, canvas: &mut RenderCanvas, text: &str, offset: &mut i32) -> BoxResult<()> {
        let mut hit_test = self.hit_tester.borrow_mut();

        if *offset > InfoBarView::MAX_INFO_OFFSET {
            return Ok(());
        }

        let x_offset = if self.format.is_sidebar() { 4 } else { -6 };
        let layout = self.text.layout_text(
            &text,
            FontSize::Small,
            LayoutRequest::init((self.position.x + x_offset) as u32, self.position.y as u32 + *offset as u32, 210, 2),
        )?;
        render_text_layout(&layout, canvas, &self.text, RenderTextOptions::init(FontColor::Black), |rect, result| {
            hit_test.add(rect, result);
        })?;

        *offset += if self.format.is_sidebar() { 20 } else { 25 };
        Ok(())
    }
}

impl View for InfoBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        {
            let mut hit_test = self.hit_tester.borrow_mut();
            hit_test.clear();
        }

        match &self.format {
            InfoBarFormat::Sidebar(sidebar_frame) => sidebar_frame.render(ecs, canvas, frame)?,
            InfoBarFormat::Card(card, rect) => canvas.copy(&card, None, *rect)?,
        }

        self.render_character_info(ecs, canvas)?;

        Ok(())
    }

    fn hit_test(&self, _ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        self.hit_tester.borrow().hit_test(x, y)
    }
}
