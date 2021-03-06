use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

use crate::atlas::prelude::*;
use crate::clash::*;

// All non-test create_entity() call should live here
// so we make sure they are marked with ToSerialize

pub fn player(ecs: &mut World, position: Point, resources: SkillResourceComponent, defenses: DefenseComponent) -> Entity {
    let player = ecs
        .create_entity()
        .with(PositionComponent::init(SizedPoint::init(position.x, position.y)))
        .with(IsCharacterComponent::init())
        .with(NamedComponent::init("Player"))
        .with(defenses)
        .with(SkillPowerComponent::init(0))
        .with(TemperatureComponent::init(Temperature::init()))
        .with(StatusComponent::init())
        .with(PlayerComponent::init())
        .with(SkillsComponent::init(&[]))
        .with(TimeComponent::init(0))
        .with(resources)
        .marked::<SimpleMarker<ToSerialize>>()
        .build();

    ecs.raise_event(EventKind::Creation(SpawnKind::Player), Some(player));
    player
}

#[allow(clippy::too_many_arguments)]
fn create_monster(
    ecs: &mut World,
    name: &str,
    kind: SpawnKind,
    behavior_kind: BehaviorKind,
    defenses: Defenses,
    position: SizedPoint,
    skill_resources: SkillResourceComponent,
    skill_power: u32,
) -> Entity {
    let monster = ecs
        .create_entity()
        .with(PositionComponent::init(position))
        .with(IsCharacterComponent::init())
        .with(NamedComponent::init(name))
        .with(DefenseComponent::init(defenses))
        .with(SkillPowerComponent::init(skill_power))
        .with(TemperatureComponent::init(Temperature::init()))
        .with(StatusComponent::init())
        .with(BehaviorComponent::init(behavior_kind))
        .with(TimeComponent::init(0))
        .with(skill_resources)
        .marked::<SimpleMarker<ToSerialize>>()
        .build();

    ecs.raise_event(EventKind::Creation(kind), Some(monster));
    monster
}

pub fn elementalist(ecs: &mut World, position: Point, difficulty: u32) {
    create_monster(
        ecs,
        "Elementalist",
        SpawnKind::Elementalist,
        BehaviorKind::Elementalist,
        Defenses::init(0, 0, 40 + 10 * difficulty, 40),
        SizedPoint::init(position.x, position.y),
        SkillResourceComponent::init(&[(AmmoKind::Charge, 60, 100)]),
        0,
    );
}

pub fn water_elemental(ecs: &mut World, position: Point, difficulty: u32) -> Entity {
    create_monster(
        ecs,
        "Water Elemental",
        SpawnKind::WaterElemental,
        BehaviorKind::WaterElemental,
        Defenses::just_health(40 + 10 * difficulty).with_resistances(Resistances::init(&[(DamageElement::ICE, 10)])),
        SizedPoint::init(position.x, position.y),
        SkillResourceComponent::init(&[]),
        0,
    )
}

pub fn fire_elemental(ecs: &mut World, position: Point, difficulty: u32) -> Entity {
    create_monster(
        ecs,
        "Fire Elemental",
        SpawnKind::FireElemental,
        BehaviorKind::FireElemental,
        Defenses::init(0, 0, 30 + 10 * difficulty, 10).with_resistances(Resistances::init(&[(DamageElement::FIRE, 10)])),
        SizedPoint::init(position.x, position.y),
        SkillResourceComponent::init(&[]),
        0,
    )
}

pub fn wind_elemental(ecs: &mut World, position: Point, difficulty: u32) -> Entity {
    create_monster(
        ecs,
        "Wind Elemental",
        SpawnKind::WindElemental,
        BehaviorKind::WindElemental,
        Defenses::init(1, 0, 0, 30 + 10 * difficulty).with_resistances(Resistances::init(&[(DamageElement::LIGHTNING, 10)])),
        SizedPoint::init(position.x, position.y),
        SkillResourceComponent::init(&[]),
        0,
    )
}

pub fn earth_elemental(ecs: &mut World, position: Point, difficulty: u32) -> Entity {
    create_monster(
        ecs,
        "Earth Elemental",
        SpawnKind::EarthElemental,
        BehaviorKind::EarthElemental,
        Defenses::init(0, 1, 0, 40 + 10 * difficulty),
        SizedPoint::init(position.x, position.y),
        SkillResourceComponent::init(&[]),
        0,
    )
}

pub fn simple_golem(ecs: &mut World, position: Point) {
    create_monster(
        ecs,
        "Simple Golem",
        SpawnKind::SimpleGolem,
        BehaviorKind::SimpleGolem,
        Defenses::init(0, 1, 0, 60),
        SizedPoint::init(position.x, position.y),
        SkillResourceComponent::init(&[]),
        0,
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
        SkillResourceComponent::init(&[(AmmoKind::Feathers, 4, 4), (AmmoKind::Eggs, 3, 3)]),
        1,
    );
}

pub fn bird_monster_add_egg(ecs: &mut World, position: SizedPoint) -> Entity {
    create_monster(
        ecs,
        "Egg",
        SpawnKind::Egg,
        BehaviorKind::Egg,
        Defenses::init(0, 2, 0, 10),
        position,
        SkillResourceComponent::init(&[]),
        0,
    )
}

pub fn bird_monster_add(ecs: &mut World, position: SizedPoint) -> Entity {
    create_monster(
        ecs,
        "Bird",
        SpawnKind::BirdSpawn,
        BehaviorKind::BirdAdd,
        Defenses::just_health(20),
        position,
        SkillResourceComponent::init(&[]),
        0,
    )
}

pub fn shadow_gunslinger(ecs: &mut World, position: Point) -> Entity {
    create_monster(
        ecs,
        "Shadow",
        SpawnKind::ShadowGunSlinger,
        BehaviorKind::ShadowGunslinger,
        Defenses::init(1, 0, 1, 1),
        SizedPoint::init(position.x, position.y),
        SkillResourceComponent::init(&[]),
        0,
    )
}

pub fn create_orb(ecs: &mut World, position: Point, attack: AttackComponent, orb: OrbComponent) -> Entity {
    ecs.create_entity()
        .with(PositionComponent::init(SizedPoint::from(position)))
        .with(NamedComponent::init(&orb.name))
        .with(attack)
        .with(orb)
        .with(BehaviorComponent::init(BehaviorKind::Orb))
        .with(TimeComponent::init(0))
        .with(FieldComponent::init())
        .marked::<SimpleMarker<ToSerialize>>()
        .build()
}

pub fn create_damage_field(ecs: &mut World, name: &str, position: SizedPoint, attack: AttackComponent, fields: FieldComponent) -> Entity {
    ecs.create_entity()
        .with(PositionComponent::init(position))
        .with(NamedComponent::init(name))
        .with(attack)
        .with(BehaviorComponent::init(BehaviorKind::Explode))
        .with(fields)
        .with(TimeComponent::init(-BASE_ACTION_COST))
        .marked::<SimpleMarker<ToSerialize>>()
        .build()
}

pub fn create_sustained_damage_field(
    ecs: &mut World,
    name: &str,
    position: SizedPoint,
    attack: AttackComponent,
    fields: FieldComponent,
    duration: u32,
) -> Entity {
    ecs.create_entity()
        .with(PositionComponent::init(position))
        .with(NamedComponent::init(name))
        .with(attack)
        .with(BehaviorComponent::init(BehaviorKind::TickDamage))
        .with(fields)
        .with(DurationComponent::init(duration))
        .with(TimeComponent::init(-BASE_ACTION_COST))
        .marked::<SimpleMarker<ToSerialize>>()
        .build()
}
