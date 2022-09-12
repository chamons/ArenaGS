// Based on (with ❤️) https://github.com/ggez/ggez-goodies/blob/master/src/scene.rs

use ggez::{self, graphics::Canvas, input::keyboard::KeyInput};

pub enum SceneSwitch<C> {
    None,
    Push(Box<dyn Scene<C>>),
    Replace(Box<dyn Scene<C>>),
    Pop,
}

pub trait Scene<C> {
    fn update(&mut self, world: &mut C, ctx: &mut ggez::Context) -> SceneSwitch<C>;
    fn draw(&mut self, world: &mut C, ctx: &mut ggez::Context, canvas: &mut Canvas);

    fn mouse_button_down_event(&mut self, _world: &mut C, _ctx: &mut ggez::Context, _x: f32, _y: f32) {}

    fn mouse_button_up_event(&mut self, _world: &mut C, _ctx: &mut ggez::Context, _x: f32, _y: f32) {}

    fn mouse_motion_event(&mut self, _world: &mut C, _ctx: &mut ggez::Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {}

    fn mouse_enter_or_leave(&mut self, _world: &mut C, _ctx: &mut ggez::Context, _entered: bool) {}

    fn key_down_event(&mut self, _world: &mut C, _ctx: &mut ggez::Context, _repeated: bool) {}

    fn key_up_event(&mut self, _world: &mut C, _ctx: &mut ggez::Context, _input: KeyInput) {}

    fn draw_previous(&self) -> bool {
        false
    }
}

impl<C> SceneSwitch<C> {
    pub fn replace<S>(scene: S) -> Self
    where
        S: Scene<C> + 'static,
    {
        SceneSwitch::Replace(Box::new(scene))
    }

    pub fn push<S>(scene: S) -> Self
    where
        S: Scene<C> + 'static,
    {
        SceneSwitch::Push(Box::new(scene))
    }
}

pub struct SceneStack<C> {
    scenes: Vec<Box<dyn Scene<C>>>,
}

impl<C> SceneStack<C> {
    pub fn new() -> Self {
        Self { scenes: Vec::new() }
    }

    pub fn push(&mut self, scene: Box<dyn Scene<C>>) {
        self.scenes.push(scene)
    }

    pub fn pop(&mut self) -> Box<dyn Scene<C>> {
        self.scenes.pop().unwrap()
    }

    pub fn current(&self) -> &dyn Scene<C> {
        &**self.scenes.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut dyn Scene<C> {
        &mut **self.scenes.last_mut().unwrap()
    }

    pub fn switch(&mut self, next_scene: SceneSwitch<C>) -> Option<Box<dyn Scene<C>>> {
        match next_scene {
            SceneSwitch::None => None,
            SceneSwitch::Pop => {
                let s = self.pop();
                Some(s)
            }
            SceneSwitch::Push(s) => {
                self.push(s);
                None
            }
            SceneSwitch::Replace(s) => {
                let old_scene = self.pop();
                self.push(s);
                Some(old_scene)
            }
        }
    }

    pub fn update(&mut self, world: &mut C, ctx: &mut ggez::Context) {
        let next_scene = {
            let current_scene = &mut **self.scenes.last_mut().unwrap();
            current_scene.update(world, ctx)
        };
        self.switch(next_scene);
    }

    fn draw_scenes(scenes: &mut [Box<dyn Scene<C>>], world: &mut C, ctx: &mut ggez::Context, canvas: &mut Canvas) {
        assert!(scenes.len() > 0);
        if let Some((current, rest)) = scenes.split_last_mut() {
            if current.draw_previous() {
                SceneStack::draw_scenes(rest, world, ctx, canvas);
            }
            current.draw(world, ctx, canvas);
        }
    }

    pub fn draw(&mut self, world: &mut C, ctx: &mut ggez::Context, canvas: &mut Canvas) {
        SceneStack::draw_scenes(&mut self.scenes, world, ctx, canvas);
    }

    pub fn mouse_button_down_event(&mut self, world: &mut C, ctx: &mut ggez::Context, x: f32, y: f32) {
        self.current_mut().mouse_button_down_event(world, ctx, x, y);
    }

    pub fn mouse_button_up_event(&mut self, world: &mut C, ctx: &mut ggez::Context, x: f32, y: f32) {
        self.current_mut().mouse_button_up_event(world, ctx, x, y);
    }

    pub fn mouse_motion_event(&mut self, world: &mut C, ctx: &mut ggez::Context, x: f32, y: f32, dx: f32, dy: f32) {
        self.current_mut().mouse_motion_event(world, ctx, x, y, dx, dy);
    }

    pub fn mouse_enter_or_leave(&mut self, world: &mut C, ctx: &mut ggez::Context, entered: bool) {
        self.current_mut().mouse_enter_or_leave(world, ctx, entered);
    }

    pub fn key_down_event(&mut self, world: &mut C, ctx: &mut ggez::Context, repeated: bool) {
        self.current_mut().key_down_event(world, ctx, repeated);
    }

    pub fn key_up_event(&mut self, world: &mut C, ctx: &mut ggez::Context, input: KeyInput) {
        self.current_mut().key_up_event(world, ctx, input);
    }
}
