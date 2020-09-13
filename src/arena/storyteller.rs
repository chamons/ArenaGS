use std::rc::Rc;

use specs::prelude::*;

use super::battle_scene::BattleScene;
use crate::after_image::{RenderContextHolder, TextRenderer};
use crate::conductor::{EventStatus, StageDirection, Storyteller};

pub struct ArenaStoryteller {}

impl Storyteller for ArenaStoryteller {
    fn follow_stage_direction(&self, direction: StageDirection, render_context: &RenderContextHolder, text_renderer: &Rc<TextRenderer>) -> EventStatus {
        match direction {
            StageDirection::NewGame => EventStatus::NewScene(Box::new(BattleScene::init(render_context, text_renderer).unwrap())),
            StageDirection::BattleEnemyDefeated => EventStatus::NewScene(Box::new(BattleScene::init(render_context, text_renderer).unwrap())),
            StageDirection::BattlePlayerDeath(description) => EventStatus::NewScene(Box::new(BattleScene::init(render_context, text_renderer).unwrap())),
        }
    }
}
