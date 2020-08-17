use std::rc::Rc;

use specs::prelude::*;

use crate::after_image::{RenderCanvas, RenderContextHolder, TextRenderer};
use crate::clash::PlayerDeadComponent;
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
        EventStatus::Continue
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        Box::new(BattleScene::init(&self.render_context, &self.text_renderer).unwrap())
    }
}
