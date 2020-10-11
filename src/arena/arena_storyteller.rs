use std::rc::Rc;

use super::death_scene::DeathScene;
use super::round_fade_scene::RoundFadeScene;
use crate::after_image::prelude::*;
use crate::conductor::{Director, EventStatus, Scene, StageDirection, Storyteller};

use super::battle_scene::BattleScene;

pub struct ArenaStoryteller {
    render_context: RenderContextHolder,
    text_renderer: Rc<TextRenderer>,
}

#[allow(dead_code)]
impl ArenaStoryteller {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>) -> ArenaStoryteller {
        ArenaStoryteller {
            render_context: Rc::clone(render_context_holder),
            text_renderer: Rc::clone(&text_renderer),
        }
    }

    fn prepare_battle_end_scene(&self, render_context: &RenderContextHolder, message: String) -> Box<dyn Scene> {
        let snapshot = Director::screen_shot(render_context).unwrap();
        Box::from(DeathScene::init(snapshot, render_context, &self.text_renderer, message).unwrap())
    }

    fn prepare_round_fade_scene(&self, render_context: &RenderContextHolder, phase: u32) -> Box<dyn Scene> {
        let snapshot = Director::screen_shot(render_context).unwrap();
        Box::from(RoundFadeScene::init(snapshot, phase))
    }
}

impl Storyteller for ArenaStoryteller {
    fn follow_stage_direction(&self, direction: StageDirection, render_context: &RenderContextHolder) -> EventStatus {
        match direction {
            StageDirection::Continue => EventStatus::Continue,
            StageDirection::NewGame(phase) => EventStatus::NewScene(Box::new(
                BattleScene::init(&self.render_context, &self.text_renderer, phase).expect("Unable to load additional battle scene"),
            )),
            StageDirection::BattlePlayerDeath(message) => EventStatus::NewScene(self.prepare_battle_end_scene(render_context, message)),
            StageDirection::BattleEnemyDefeated(phase) => EventStatus::NewScene(self.prepare_round_fade_scene(render_context, phase)),
        }
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        Box::new(BattleScene::init(&self.render_context, &self.text_renderer, 0).expect("Unable to load initial battle scene"))
    }
}
