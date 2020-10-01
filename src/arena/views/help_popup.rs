use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::MouseButton;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::{render_text_layout, HitTestResult, View};
use crate::after_image::*;
use crate::atlas::{BoxResult, Point, SizedPoint};
use crate::clash::{all_skill_image_filesnames, find_entity_at_location, find_field_at_location, HelpHeader, HelpInfo};

enum HelpPopupSize {
    Unknown,
    Small,
    Medium,
    Large,
}

pub struct HelpPopup {
    enabled: bool,
    help: Option<HelpInfo>,
    start_mouse: Option<Point>,
    text_renderer: Rc<TextRenderer>,
    small_frame: Texture,
    medium_frame: Texture,
    large_frame: Texture,
    close_button: Texture,
    close_button_frame: RefCell<Option<SDLRect>>,
    size: HelpPopupSize,
    symbol_cache: IconCache,
    icons: IconCache,
}

impl HelpPopup {
    pub fn init(render_context: &RenderContext, text_renderer: Rc<TextRenderer>) -> BoxResult<HelpPopup> {
        let loader = IconLoader::init_ui()?;
        Ok(HelpPopup {
            enabled: false,
            start_mouse: None,
            text_renderer,
            small_frame: loader.get(render_context, "help_small.png")?,
            medium_frame: loader.get(render_context, "help_medium.png")?,
            large_frame: loader.get(render_context, "help_large.png")?,
            close_button: loader.get(render_context, "close.png")?,
            close_button_frame: RefCell::new(None),
            symbol_cache: IconCache::init(render_context, IconLoader::init_symbols()?, &["plain-dagger.png"])?,
            icons: IconCache::init(render_context, IconLoader::init_icons()?, &all_skill_image_filesnames())?,
            size: HelpPopupSize::Unknown,
            help: None,
        })
    }

    pub fn enable(&mut self, ecs: &World, mouse_position: Option<Point>, result: HitTestResult) {
        let help = match result {
            HitTestResult::Text(text) => Some(HelpInfo::find(&text)),
            HitTestResult::Icon(icon) => Some(HelpInfo::find_icon(icon)),
            HitTestResult::Enemy(point) => Some(HelpInfo::find_entity(ecs, find_entity_at_location(ecs, point).unwrap())),
            HitTestResult::Field(point) => Some(HelpInfo::find_field(ecs, find_field_at_location(ecs, &SizedPoint::from(point)).unwrap())),
            HitTestResult::Orb(point) => Some(HelpInfo::find_orb(ecs, find_entity_at_location(ecs, point).unwrap())),
            HitTestResult::Skill(name) => Some(HelpInfo::find(&name)),
            HitTestResult::Status(status) => Some(HelpInfo::find_status(status)),
            HitTestResult::None | HitTestResult::Tile(_) => return,
        };
        self.enabled = true;
        self.start_mouse = mouse_position;
        self.size = if let Some(help) = &help {
            match help.text.len() {
                1..=2 => HelpPopupSize::Small,
                _ => HelpPopupSize::Medium,
            }
        } else {
            HelpPopupSize::Medium
        };
        self.help = help;
    }

    pub fn force_size_large(&mut self) {
        self.size = HelpPopupSize::Large;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    const MOUSE_POPUP_DRIFT: u32 = 10;
    pub fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) {
        if !self.enabled {
            return;
        }

        // If we have a mouse relative position, movement and clicking close
        if let Some(start_mouse) = self.start_mouse {
            if button.is_some() {
                self.enabled = false;
                return;
            }

            if start_mouse.distance_to(Point::init(x as u32, y as u32)).unwrap_or(10) > HelpPopup::MOUSE_POPUP_DRIFT {
                self.enabled = false;
            }
        } else {
            // Else look for the close button
            if let Some(button) = button {
                if button == MouseButton::Left {
                    let f = self.close_button_frame.borrow();
                    if let Some(close_button_frame) = *f {
                        if close_button_frame.contains_point(SDLPoint::new(x, y)) {
                            self.enabled = false;
                        }
                    }
                }
            }
        }
    }

    fn get_frame_size(&self) -> (i32, i32) {
        match self.size {
            HelpPopupSize::Small => (225, 146),
            HelpPopupSize::Medium => (225, 321),
            HelpPopupSize::Large => (335, 523),
            HelpPopupSize::Unknown => panic!("Unknown help size"),
        }
    }

    fn get_help_popup_frame(&self, canvas: &RenderCanvas) -> BoxResult<SDLRect> {
        let (output_width, _) = canvas.output_size()?;
        let (width, height) = self.get_frame_size();
        let (mouse_x, mouse_y) = if let Some(start_mouse) = self.start_mouse {
            (start_mouse.x as i32, start_mouse.y as i32)
        } else {
            (100, 100)
        };
        let on_right = width + mouse_x < output_width as i32;
        let on_top = mouse_y - height > 0;
        let popup_x = if on_right { mouse_x } else { mouse_x - width };
        let popup_y = if on_top { mouse_y - height } else { mouse_y };

        Ok(SDLRect::new(popup_x, popup_y, width as u32, height as u32))
    }

    fn get_help_popup_close_frame(&self, canvas: &RenderCanvas) -> BoxResult<SDLRect> {
        let frame = self.get_help_popup_frame(canvas)?;
        Ok(SDLRect::new(frame.x() + frame.width() as i32 - 24 - 2, frame.y() + 2, 24, 24))
    }

    fn get_background(&self) -> &Texture {
        match self.size {
            HelpPopupSize::Small => &self.small_frame,
            HelpPopupSize::Medium => &self.medium_frame,
            HelpPopupSize::Large => &self.large_frame,
            HelpPopupSize::Unknown => panic!("Unknown help size"),
        }
    }
}

const HELP_OFFSET: u32 = 25;
impl View for HelpPopup {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        if self.enabled {
            let frame = self.get_help_popup_frame(canvas)?;
            canvas.copy(self.get_background(), None, frame)?;

            if self.start_mouse.is_none() {
                // Cache this as we can't calculate this during mouse events
                let close_frame = {
                    let mut f = self.close_button_frame.borrow_mut();
                    if f.is_none() {
                        *f = Some(self.get_help_popup_close_frame(canvas)?);
                    }
                    f.unwrap()
                };

                canvas.copy(&self.close_button, None, close_frame)?;
            }

            let mut y = 0;
            if let Some(help) = &self.help {
                match &help.header {
                    HelpHeader::Image(title, file) => {
                        let (text_width, _) = self.text_renderer.render_text(
                            &title,
                            frame.x() + HELP_OFFSET as i32,
                            frame.y() + 2 + HELP_OFFSET as i32,
                            canvas,
                            FontSize::Bold,
                            FontColor::White,
                        )?;

                        let image = self.icons.get(file);
                        canvas.copy(
                            image,
                            None,
                            SDLRect::new(text_width as i32 + 10 + frame.x() + HELP_OFFSET as i32, frame.y() + HELP_OFFSET as i32, 24, 24),
                        )?;
                        y += 40;
                    }
                    HelpHeader::Text(title) => {
                        self.text_renderer.render_text(
                            &title,
                            frame.x() + HELP_OFFSET as i32,
                            frame.y() + HELP_OFFSET as i32,
                            canvas,
                            FontSize::Bold,
                            FontColor::White,
                        )?;
                        y += 40;
                    }
                    HelpHeader::None => {}
                }
                for help_chunk in &help.text {
                    let layout = self.text_renderer.layout_text(
                        &help_chunk,
                        FontSize::Small,
                        LayoutRequest::init(
                            frame.x() as u32 + HELP_OFFSET,
                            y + frame.y() as u32 + HELP_OFFSET,
                            frame.width() - (HELP_OFFSET * 2) - 10,
                            2,
                        ),
                    )?;
                    render_text_layout(&layout, canvas, &mut None, &self.text_renderer, &self.symbol_cache, FontColor::White, 0)?;
                    y += layout.line_count * 20;
                }
            }
        }

        Ok(())
    }
}
