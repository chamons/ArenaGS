use enum_iterator::IntoEnumIterator;
use num_enum::IntoPrimitive;
use specs::prelude::*;
use specs_derive::Component;

use crate::after_image::SpriteState;

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
}

#[derive(Component)]
pub struct RenderComponent {
    pub sprite_id: u32,
    pub sprite_state: SpriteState,
    pub z_order: i32,
}

impl RenderComponent {
    pub fn init(sprite_kind: SpriteKinds) -> RenderComponent {
        RenderComponent::init_with_order(sprite_kind, 0)
    }

    pub fn init_with_order(sprite_kind: SpriteKinds, z_order: i32) -> RenderComponent {
        RenderComponent {
            sprite_id: sprite_kind.into(),
            sprite_state: SpriteState::None(),
            z_order,
        }
    }
}
