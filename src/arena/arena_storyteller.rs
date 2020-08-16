use std::rc::Rc;

use crate::after_image::{RenderCanvas, RenderContextHolder, TextRenderer};
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
    fn check_place(&self) -> EventStatus {
        EventStatus::Continue
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        Box::new(BattleScene::init(&self.render_context, &self.text_renderer).unwrap())
    }
}
