use std::rc::Rc;

use num_traits::FromPrimitive;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::{ContextData, HitTestResult, View};
use crate::after_image::{IconCache, IconLoader, RenderCanvas, RenderContext};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_player, StatusComponent, StatusKind};

pub struct StatusBarView {
    views: Vec<StatusBarItemView>,
}

impl StatusBarView {
    pub fn init(render_context: &RenderContext, position: SDLPoint) -> BoxResult<StatusBarView> {
        let cache = Rc::new(IconCache::init(render_context, IconLoader::init_icons()?, all_icon_filenames())?);
        let mut views = vec![];
        let ui = IconLoader::init_ui()?;
        for i in 0..10 {
            views.push(StatusBarItemView::init(
                SDLPoint::new(position.x() + i * 58, position.y()),
                ui.get(render_context, "status_frame.png")?,
                &cache,
            ));
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

        let mut draw_count = 0;
        for i in 0..10 {
            if let Some(kind) = status_names.get(i) {
                if kind.should_display() {
                    self.views[draw_count].render(ecs, canvas, frame, &ContextData::Number((*kind).into()))?;
                    draw_count += 1;
                }
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
    frame: Texture,
}

impl StatusBarItemView {
    pub fn init(position: SDLPoint, frame: Texture, icons: &Rc<IconCache>) -> StatusBarItemView {
        StatusBarItemView {
            position,
            icons: Rc::clone(icons),
            frame,
        }
    }
}

impl View for StatusBarItemView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64, context: &ContextData) -> BoxResult<()> {
        let kind = match context {
            ContextData::Number(value) => StatusKind::from_u32(*value).unwrap(),
            _ => panic!("StatusBarItemView context wrong type?"),
        };

        let icon = self.icons.get(get_icon_name_for_status(kind));

        canvas.copy(&icon, SDLRect::new(0, 0, 48, 48), SDLRect::new(self.position.x(), self.position.y(), 48, 48))?;
        canvas.copy(&self.frame, None, SDLRect::new(self.position.x() - 2, self.position.y() - 2, 54, 54))?;

        Ok(())
    }

    fn hit_test(&self, _ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        None
    }
}

pub fn get_icon_name_for_status(kind: StatusKind) -> &'static str {
    match kind {
        StatusKind::Burning => "SpellBook08_130.png",
        StatusKind::Frozen => "SpellBook08_111.png",
        StatusKind::Ignite => "b_31_1.png",
        StatusKind::Cyclone => "b_40_02.png",
        StatusKind::Magnum => "b_30.png",
        StatusKind::StaticCharge => "SpellBook06_89.png",
        StatusKind::Aimed => "SpellBook08_83.png",
        StatusKind::Armored => "SpellBook08_122.png",
        StatusKind::Regen => "SpellBook08_73.png",
        StatusKind::Flying | StatusKind::RegenTick => "",
        #[cfg(test)]
        _ => "",
    }
}

pub fn all_icon_filenames() -> &'static [&'static str] {
    &[
        "SpellBook08_130.png",
        "SpellBook08_111.png",
        "b_31_1.png",
        "b_40_02.png",
        "b_30.png",
        "SpellBook06_89.png",
        "SpellBook08_83.png",
        "SpellBook08_122.png",
        "SpellBook08_73.png",
    ]
}
