use std::rc::Rc;

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::after_image::{RenderCanvas, RenderContextHolder, TextRenderer};
use crate::atlas::{BoxResult};
use crate::conductor::{Scene, StageDirection, EventStatus, Storyteller};

pub struct ImageTesterScene {
//    view: Box<dyn View>,
}

impl ImageTesterScene {
    pub fn init(_render_context_holder: &RenderContextHolder, _text_renderer: &Rc<TextRenderer>) -> BoxResult<ImageTesterScene> {
        Ok(ImageTesterScene{})
    }

}

impl Scene for ImageTesterScene {
    fn handle_key(&mut self, _keycode: Keycode) {
    }

    fn handle_mouse(&mut self, _x: i32, _y: i32, _button: Option<MouseButton>) {
    }

    fn render(&mut self, _canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        Ok(())
    }

    fn tick(&mut self, _frame: u64) {
    }

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        StageDirection::Continue
    }
}

pub struct ImageTesterStoryteller {
    render_context: RenderContextHolder,
    text_renderer: Rc<TextRenderer>,
}

impl ImageTesterStoryteller {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>) -> ImageTesterStoryteller {
        ImageTesterStoryteller {
            render_context: Rc::clone(render_context_holder),
            text_renderer: Rc::clone(&text_renderer),
        }
    }
}

impl Storyteller for ImageTesterStoryteller {
    fn follow_stage_direction(&self, _direction: StageDirection, _render_context: &RenderContextHolder) -> EventStatus {
        EventStatus::Continue
    }

    fn initial_scene(&self) -> Box<dyn Scene> {
        Box::new(ImageTesterScene::init(&self.render_context, &self.text_renderer).unwrap())
    }
}
