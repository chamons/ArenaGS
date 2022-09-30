use ggez::glam::Vec2;
use keyframe::{functions::Step, AnimationSequence, Keyframe};

use crate::core::{AnimationState, Appearance, AppearanceKind};

use super::BackingImage;

enum SpriteSize {
    Detailed,
    LargeEnemy,
    Bolt,
}

impl BackingImage for AppearanceKind {
    fn filename(&self) -> &str {
        match self {
            AppearanceKind::MaleBrownHairBlueBody => "/images/battle/1_1.png",
            AppearanceKind::Golem => "/images/monsters/$monster_golem1.png",
            AppearanceKind::FireBolt => "/images/bolts/fire.png",
        }
    }
}

impl Appearance {
    pub fn filename(&self) -> &str {
        self.kind.filename()
    }

    const IDLE_ANIMATION_LENGTH: f32 = 140.0 / 3.0;
    pub fn create_idle_animation(&self) -> AnimationSequence<f32> {
        self.create_animation(Appearance::IDLE_ANIMATION_LENGTH)
    }

    const DEFAULT_ANIMATION_LENGTH: f32 = 120.0 / 3.0;
    pub fn create_standard_sprite_animation(&self) -> AnimationSequence<f32> {
        self.create_animation(Appearance::DEFAULT_ANIMATION_LENGTH)
    }

    fn create_animation(&self, animation_length: f32) -> AnimationSequence<f32> {
        let frames: Vec<Keyframe<f32>> = (0..self.sprite_animation_length())
            .map(|i| (i as f32, i as f32 * animation_length, Step).into())
            .collect();
        AnimationSequence::from(frames)
    }

    pub fn sprite_rect(&self, animation_offset: usize) -> (usize, usize) {
        let index = self.sprite_index(animation_offset);
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
            SpriteSize::Bolt => 2.0,
        }
    }

    pub fn sprite_offset(&self) -> Vec2 {
        match self.sprite_size_class() {
            SpriteSize::Detailed => (0.0, -17.0).into(),
            SpriteSize::LargeEnemy => self.large_enemy_size_class().offset(),
            SpriteSize::Bolt => (0.0, 0.0).into(),
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
            SpriteSize::Bolt => (64, 64),
        }
    }

    fn sprite_index(&self, animation_offset: usize) -> usize {
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
            SpriteSize::Bolt => match self.kind {
                AppearanceKind::FireBolt => 10,
                _ => panic!("Unexpected bolt kind"),
            },
        }
    }

    fn sprite_sheet_size(&self) -> usize {
        match self.sprite_size_class() {
            SpriteSize::Detailed => 9,
            SpriteSize::LargeEnemy => 3,
            SpriteSize::Bolt => match self.kind {
                AppearanceKind::FireBolt => 5,
                _ => panic!("Unexpected bolt kind"),
            },
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
            AppearanceKind::FireBolt => SpriteSize::Bolt,
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
