mod battle_scene;
pub use battle_scene::BattleScene;

mod debug_overlay;
use bevy_ecs::world::World;
pub use debug_overlay::*;
use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Transform},
    mint,
};

use super::ScreenScale;

// The map is placed at 16x16 but the first maptile is a ways off the corner
pub const MAP_IMAGE_POSITION: mint::Point2<f32> = mint::Point2 { x: 16.0, y: 16.0 };
pub const MAP_CORNER_X: f32 = 50.0;
pub const MAP_CORNER_Y: f32 = 50.0;
pub const TILE_SIZE: f32 = 56.0;

pub fn screen_point_for_map_grid(x: usize, y: usize) -> Vec2 {
    Vec2::new(MAP_CORNER_X + (x as f32) * TILE_SIZE, MAP_CORNER_Y + (y as f32) * TILE_SIZE)
}

fn draw_image(canvas: &mut Canvas, world: &mut World, image: &str, position: mint::Point2<f32>) {
    let screen_scale = world.get_resource::<ScreenScale>().unwrap().scale as f32;
    let images = world.get_resource::<crate::ui::ImageCache>().unwrap();

    canvas.draw(images.get(image), get_image_draw_params(screen_scale, position));
}

fn get_image_draw_params(screen_scale: f32, dest: mint::Point2<f32>) -> DrawParam {
    DrawParam {
        transform: Transform::Values {
            dest,
            rotation: 0.0,
            scale: mint::Vector2 {
                x: screen_scale,
                y: screen_scale,
            },
            offset: mint::Point2 { x: 0.0, y: 0.0 },
        },
        ..Default::default()
    }
}
