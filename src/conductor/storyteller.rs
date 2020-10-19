use specs::prelude::*;

use super::{EventStatus, Scene};
use crate::after_image::prelude::*;

pub enum StageDirection {
    Continue,
    BattlePlayerDeath(String),

    // The World here is a separate instance
    // just with a ProgressionState black from (from this layer)
    ShowRewards(World),
    ShowCharacter(World),
    NewRound(World),
    BattleEnemyDefeated(World),
}

pub trait Storyteller {
    fn initial_scene(&self) -> Box<dyn Scene>;
    fn follow_stage_direction(&self, direction: StageDirection, render_context: &RenderContextHolder) -> EventStatus;
}
