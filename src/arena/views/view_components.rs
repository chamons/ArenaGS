use std::rc::Rc;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::{ContextData, View};
use crate::after_image::{RenderCanvas, UILoader};
use crate::atlas::BoxResult;

pub struct Frame {
    position: SDLPoint,
    ui_loader: Rc<UILoader>,
}

impl Frame {
    pub fn init(position: SDLPoint, ui_loader: &Rc<UILoader>) -> BoxResult<Frame> {
        Ok(Frame {
            position,
            ui_loader: Rc::clone(ui_loader),
        })
    }

    fn draw_corner(&self, canvas: &mut RenderCanvas, point: &SDLPoint, angle: f64) -> BoxResult<()> {
        let corner = self.ui_loader.get("frame_01_03.png");
        let image_rect = SDLRect::new(0, 0, 272, 272);
        canvas.copy_ex(corner, image_rect, SDLRect::new(point.x(), point.y(), 68, 68), angle, None, false, false)?;

        Ok(())
    }

    fn relative_position(&self, x: i32, y: i32) -> SDLPoint {
        SDLPoint::new(self.position.x() + x, self.position.y() + y)
    }
}

impl View for Frame {
    fn render(&self, _: &World, canvas: &mut RenderCanvas, _frame: u64, _context: &ContextData) -> BoxResult<()> {
        self.draw_corner(canvas, &self.position, 0.0)?;
        self.draw_corner(canvas, &self.relative_position(68, 0), 90.0)?;
        self.draw_corner(canvas, &self.relative_position(68, 68), 180.0)?;
        self.draw_corner(canvas, &self.relative_position(0, 68), 270.0)?;
        Ok(())
    }
}
