use std::cmp;
use std::rc::Rc;

use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::{HitTestResult, View};
use crate::after_image::{RenderCanvas, RenderContext};
use crate::atlas::BoxResult;

pub struct StatusBarView {
    position: SDLPoint,
}

impl StatusBarView {
    pub fn init(render_context: &RenderContext, position: SDLPoint) -> BoxResult<StatusBarView> {
        Ok(StatusBarView { position })
    }
}

impl View for StatusBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        Ok(())
    }

    fn hit_test(&self, _ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        None
    }
}

struct StatusBarItemView {}

impl StatusBarItemView {
    pub fn init(position: SDLPoint, render_context: &RenderContext) -> StatusBarItemView {
        StatusBarItemView {}
    }
}

impl View for StatusBarItemView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        Ok(())
    }

    fn hit_test(&self, _ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        None
    }
}
