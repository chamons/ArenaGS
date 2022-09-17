use bevy_ecs::world::World;
use ggez::{
    graphics::{Canvas, Color, DrawParam, Transform},
    mint,
};

use super::{game_state::ScreenScale, *};

pub struct BattleScene {}

impl BattleScene {
    pub fn new() -> BattleScene {
        BattleScene {}
    }
}

fn get_image_draw_params(world: &mut World) -> DrawParam {
    let scale = world.get_resource::<ScreenScale>().unwrap();
    DrawParam {
        transform: Transform::Values {
            dest: mint::Point2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            scale: mint::Vector2 {
                x: scale.scale as f32,
                y: scale.scale as f32,
            },
            offset: mint::Point2 { x: 0.0, y: 0.0 },
        },
        ..Default::default()
    }
}

impl Scene<World> for BattleScene {
    fn update(&mut self, world: &mut World, ctx: &mut ggez::Context) -> SceneSwitch<World> {
        SceneSwitch::None
    }

    fn draw(&mut self, world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {
        let image_draw_params = get_image_draw_params(world);

        let images = world.get_resource::<crate::ui::ImageCache>().unwrap();
        canvas.draw(images.get("/maps/beach/map1.png"), image_draw_params);

        let map = world.get_resource::<crate::core::map::Map>().unwrap();
    }
}
