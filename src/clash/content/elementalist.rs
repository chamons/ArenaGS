// The ai macros can add "unnecessary" returns occationally
#![allow(clippy::needless_return)]

use std::collections::HashMap;

use rand::prelude::*;
use specs::prelude::*;

use super::super::*;
use crate::sequence;
use crate::try_behavior;

const TIDAL_SURGE_SIZE: u32 = 2;
const HEALING_MIST_RANGE: u32 = 5;
const MAGMA_ERUPTION_RANGE: u32 = 7;

pub fn elementalist_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.add_skill(
        SkillInfo::init_with_distance(
            "Tidal Surge",
            None,
            TargetType::AnyoneButSelf,
            SkillEffect::ConeAttack(Damage::init(3).with_option(DamageOptions::KNOCKBACK), ConeKind::Water, TIDAL_SURGE_SIZE),
            Some(1),
            false,
        )
        .with_cooldown(200),
    );

    m.add_skill(SkillInfo::init_with_distance(
        "Ice Shard",
        None,
        TargetType::Player,
        SkillEffect::RangedAttack(Damage::init(2), BoltKind::Water),
        Some(2),
        false,
    ));

    m.add_skill(SkillInfo::init_with_distance(
        "Healing Mist",
        None,
        TargetType::Any,
        SkillEffect::Buff(StatusKind::Regen, 400),
        Some(HEALING_MIST_RANGE),
        false,
    ));

    m.add_skill(
        SkillInfo::init_with_distance(
            "Magma Eruption",
            None,
            TargetType::Any,
            SkillEffect::Field(
                FieldEffect::SustainedDamage(Damage::init(1).with_option(DamageOptions::PIERCE_DEFENSES), 6),
                FieldKind::Fire,
            ),
            Some(MAGMA_ERUPTION_RANGE),
            false,
        )
        .with_cooldown(200),
    );

    m.add_skill(SkillInfo::init_with_distance(
        "Lava Bolt",
        None,
        TargetType::Player,
        SkillEffect::RangedAttack(Damage::init(2), BoltKind::Fire),
        Some(4),
        false,
    ));

    m.add_skill(SkillInfo::init_with_distance(
        "Lightning Surge",
        None,
        TargetType::Player,
        SkillEffect::RangedAttack(Damage::init(3), BoltKind::Lightning),
        Some(4),
        false,
    ));

    m.add_skill(
        SkillInfo::init_with_distance(
            "Hailstone",
            None,
            TargetType::Player,
            SkillEffect::Field(FieldEffect::Damage(Damage::init(4), 1), FieldKind::Hail),
            Some(8),
            false,
        )
        .with_cooldown(200),
    );

    m.add_skill(
        SkillInfo::init_with_distance(
            "Earthen Rage",
            None,
            TargetType::Player,
            SkillEffect::ChargeAttack(Damage::init(2).with_option(DamageOptions::KNOCKBACK), WeaponKind::Sword),
            Some(5),
            false,
        )
        .with_cooldown(400),
    );

    m.add_skill(
        SkillInfo::init_with_distance(
            "Rock Slide",
            None,
            TargetType::Player,
            SkillEffect::ChargeAttack(Damage::init(2), WeaponKind::Sword),
            Some(3),
            false,
        )
        .with_cooldown(300),
    );

    m.add_skill(SkillInfo::init_with_distance(
        "Pummel",
        None,
        TargetType::Player,
        SkillEffect::MeleeAttack(Damage::init(3), WeaponKind::Sword),
        Some(1),
        false,
    ));
    m.add_skill(
        SkillInfo::init(
            "Summon Elemental (Water)",
            None,
            TargetType::Tile,
            SkillEffect::Spawn(SpawnKind::WaterElemental),
        )
        .with_ammo(AmmoKind::Charge, 60),
    );
    m.add_skill(
        SkillInfo::init("Summon Elemental (Fire)", None, TargetType::Tile, SkillEffect::Spawn(SpawnKind::FireElemental)).with_ammo(AmmoKind::Charge, 60),
    );
    m.add_skill(
        SkillInfo::init("Summon Elemental (Wind)", None, TargetType::Tile, SkillEffect::Spawn(SpawnKind::WindElemental)).with_ammo(AmmoKind::Charge, 60),
    );
    m.add_skill(
        SkillInfo::init(
            "Summon Elemental (Earth)",
            None,
            TargetType::Tile,
            SkillEffect::Spawn(SpawnKind::EarthElemental),
        )
        .with_ammo(AmmoKind::Charge, 60),
    );

    m.add_skill(SkillInfo::init(
        "Frost Armor",
        None,
        TargetType::None,
        sequence!(SkillEffect::Buff(StatusKind::Armored, 2000), SkillEffect::ReloadSome(AmmoKind::Charge, 5)),
    ));

    m.add_skill(SkillInfo::init(
        "Invoke the Elements",
        None,
        TargetType::None,
        SkillEffect::ReloadSome(AmmoKind::Charge, 10),
    ));

    m.add_skill(
        SkillInfo::init_with_distance(
            "Call Lightning",
            None,
            TargetType::Player,
            sequence!(
                SkillEffect::Field(FieldEffect::Damage(Damage::init(3), 0), FieldKind::Lightning),
                SkillEffect::ReloadSome(AmmoKind::Charge, 5)
            ),
            Some(6),
            false,
        )
        .with_cooldown(200),
    );
}

const MAX_ELEMENTS_SUMMONED: u32 = 4;

fn get_elemental_kind(ecs: &World, entity: Entity) -> Option<ElementalKind> {
    if let Some(b) = ecs.read_storage::<BehaviorComponent>().get(entity) {
        match b.behavior {
            BehaviorKind::WaterElemental => Some(ElementalKind::Water),
            BehaviorKind::FireElemental => Some(ElementalKind::Fire),
            BehaviorKind::WindElemental => Some(ElementalKind::Wind),
            BehaviorKind::EarthElemental => Some(ElementalKind::Earth),
            _ => None,
        }
    } else {
        None
    }
}

fn is_elemental(ecs: &World, entity: Entity) -> bool {
    get_elemental_kind(ecs, entity).is_some()
}

fn get_elemental_summon_count(ecs: &World) -> u32 {
    find_all_characters(ecs).iter().filter(|&&c| is_elemental(ecs, c)).count() as u32
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ElementalKind {
    Water,
    Fire,
    Wind,
    Earth,
}

pub fn get_elemental_summon_to_use(ecs: &World) -> &'static str {
    let mut elements = vec![ElementalKind::Water, ElementalKind::Fire, ElementalKind::Wind, ElementalKind::Earth];

    for e in find_all_characters(ecs).iter().filter(|&&c| is_elemental(ecs, c)) {
        match get_elemental_kind(ecs, *e) {
            Some(ElementalKind::Water) => {
                elements.swap_remove(elements.iter().position(|x| *x == ElementalKind::Water).unwrap());
            }
            Some(ElementalKind::Fire) => {
                elements.swap_remove(elements.iter().position(|x| *x == ElementalKind::Fire).unwrap());
            }
            Some(ElementalKind::Wind) => {
                elements.swap_remove(elements.iter().position(|x| *x == ElementalKind::Wind).unwrap());
            }
            Some(ElementalKind::Earth) => {
                elements.swap_remove(elements.iter().position(|x| *x == ElementalKind::Earth).unwrap());
            }
            _ => panic!("Unexpected item in get_elemental_summon_to_use"),
        }
    }
    elements.shuffle(&mut ecs.fetch_mut::<RandomComponent>().rand);
    match elements[0] {
        ElementalKind::Water => "Summon Elemental (Water)",
        ElementalKind::Fire => "Summon Elemental (Fire)",
        ElementalKind::Wind => "Summon Elemental (Wind)",
        ElementalKind::Earth => "Summon Elemental (Earth)",
    }
}

pub fn elementalist_action(ecs: &mut World, enemy: &Entity) {
    if get_elemental_summon_count(ecs) < MAX_ELEMENTS_SUMMONED {
        try_behavior!(use_skill_with_random_target(ecs, enemy, get_elemental_summon_to_use(ecs), 6));
    }

    if !ecs.has_status(enemy, StatusKind::Armored) {
        try_behavior!(use_skill(ecs, enemy, "Frost Armor"));
    }

    let player_position = ecs.get_position(&find_player(ecs));
    if find_field_at_location(ecs, &player_position).is_none() {
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Call Lightning"));
    }
    try_behavior!(use_skill(ecs, enemy, "Invoke the Elements"));
    wait(ecs, *enemy);
}

pub fn water_elemental_action(ecs: &mut World, enemy: &Entity) {
    let current_position = ecs.get_position(enemy);
    let distance = distance_to_player(ecs, enemy).unwrap_or(0);
    if distance <= 3 {
        if let Some(cone_target) = check_for_cone_striking_player(ecs, enemy, TIDAL_SURGE_SIZE) {
            try_behavior!(use_skill_at_position(ecs, enemy, "Tidal Surge", cone_target));
        }
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Ice Shard"));
    }

    if let Some(target) = any_ally_without_buff_in_range(ecs, enemy, StatusKind::Regen, HEALING_MIST_RANGE) {
        try_behavior!(use_skill_at_position(
            ecs,
            enemy,
            "Healing Mist",
            ecs.get_position(&target).nearest_point_to(current_position)
        ));
    }

    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}

pub fn fire_elemental_action(ecs: &mut World, enemy: &Entity) {
    try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Lava Bolt"));

    let player_position = ecs.get_position(&find_player(ecs));
    if find_field_at_location(ecs, &player_position).is_none() {
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Magma Eruption"));
    }

    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}

pub fn wind_elemental_action(ecs: &mut World, enemy: &Entity) {
    try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Lightning Surge"));

    let player_position = ecs.get_position(&find_player(ecs));
    if find_field_at_location(ecs, &player_position).is_none() {
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Hailstone"));
    }

    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}

pub fn earth_elemental_action(ecs: &mut World, enemy: &Entity) {
    let distance = distance_to_player(ecs, enemy).unwrap_or(0);
    try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Pummel"));
    if distance < 6 {
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Earthen Rage"));
    }
    if distance < 4 {
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Rock Slide"));
    }
    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}
