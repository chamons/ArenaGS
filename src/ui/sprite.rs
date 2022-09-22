use std::cmp;

use bevy_ecs::world::World;
use ggez::{glam::Vec2, mint};

use crate::core::{AnimationState, Appearance, AppearanceKind};

use super::ScreenScale;

pub fn draw(render_position: Vec2, appearance: &Appearance, world: &World) {
    let screen_scale = world.get_resource::<ScreenScale>().unwrap().scale as f32;
    let images = world.get_resource::<crate::ui::ImageCache>().unwrap();

    let image = images.get(appearance.filename());
}

fn get_sprite_sheet_rect_for_index(i: usize, width: f32, height: f32) -> mint::Point2<f32> {
    let row = i % 9;
    let col = i / 9;
    mint::Point2 {
        x: width * row as f32,
        y: height * col as f32,
    }
}

fn get_animation_frame(number_of_frames: usize, animation_length: usize, current_frame: u64) -> usize {
    let period = animation_length / number_of_frames;
    let current_frame = current_frame as usize % animation_length;
    let current_frame = (current_frame / period) as usize;
    cmp::min(current_frame, number_of_frames - 1)
}

enum SpriteSize {
    Detailed,
    LargeEnemy,
}

impl Appearance {
    pub fn filename(&self) -> &'static str {
        match self.kind {
            AppearanceKind::MaleBrownHairBlueBody => "/images/battle/1_1.png",
            AppearanceKind::Golem => "/images/monsters/$monster_golem1.png",
        }
    }

    pub fn sprite_rect(&self, frame: u64) -> (usize, usize) {
        let index = self.sprite_index(frame);
        let sheet_size = self.sprite_sheet_size();
        let row = index % sheet_size;
        let col = index / sheet_size;

        let (width, height) = self.sprite_size();
        (width * row, height * col)
    }

    pub fn sprite_scale(&self) -> f64 {
        match self.sprite_size_class() {
            SpriteSize::Detailed => 1.0,
            SpriteSize::LargeEnemy => self.large_enemy_size_class().scale(),
        }
    }

    pub fn sprite_offset(&self) -> (f64, f64) {
        match self.sprite_size_class() {
            SpriteSize::Detailed => (0.0, 0.0),
            SpriteSize::LargeEnemy => self.large_enemy_size_class().offset(),
        }
    }

    fn sprite_size(&self) -> (usize, usize) {
        match self.sprite_size_class() {
            SpriteSize::Detailed => (144, 144),
            SpriteSize::LargeEnemy => match self.large_enemy_size_class() {
                LargeEnemySize::Normal => (94, 100),
                LargeEnemySize::Bird => (122, 96),
                LargeEnemySize::LargeBird => (122, 96),
            },
        }
    }

    fn sprite_index(&self, frame: u64) -> usize {
        match self.sprite_size_class() {
            SpriteSize::Detailed => {
                let animation_length = match self.state {
                    AnimationState::Idle => 55,
                    _ => 15,
                };
                let offset = get_animation_frame(3, animation_length, frame);

                // The detailed character sheets are somewhat strangely laid out
                // 1, 2, 0
                let offset = match offset {
                    0 => 2,
                    1 => 0,
                    2 => 1,
                    _ => panic!("Unexpected animation offset"),
                };

                let index_base = match self.state {
                    AnimationState::Idle => 0,
                    AnimationState::AttackOne => 3,
                    AnimationState::Walk => 6,
                    AnimationState::AttackTwo => 12,
                    AnimationState::Cheer => 15,
                    AnimationState::Magic => 18,
                    AnimationState::Bow => 21,
                    AnimationState::Crouch => 24,
                    AnimationState::Hit => 36,
                    AnimationState::Status => 42,
                    AnimationState::Item => 48,
                };

                index_base + offset
            }
            SpriteSize::LargeEnemy => frame as usize % 3,
        }
    }

    fn sprite_sheet_size(&self) -> usize {
        match self.sprite_size_class() {
            SpriteSize::Detailed => 9,
            SpriteSize::LargeEnemy => 3,
        }
    }

    fn large_enemy_size_class(&self) -> LargeEnemySize {
        match self.kind {
            AppearanceKind::Golem => LargeEnemySize::Normal,
            _ => panic!("Unset large enemy size"),
        }
    }

    fn sprite_size_class(&self) -> SpriteSize {
        match self.kind {
            AppearanceKind::MaleBrownHairBlueBody => SpriteSize::Detailed,
            AppearanceKind::Golem => SpriteSize::LargeEnemy,
        }
    }
}

enum LargeEnemySize {
    Normal,
    Bird,
    LargeBird,
}

impl LargeEnemySize {
    fn scale(&self) -> f64 {
        match self {
            LargeEnemySize::LargeBird => 1.5,
            _ => 1.0,
        }
    }

    fn offset(&self) -> (f64, f64) {
        match self {
            LargeEnemySize::Normal => (0.0, 0.0),
            LargeEnemySize::Bird | LargeEnemySize::LargeBird => (1.0, -20.0),
        }
    }
}
