use std::collections::HashMap;

use super::super::*;

pub fn add_test_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.insert("TestNone", SkillInfo::init(None, TargetType::None, SkillEffect::None));
    m.insert("TestTile", SkillInfo::init(None, TargetType::Tile, SkillEffect::None));
    m.insert("TestEnemy", SkillInfo::init(None, TargetType::Enemy, SkillEffect::None));
    m.insert(
        "TestWithRange",
        SkillInfo::init_with_distance(None, TargetType::Tile, SkillEffect::None, Some(2), false),
    );
    m.insert(
        "TestMove",
        SkillInfo::init_with_distance(None, TargetType::Tile, SkillEffect::Move, Some(2), false),
    );
    m.insert(
        "TestRanged",
        SkillInfo::init_with_distance(
            None,
            TargetType::Enemy,
            SkillEffect::RangedAttack(Damage::init(2), BoltKind::Fire),
            Some(2),
            false,
        ),
    );
    m.insert(
        "TestMelee",
        SkillInfo::init_with_distance(
            None,
            TargetType::Enemy,
            SkillEffect::MeleeAttack(Damage::init(2), WeaponKind::Sword),
            Some(1),
            false,
        ),
    );
    m.insert(
        "TestAmmo",
        SkillInfo::init(None, TargetType::None, SkillEffect::None).with_ammo(AmmoKind::Bullets, 1),
    );
    m.insert(
        "TestMultiAmmo",
        SkillInfo::init(None, TargetType::None, SkillEffect::None).with_ammo(AmmoKind::Bullets, 3),
    );
    m.insert("TestReload", SkillInfo::init(None, TargetType::None, SkillEffect::Reload(AmmoKind::Bullets)));
    m.insert(
        "TestExhaustion",
        SkillInfo::init(None, TargetType::None, SkillEffect::None).with_exhaustion(50.0),
    );
    m.insert("TestFocus", SkillInfo::init(None, TargetType::None, SkillEffect::None).with_focus_use(0.5));
    m.insert(
        "TestMultiple",
        SkillInfo::init(None, TargetType::None, SkillEffect::None)
            .with_ammo(AmmoKind::Bullets, 1)
            .with_exhaustion(25.0),
    );
    m.insert(
        "TestField",
        SkillInfo::init(None, TargetType::Any, SkillEffect::Field(Damage::init(1), FieldKind::Fire)),
    );
    m.insert(
        "TestMoveAndShoot",
        SkillInfo::init_with_distance(
            None,
            TargetType::Tile,
            SkillEffect::MoveAndShoot(Damage::init(1), Some(5), BoltKind::Fire),
            Some(1),
            true,
        ),
    );

    m.insert("Buff", SkillInfo::init(None, TargetType::None, SkillEffect::Buff(StatusKind::Aimed, 300)));

    m.insert(
        "BuffAndMove",
        SkillInfo::init_with_distance(
            None,
            TargetType::Tile,
            SkillEffect::BuffThen(StatusKind::Aimed, 200, Box::from(SkillEffect::Move)),
            Some(1),
            true,
        ),
    );

    m.insert(
        "ShootThenBuff",
        SkillInfo::init_with_distance(
            None,
            TargetType::Enemy,
            SkillEffect::ThenBuff(Box::from(SkillEffect::RangedAttack(Damage::init(2), BoltKind::Fire)), StatusKind::Aimed, 200),
            Some(1),
            true,
        ),
    );
    m.insert("TestNoTime", SkillInfo::init(None, TargetType::None, SkillEffect::None).with_no_time());
    m.insert("TestSpawn", SkillInfo::init(None, TargetType::Tile, SkillEffect::Spawn(SpawnKind::Bird)));
}
