use crate::core::Appearance;
use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Rect, Transform},
    mint::{self, Point2},
};

use super::{Animation, ImageCache};

pub fn render_sprite(canvas: &mut Canvas, render_position: Vec2, appearance: &Appearance, animation: &Animation, images: &ImageCache) {
    let image = images.get(appearance.filename()).clone();
    let animation_offset = animation.sprite.as_ref().map(|a| a.now() as usize).unwrap_or(0);

    let (image_offset_x, image_offset_y) = appearance.sprite_rect(animation_offset);
    let scale = appearance.sprite_scale();
    let offset = appearance.sprite_offset();
    let render_position = render_position + offset;
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
