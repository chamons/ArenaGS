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

#[allow(dead_code)]
pub enum ButtonKind {
    Image(Texture),
    Text(String, Frame, Rc<TextRenderer>),
}

pub struct Button {
    pub frame: SDLRect,
    kind: ButtonKind,
    delegate: ButtonDelegate,
    active: bool,
}

pub type ButtonQuery = Box<dyn Fn() -> bool>;
pub type ButtonHandler = Box<dyn Fn()>;

pub struct ButtonDelegate {
    enabled: Option<ButtonQuery>,
    handler: Option<ButtonHandler>,
}

impl ButtonDelegate {
    pub fn init() -> ButtonDelegate {
        ButtonDelegate { enabled: None, handler: None }
    }

    pub fn enabled(mut self, enabled: ButtonQuery) -> ButtonDelegate {
        self.enabled = Some(enabled);
        self
    }

    pub fn handler(mut self, handler: ButtonHandler) -> ButtonDelegate {
        self.handler = Some(handler);
        self
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.as_ref().map_or(true, |e| (e)())
    }
}

impl Button {
    #[allow(dead_code)]
    pub fn image(frame: SDLRect, image: Texture, delegate: ButtonDelegate) -> BoxResult<Button> {
        Ok(Button {
            frame,
            kind: ButtonKind::Image(image),
            active: false,
            delegate,
        })
    }

    pub fn text(
        corner: SDLPoint,
        text: &str,
        render_context: &RenderContext,
        text_renderer: &Rc<TextRenderer>,
        full_frame: bool,
        active: bool,
        delegate: ButtonDelegate,
    ) -> BoxResult<Button> {
        let text_frame = Frame::init(corner, render_context, if full_frame { FrameKind::ButtonFull } else { FrameKind::Button })?;
        let text_size = text_frame.frame_size();
        Ok(Button {
            frame: SDLRect::new(corner.x(), corner.y(), text_size.0, text_size.1),
            kind: ButtonKind::Text(text.to_string(), text_frame, Rc::clone(text_renderer)),
            active,
            delegate,
        })
    }
}

impl View for Button {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        if self.delegate.is_enabled() {
            match &self.kind {
                ButtonKind::Image(background) => canvas.copy(&background, None, self.frame)?,
                ButtonKind::Text(text, text_frame, text_renderer) => {
                    text_frame.render(ecs, canvas, frame)?;
                    text_renderer.render_text_centered(
                        text,
                        self.frame.x() + 2,
                        self.frame.y() + 10,
                        text_frame.frame_size().0 - 4,
                        canvas,
                        FontSize::Bold,
                        if self.active { FontColor::White } else { FontColor::Brown },
                    )?;
                }
            };
        }

        Ok(())
    }

    fn handle_mouse_click(&mut self, _ecs: &World, x: i32, y: i32, button: Option<MouseButton>) {
        if !self.delegate.is_enabled() {
            return;
        }

        if let Some(button) = button {
            if button == MouseButton::Left {
                if self.frame.contains_point(SDLPoint::new(x, y)) {
                    if let Some(handler) = &self.delegate.handler {
                        (handler)();
                    }
                }
            }
        }
    }

    fn hit_test(&self, _ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
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
        let tabs: Vec<(Button, Box<dyn View>)> = tabs
            .drain(0..)
            .enumerate()
            .map(|(i, b)| {
                let button = Button::text(
                    SDLPoint::new(corner.x() + (tab_button_start + button_width * i as i32), corner.y()),
                    &b.text,
                    render_context,
                    text_renderer,
                    false,
                    i == 0,
                    ButtonDelegate::init(),
                )
                .expect("Unable to create TabView button");
                (button, b.view)
            })
            .collect();
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

    fn handle_mouse_click(&mut self, ecs: &World, x: i32, y: i32, button: Option<MouseButton>) {
        if let Some(button) = button {
            if button == MouseButton::Left {
                let tab_hit = self.tabs.iter().enumerate().filter_map(|(i, (b, _))| b.hit_test(ecs, x, y).map(|_| i)).next();
                if let Some(index) = tab_hit {
                    self.tabs[*self.index.borrow()].0.active = false;
                    *self.index.borrow_mut() = index;
                    self.tabs[index].0.active = true;
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
