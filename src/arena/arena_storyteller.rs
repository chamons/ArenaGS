use std::rc::Rc;

use specs::prelude::*;

use super::death_scene::DeathScene;
use crate::after_image::{RenderContextHolder, TextRenderer};
use crate::clash::{CharacterInfoComponent, PlayerComponent, PlayerDeadComponent};
use crate::conductor::{Director, EventStatus, Scene, StageDirection, Storyteller};

use super::battle_scene::BattleScene;

pub struct ArenaStoryteller {
    render_context: RenderContextHolder,
    text_renderer: Rc<TextRenderer>,
}

impl ArenaStoryteller {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>) -> ArenaStoryteller {
        ArenaStoryteller {
            render_context: Rc::clone(render_context_holder),
            text_renderer: Rc::clone(&text_renderer),
        }
    }

    fn prepare_battle_end_scene(&self, render_context: &RenderContextHolder, message: String) -> Box<dyn Scene> {
        let snapshot = Director::screen_shot(render_context).unwrap();
        Box::from(DeathScene::init(snapshot, message))
    }
}

impl Storyteller for ArenaStoryteller {
    fn follow_stage_direction(&self, direction: StageDirection, render_context: &RenderContextHolder) -> EventStatus {
        match direction {
            StageDirection::Continue => EventStatus::Continue,
            StageDirection::NewGame => EventStatus::NewScene(self.initial_scene()),
            StageDirection::BattlePlayerDeath(message) => EventStatus::NewScene(self.prepare_battle_end_scene(render_context, message)),
            StageDirection::BattleEnemyDefeated => EventStatus::NewScene(self.initial_scene()),
        }
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        Box::new(BattleScene::init(&self.render_context, &self.text_renderer).unwrap())
    }
}
