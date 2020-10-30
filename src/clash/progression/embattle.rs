use std::collections::HashMap;

use specs::prelude::*;

use crate::atlas::prelude::*;
use crate::clash::content::{gunslinger, spawner};
use crate::clash::*;


pub fn create_player(ecs: &mut World, skills: &mut SkillsResource, player_position: Point) {
    // Need to modify this to take defense mods and resource mods
    spawner::player(ecs, player_position);

    // This needs to completely go and have callback
    gunslinger::gunslinger_skills(skills);

    // Need something for attack modes

    // Get rid of gunslinger::get_weapon_skills

    // Make attune ammo rotate past what we don't have
    // Don't add it if we have only one kind
}

fn collect_attack_modes<F>(ecs: &World, get: F) -> Vec<String>
where
    F: Fn(&str) -> String,
{
    let mut modes = vec![];
    modes.push(get("Default"));
    for e in ecs.read_resource::<ProgressionComponent>().state.equipment.all() {
        if let Some(e) = e {
            for effect in e.effect {
                match effect {
                    EquipmentEffect::UnlocksAbilityMode(kind) => modes.push(kind),
                    _ => {}
                }
            }
        }
    }
    modes
}

fn collect_attack_skills<F>(ecs: &World, skills: &mut SkillsResource, get: F)
where
    F: Fn(&str) -> SkillInfo,
{
    let mut attacks = vec![];
    let mut weapon_range = 0;
    let mut weapon_strength = 0;

    let mut skill_range = HashMap::new();
    let mut add_skill_range = |kind: String, delta: i32| *skill_range.entry(kind).or_insert(0) += delta;

    let mut skill_damage = HashMap::new();
    let mut add_skill_damage = |kind: String, delta: i32| *skill_damage.entry(kind).or_insert(0) += delta;

    for e in ecs.read_resource::<ProgressionComponent>().state.equipment.all() {
        if let Some(e) = e {
            for effect in e.effect {
                match effect {
                    EquipmentEffect::UnlocksAbilityClass(kind) => {
                        attacks.push(get(&kind));
                    }
                    EquipmentEffect::ModifiesWeaponRange(delta) => weapon_range += delta,
                    EquipmentEffect::ModifiesWeaponStrength(delta) => weapon_strength += delta,
                    EquipmentEffect::ModifiesSkillRange(delta, skill) => add_skill_range(skill, delta),
                    EquipmentEffect::ModifiesSkillStrength(delta, skill) => add_skill_damage(skill, delta),
                    _ => {}
                }
            }
        }
    }

    if attacks.len() == 0 {
        attacks.push(get("Default"));
    }

    for mut a in attacks {
        a.range = a.range.map(|r| ((r as i32 + weapon_range + skill_range.get(&a.name).unwrap_or(&0)) as u32));

        let new_damage = |damage: Damage| Damage::init((damage.dice() as i32 + weapon_strength + skill_damage.get(&a.name).unwrap_or(&0)) as u32);
        a.effect = match a.effect {
            SkillEffect::RangedAttack(damage, kind) => SkillEffect::RangedAttack(new_damage(damage), kind),
            SkillEffect::MeleeAttack(damage, kind) => SkillEffect::MeleeAttack(new_damage(damage), kind),
            SkillEffect::ConeAttack(damage, kind, range) => SkillEffect::ConeAttack(new_damage(damage), kind, range),
            SkillEffect::ChargeAttack(damage, kind) => SkillEffect::ChargeAttack(new_damage(damage), kind),
            SkillEffect::MoveAndShoot(damage, range, kind) => SkillEffect::MoveAndShoot(new_damage(damage), range, kind),
            SkillEffect::Orb(damage, kind, speed, duration) => SkillEffect::Orb(new_damage(damage), kind, speed, duration),
            SkillEffect::None
            | SkillEffect::Move
            | SkillEffect::Reload(_)
            | SkillEffect::ReloadSome(_, _)
            | SkillEffect::ReloadSomeRandom(_, _)
            | SkillEffect::Field(_, _)
            | SkillEffect::ReloadAndRotateAmmo()
            | SkillEffect::Buff(_, _)
            | SkillEffect::Spawn(_)
            | SkillEffect::SpawnReplace(_)
            | SkillEffect::Sequence(_, _) => a.effect,
        };
        skills.add(a);
    }
}

fn collect_defense_modifier(ecs: &World) -> (i32, i32, i32, i32) {
    let mut armor = 0;
    let mut dodge = 0;
    let mut absorb = 0;
    let mut health = 0;

    for e in ecs.read_resource::<ProgressionComponent>().state.equipment.all() {
        if let Some(e) = e {
            for effect in e.effect {
                match effect {
                    EquipmentEffect::ModifiesArmor(delta) => armor += delta,
                    EquipmentEffect::ModifiesDodge(delta) => dodge += delta,
                    EquipmentEffect::ModifiesAbsorb(delta) => absorb += delta,
                    EquipmentEffect::ModifiesMaxHealth(delta) => health += delta,
                    _ => {}
                }
            }
        }
    }
    (armor, dodge, absorb, health)
}

fn collect_resource_modifier(ecs: &World) -> Vec<(AmmoKind, i32)> {
    let mut resources = HashMap::new();

    let mut add = |kind: AmmoKind, delta: i32| *resources.entry(kind).or_insert(0) += delta;

    for e in ecs.read_resource::<ProgressionComponent>().state.equipment.all() {
        if let Some(e) = e {
            for effect in e.effect {
                match effect {
                    EquipmentEffect::ModifiesResourceTotal(delta, kind) => match kind.as_str() {
                        "Bullets" => {
                            add(AmmoKind::Bullets, delta);
                        }
                        _ => panic!("Unknown ammo kind in collect_resource_modifier {}", kind),
                    },
                    _ => {}
                }
            }
        }
    }
    resources.iter().map(|(&kind, &delta)| (kind, delta)).collect()
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    fn equip_test_state(equip: &[(EquipmentKinds, EquipmentItem, usize)]) -> World {
        let mut ecs = World::new();
        let mut state = ProgressionState::init_empty();
        state.equipment = Equipment::init(4, 4, 4, 4);
        for (kind, item, index) in equip {
            state.equipment.add(*kind, item.clone(), *index);
        }
        ecs.insert(ProgressionComponent::init(state));
        ecs
    }

    fn eq(name: &str, kind: EquipmentKinds, effect: &[EquipmentEffect], index: usize) -> (EquipmentKinds, EquipmentItem, usize) {
        (kind, EquipmentItem::init(name, None, kind, effect), index)
    }

    #[test]
    fn defense_modifiers() {
        let ecs = equip_test_state(&[
            eq(
                "a",
                EquipmentKinds::Armor,
                &[EquipmentEffect::ModifiesArmor(1), EquipmentEffect::ModifiesDodge(1)],
                0,
            ),
            eq("b", EquipmentKinds::Armor, &[EquipmentEffect::ModifiesDodge(1)], 1),
            eq(
                "c",
                EquipmentKinds::Armor,
                &[EquipmentEffect::ModifiesAbsorb(2), EquipmentEffect::ModifiesMaxHealth(2)],
                2,
            ),
            eq(
                "d",
                EquipmentKinds::Armor,
                &[EquipmentEffect::ModifiesAbsorb(1), EquipmentEffect::ModifiesMaxHealth(2)],
                3,
            ),
        ]);
        let (armor, dodge, absorb, life) = collect_defense_modifier(&ecs);
        assert_eq!(1, armor);
        assert_eq!(2, dodge);
        assert_eq!(3, absorb);
        assert_eq!(4, life);
    }

    #[test]
    fn resource_modifiers() {
        let ecs = equip_test_state(&[
            eq(
                "a",
                EquipmentKinds::Weapon,
                &[EquipmentEffect::ModifiesResourceTotal(-1, "Bullets".to_string())],
                0,
            ),
            eq(
                "b",
                EquipmentKinds::Weapon,
                &[EquipmentEffect::ModifiesResourceTotal(-1, "Bullets".to_string())],
                1,
            ),
        ]);

        let resources = collect_resource_modifier(&ecs);
        assert_eq!(1, resources.len());
        let (kind, delta) = resources[0];
        assert_eq!(AmmoKind::Bullets, kind);
        assert_eq!(-2, delta);
    }

    #[test]
    fn attack_skills_default() {
        let ecs = equip_test_state(&[]);

        let mut skills = SkillsResource::init();
        collect_attack_skills(&ecs, &mut skills, |name| match name {
            "Default" => SkillInfo::init("Basic Attack", None, TargetType::Any, SkillEffect::None),
            _ => panic!(),
        });
        assert_eq!(1, skills.skills.len());
        assert_eq!("Basic Attack", skills.get("Basic Attack").name);
    }

    #[test]
    fn attack_skills_skill_unlock() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[EquipmentEffect::UnlocksAbilityClass("Triple Shot".to_string())],
            0,
        )]);

        let mut skills = SkillsResource::init();
        collect_attack_skills(&ecs, &mut skills, |name| match name {
            "Triple Shot" => SkillInfo::init("Triple Shot", None, TargetType::Any, SkillEffect::None),
            _ => panic!(),
        });
        assert_eq!(1, skills.skills.len());
        assert_eq!("Triple Shot", skills.get("Triple Shot").name);
    }

    #[test]
    fn attack_skills_weapon_range() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[
                EquipmentEffect::UnlocksAbilityClass("Triple Shot".to_string()),
                EquipmentEffect::ModifiesWeaponRange(-1),
            ],
            0,
        )]);

        let mut skills = SkillsResource::init();
        collect_attack_skills(&ecs, &mut skills, |name| match name {
            "Triple Shot" => SkillInfo::init_with_distance("Triple Shot", None, TargetType::Any, SkillEffect::None, Some(5), true),
            _ => panic!(),
        });
        assert_eq!(1, skills.skills.len());
        assert_eq!(Some(4), skills.get("Triple Shot").range);
    }

    #[test]
    fn attack_skills_weapon_damage() {
        let ecs = equip_test_state(&[eq("a", EquipmentKinds::Weapon, &[EquipmentEffect::ModifiesWeaponStrength(1)], 0)]);

        let mut skills = SkillsResource::init();
        collect_attack_skills(&ecs, &mut skills, |name| match name {
            "Default" => SkillInfo::init_with_distance(
                "Basic Attack",
                None,
                TargetType::Any,
                SkillEffect::MeleeAttack(Damage::init(3), WeaponKind::Sword),
                Some(5),
                true,
            ),
            _ => panic!(),
        });
        assert_eq!(1, skills.skills.len());
        match skills.get("Basic Attack").effect {
            SkillEffect::MeleeAttack(damage, _) => assert_eq!(4, damage.dice()),
            _ => panic!(),
        }
    }

    #[test]
    fn attack_skills_skill_range() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[
                EquipmentEffect::UnlocksAbilityClass("Triple Shot".to_string()),
                EquipmentEffect::ModifiesSkillRange(-1, "Triple Shot".to_string()),
            ],
            0,
        )]);

        let mut skills = SkillsResource::init();
        collect_attack_skills(&ecs, &mut skills, |name| match name {
            "Triple Shot" => SkillInfo::init_with_distance("Triple Shot", None, TargetType::Any, SkillEffect::None, Some(5), true),
            _ => panic!(),
        });
        assert_eq!(1, skills.skills.len());
        assert_eq!(Some(4), skills.get("Triple Shot").range);
    }

    #[test]
    fn attack_skills_skill_damage() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[EquipmentEffect::ModifiesSkillStrength(1, "Basic Attack".to_string())],
            0,
        )]);

        let mut skills = SkillsResource::init();
        collect_attack_skills(&ecs, &mut skills, |name| match name {
            "Default" => SkillInfo::init_with_distance(
                "Basic Attack",
                None,
                TargetType::Any,
                SkillEffect::MeleeAttack(Damage::init(3), WeaponKind::Sword),
                Some(5),
                true,
            ),
            _ => panic!(),
        });
        assert_eq!(1, skills.skills.len());
        match skills.get("Basic Attack").effect {
            SkillEffect::MeleeAttack(damage, _) => assert_eq!(4, damage.dice()),
            _ => panic!(),
        }
    }

    #[test]
    fn attack_modes() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[EquipmentEffect::UnlocksAbilityMode("Inferno Ammo".to_string())],
            0,
        )]);

        let modes = collect_attack_modes(&ecs, |name| match name {
            "Default" => "Magnum Ammo".to_string(),
            "Inferno Ammo" => "Inferno Ammo".to_string(),
            _ => panic!(),
        });
        assert_eq!(2, modes.len());
        assert_eq!("Magnum Ammo", modes[0]);
        assert_eq!("Inferno Ammo", modes[1]);
    }

    #[test]
    fn gunslinger_smoke() {
        let mut ecs = create_test_state().with_map().build();)
        let mut skills = SkillsResource::init();
        create_player(&mut ecs, &mut skills, Point::init(0, 0));
        assert_eq!(1, skills.skills.len());
        assert_eq!("Snap Shot", skills.skills.get("Snap Shot").unwrap().name);
    }
}
