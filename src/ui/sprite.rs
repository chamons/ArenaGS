use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Rect, Transform},
    mint::{self, Point2},
};
use keyframe::{functions::Step, AnimationSequence, Keyframe};

use crate::core::{AnimationState, Appearance, AppearanceKind};

use super::ImageCache;

pub fn draw(canvas: &mut Canvas, render_position: Vec2, appearance: &Appearance, images: &ImageCache) {
    let image = images.get(appearance.filename()).clone();

    let (image_offset_x, image_offset_y) = appearance.sprite_rect();
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

    pub fn create_animation(&self) -> AnimationSequence<f32> {
        let animation_length = 140.0 / 3.0;
        let frames: Vec<Keyframe<f32>> = (0..self.sprite_animation_length())
            .map(|i| (i as f32, i as f32 * animation_length, Step).into())
            .collect();
        AnimationSequence::from(frames)
    }

    pub fn sprite_rect(&self) -> (usize, usize) {
        let index = self.sprite_index();
        let sheet_size = self.sprite_sheet_size();
        let row = index / sheet_size;
        let col = index % sheet_size;

        let (width, height) = self.sprite_size();
        (width * col, height * row)
    }

    pub fn sprite_scale(&self) -> f32 {
        match self.sprite_size_class() {
            SpriteSize::Detailed => 0.65,
            SpriteSize::LargeEnemy => self.large_enemy_size_class().scale(),
        }
    }

    pub fn sprite_offset(&self) -> Vec2 {
        match self.sprite_size_class() {
            SpriteSize::Detailed => (0.0, -30.0).into(),
            SpriteSize::LargeEnemy => self.large_enemy_size_class().offset(),
        }
    }

    pub fn sprite_size(&self) -> (usize, usize) {
        match self.sprite_size_class() {
            SpriteSize::Detailed => (144, 144),
            SpriteSize::LargeEnemy => match self.large_enemy_size_class() {
                LargeEnemySize::Normal => (94, 100),
                LargeEnemySize::Bird => (122, 96),
                LargeEnemySize::LargeBird => (122, 96),
            },
        }
    }

    fn sprite_index(&self) -> usize {
        let animation_offset = self.animation.as_ref().map(|a| a.now() as usize).unwrap_or(0);

        match self.sprite_size_class() {
            SpriteSize::Detailed => {
                // The detailed character sheets are somewhat strangely laid out
                // 1, 2, 0
                let offset = match animation_offset {
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
            SpriteSize::LargeEnemy => animation_offset,
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

    fn sprite_animation_length(&self) -> usize {
        3
    }
}

enum LargeEnemySize {
    Normal,
    Bird,
    LargeBird,
}

impl LargeEnemySize {
    fn scale(&self) -> f32 {
        match self {
            LargeEnemySize::LargeBird => 1.5,
            _ => 1.0,
        }
    }

    fn offset(&self) -> Vec2 {
        match self {
            LargeEnemySize::Normal => (0.0, 0.0).into(),
            LargeEnemySize::Bird | LargeEnemySize::LargeBird => (1.0, -20.0).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use keyframe::*;

    use super::*;

    #[test]
    fn animation_period_test() {
        let expected = [0, 1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0];
        let mut animation = keyframes![(0.0, 0.0), (1.0, 10.0), (2.0, 20.0)];
        for i in (0..100).step_by(10) {
            let current_expected = expected[i / 10];
            let result = animation.now() as u64;
            assert_eq!(
                current_expected, result,
                "
Animation value {}
Expected {}
Frame {}
",
                result, current_expected, i
            );
            animation.advance_and_maybe_reverse(10.0);
        }
    }
}
