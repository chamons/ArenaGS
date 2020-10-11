use std::rc::Rc;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::{HitTestResult, View};
use crate::after_image::{IconCache, IconLoader, RenderCanvas, RenderContext};
use crate::atlas::prelude::*;
use crate::clash::{find_player, StatusComponent, StatusKind};

pub struct StatusBarView {
    views: Vec<StatusBarItemView>,
}

impl StatusBarView {
    pub fn init(render_context: &RenderContext, position: SDLPoint) -> BoxResult<StatusBarView> {
        let cache = Rc::new(IconCache::init(render_context, IconLoader::init_icons(), all_icon_filenames())?);
        let mut views = vec![];
        let ui = IconLoader::init_ui();
        for i in 0..10 {
            views.push(StatusBarItemView::init(
                SDLPoint::new(position.x() + i as i32 * 58, position.y()),
                ui.get(render_context, "status_frame.png")?,
                &cache,
                i,
            ));
        }
        Ok(StatusBarView { views })
    }
}

impl View for StatusBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        for view in self.views.iter() {
            view.render(ecs, canvas, frame)?;
        }

        Ok(())
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        for view in self.views.iter() {
            if view.rect.contains_point(SDLPoint::new(x, y)) {
                return view.hit_test(ecs, x, y);
            }
        }
        None
    }
}

struct StatusBarItemView {
    rect: SDLRect,
    icons: Rc<IconCache>,
    frame: Texture,
    status_index: usize,
}

impl StatusBarItemView {
    pub fn init(position: SDLPoint, frame: Texture, icons: &Rc<IconCache>, status_index: usize) -> StatusBarItemView {
        StatusBarItemView {
            rect: SDLRect::new(position.x(), position.y(), 48, 48),
            icons: Rc::clone(icons),
            frame,
            status_index,
        }
    }

    fn get_associated_status(&self, ecs: &World) -> Option<StatusKind> {
        let statuses = ecs.read_storage::<StatusComponent>();
        statuses
            .grab(find_player(&ecs))
            .status
            .get_all()
            .iter()
            .filter(|x| x.should_display())
            .nth(self.status_index)
            .as_ref()
            .copied()
            .copied()
    }
}

impl View for StatusBarItemView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        if let Some(kind) = self.get_associated_status(ecs) {
            let icon = self.icons.get(get_icon_name_for_status(kind));

            canvas.copy(&icon, None, self.rect)?;
            canvas.copy(&self.frame, None, SDLRect::new(self.rect.x() - 2, self.rect.y() - 2, 54, 54))?;
        }
        Ok(())
    }

    fn hit_test(&self, ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        if let Some(kind) = self.get_associated_status(ecs) {
            Some(HitTestResult::Status(kind))
        } else {
            None
        }
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
