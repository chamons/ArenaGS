use bevy_ecs::world::World;
use ggez::graphics::Canvas;

use super::*;

pub struct BattleScene {}

impl BattleScene {
    pub fn new() -> BattleScene {
        BattleScene {}
    }
}

impl Scene<World> for BattleScene {
    fn update(&mut self, world: &mut World, ctx: &mut ggez::Context) -> SceneSwitch<World> {
        SceneSwitch::None
    }

    fn draw(&mut self, world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {}
}
