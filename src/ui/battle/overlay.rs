use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Transform},
    mint::{self, Point2},
};

use crate::{core::Position, ui::ImageCache};

use super::TILE_SIZE;

pub fn render_sprite(canvas: &mut Canvas, screen_position: Vec2, position: &Position, draw_bracket: bool, images: &ImageCache) {
    let size = overlay_size(position);
    let screen_position = Vec2 {
        x: screen_position.x + 3.0 - (TILE_SIZE * size as f32) / 2.0,
        y: screen_position.y - 2.0 + (TILE_SIZE * size as f32) / 2.0,
    };

    render_lifebar(canvas, screen_position, 0.9, 0.0, size, images);
    if draw_bracket {
        render_bracket(canvas, screen_position, size, images);
    }
}

fn render_bracket(canvas: &mut Canvas, position: Vec2, scale: u32, images: &ImageCache) {
    let bracket = match scale {
        1 => images.get("/images/frames/small_frame.png"),
        _ => images.get("/images/frames/large_frame.png"),
    };

    let frame_params = DrawParam {
        transform: Transform::Values {
            dest: Point2 {
                x: position.x + 2.0 * (scale as f32),
                y: position.y - (TILE_SIZE * scale as f32) + (7.0 * scale as f32),
            },
            rotation: 0.0,
            scale: mint::Vector2 { x: 1.0, y: 1.0 },
            offset: mint::Point2 { x: 0.0, y: 0.0 },
        },
        ..Default::default()
    };
    canvas.draw(bracket, frame_params);
}

fn render_lifebar(canvas: &mut Canvas, position: Vec2, life_percentage: f32, absorb_percentage: f32, scale: u32, images: &ImageCache) {
    let frame_image = images.get("/ui/life_frame.png");
    let life_image = images.get("/ui/life_bar.png");
    let absorb_image = images.get("/ui/absorb_bar.png");

    let (stat_image, stat) = if absorb_percentage > 0.0 {
        (absorb_image, absorb_percentage)
    } else {
        (life_image, life_percentage)
    };

    let stat_params = DrawParam {
        transform: Transform::Values {
            dest: Point2 {
                x: position.x + 1.0,
                y: position.y + 1.0,
            },
            rotation: 0.0,
            scale: mint::Vector2 {
                x: 1.8 * (0.79 * scale as f32) * stat,
                y: 1.0,
            },
            offset: mint::Point2 { x: 0.0, y: 0.0 },
        },
        ..Default::default()
    };
    canvas.draw(stat_image, stat_params);

    let frame_params = DrawParam {
        transform: Transform::Values {
            dest: position.into(),
            rotation: 0.0,
            scale: mint::Vector2 {
                x: 0.79 * scale as f32,
                y: 1.0,
            },
            offset: mint::Point2 { x: 0.0, y: 0.0 },
        },
        ..Default::default()
    };
    canvas.draw(frame_image, frame_params);
}

fn overlay_size(position: &Position) -> u32 {
    if position.position.width == 1 && position.position.height == 1 {
        1
    } else if position.position.width == 2 && position.position.height == 2 {
        2
    } else {
        panic!();
    }
}
