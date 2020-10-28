use std::collections::HashMap;

use super::super::*;
use crate::sequence;

pub fn add_test_skills(m: &mut SkillsResource) {
    m.add(SkillInfo::init("TestNone", None, TargetType::None, SkillEffect::None));
    m.add(SkillInfo::init("TestTile", None, TargetType::Tile, SkillEffect::None));
    m.add(SkillInfo::init("TestEnemy", None, TargetType::Enemy, SkillEffect::None));
    m.add(SkillInfo::init_with_distance(
        "TestWithRange",
        None,
        TargetType::Tile,
        SkillEffect::None,
        Some(2),
        false,
    ));
    m.add(SkillInfo::init_with_distance(
        "TestMove",
        None,
        TargetType::Tile,
        SkillEffect::Move,
        Some(2),
        false,
    ));
    m.add(SkillInfo::init_with_distance(
        "TestRanged",
        None,
        TargetType::Enemy,
        SkillEffect::RangedAttack(Damage::init(2), BoltKind::Fire),
        Some(2),
        false,
    ));
    m.add(SkillInfo::init_with_distance(
        "TestMelee",
        None,
        TargetType::Enemy,
        SkillEffect::MeleeAttack(Damage::init(2), WeaponKind::Sword),
        Some(1),
        false,
    ));
    m.add(SkillInfo::init_with_distance(
        "TestOrb",
        None,
        TargetType::Any,
        SkillEffect::Orb(Damage::init(1), OrbKind::Feather, 2, 8),
        Some(5),
        false,
    ));
    m.add(SkillInfo::init("TestAmmo", None, TargetType::None, SkillEffect::None).with_ammo(AmmoKind::Bullets, 1));
    m.add(SkillInfo::init("TestMultiAmmo", None, TargetType::None, SkillEffect::None).with_ammo(AmmoKind::Bullets, 3));
    m.add(SkillInfo::init("TestReload", None, TargetType::None, SkillEffect::Reload(AmmoKind::Bullets)));
    m.add(SkillInfo::init(
        "TestReloadSome",
        None,
        TargetType::None,
        SkillEffect::ReloadSome(AmmoKind::Bullets, 2),
    ));
    m.add(SkillInfo::init(
        "TestReloadOne",
        None,
        TargetType::None,
        SkillEffect::ReloadSome(AmmoKind::Bullets, 1),
    ));
    m.add(SkillInfo::init(
        "TestReloadSomeRandom",
        None,
        TargetType::None,
        SkillEffect::ReloadSomeRandom(AmmoKind::Bullets, 3),
    ));

    m.add(SkillInfo::init("TestExhaustion", None, TargetType::None, SkillEffect::None).with_exhaustion(50.0));
    m.add(SkillInfo::init("TestFocus", None, TargetType::None, SkillEffect::None).with_focus_use(0.5));
    m.add(
        SkillInfo::init("TestMultiple", None, TargetType::None, SkillEffect::None)
            .with_ammo(AmmoKind::Bullets, 1)
            .with_exhaustion(25.0),
    );
    m.add(SkillInfo::init(
        "TestField",
        None,
        TargetType::Any,
        SkillEffect::Field(FieldEffect::Damage(Damage::init(1), 0), FieldKind::Fire),
    ));
    m.add(SkillInfo::init(
        "TestLargeField",
        None,
        TargetType::Any,
        SkillEffect::Field(FieldEffect::Damage(Damage::init(1), 1), FieldKind::Fire),
    ));
    m.add(SkillInfo::init_with_distance(
        "TestMoveAndShoot",
        None,
        TargetType::Tile,
        SkillEffect::MoveAndShoot(Damage::init(1), Some(5), BoltKind::Fire),
        Some(1),
        true,
    ));
    m.add(SkillInfo::init("Buff", None, TargetType::None, SkillEffect::Buff(StatusKind::Aimed, 300)));
    m.add(SkillInfo::init("BuffOthers", None, TargetType::Any, SkillEffect::Buff(StatusKind::Aimed, 300)));
    m.add(SkillInfo::init(
        "BuffAndSwing",
        None,
        TargetType::Enemy,
        sequence!(
            SkillEffect::Buff(StatusKind::Armored, 300),
            SkillEffect::MeleeAttack(Damage::init(2), WeaponKind::Sword)
        ),
    ));
    m.add(SkillInfo::init_with_distance(
        "BuffAndMove",
        None,
        TargetType::Tile,
        sequence!(SkillEffect::Buff(StatusKind::Aimed, 200), SkillEffect::Move),
        Some(1),
        true,
    ));
    m.add(SkillInfo::init_with_distance(
        "ShootThenBuff",
        None,
        TargetType::Enemy,
        sequence!(
            SkillEffect::RangedAttack(Damage::init(2), BoltKind::Fire),
            SkillEffect::Buff(StatusKind::Aimed, 200)
        ),
        Some(1),
        true,
    ));
    m.add(SkillInfo::init("TestNoTime", None, TargetType::None, SkillEffect::None).with_no_time());
    m.add(SkillInfo::init("TestSpawn", None, TargetType::Tile, SkillEffect::Spawn(SpawnKind::BirdSpawn)));
    m.add(SkillInfo::init(
        "TestSpawnField",
        None,
        TargetType::Any,
        SkillEffect::Field(FieldEffect::Spawn(SpawnKind::BirdSpawn), FieldKind::Fire),
    ));
    m.add(SkillInfo::init(
        "TestReplaceSpawn",
        None,
        TargetType::None,
        SkillEffect::SpawnReplace(SpawnKind::BirdSpawn),
    ));
    m.add(SkillInfo::init_with_distance(
        "TestTap",
        None,
        TargetType::Enemy,
        SkillEffect::RangedAttack(Damage::init(0), BoltKind::Fire),
        Some(2),
        false,
    ));
    m.add(SkillInfo::init_with_distance(
        "TestCharge",
        None,
        TargetType::Any,
        SkillEffect::ChargeAttack(Damage::init(1), WeaponKind::Sword),
        Some(3),
        false,
    ));
    m.add(SkillInfo::init("TestCooldown", None, TargetType::None, SkillEffect::None).with_cooldown(200));
    m.add(
        SkillInfo::init("TestCooldownStartSpent", None, TargetType::None, SkillEffect::None)
            .with_cooldown(200)
            .with_cooldown_spent(),
    );

    m.add(SkillInfo::init(
        "TestSequence",
        None,
        TargetType::None,
        sequence!(SkillEffect::Buff(StatusKind::Aimed, 100), SkillEffect::Buff(StatusKind::Armored, 100)),
    ));
}
