use std::rc::Rc;

use specs::prelude::*;

use super::battle_scene::BattleScene;
use super::death_scene::DeathScene;
use super::round_fade_scene::RoundFadeScene;
use crate::after_image::prelude::*;
use crate::clash::{new_game, Equipment, ProgressionComponent};
use crate::conductor::{Director, EventStatus, Scene, StageDirection, Storyteller};

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

    fn prepare_round_fade_scene(&self, render_context: &RenderContextHolder, ecs: World) -> Box<dyn Scene> {
        let snapshot = Director::screen_shot(render_context).unwrap();
        Box::from(RoundFadeScene::init(snapshot, ecs))
    }
}

impl Storyteller for ArenaStoryteller {
    fn follow_stage_direction(&self, direction: StageDirection, render_context: &RenderContextHolder) -> EventStatus {
        match direction {
            StageDirection::Continue => EventStatus::Continue,
            StageDirection::BattlePlayerDeath(message) => EventStatus::NewScene(self.prepare_battle_end_scene(render_context, message)),

            StageDirection::ShowRewards(state) => EventStatus::NewScene(Box::from(
                crate::intermission::RewardScene::init(render_context, &self.text_renderer, state).expect("Unable to load reward scene"),
            )),
            StageDirection::ShowCharacter(state) => EventStatus::NewScene(Box::from(
                crate::intermission::CharacterScene::init(render_context, &self.text_renderer, state).expect("Unable to load character scene"),
            )),
            StageDirection::NewRound(state) => EventStatus::NewScene(Box::new(
                BattleScene::init(&self.render_context, &self.text_renderer, state).expect("Unable to load additional battle scene"),
            )),
            StageDirection::BattleEnemyDefeated(state) => EventStatus::NewScene(self.prepare_round_fade_scene(render_context, state)),
        }
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        let new_state = new_game::new_game_intermission_state();
        {
            let mut state = &mut new_state.write_resource::<ProgressionComponent>().state;
            state.equipment = Equipment::init(4, 3, 2, 1);
            state.experience = 200;
        }
        Box::new(BattleScene::init(&self.render_context, &self.text_renderer, new_state).expect("Unable to load initial battle scene"))
    }
}
