use std::rc::Rc;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::{ContextData, HitTestResult, View};
use crate::after_image::{IconCache, IconLoader, RenderCanvas, RenderContext};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_player, StatusComponent};

pub struct StatusBarView {
    views: Vec<StatusBarItemView>,
}

impl StatusBarView {
    pub fn init(render_context: &RenderContext, position: SDLPoint) -> BoxResult<StatusBarView> {
        let cache = Rc::new(IconCache::init(render_context, IconLoader::init("glass")?, all_icon_names())?);
        let mut views = vec![];
        for i in 0..10 {
            views.push(StatusBarItemView::init(SDLPoint::new(position.x() + i * 58, position.y()), &cache));
        }
        Ok(StatusBarView { views })
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
    icons: Rc<IconCache>,
}

impl StatusBarItemView {
    pub fn init(position: SDLPoint, icons: &Rc<IconCache>) -> StatusBarItemView {
        StatusBarItemView {
            position,
            icons: Rc::clone(icons),
        }
    }
}

impl View for StatusBarItemView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64, context: &ContextData) -> BoxResult<()> {
        let name = match context {
            ContextData::String(name) => name,
            _ => panic!("StatusBarItemView context wrong type?"),
        };
        let icon = self.icons.get(get_icon_name_for_status(name));

        canvas.copy(&icon, SDLRect::new(0, 0, 48, 48), SDLRect::new(self.position.x(), self.position.y(), 48, 48))?;

        Ok(())
    }

    fn hit_test(&self, _ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        None
    }
}

pub fn get_icon_name_for_status(status_name: &str) -> &'static str {
    match status_name {
        "Fire Ammo" => "SpellBook08_130.png",
        "Ice Ammo" => "SpellBook08_117.png",
        _ => panic!("Unknown status icon {} for display", status_name),
    }
}

pub fn all_icon_names() -> &'static [&'static str] {
    &["SpellBook08_130.png", "SpellBook08_117.png"]
}
