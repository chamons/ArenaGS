use enum_iterator::IntoEnumIterator;
use num_enum::IntoPrimitive;
use specs::prelude::*;
use specs_derive::Component;

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

    FireBolt,
    BulletBolt,

    Bomb,
    Explosion,
}

#[derive(PartialEq, IntoEnumIterator)]
#[allow(dead_code)]
pub enum RenderOrder {
    Background,
    Normal,
    Top,
}

#[derive(Component)]
pub struct RenderComponent {
    pub sprite_id: u32,
    pub sprite_state: u32,
    pub order: RenderOrder,
}

impl RenderComponent {
    pub fn init(sprite_kind: SpriteKinds) -> RenderComponent {
        RenderComponent {
            sprite_id: sprite_kind.into(),
            sprite_state: 0,
            order: RenderOrder::Normal,
        }
    }

    pub fn init_with_char_state(sprite_kind: SpriteKinds, sprite_state: CharacterAnimationState) -> RenderComponent {
        RenderComponent {
            sprite_id: sprite_kind.into(),
            sprite_state: sprite_state.into(),
            order: RenderOrder::Normal,
        }
    }

    pub fn init_with_order(sprite_kind: SpriteKinds, order: RenderOrder) -> RenderComponent {
        RenderComponent {
            sprite_id: sprite_kind.into(),
            sprite_state: 0,
            order,
        }
    }
}
