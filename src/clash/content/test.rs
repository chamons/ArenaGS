use std::collections::HashMap;

use super::super::*;

pub fn add_test_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.add_skill(SkillInfo::init("TestNone", None, TargetType::None, SkillEffect::None));
    m.add_skill(SkillInfo::init("TestTile", None, TargetType::Tile, SkillEffect::None));
    m.add_skill(SkillInfo::init("TestEnemy", None, TargetType::Enemy, SkillEffect::None));
    m.add_skill(SkillInfo::init_with_distance(
        "TestWithRange",
        None,
        TargetType::Tile,
        SkillEffect::None,
        Some(2),
        false,
    ));
    m.add_skill(SkillInfo::init_with_distance(
        "TestMove",
        None,
        TargetType::Tile,
        SkillEffect::Move,
        Some(2),
        false,
    ));
    m.add_skill(SkillInfo::init_with_distance(
        "TestRanged",
        None,
        TargetType::Enemy,
        SkillEffect::RangedAttack(Damage::init(2), BoltKind::Fire),
        Some(2),
        false,
    ));
    m.add_skill(SkillInfo::init_with_distance(
        "TestMelee",
        None,
        TargetType::Enemy,
        SkillEffect::MeleeAttack(Damage::init(2), WeaponKind::Sword),
        Some(1),
        false,
    ));
    m.add_skill(SkillInfo::init("TestAmmo", None, TargetType::None, SkillEffect::None).with_ammo(AmmoKind::Bullets, 1));
    m.add_skill(SkillInfo::init("TestMultiAmmo", None, TargetType::None, SkillEffect::None).with_ammo(AmmoKind::Bullets, 3));
    m.add_skill(SkillInfo::init("TestReload", None, TargetType::None, SkillEffect::Reload(AmmoKind::Bullets)));
    m.add_skill(SkillInfo::init("TestExhaustion", None, TargetType::None, SkillEffect::None).with_exhaustion(50.0));
    m.add_skill(SkillInfo::init("TestFocus", None, TargetType::None, SkillEffect::None).with_focus_use(0.5));
    m.add_skill(
        SkillInfo::init("TestMultiple", None, TargetType::None, SkillEffect::None)
            .with_ammo(AmmoKind::Bullets, 1)
            .with_exhaustion(25.0),
    );
    m.add_skill(SkillInfo::init(
        "TestField",
        None,
        TargetType::Any,
        SkillEffect::Field(FieldEffect::Damage(Damage::init(1), 0), FieldKind::Fire),
    ));
    m.add_skill(SkillInfo::init(
        "TestLargeField",
        None,
        TargetType::Any,
        SkillEffect::Field(FieldEffect::Damage(Damage::init(1), 1), FieldKind::Fire),
    ));
    m.add_skill(SkillInfo::init_with_distance(
        "TestMoveAndShoot",
        None,
        TargetType::Tile,
        SkillEffect::MoveAndShoot(Damage::init(1), Some(5), BoltKind::Fire),
        Some(1),
        true,
    ));
    m.add_skill(SkillInfo::init("Buff", None, TargetType::None, SkillEffect::Buff(StatusKind::Aimed, 300)));
    m.add_skill(SkillInfo::init("BuffOthers", None, TargetType::Any, SkillEffect::Buff(StatusKind::Aimed, 300)));
    m.add_skill(SkillInfo::init(
        "BuffAndSwing",
        None,
        TargetType::Enemy,
        SkillEffect::BuffThen(
            StatusKind::Armored,
            300,
            Box::from(SkillEffect::MeleeAttack(Damage::init(2), WeaponKind::Sword)),
        ),
    ));
    m.add_skill(SkillInfo::init_with_distance(
        "BuffAndMove",
        None,
        TargetType::Tile,
        SkillEffect::BuffThen(StatusKind::Aimed, 200, Box::from(SkillEffect::Move)),
        Some(1),
        true,
    ));
    m.add_skill(SkillInfo::init_with_distance(
        "ShootThenBuff",
        None,
        TargetType::Enemy,
        SkillEffect::ThenBuff(Box::from(SkillEffect::RangedAttack(Damage::init(2), BoltKind::Fire)), StatusKind::Aimed, 200),
        Some(1),
        true,
    ));
    m.add_skill(SkillInfo::init("TestNoTime", None, TargetType::None, SkillEffect::None).with_no_time());
    m.add_skill(SkillInfo::init("TestSpawn", None, TargetType::Tile, SkillEffect::Spawn(SpawnKind::BirdSpawn)));
    m.add_skill(SkillInfo::init(
        "TestSpawnField",
        None,
        TargetType::Any,
        SkillEffect::Field(FieldEffect::Spawn(SpawnKind::BirdSpawn), FieldKind::Fire),
    ));
    m.add_skill(SkillInfo::init(
        "TestReplaceSpawn",
        None,
        TargetType::None,
        SkillEffect::SpawnReplace(SpawnKind::BirdSpawn),
    ));
    m.add_skill(SkillInfo::init_with_distance(
        "TestTap",
        None,
        TargetType::Enemy,
        SkillEffect::RangedAttack(Damage::init(0), BoltKind::Fire),
        Some(2),
        false,
    ));
    m.add_skill(SkillInfo::init_with_distance(
        "TestCharge",
        None,
        TargetType::Any,
        SkillEffect::ChargeAttack(Damage::init(1), WeaponKind::Sword),
        Some(3),
        false,
    ));
}
