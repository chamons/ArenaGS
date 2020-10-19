use std::rc::Rc;

use specs::prelude::*;

use super::battle_scene::BattleScene;
use super::death_scene::DeathScene;
use super::round_fade_scene::RoundFadeScene;
use crate::after_image::prelude::*;
use crate::clash::{ProgressionComponent, ProgressionState};
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

    fn prepare_round_fade_scene(&self, render_context: &RenderContextHolder, progression: ProgressionState) -> Box<dyn Scene> {
        let snapshot = Director::screen_shot(render_context).unwrap();
        Box::from(RoundFadeScene::init(snapshot, progression))
    }
}

fn get_progression(ecs: &World) -> ProgressionState {
    ecs.read_resource::<ProgressionComponent>().state.clone()
}

pub fn create_stage_direction_from_state(state: &ProgressionState) -> World {
    let mut world = World::new();
    world.insert(ProgressionComponent::init(state.clone()));
    return world;
}

impl Storyteller for ArenaStoryteller {
    fn follow_stage_direction(&self, direction: StageDirection, render_context: &RenderContextHolder) -> EventStatus {
        match direction {
            StageDirection::Continue => EventStatus::Continue,
            StageDirection::BattlePlayerDeath(message) => EventStatus::NewScene(self.prepare_battle_end_scene(render_context, message)),

            StageDirection::ShowRewards(state) => EventStatus::NewScene(Box::from(
                crate::intermission::RewardScene::init(render_context, &self.text_renderer, get_progression(&state)).expect("Unable to load reward scene"),
            )),
            StageDirection::ShowCharacter(state) => EventStatus::NewScene(Box::from(
                crate::intermission::CharacterScene::init(render_context, &self.text_renderer, get_progression(&state))
                    .expect("Unable to load character scene"),
            )),
            StageDirection::NewRound(state) => EventStatus::NewScene(Box::new(
                BattleScene::init(&self.render_context, &self.text_renderer, get_progression(&state)).expect("Unable to load additional battle scene"),
            )),
            StageDirection::BattleEnemyDefeated(state) => EventStatus::NewScene(self.prepare_round_fade_scene(render_context, get_progression(&state))),
        }
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        Box::new(BattleScene::init(&self.render_context, &self.text_renderer, ProgressionState::init(0)).expect("Unable to load initial battle scene"))
    }
}
