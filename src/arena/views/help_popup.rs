use std::rc::Rc;

use sdl2::mouse::MouseButton;
use specs::prelude::*;

use super::{ContextData, HitTestResult, View};
use crate::after_image::{RenderCanvas, TextRenderer};
use crate::atlas::{BoxResult, Point};

pub struct HelpPopup {
    enabled: bool,
    start_mouse: Point,
    text: Rc<TextRenderer>,
}

impl HelpPopup {
    pub fn init(text: Rc<TextRenderer>) -> HelpPopup {
        HelpPopup {
            enabled: false,
            start_mouse: Point::init(0, 0),
            text,
        }
    }

    pub fn enable(&mut self, x: i32, y: i32, result: HitTestResult) {
        let text = match result {
            HitTestResult::Text(text) => Some(text),
            HitTestResult::Icon(icon) => Some(format!("{:?}", icon)),
            _ => None,
        };
        if let Some(text) = text {
            self.enabled = true;
            self.start_mouse = Point::init(x as u32, y as u32);
        }
    }

    const MOUSE_POPUP_DRIFT: u32 = 5;
    pub fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) {
        if !self.enabled {
            return;
        }

        if button.is_some() {
            self.enabled = false;
            return;
        }

        if self.start_mouse.distance_to(Point::init(x as u32, y as u32)).unwrap_or(10) > HelpPopup::MOUSE_POPUP_DRIFT {
            self.enabled = false;
        }
    }
}

impl View for HelpPopup {
    fn render(&self, _ecs: &World, _canvas: &mut RenderCanvas, _frame: u64, _context: &ContextData) -> BoxResult<()> {
        Ok(())
    }
}
