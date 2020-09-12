use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

use crate::atlas::{Point, SizedPoint, ToSerialize};
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

fn create_monster(ecs: &mut World, name: &str, kind: SpawnKind, behavior_kind: BehaviorKind, defenses: Defenses, position: SizedPoint) {
    let monster = ecs
        .create_entity()
        .with(PositionComponent::init(position))
        .with(CharacterInfoComponent::init(CharacterInfo::init(name, defenses, Temperature::init())))
        .with(StatusComponent::init())
        .with(BehaviorComponent::init(behavior_kind))
        .with(TimeComponent::init(0))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();

    ecs.raise_event(EventKind::Creation(kind), Some(monster));
}

pub fn bird_monster(ecs: &mut World) {
    create_monster(
        ecs,
        "Giant Bird",
        SpawnKind::Bird,
        BehaviorKind::Bird,
        Defenses::just_health(150),
        SizedPoint::init_multi(5, 8, 2, 2),
    );
}

pub fn bird_monster_add_egg(ecs: &mut World, position: SizedPoint) {
    create_monster(ecs, "Egg", SpawnKind::Egg, BehaviorKind::Egg, Defenses::init(0, 2, 0, 20), position);
}

pub fn bird_monster_add(ecs: &mut World, position: SizedPoint) {
    create_monster(ecs, "Bird", SpawnKind::BirdSpawn, BehaviorKind::BirdAdd, Defenses::just_health(40), position);
}

pub fn create_orb(ecs: &mut World, position: Point, attack: AttackComponent, orb: OrbComponent) -> Entity {
    ecs.create_entity()
        .with(PositionComponent::init(SizedPoint::from(position)))
        .with(attack)
        .with(orb)
        .with(BehaviorComponent::init(BehaviorKind::Orb))
        .with(TimeComponent::init(0))
        .with(FieldComponent::init())
        .marked::<SimpleMarker<ToSerialize>>()
        .build()
}

pub fn create_damage_field(ecs: &mut World, position: SizedPoint, attack: AttackComponent, color: (u8, u8, u8)) -> Entity {
    ecs.create_entity()
        .with(PositionComponent::init(position))
        .with(attack)
        .with(BehaviorComponent::init(BehaviorKind::Explode))
        .with(FieldComponent::init_single(color.0, color.1, color.2))
        .with(TimeComponent::init(-BASE_ACTION_COST))
        .marked::<SimpleMarker<ToSerialize>>()
        .build()
}
