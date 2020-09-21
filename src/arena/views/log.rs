use std::rc::Rc;

use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::view_components::{Frame, FrameKind};
use super::{ContextData, View};
use crate::after_image::{FontColor, FontSize, IconLoader, LayoutRequest, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::BoxResult;
use crate::clash::{LogComponent, LOG_COUNT};

pub struct LogView {
    position: SDLPoint,
    text: Rc<TextRenderer>,
    frame: Frame,
}

impl LogView {
    pub fn init(position: SDLPoint, render_context: &RenderContext, text: Rc<TextRenderer>) -> BoxResult<LogView> {
        Ok(LogView {
            position,
            text,
            frame: Frame::init(
                SDLPoint::new(position.x() - 27, position.y() - 9),
                render_context,
                &IconLoader::init_ui()?,
                FrameKind::Log,
            )?,
        })
    }

    fn render_log(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let log = &ecs.read_resource::<LogComponent>().log;
        let mut logs = vec![];
        let mut line_count = 0;

        // Start from the index and collect until you hit the front or LOG_COUNT lines
        for l in log.logs.iter().skip(log.index) {
            let layout = self.text.layout_text(
                &l,
                FontSize::Small,
                LayoutRequest::init(self.position.x as u32, self.position.y as u32 + 15, 210, 2),
            )?;
            if line_count + layout.line_count <= LOG_COUNT as u32 {
                line_count += layout.line_count;
                logs.push(layout);
            } else {
                break;
            }
        }

        // Then reverse the list to paint
        line_count = 0;
        for layout in logs.iter().rev() {
            for l in &layout.chunks {
                self.text.render_text(
                    &l.text,
                    l.position.x as i32,
                    l.position.y as i32 + 20 * line_count as i32,
                    canvas,
                    FontSize::Tiny,
                    FontColor::Black,
                )?;
            }
            line_count += layout.line_count;
        }

        Ok(())
    }
}

impl View for LogView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64, context: &ContextData) -> BoxResult<()> {
        self.frame.render(ecs, canvas, frame, context)?;
        self.render_log(ecs, canvas)?;

        Ok(())
    }
}
