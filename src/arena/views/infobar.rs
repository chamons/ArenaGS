use std::cell::RefCell;
use std::rc::Rc;

use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::view_components::{Frame, FrameKind};
use super::{render_text_layout, HitTestResult, TextHitTester, View};
use crate::after_image::prelude::*;
use crate::after_image::LayoutRequest;
use crate::atlas::prelude::*;
use crate::clash::{find_enemies, find_player, summarize_character};

pub struct InfoBarView {
    position: SDLPoint,
    text: Rc<TextRenderer>,
    frame: Frame,
    hit_tester: RefCell<TextHitTester>,
}

impl InfoBarView {
    pub fn init(position: SDLPoint, render_context: &RenderContext, text: Rc<TextRenderer>) -> BoxResult<InfoBarView> {
        Ok(InfoBarView {
            position,
            text,
            frame: Frame::init(SDLPoint::new(position.x() - 27, position.y() - 20), render_context, FrameKind::InfoBar)?,
            hit_tester: RefCell::new(TextHitTester::init()),
        })
    }

    fn render_character_info(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let mut offset = 5;
        self.render_character(canvas, ecs, find_player(&ecs), &mut offset, false)?;
        offset += 40;

        for e in find_enemies(&ecs) {
            self.small_text(canvas, "Enemy:", &mut offset)?;
            self.render_character(canvas, ecs, e, &mut offset, true)?;
            offset += 20;
        }
        Ok(())
    }

    fn render_character(&self, canvas: &mut RenderCanvas, ecs: &World, entity: Entity, offset: &mut i32, show_status_effect: bool) -> BoxResult<()> {
        summarize_character(ecs, entity, show_status_effect, true, |t| self.small_text(canvas, t, offset).unwrap());
        Ok(())
    }

    const MAX_INFO_OFFSET: i32 = 480;
    fn small_text(&self, canvas: &mut RenderCanvas, text: &str, offset: &mut i32) -> BoxResult<()> {
        let mut hit_test = self.hit_tester.borrow_mut();

        if *offset > InfoBarView::MAX_INFO_OFFSET {
            return Ok(());
        }

        let layout = self.text.layout_text(
            &text,
            FontSize::Small,
            LayoutRequest::init(self.position.x as u32 + 4, self.position.y as u32 + *offset as u32, 210, 2),
        )?;
        render_text_layout(&layout, canvas, &self.text, None, FontColor::Black, 0, false, |rect, result| {
            hit_test.add(rect, result);
        })?;

        *offset += 20;
        Ok(())
    }
}

impl View for InfoBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        {
            let mut hit_test = self.hit_tester.borrow_mut();
            hit_test.clear();
        }

        self.frame.render(ecs, canvas, frame)?;
        self.render_character_info(ecs, canvas)?;

        Ok(())
    }

    fn hit_test(&self, _ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        self.hit_tester.borrow().hit_test(x, y)
    }
}
