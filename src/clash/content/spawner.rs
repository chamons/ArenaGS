use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

use crate::atlas::{SizedPoint, ToSerialize};
use crate::clash::*;

// All non-test create_entity() call should live here
// so we make sure they are marked with ToSerialize

pub fn player(ecs: &mut World) {
    let player = ecs
        .create_entity()
        .with(PositionComponent::init(SizedPoint::init(4, 4)))
        .with(CharacterInfoComponent::init(CharacterInfo::init(
            "Player",
            Defenses::just_health(10),
            Temperature::init(),
        )))
        .with(StatusComponent::init())
        .with(PlayerComponent::init())
        .with(SkillsComponent::init(&[]))
        .with(TimeComponent::init(0))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();

    content::gunslinger::setup_gunslinger(ecs, &player);

    ecs.raise_event(EventKind::Creation(SpawnKind::Player), Some(player));
}

pub fn bird_monster(ecs: &mut World) {
    let bird = ecs
        .create_entity()
        .with(PositionComponent::init(SizedPoint::init_multi(5, 8, 2, 2)))
        .with(CharacterInfoComponent::init(CharacterInfo::init(
            "Giant Bird",
            Defenses::just_health(150),
            Temperature::init(),
        )))
        .with(StatusComponent::init())
        .with(BehaviorComponent::init(BehaviorKind::Bird))
        .with(TimeComponent::init(0))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();

    ecs.raise_event(EventKind::Creation(SpawnKind::Bird), Some(bird));
}

pub fn bird_monster_add(ecs: &mut World, position: SizedPoint) {
    let bird = ecs
        .create_entity()
        .with(PositionComponent::init(position))
        .with(CharacterInfoComponent::init(CharacterInfo::init(
            "Bird",
            Defenses::just_health(40),
            Temperature::init(),
        )))
        .with(StatusComponent::init())
        .with(BehaviorComponent::init(BehaviorKind::BirdAdd))
        .with(TimeComponent::init(0))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();

    ecs.raise_event(EventKind::Creation(SpawnKind::BirdSpawn), Some(bird));
}
