use enum_iterator::IntoEnumIterator;
use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};

use crate::after_image::CharacterAnimationState;

#[allow(dead_code)]
#[derive(Hash, PartialEq, Eq, IntoEnumIterator, IntoPrimitive)]
#[repr(u32)]
pub enum SpriteKinds {
    BeachBackground,

    MaleBrownHairBlueBody,
    MaleBlueHairRedBody,

    MonsterBirdBrown,
    MonsterBirdBlue,
    MonsterBirdRed,

    SmallMonsterBirdBrown,
    SmallMonsterBirdBlue,
    SmallMonsterBirdRed,

    Egg,
    Elementalist,
    WaterElemental,
    FireElemental,
    WindElemental,
    EarthElemental,

    NoBolt,
    FireBolt,
    WaterBolt,
    LightningOrb,
    Bullet,
    FireBullet,
    AirBullet,
    Smoke,

    Bomb,
    Explosion,
    LightningStrike,
    Cloud,
    FireColumn,
    WaterColumn,
}

#[derive(PartialEq, IntoEnumIterator, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub enum RenderOrder {
    Background,
    Normal,
    Top,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RenderInfo {
    pub sprite_id: u32,
    pub sprite_state: u32,
    pub order: RenderOrder,
}

impl RenderInfo {
    pub fn init(sprite_kind: SpriteKinds) -> RenderInfo {
        RenderInfo {
            sprite_id: sprite_kind.into(),
            sprite_state: 0,
            order: RenderOrder::Normal,
        }
    }

    pub fn init_with_char_state(sprite_kind: SpriteKinds, sprite_state: CharacterAnimationState) -> RenderInfo {
        RenderInfo {
            sprite_id: sprite_kind.into(),
            sprite_state: sprite_state.into(),
            order: RenderOrder::Normal,
        }
    }

    pub fn init_with_order(sprite_kind: SpriteKinds, order: RenderOrder) -> RenderInfo {
        RenderInfo {
            sprite_id: sprite_kind.into(),
            sprite_state: 0,
            order,
        }
    }
}
