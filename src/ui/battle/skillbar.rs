use bevy_ecs::prelude::*;
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, TextLayout},
};

use crate::{
    core::{Skill, SkillKind},
    ui::{BackingImage, ImageCache, GAME_HEIGHT, GAME_WIDTH},
};

const BORDER_WIDTH: f32 = 4.0;
const ICON_SIZE: f32 = 48.0;
const MAX_ICON_COUNT: f32 = 10.0;

pub fn skillbar_draw(world: &mut World, canvas: &mut Canvas) {
    let images = world.get_resource::<crate::ui::ImageCache>().unwrap();

    let skills = vec![Skill::new(SkillKind::Shoot), Skill::new(SkillKind::Dodge)];

    let base_position = Vec2 {
        x: GAME_WIDTH / 4.0,
        y: GAME_HEIGHT - 68.0,
    };
    let offset = get_skillbar_offset(&skills);
    for (i, skill) in skills.iter().enumerate().take(10) {
        let position = Vec2 {
            x: base_position.x + offset + BORDER_WIDTH + (ICON_SIZE + BORDER_WIDTH + 1.0) * i as f32,
            y: base_position.y + BORDER_WIDTH + 1.0,
        };
        draw_skill(skill, i, position, images, canvas);
    }
}

fn draw_skill(skill: &Skill, index: usize, position: Vec2, images: &ImageCache, canvas: &mut Canvas) {
    let skillbar_frame = images.get("/ui/skillbar_frame.png");
    let skill_image = images.get(skill.kind.filename());
    canvas.draw(skill_image, Vec2::new(position.x + BORDER_WIDTH / 2.0, position.y + BORDER_WIDTH / 2.0));

    canvas.draw(skillbar_frame, position);
    canvas.draw(
        graphics::Text::new(&format!("{}", map_index_to_hotkey(index)))
            .set_font("default")
            .set_scale(11.0)
            .set_bounds(Vec2::new(4.0, 7.0))
            .set_layout(TextLayout::center()),
        Vec2::new(position.x + (ICON_SIZE / 2.0), position.y + ICON_SIZE),
    );
}

pub fn map_index_to_hotkey(index: usize) -> usize {
    match index {
        9 => 0,
        _ => index + 1,
    }
}

fn get_skillbar_offset(skills: &[Skill]) -> f32 {
    (MAX_ICON_COUNT as f32 - skills.len() as f32) * (ICON_SIZE + BORDER_WIDTH) / 2.0
}

impl BackingImage for SkillKind {
    fn filename(&self) -> &str {
        match self {
            SkillKind::Shoot => "/icons/items/gun_08_b.PNG",
            SkillKind::Dodge => "/icons/spell/SpellBook02_44.png",
        }
    }
}
