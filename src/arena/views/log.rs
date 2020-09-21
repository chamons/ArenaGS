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
        for (i, entry) in log.get(log.index, LOG_COUNT).iter().enumerate() {
            self.text.render_text(
                &entry,
                self.position.x,
                self.position.y + (i as i32 * 20) + 15,
                canvas,
                FontSize::Tiny,
                FontColor::Black,
            )?;
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
