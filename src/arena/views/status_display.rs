use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::{ContextData, HitTestResult, View};
use crate::after_image::{RenderCanvas, RenderContext, IconLoader};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_player, EventKind, StatusComponent};

pub struct StatusBarView {
    position: SDLPoint,
    views: Vec<StatusBarItemView>,
    icons: IconLoader,
}

impl StatusBarView {
    pub fn init(render_context: &RenderContext, position: SDLPoint, ecs: &World) -> BoxResult<StatusBarView> {
        let icons = IconLoader::init("glass")?;
        let mut views = vec![];
        for i in 0..10 {
            views.push(StatusBarItemView::init(SDLPoint::new(position.x() + i * 44, position.y()), i as u32));
        }
        Ok(StatusBarView { position, views, icons })
    }
}

impl View for StatusBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64, context: &ContextData) -> BoxResult<()> {
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
    index: u32,
    pub name: Option<String>,
}

impl StatusBarItemView {
    pub fn init(position: SDLPoint, index: u32) -> StatusBarItemView {
        StatusBarItemView { position, index, name: None }
    }
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
