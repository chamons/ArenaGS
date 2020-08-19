use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::super::IconLoader;
use super::{ContextData, HitTestResult, View};
use crate::after_image::{RenderCanvas, RenderContext, RenderContextHolder};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_player, EventKind, StatusComponent};

pub struct StatusBarView {
    position: SDLPoint,
    views: Vec<StatusBarItemView>,
}

impl StatusBarView {
    pub fn init(render_context_holder: &RenderContextHolder, position: SDLPoint, ecs: &World) -> BoxResult<StatusBarView> {
        let icons = Rc::new(IconLoader::init(Rc::clone(render_context_holder), "glass")?);
        let mut views = vec![];
        for i in 0..10 {
            views.push(StatusBarItemView::init(SDLPoint::new(position.x() + i * 44, position.y()), Rc::clone(&icons)));
        }
        Ok(StatusBarView { position, views })
    }
}

impl View for StatusBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64, _context: &ContextData) -> BoxResult<()> {
        let player = find_player(&ecs);
        let statuses = ecs.read_storage::<StatusComponent>();
        let status = &statuses.grab(player).status;
        let status_names = status.get_all();
        for i in 0..10 {
            if let Some(name) = status_names.get(i) {
                self.views[i].render(ecs, canvas, frame, &ContextData::String(name.to_string()))?;
            }
        }

        Ok(())
    }

    fn hit_test(&self, _ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        None
    }
}

struct StatusBarItemView {
    position: SDLPoint,
    icons: Rc<IconLoader>,
}

impl StatusBarItemView {
    pub fn init(position: SDLPoint, icons: Rc<IconLoader>) -> StatusBarItemView {
        StatusBarItemView { position, icons }
    }

    // pub fn get_icon(&self, status_name: &str, canvas: &mut RenderCanvas) -> Texture {
    //     match status_name {
    //         "Fire Ammo" => self.icons.get(),
    //     }
    // }
}

impl View for StatusBarItemView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64, context: &ContextData) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.fill_rect(SDLRect::new(self.position.x(), self.position.y(), 32, 32))?;
        Ok(())
    }

    fn hit_test(&self, _ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        None
    }
}
