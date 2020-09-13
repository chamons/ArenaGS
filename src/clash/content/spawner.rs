use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

use crate::atlas::{Point, SizedPoint, ToSerialize};
use crate::clash::*;

// All non-test create_entity() call should live here
// so we make sure they are marked with ToSerialize

pub fn player(ecs: &mut World, position: Point) {
    let player = ecs
        .create_entity()
        .with(PositionComponent::init(SizedPoint::init(position.x, position.y)))
        .with(CharacterInfoComponent::init(CharacterInfo::init(
            "Player",
            Defenses::init(2, 0, 0, 10),
            Temperature::init(),
            0,
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

fn create_monster(ecs: &mut World, name: &str, kind: SpawnKind, behavior_kind: BehaviorKind, defenses: Defenses, position: SizedPoint, skill_power: u32) {
    let monster = ecs
        .create_entity()
        .with(PositionComponent::init(position))
        .with(CharacterInfoComponent::init(CharacterInfo::init(
            name,
            defenses,
            Temperature::init(),
            skill_power,
        )))
        .with(StatusComponent::init())
        .with(BehaviorComponent::init(behavior_kind))
        .with(TimeComponent::init(0))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();

    ecs.raise_event(EventKind::Creation(kind), Some(monster));
}

pub fn elementalist(ecs: &mut World, position: Point, difficulty: u32) {
    create_monster(
        ecs,
        "Elementalist",
        SpawnKind::Elementalist,
        BehaviorKind::Elementalist,
        Defenses::init(0, 0, 20, 80 + 10 * difficulty),
        SizedPoint::init(position.x, position.y),
        difficulty,
    );
}

pub fn water_elemental(ecs: &mut World, position: Point, difficulty: u32) {
    create_monster(
        ecs,
        "Water Elemental",
        SpawnKind::WaterElemental,
        BehaviorKind::WaterElemental,
        Defenses::just_health(40 + 10 * difficulty),
        SizedPoint::init(position.x, position.y),
        difficulty,
    );
}

pub fn fire_elemental(ecs: &mut World, position: Point, difficulty: u32) {
    create_monster(
        ecs,
        "Fire Elemental",
        SpawnKind::FireElemental,
        BehaviorKind::FireElemental,
        Defenses::just_health(40 + 10 * difficulty),
        SizedPoint::init(position.x, position.y),
        difficulty,
    );
}

pub fn wind_elemental(ecs: &mut World, position: Point, difficulty: u32) {
    create_monster(
        ecs,
        "Wind Elemental",
        SpawnKind::WindElemental,
        BehaviorKind::WindElemental,
        Defenses::just_health(40 + 10 * difficulty),
        SizedPoint::init(position.x, position.y),
        difficulty,
    );
}

pub fn earth_elemental(ecs: &mut World, position: Point, difficulty: u32) {
    create_monster(
        ecs,
        "Earth Elemental",
        SpawnKind::EarthElemental,
        BehaviorKind::EarthElemental,
        Defenses::just_health(40 + 10 * difficulty),
        SizedPoint::init(position.x, position.y),
        difficulty,
    );
}

pub fn bird_monster(ecs: &mut World, position: Point, difficulty: u32) {
    create_monster(
        ecs,
        "Giant Bird",
        SpawnKind::Bird,
        BehaviorKind::Bird,
        Defenses::just_health(150 + 20 * difficulty),
        SizedPoint::init_multi(position.x, position.y, 2, 2),
        1 + difficulty,
    );
}

pub fn bird_monster_add_egg(ecs: &mut World, position: SizedPoint) {
    create_monster(ecs, "Egg", SpawnKind::Egg, BehaviorKind::Egg, Defenses::init(0, 2, 0, 10), position, 0);
}

pub fn bird_monster_add(ecs: &mut World, position: SizedPoint) {
    create_monster(ecs, "Bird", SpawnKind::BirdSpawn, BehaviorKind::BirdAdd, Defenses::just_health(20), position, 0);
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

pub fn create_damage_field(ecs: &mut World, position: SizedPoint, attack: AttackComponent, fields: FieldComponent) -> Entity {
    ecs.create_entity()
        .with(PositionComponent::init(position))
        .with(attack)
        .with(BehaviorComponent::init(BehaviorKind::Explode))
        .with(fields)
        .with(TimeComponent::init(-BASE_ACTION_COST))
        .marked::<SimpleMarker<ToSerialize>>()
        .build()
}
