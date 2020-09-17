use std::rc::Rc;

use super::death_scene::DeathScene;
use super::round_fade_scene::RoundFadeScene;
use crate::after_image::{RenderContextHolder, TextRenderer};
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
        Box::from(DeathScene::init(snapshot, &mut render_context.borrow_mut().canvas, &self.text_renderer, message).unwrap())
    }

    fn prepare_round_fade_scene(&self, render_context: &RenderContextHolder, difficulty: u32) -> Box<dyn Scene> {
        let snapshot = Director::screen_shot(render_context).unwrap();
        Box::from(RoundFadeScene::init(snapshot, difficulty))
    }
}

impl Storyteller for ArenaStoryteller {
    fn follow_stage_direction(&self, direction: StageDirection, render_context: &RenderContextHolder) -> EventStatus {
        match direction {
            StageDirection::Continue => EventStatus::Continue,
            StageDirection::NewGame(difficulty) => {
                EventStatus::NewScene(Box::new(BattleScene::init(&self.render_context, &self.text_renderer, difficulty).unwrap()))
            }
            StageDirection::BattlePlayerDeath(message) => EventStatus::NewScene(self.prepare_battle_end_scene(render_context, message)),
            StageDirection::BattleEnemyDefeated(difficulty) => EventStatus::NewScene(self.prepare_round_fade_scene(render_context, difficulty)),
        }
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        Box::new(BattleScene::init(&self.render_context, &self.text_renderer, 0).unwrap())
    }
}
