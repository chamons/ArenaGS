use bevy_ecs::world::World;
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, DrawParam, Rect, Transform},
    mint::{self, Point2},
};

use super::{Animation, ImageCache, MAP_CORNER_X, MAP_CORNER_Y, TILE_SIZE};
use crate::core::{Appearance, Map};

pub fn render_sprite(canvas: &mut Canvas, screen_position: Vec2, appearance: &Appearance, animation: &Animation, images: &ImageCache) {
    let image = images.get(appearance.filename()).clone();
    let animation_offset = animation.sprite.as_ref().map(|a| a.now() as usize).unwrap_or(0);

    let (image_offset_x, image_offset_y) = appearance.sprite_rect(animation_offset);
    let scale = appearance.sprite_scale();
    let offset = appearance.sprite_offset();
    let render_position = screen_position + offset;
    let sprite_size = appearance.sprite_size();

    let draw_params = DrawParam {
        src: Rect {
            x: image_offset_x as f32 / image.width() as f32,
            y: image_offset_y as f32 / image.height() as f32,
            w: sprite_size.0 as f32 / image.width() as f32,
            h: sprite_size.1 as f32 / image.height() as f32,
        },
        transform: Transform::Values {
            rotation: 0.0,
            scale: mint::Vector2 {
                x: scale as f32,
                y: scale as f32,
            },
            offset: mint::Point2 { x: 0.5, y: 0.5 },
            dest: Point2 {
                x: render_position.x,
                y: render_position.y,
            },
        },
        ..Default::default()
    };

    canvas.draw(&image, draw_params);
}

pub fn draw_image(canvas: &mut Canvas, world: &mut World, image: &str, position: mint::Point2<f32>) {
    let images = world.get_resource::<crate::ui::ImageCache>().unwrap();

    canvas.draw(images.get(image), get_image_draw_params(position));
}

pub const GRID_COLOR: Color = Color::new(196.0 / 255.0, 196.0 / 255.0, 196.0 / 255.0, 1.0);
pub const TILE_BORDER: f32 = 2.0;

pub fn draw_map_grid(canvas: &mut Canvas, ctx: &mut ggez::Context) {
    let map_width = Map::MAX_TILES as f32 * TILE_SIZE;
    let grid_horz_edge = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect::new(0.0, 0.0, map_width + 1.0, TILE_BORDER), GRID_COLOR).unwrap();
    canvas.draw(&grid_horz_edge, Vec2::new(MAP_CORNER_X, MAP_CORNER_Y));
    canvas.draw(&grid_horz_edge, Vec2::new(MAP_CORNER_X, MAP_CORNER_Y + map_width));

    let grid_vert_edge = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect::new(0.0, 0.0, TILE_BORDER, map_width + 1.0), GRID_COLOR).unwrap();
    canvas.draw(&grid_vert_edge, Vec2::new(MAP_CORNER_X, MAP_CORNER_Y));
    canvas.draw(&grid_vert_edge, Vec2::new(MAP_CORNER_X + map_width, MAP_CORNER_Y));

    for x in 0..Map::MAX_TILES {
        canvas.draw(&grid_vert_edge, Vec2::new(MAP_CORNER_X + (x as f32 * TILE_SIZE), MAP_CORNER_Y));
        for y in 0..Map::MAX_TILES {
            canvas.draw(&grid_horz_edge, Vec2::new(MAP_CORNER_X, MAP_CORNER_Y + (y as f32 * TILE_SIZE)));
        }
    }
}

fn get_image_draw_params(dest: mint::Point2<f32>) -> DrawParam {
    DrawParam {
        transform: Transform::Values {
            dest,
            rotation: 0.0,
            scale: mint::Vector2 { x: 1.0, y: 1.0 },
            offset: mint::Point2 { x: 0.0, y: 0.0 },
        },
        ..Default::default()
    }
}
