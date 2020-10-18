use super::{EventStatus, Scene};
use crate::after_image::prelude::*;

pub enum StageDirection {
    Continue,
    ShowRewards(u32),
    ShowCharacter(u32),
    NewRound(u32),
    BattlePlayerDeath(String),
    BattleEnemyDefeated(u32),
}
pub trait Storyteller {
    fn initial_scene(&self) -> Box<dyn Scene>;
    fn follow_stage_direction(&self, direction: StageDirection, render_context: &RenderContextHolder) -> EventStatus;
}
