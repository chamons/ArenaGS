use specs::prelude::*;
use specs_derive::Component;

use sdl2::pixels::Color;

#[derive(Component)]
pub struct FieldComponent {
    pub color: Color,
}

impl FieldComponent {
    pub fn init(r: u8, g: u8, b: u8) -> FieldComponent {
        FieldComponent {
            color: Color::from((r, g, b, 140)),
        }
    }
}
