use std::rc::Rc;

use sdl2::mouse::MouseButton;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::{ContextData, HitTestResult, View};
use crate::after_image::{IconLoader, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::{BoxResult, Point};

pub struct HelpPopup {
    enabled: bool,
    start_mouse: Point,
    text_renderer: Rc<TextRenderer>,
    background: Texture,
}

impl HelpPopup {
    pub fn init(render_context: &RenderContext, text_renderer: Rc<TextRenderer>) -> BoxResult<HelpPopup> {
        Ok(HelpPopup {
            enabled: false,
            start_mouse: Point::init(0, 0),
            text_renderer,
            background: IconLoader::init_ui()?.get(render_context, "help_large.png")?,
        })
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

    fn get_frame_size(&self) -> (i32, i32) {
        (335, 523)
    }

    fn get_help_popup_frame(&self, canvas: &mut RenderCanvas) -> BoxResult<SDLRect> {
        let (output_width, _) = canvas.output_size()?;
        let (width, height) = self.get_frame_size();
        let (mouse_x, mouse_y) = (self.start_mouse.x as i32, self.start_mouse.y as i32);
        let on_right = width + mouse_x < output_width as i32;
        let on_top = mouse_y - height > 0;
        let popup_x = if on_right { mouse_x } else { mouse_x - width };
        let popup_y = if on_top { mouse_y - height } else { mouse_y };

        Ok(SDLRect::new(popup_x, popup_y, width as u32, height as u32))
    }
}

impl View for HelpPopup {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64, _context: &ContextData) -> BoxResult<()> {
        if self.enabled {
            let frame = self.get_help_popup_frame(canvas)?;
            canvas.copy(&self.background, None, frame)?;
        }

        Ok(())
    }
}
