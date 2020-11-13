use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::props::{HitTestResult, View};

pub struct EmptyView {}

#[allow(dead_code)]
impl EmptyView {
    pub fn init() -> BoxResult<EmptyView> {
        Ok(EmptyView {})
    }
}

impl View for EmptyView {
    fn render(&self, _: &World, _canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        Ok(())
    }
}

pub struct Frame {
    position: SDLPoint,
    frame: Texture,
    kind: FrameKind,
}

pub enum FrameKind {
    InfoBar,
    Log,
    Map,
    Button,
    ButtonFull,
}

impl Frame {
    pub fn init(position: SDLPoint, render_context: &RenderContext, kind: FrameKind) -> BoxResult<Frame> {
        let image = match kind {
            FrameKind::InfoBar => "info_frame.png",
            FrameKind::Log => "log_frame.png",
            FrameKind::Map => "map_frame.png",
            FrameKind::Button => "button_frame.png",
            FrameKind::ButtonFull => "button_frame_full.png",
        };
        Ok(Frame {
            position,
            frame: IconLoader::init_ui().get(render_context, image)?,
            kind,
        })
    }

    pub fn frame_size(&self) -> (u32, u32) {
        match self.kind {
            FrameKind::InfoBar => (271, 541),
            FrameKind::Log => (271, 227),
            FrameKind::Map => (753, 768),
            FrameKind::Button | FrameKind::ButtonFull => (145, 42),
        }
    }
}

impl View for Frame {
    fn render(&self, _: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        let frame_size = self.frame_size();
        canvas.copy(
            &self.frame,
            None,
            SDLRect::new(self.position.x(), self.position.y(), frame_size.0, frame_size.1),
        )?;
        Ok(())
    }
}

pub struct Button {
    pub frame: SDLRect,
    delegate: ButtonDelegate,
    active: bool,
    text: String,
    text_frame: Frame,
    text_renderer: Rc<TextRenderer>,
    font_size: FontSize,
}

#[derive(Eq, PartialEq)]
pub enum ButtonEnabledState {
    Shown,
    Ghosted,
    Hide,
}

pub type ButtonEnabled = Box<dyn Fn(&World) -> ButtonEnabledState>;
pub type ButtonHandler = Box<dyn Fn(&mut World)>;

pub struct ButtonDelegate {
    enabled: Option<ButtonEnabled>,
    handler: Option<ButtonHandler>,
}

impl ButtonDelegate {
    pub fn init() -> ButtonDelegate {
        ButtonDelegate { enabled: None, handler: None }
    }

    pub fn enabled(mut self, enabled: ButtonEnabled) -> ButtonDelegate {
        self.enabled = Some(enabled);
        self
    }

    pub fn handler(mut self, handler: ButtonHandler) -> ButtonDelegate {
        self.handler = Some(handler);
        self
    }

    pub fn enabled_state(&self, ecs: &World) -> ButtonEnabledState {
        self.enabled.as_ref().map_or(ButtonEnabledState::Shown, |e| (e)(ecs))
    }
}

impl Button {
    pub fn text(corner: SDLPoint, text: &str, render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, delegate: ButtonDelegate) -> BoxResult<Button> {
        let text_frame = Frame::init(corner, render_context, FrameKind::ButtonFull)?;
        let text_size = text_frame.frame_size();
        Ok(Button {
            frame: SDLRect::new(corner.x(), corner.y(), text_size.0, text_size.1),
            text: text.to_string(),
            text_frame,
            text_renderer: Rc::clone(text_renderer),
            font_size: FontSize::Bold,
            active: true,
            delegate,
        })
    }

    pub fn tab(
        corner: SDLPoint,
        text: &str,
        render_context: &RenderContext,
        text_renderer: &Rc<TextRenderer>,
        active: bool,
        delegate: ButtonDelegate,
    ) -> BoxResult<Button> {
        let text_frame = Frame::init(corner, render_context, FrameKind::Button)?;
        let text_size = text_frame.frame_size();
        Ok(Button {
            frame: SDLRect::new(corner.x(), corner.y(), text_size.0, text_size.1),
            text: text.to_string(),
            text_frame,
            text_renderer: Rc::clone(text_renderer),
            font_size: FontSize::Bold,
            active,
            delegate,
        })
    }

    pub fn with_size(mut self, font_size: FontSize) -> Button {
        self.font_size = font_size;
        self
    }
}

impl View for Button {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let enable_state = self.delegate.enabled_state(ecs);
        if enable_state == ButtonEnabledState::Hide {
            return Ok(());
        }

        self.text_frame.render(ecs, canvas, frame)?;

        let text_y_offset = match self.font_size {
            FontSize::Bold => 10,
            _ => 13,
        };

        self.text_renderer.render_text_centered(
            &self.text,
            self.frame.x() + 8,
            self.frame.y() + text_y_offset,
            self.text_frame.frame_size().0 - 16,
            canvas,
            self.font_size,
            if self.active { FontColor::White } else { FontColor::Brown },
        )?;

        if enable_state == ButtonEnabledState::Ghosted {
            canvas.set_draw_color(Color::RGBA(12, 12, 12, 196));
            canvas.fill_rect(self.frame)?;
        }

        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        if self.delegate.enabled_state(ecs) == ButtonEnabledState::Hide {
            return;
        }

        if let Some(button) = button {
            if button == MouseButton::Left {
                if self.frame.contains_point(SDLPoint::new(x, y)) {
                    if let Some(handler) = &self.delegate.handler {
                        (handler)(ecs);
                    }
                }
            }
        }
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        if self.delegate.enabled_state(ecs) == ButtonEnabledState::Hide {
            return None;
        }

        if self.frame.contains_point(SDLPoint::new(x, y)) {
            Some(HitTestResult::Button)
        } else {
            None
        }
    }
}

pub struct TabInfo {
    text: String,
    view: Box<dyn View>,
}

impl TabInfo {
    pub fn init(text: &str, view: Box<dyn View>) -> TabInfo {
        TabInfo { text: text.to_string(), view }
    }
}
pub struct TabView {
    frame: SDLRect,
    background: Texture,
    tabs: Vec<(Button, Box<dyn View>)>,
    index: RefCell<usize>,
}

impl TabView {
    pub fn init(corner: SDLPoint, render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, mut tabs: Vec<TabInfo>) -> BoxResult<TabView> {
        let button_width: i32 = 150;
        let tab_button_total_width = button_width * tabs.len() as i32;
        let (canvas_width, canvas_height) = render_context.canvas.logical_size();
        let tab_view_height = canvas_height as i32 - (corner.y() * 2);
        let tab_view_width = canvas_width as i32 - (corner.x() * 2);
        let tab_button_start = (tab_view_width - tab_button_total_width) / 2;
        let mut tabs: Vec<(Button, Box<dyn View>)> = tabs
            .drain(0..)
            .enumerate()
            .map(|(i, b)| {
                let button = Button::tab(
                    SDLPoint::new(corner.x() + (tab_button_start + button_width * i as i32), corner.y()),
                    &b.text,
                    render_context,
                    text_renderer,
                    i == 0,
                    ButtonDelegate::init(),
                )
                .expect("Unable to create TabView button");
                (button, b.view)
            })
            .collect();
        tabs[0].1.on_tab_swap();
        Ok(TabView {
            frame: SDLRect::new(corner.x(), corner.y(), tab_view_width as u32, tab_view_height as u32),
            background: IconLoader::init_ui().get(render_context, "tab_background.png")?,
            tabs,
            index: RefCell::new(0),
        })
    }
}

impl View for TabView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.copy(&self.background, None, self.frame)?;
        for (b, _) in &self.tabs {
            b.render(ecs, canvas, frame)?;
        }
        self.tabs[*self.index.borrow()].1.render(ecs, canvas, frame)?;
        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        if let Some(button) = button {
            if button == MouseButton::Left {
                let tab_hit = self.tabs.iter().enumerate().filter_map(|(i, (b, _))| b.hit_test(ecs, x, y).map(|_| i)).next();
                if let Some(index) = tab_hit {
                    self.tabs[*self.index.borrow()].0.active = false;
                    *self.index.borrow_mut() = index;
                    self.tabs[index].0.active = true;
                    self.tabs[index].1.on_tab_swap();
                }
            }
        }
        self.tabs[*self.index.borrow()].1.handle_mouse_click(ecs, x, y, button);
    }

    fn handle_mouse_move(&mut self, ecs: &World, x: i32, y: i32, state: MouseState) {
        self.tabs[*self.index.borrow()].1.handle_mouse_move(ecs, x, y, state);
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        self.tabs.iter().filter_map(|(_, t)| t.hit_test(ecs, x, y)).next()
    }
}

pub struct LifeBar {
    lifebar_frame: Texture,
    lifebar: Texture,
    absorb: Texture,
}

impl LifeBar {
    pub fn init(render_context: &RenderContext) -> BoxResult<LifeBar> {
        let loader = IconLoader::init_ui();
        Ok(LifeBar {
            lifebar_frame: loader.get(render_context, "life_frame.png")?,
            lifebar: loader.get(render_context, "life_bar.png")?,
            absorb: loader.get(render_context, "absorb_bar.png")?,
        })
    }

    pub fn render(&self, frame: SDLRect, canvas: &mut RenderCanvas, life_percentage: f64, absorb_percentage: f64) -> BoxResult<()> {
        let show_absorb = absorb_percentage > 0.0;
        let percentage = if show_absorb { absorb_percentage } else { life_percentage };
        let mut inner_frame = frame;
        inner_frame.offset(0, 1);
        inner_frame.resize((inner_frame.width() as f64 * percentage).round() as u32, inner_frame.height() - 2);

        if show_absorb {
            canvas.copy(&self.absorb, None, inner_frame)?;
        } else {
            canvas.copy(&self.lifebar, None, inner_frame)?;
        }

        canvas.copy(&self.lifebar_frame, None, frame)?;

        Ok(())
    }
}
