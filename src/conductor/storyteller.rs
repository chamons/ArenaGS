use super::{EventStatus, Scene};
use crate::after_image::RenderContextHolder;
pub enum StageDirection {
    Continue,
    NewGame,
    BattlePlayerDeath(String),
    BattleEnemyDefeated,
}
pub trait Storyteller {
    fn initial_scene(&self) -> Box<dyn Scene>;
    fn follow_stage_direction(&self, direction: StageDirection, render_context: &RenderContextHolder) -> EventStatus;
}
