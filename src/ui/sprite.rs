use ggez::glam::Vec2;
use keyframe::{functions::Step, AnimationSequence, Keyframe};

use crate::core::{AnimationState, Appearance, AppearanceKind};

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

    pub fn create_animation(&self, frame: Option<f64>) -> AnimationSequence<f32> {
        let animation_length = 140.0 / 3.0;
        let frames: Vec<Keyframe<f32>> = (0..self.sprite_animation_length())
            .map(|i| (i as f32, i as f32 * animation_length, Step).into())
            .collect();
        let mut animation = AnimationSequence::from(frames);
        if let Some(frame) = frame {
            animation.advance_to(frame);
        }
        animation
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

#[allow(dead_code)]
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
