use std::rc::Rc;

use specs::prelude::*;

use crate::after_image::{RenderContextHolder, TextRenderer};
use crate::clash::{CharacterInfoComponent, PlayerComponent, PlayerDeadComponent};
use crate::conductor::{EventStatus, Scene, Storyteller};

use super::BattleScene;

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
}

impl Storyteller for ArenaStoryteller {
    fn check_place(&self, ecs: &World) -> EventStatus {
        if ecs.try_fetch::<PlayerDeadComponent>().is_some() {
            return EventStatus::NewScene(self.initial_scene());
        }
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let character_infos = ecs.read_storage::<CharacterInfoComponent>();
        let player = ecs.read_storage::<PlayerComponent>();

        let non_player_character_count = (&entities, &character_infos, (&player).maybe()).join().filter(|(_, _, p)| p.is_none()).count();
        if non_player_character_count == 0 {
            return EventStatus::NewScene(self.initial_scene());
        }
        EventStatus::Continue
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        Box::new(BattleScene::init(&self.render_context, &self.text_renderer).unwrap())
    }
}
