use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

use super::components::*;
use crate::after_image::CharacterAnimationState;
use crate::atlas::{SizedPoint, ToSerialize};
use crate::clash::*;

// All non-test create_entity() call should live here
// so we make sure they are marked with ToSerialize

pub fn player(ecs: &mut World) {
    ecs.create_entity()
        .with(RenderComponent::init(RenderInfo::init_with_char_state(
            SpriteKinds::MaleBrownHairBlueBody,
            CharacterAnimationState::Idle,
        )))
        .with(PositionComponent::init(SizedPoint::init(4, 4)))
        .with(CharacterInfoComponent::init(CharacterInfo::init(
            "Player",
            Defenses::just_health(10),
            Temperature::init(),
        )))
        .with(StatusComponent::init())
        .with(PlayerComponent::init())
        .with(TimeComponent::init(0))
        .with(SkillResourceComponent::init(&[(AmmoKind::Bullets, 6)]).with_focus(1.0))
        .with(SkillsComponent::init(&[]))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();
}

pub fn bird_monster(ecs: &mut World) {
    ecs.create_entity()
        .with(RenderComponent::init(RenderInfo::init(SpriteKinds::MonsterBirdBrown)))
        .with(PositionComponent::init(SizedPoint::init_multi(5, 5, 2, 2)))
        .with(CharacterInfoComponent::init(CharacterInfo::init(
            "Giant Bird",
            Defenses::just_health(25),
            Temperature::init(),
        )))
        .with(StatusComponent::init())
        .with(BehaviorComponent::init(BehaviorKind::Random))
        .with(TimeComponent::init(0))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();
}

pub fn map_background(ecs: &mut World) {
    ecs.create_entity()
        .with(RenderComponent::init(RenderInfo::init_with_order(
            SpriteKinds::BeachBackground,
            RenderOrder::Background,
        )))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();
}
