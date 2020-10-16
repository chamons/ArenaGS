use std::mem;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::{HitTestResult, View};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;

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
}

impl Frame {
    pub fn init(position: SDLPoint, render_context: &RenderContext, kind: FrameKind) -> BoxResult<Frame> {
        let image = match kind {
            FrameKind::InfoBar => "info_frame.png",
            FrameKind::Log => "log_frame.png",
            FrameKind::Map => "map_frame.png",
            FrameKind::Button => "button_frame.png",
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
            FrameKind::Button => (145, 42),
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

pub enum ButtonKind {
    Image(Texture),
    Text(String, Frame, Rc<TextRenderer>),
}

pub type EnabledHandler = dyn Fn(&World) -> bool;
pub type ButtonHandler = dyn Fn() -> Option<HitTestResult>;

pub struct Button {
    frame: SDLRect,
    kind: ButtonKind,
    enabled: Box<EnabledHandler>,
    handler: Box<ButtonHandler>,
}

impl Button {
    pub fn image(
        frame: SDLRect,
        image: Texture,
        enabled: impl Fn(&World) -> bool + 'static,
        handler: impl Fn() -> Option<HitTestResult> + 'static,
    ) -> BoxResult<Button> {
        Ok(Button {
            frame,
            kind: ButtonKind::Image(image),
            enabled: Box::new(enabled),
            handler: Box::new(handler),
        })
    }

    pub fn text(
        corner: SDLPoint,
        text: &str,
        render_context: &RenderContext,
        text_renderer: &Rc<TextRenderer>,
        enabled: impl Fn(&World) -> bool + 'static,
        handler: impl Fn() -> Option<HitTestResult> + 'static,
    ) -> BoxResult<Button> {
        let text_frame = Frame::init(corner, render_context, FrameKind::Button)?;
        let text_size = text_frame.frame_size();
        Ok(Button {
            frame: SDLRect::new(corner.x(), corner.y(), text_size.0, text_size.1),
            kind: ButtonKind::Text(text.to_string(), text_frame, Rc::clone(text_renderer)),
            enabled: Box::new(enabled),
            handler: Box::new(handler),
        })
    }
}

impl View for Button {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        match &self.kind {
            ButtonKind::Image(background) => canvas.copy(&background, None, self.frame)?,
            ButtonKind::Text(text, text_frame, text_renderer) => {
                text_frame.render(ecs, canvas, frame)?;
                text_renderer.render_text_centered(
                    text,
                    self.frame.x(),
                    self.frame.y() + 10,
                    text_frame.frame_size().0,
                    canvas,
                    FontSize::Bold,
                    FontColor::Brown,
                )?;
            }
        };

        if !(self.enabled)(ecs) {
            canvas.set_draw_color(Color::RGBA(12, 12, 12, 196));
            canvas.fill_rect(self.frame)?;
        }

        Ok(())
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        if !(self.enabled)(ecs) {
            return None;
        }

        if self.frame.contains_point(SDLPoint::new(x, y)) {
            (self.handler)()
        } else {
            None
        }
    }
}

pub struct TabButtonInfo {
    text: String,
    enabled: Box<EnabledHandler>,
    handler: Box<ButtonHandler>,
}

impl TabButtonInfo {
    pub fn init(text: &str, enabled: impl Fn(&World) -> bool + 'static, handler: impl Fn() -> Option<HitTestResult> + 'static) -> TabButtonInfo {
        TabButtonInfo {
            text: text.to_string(),
            enabled: Box::new(enabled),
            handler: Box::new(handler),
        }
    }
}

pub struct TabView {
    frame: SDLRect,
    background: Texture,
    buttons: Vec<Button>,
}

impl TabView {
    pub fn init(frame: SDLRect, render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, mut buttons: Vec<TabButtonInfo>) -> BoxResult<TabView> {
        let buttons: Vec<Button> = buttons
            .drain(0..)
            .enumerate()
            .map(|(i, b)| {
                Button::text(
                    SDLPoint::new(frame.x() + (75 + 150 * i as i32), frame.y()),
                    &b.text,
                    render_context,
                    text_renderer,
                    b.enabled,
                    b.handler,
                )
                .expect("Unable to create TabView button")
            })
            .collect();
        Ok(TabView {
            frame,
            background: IconLoader::init_ui().get(render_context, "bg_04.png")?,
            buttons,
        })
    }
}

impl View for TabView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.copy(&self.background, None, self.frame)?;
        for b in &self.buttons {
            b.render(ecs, canvas, frame)?;
        }
        Ok(())
    }

    fn hit_test(&self, _ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        None
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
