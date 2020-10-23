use std::cell::RefCell;
use std::rc::Rc;

use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::super::{LogIndexDelta, LogIndexPosition};
use crate::after_image::prelude::*;
use crate::after_image::LayoutRequest;
use crate::atlas::prelude::*;
use crate::clash::{EventKind, LogComponent, LogDirection, LOG_COUNT};
use crate::props::{render_text_layout, Frame, FrameKind, HitTestResult, TextHitTester, View};

pub struct LogView {
    position: SDLPoint,
    text: Rc<TextRenderer>,
    icons: IconCache,
    frame: Frame,
    hit_tester: RefCell<TextHitTester>,
}

impl LogView {
    pub fn init(position: SDLPoint, render_context: &RenderContext, text: Rc<TextRenderer>) -> BoxResult<LogView> {
        Ok(LogView {
            position,
            text,
            frame: Frame::init(SDLPoint::new(position.x() - 27, position.y() - 9), render_context, FrameKind::Log)?,
            icons: IconCache::init(render_context, IconLoader::init_symbols(), &["plain-dagger.png"])?,
            hit_tester: RefCell::new(TextHitTester::init()),
        })
    }

    fn find_end_index(&self, ecs: &World) -> BoxResult<usize> {
        let log = &ecs.read_resource::<LogComponent>().log;
        let mut line_count = 0;

        // Walk back counting until we hit LOG_SIZE lines
        for (i, l) in log.logs.iter().enumerate().rev() {
            let count = self.text.layout_text(&l, FontSize::Small, LayoutRequest::init(0, 0, 210, 0))?.line_count;
            if count + line_count > LOG_COUNT as u32 {
                return Ok(i + 1);
            }
            line_count += count;
        }

        // If we don't find enough, your end is 0
        Ok(0)
    }

    fn calculate_index(&self, ecs: &World) -> BoxResult<usize> {
        let mut index = ecs.read_resource::<LogIndexPosition>().index;
        match ecs.read_resource::<LogIndexPosition>().delta {
            LogIndexDelta::PageDown => index = std::cmp::min(index + LOG_COUNT, self.find_end_index(ecs)?),
            LogIndexDelta::PageUp => index = std::cmp::max(index as i32 - LOG_COUNT as i32, 0) as usize,
            LogIndexDelta::JumpToEnd => index = self.find_end_index(ecs)?,
            LogIndexDelta::None => {}
        }

        ecs.write_resource::<LogIndexPosition>().index = index;
        ecs.write_resource::<LogIndexPosition>().delta = LogIndexDelta::None;
        Ok(index)
    }

    fn render_log(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let mut hit_test = self.hit_tester.borrow_mut();
        hit_test.clear();

        let index = self.calculate_index(ecs)?;
        let log = &ecs.read_resource::<LogComponent>().log;
        let mut line_count = 0;

        for l in log.logs.iter().skip(index) {
            let layout = self.text.layout_text(
                &l,
                FontSize::Small,
                LayoutRequest::init(self.position.x as u32, self.position.y as u32 + 15, 210, 2),
            )?;
            if line_count + layout.line_count <= LOG_COUNT as u32 {
                let line_y_offset = 20 * line_count as i32;
                render_text_layout(
                    &layout,
                    canvas,
                    &self.text,
                    Some(&self.icons),
                    FontColor::Black,
                    line_y_offset,
                    true,
                    |rect, result| {
                        hit_test.add(rect, result);
                    },
                )?;

                line_count += layout.line_count;
            } else {
                break;
            }
        }

        Ok(())
    }
}

// The trouble is that it is not possible to know how many lines a log entry
// will take without the font, and without shoving that into global state (ewwww)
// we can't scroll with any reasonableness, and we can't (yet) dispatch into a view
// So we shove into LogIndexPosition the last delta and it'll get accounted for
// next render. :shrug:
pub fn log_event(ecs: &mut World, kind: EventKind, _target: Option<Entity>) {
    match kind {
        EventKind::LogScrolled(direction) => match direction {
            LogDirection::Forward => ecs.write_resource::<LogIndexPosition>().delta = LogIndexDelta::PageDown,
            LogDirection::Backwards => ecs.write_resource::<LogIndexPosition>().delta = LogIndexDelta::PageUp,
            LogDirection::SnapToEnd => ecs.write_resource::<LogIndexPosition>().delta = LogIndexDelta::JumpToEnd,
        },
        _ => {}
    }
}

impl View for LogView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.frame.render(ecs, canvas, frame)?;
        self.render_log(ecs, canvas)?;

        Ok(())
    }

    fn hit_test(&self, _ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        self.hit_tester.borrow().hit_test(x, y)
    }
}
