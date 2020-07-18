use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::View;
use crate::after_image::{FontColor, FontSize, RenderCanvas, TextRenderer};
use crate::arena::components::BattleSceneState;
use crate::arena::read_state;
use crate::atlas::BoxResult;

pub struct DebugView<'a> {
    position: SDLPoint,
    text: &'a TextRenderer<'a>,
}

impl<'a> DebugView<'a> {
    pub fn init(position: SDLPoint, text: &'a TextRenderer<'a>) -> BoxResult<DebugView> {
        Ok(DebugView { position, text })
    }
}

impl<'a> View for DebugView<'a> {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        if let BattleSceneState::Debug(kind) = read_state(&ecs) {
            let state = format!("Debug: {}", kind.to_string());
            self.text
                .render_text(&state, self.position.x, self.position.y, canvas, FontSize::Small, FontColor::Red)?;
        }
        Ok(())
    }
}
