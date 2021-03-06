use std::cell::RefCell;
use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::{render_text_layout, TextHitTester};
use crate::after_image::*;
use crate::atlas::prelude::*;
use crate::clash::{find_entity_at_location, find_field_at_location, EquipmentResource, HelpHeader, HelpInfo, SkillsResource};
use crate::props::{HitTestResult, RenderTextOptions, View};

enum HelpPopupSize {
    Unknown,
    Small,
    Medium,
    Large,
}

enum HelpPopupState {
    None,
    Tooltip {
        start_mouse: Point,
        topic: HitTestResult,
    },
    Modal {
        hit_tester: RefCell<TextHitTester>,
        topic_history: Vec<HitTestResult>,
    },
}

pub struct HelpPopup {
    text_renderer: Rc<TextRenderer>,
    state: HelpPopupState,
    help: Option<HelpInfo>,
    size: HelpPopupSize,
    icons: IconCache,
    symbols: IconCache,
    ui: IconCache,
}

impl HelpPopup {
    pub fn init(ecs: &World, render_context: &RenderContext, text_renderer: Rc<TextRenderer>, equipment_mode: bool) -> BoxResult<HelpPopup> {
        let skills = ecs.read_resource::<SkillsResource>();
        let equipment = ecs.read_resource::<EquipmentResource>();
        let images: Vec<String> = if equipment_mode {
            IconLoader::all_icons()
        } else {
            let mut icons = skills.all_skill_image_files();
            icons.extend(equipment.all_skill_image_files());
            icons
        };
        Ok(HelpPopup {
            state: HelpPopupState::None,
            text_renderer,
            ui: IconCache::init(
                render_context,
                IconLoader::init_ui(),
                &[
                    "help_small.png",
                    "help_medium.png",
                    "help_large.png",
                    "close.png",
                    "back_button.png",
                    "forward_button.png",
                ],
            )?,
            symbols: IconCache::init(render_context, IconLoader::init_symbols(), &["plain-dagger.png"])?,
            icons: IconCache::init(render_context, IconLoader::init_icons(), &images)?,
            size: HelpPopupSize::Unknown,
            help: None,
        })
    }

    fn lookup_hittest(result: &HitTestResult, ecs: &World) -> Option<HelpInfo> {
        match result {
            HitTestResult::Text(text) => Some(HelpInfo::find(ecs, &text)),
            HitTestResult::Icon(icon) => Some(HelpInfo::find_icon(*icon)),
            HitTestResult::Enemy(point) => Some(HelpInfo::find_entity(ecs, find_entity_at_location(ecs, *point).unwrap())),
            HitTestResult::Field(point) => Some(HelpInfo::find_field(ecs, find_field_at_location(ecs, &SizedPoint::from(*point)).unwrap())),
            HitTestResult::Orb(point) => Some(HelpInfo::find_orb(ecs, find_entity_at_location(ecs, *point).unwrap())),
            HitTestResult::Skill(name) => Some(HelpInfo::find(ecs, &name)),
            HitTestResult::Status(status) => Some(HelpInfo::find_status(ecs, *status)),
            _ => None,
        }
    }

    pub fn enable(&mut self, ecs: &World, mouse_position: Option<Point>, result: HitTestResult) {
        let help = HelpPopup::lookup_hittest(&result, ecs);
        if help.is_none() {
            return;
        }

        if let Some(mouse_position) = mouse_position {
            self.state = HelpPopupState::Tooltip {
                start_mouse: mouse_position,
                topic: result,
            }
        } else {
            self.state = HelpPopupState::Modal {
                hit_tester: RefCell::new(TextHitTester::init()),
                topic_history: vec![result],
            }
        }
        self.size = if let Some(help) = &help {
            match help.text.len() {
                0..=2 => HelpPopupSize::Small,
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
        match self.state {
            HelpPopupState::Modal { .. } | HelpPopupState::Tooltip { .. } => true,
            HelpPopupState::None => false,
        }
    }

    pub fn is_modal(&self) -> bool {
        match self.state {
            HelpPopupState::Modal { .. } => true,
            HelpPopupState::Tooltip { .. } | HelpPopupState::None => false,
        }
    }

    pub fn disable(&mut self) {
        self.state = HelpPopupState::None;
    }

    pub fn pin_help_open(&mut self) {
        let current_topic = match &self.state {
            HelpPopupState::Tooltip { topic, .. } => topic.clone(),
            _ => panic!("pin_help_open called on non-tooltip help?"),
        };

        self.state = HelpPopupState::Modal {
            hit_tester: RefCell::new(TextHitTester::init()),
            topic_history: vec![HitTestResult::Text("Top Level Help".to_string()), current_topic],
        };
        self.force_size_large();
    }

    pub fn pop_topic(&mut self, ecs: &World) {
        match &mut self.state {
            HelpPopupState::Modal { topic_history, .. } => {
                topic_history.pop();
                self.help = HelpPopup::lookup_hittest(&topic_history.last().unwrap(), ecs);
                assert!(topic_history.len() >= 1);
            }
            _ => panic!("Pop help topic in non-modal"),
        }
    }

    fn should_close_help(&mut self, x: i32, y: i32, button: Option<MouseButton>) -> bool {
        match &self.state {
            HelpPopupState::Tooltip { .. } => {
                if button.is_some() {
                    return true;
                }
            }
            HelpPopupState::Modal { hit_tester, .. } => {
                // Look for the close button
                if let Some(button) = button {
                    if button == MouseButton::Left {
                        if hit_tester.borrow().hit_test(x, y).map_or(false, |h| h.is_close_button()) {
                            return true;
                        }
                    }
                }
            }
            HelpPopupState::None => {}
        }
        false
    }

    pub fn handle_mouse_click(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        match self.state {
            HelpPopupState::Tooltip { .. } => {
                if let Some(button) = button {
                    if button == MouseButton::Middle {
                        self.pin_help_open();
                        return;
                    }
                }
            }
            _ => {}
        }

        if self.should_close_help(x, y, button) {
            self.disable();
            return;
        }

        if let Some(button) = button {
            if button == MouseButton::Left || button == MouseButton::Middle {
                match &mut self.state {
                    HelpPopupState::Modal { hit_tester, topic_history } => {
                        let hit = hit_tester.borrow().hit_test(x, y);
                        if let Some(hit) = hit {
                            if let Some(help) = HelpPopup::lookup_hittest(&hit, ecs) {
                                self.help = Some(help);
                                topic_history.push(hit);
                                return;
                            } else if hit.is_back_button() {
                                self.pop_topic(ecs);
                                return;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn get_frame_size(&self) -> (i32, i32) {
        match self.size {
            HelpPopupSize::Small => (225, 146),
            HelpPopupSize::Medium => (225, 321),
            HelpPopupSize::Large => (670, 523),
            HelpPopupSize::Unknown => panic!("Unknown help size"),
        }
    }

    fn get_popup_origin(&self) -> (i32, i32) {
        match self.state {
            HelpPopupState::Tooltip { start_mouse, .. } => (start_mouse.x as i32, start_mouse.y as i32),
            HelpPopupState::Modal { .. } | HelpPopupState::None => (50, 100),
        }
    }

    fn get_help_popup_frame(&self, canvas: &RenderCanvas) -> BoxResult<SDLRect> {
        let (output_width, _) = canvas.output_size()?;
        let (width, height) = self.get_frame_size();

        let (mouse_x, mouse_y) = self.get_popup_origin();

        let on_right = width + mouse_x < output_width as i32;
        let on_top = mouse_y - height > 0;
        let popup_x = if on_right { mouse_x } else { mouse_x - width };
        let popup_y = if on_top { mouse_y - height } else { mouse_y };

        Ok(SDLRect::new(popup_x, popup_y, width as u32, height as u32))
    }

    fn get_background(&self) -> &Texture {
        match self.size {
            HelpPopupSize::Small => &self.ui.get("help_small.png"),
            HelpPopupSize::Medium => &self.ui.get("help_medium.png"),
            HelpPopupSize::Large => &self.ui.get("help_large.png"),
            HelpPopupSize::Unknown => panic!("Unknown help size"),
        }
    }

    fn clear_hit_areas(&self) {
        match &self.state {
            HelpPopupState::Modal { hit_tester, .. } => {
                hit_tester.borrow_mut().clear();
            }
            _ => {}
        }
    }

    fn note_hit_area(&self, rect: SDLRect, result: HitTestResult) {
        match &self.state {
            HelpPopupState::Modal { hit_tester, .. } => {
                hit_tester.borrow_mut().add(rect, result);
            }
            _ => {}
        }
    }

    pub fn handle_key(&mut self, ecs: &World, keycode: Keycode, _keymod: Mod) {
        if self.is_enabled() && keycode == Keycode::Escape {
            self.disable();
        }

        match keycode {
            Keycode::F1 => {
                self.enable(ecs, None, HitTestResult::Text("Top Level Help".to_string()));
                self.force_size_large();
            }
            _ => {}
        }
    }

    pub fn handle_mouse_event(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>, views: &[Box<dyn View>]) -> bool {
        self.handle_mouse_click(ecs, x, y, button);
        // Prevent stray clicks from passing through
        if self.is_enabled() {
            return true;
        }

        if let Some(button) = button {
            if button == MouseButton::Middle {
                let hit = views.iter().rev().filter_map(|v| v.hit_test(ecs, x, y)).next();
                if let Some(hit) = hit {
                    self.enable(ecs, Some(Point::init(x as u32, y as u32)), hit);
                }
            }
        }
        false
    }
}

const HELP_OFFSET: u32 = 25;
const MOUSE_POPUP_DRIFT: u32 = 10;

impl View for HelpPopup {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        self.clear_hit_areas();
        match &self.state {
            HelpPopupState::None => {
                return Ok(());
            }
            _ => {}
        }

        let frame_size = self.get_frame_size();
        let frame = self.get_help_popup_frame(canvas)?;
        canvas.copy(self.get_background(), None, frame)?;

        match &self.state {
            HelpPopupState::Modal { topic_history, .. } => {
                let close_frame = SDLRect::new(frame.x() + frame.width() as i32 - 24 - 18, frame.y() + 16, 24, 24);
                canvas.copy(&self.ui.get("close.png"), None, close_frame)?;
                self.note_hit_area(close_frame, HitTestResult::CloseButton);

                if topic_history.len() > 1 {
                    let close_frame = SDLRect::new(frame.x() + frame.width() as i32 - 42 - 18, frame.y() + 19, 12, 18);
                    canvas.copy(&self.ui.get("back_button.png"), None, close_frame)?;
                    self.note_hit_area(close_frame, HitTestResult::BackButton);
                }
            }
            _ => {}
        }

        let mut y = 0;
        if let Some(help) = &self.help {
            match &help.header {
                HelpHeader::Image(title, file) => {
                    let request = LayoutRequest::init(
                        frame.x() as u32 + HELP_OFFSET,
                        frame.y() as u32 + 2 + HELP_OFFSET,
                        frame_size.0 as u32 - (HELP_OFFSET * 2) - 24 - 4,
                        2,
                    );
                    // We really want the text to be on one line, so we use layout_text to tell us
                    // if it fits in one line/chunk, and if not we use a smaller font
                    let layout = self.text_renderer.layout_text(&title, FontSize::Bold, request)?;

                    let (text_width, _) = self.text_renderer.render_text(
                        &title,
                        layout.chunks[0].position.x as i32,
                        layout.chunks[0].position.y as i32,
                        canvas,
                        if layout.chunks.len() > 1 { FontSize::Small } else { FontSize::Bold },
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
                        4,
                    ),
                )?;

                y += layout.line_count * 22;
                if y as i32 >= frame_size.1 - 20 {
                    break;
                }

                let underline_links = self.is_modal();
                render_text_layout(
                    &layout,
                    canvas,
                    &self.text_renderer,
                    RenderTextOptions::init(FontColor::White)
                        .with_icons(&self.symbols)
                        .with_underline_links(underline_links),
                    |rect, result| self.note_hit_area(rect, result),
                )?;
            }
        }

        Ok(())
    }

    fn handle_mouse_move(&mut self, _ecs: &World, x: i32, y: i32, _state: MouseState) {
        match &self.state {
            HelpPopupState::Tooltip { start_mouse, .. } => {
                if start_mouse.distance_to(Point::init(x as u32, y as u32)).unwrap_or(10) > MOUSE_POPUP_DRIFT {
                    self.disable();
                }
            }
            _ => {}
        }
    }
}
