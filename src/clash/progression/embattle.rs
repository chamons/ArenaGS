use std::collections::HashMap;
use std::iter::Iterator;

use specs::prelude::*;

use crate::atlas::prelude::*;
use crate::clash::content::{gunslinger, spawner};
use crate::clash::*;

pub fn load_equipment_for_help(_ecs: &World, equip: &mut EquipmentResource) {
    for e in content::gunslinger::get_equipment() {
        equip.add(e);
    }
}

pub fn load_skills_for_help(ecs: &World, skills: &mut SkillsResource) {
    gunslinger::instance_skills(ecs, None, skills);
}

pub fn create_player(ecs: &mut World, skills: &mut SkillsResource, player_position: Point) {
    let (dodge, armor, absorb, health) = collect_defense_modifier(ecs);
    let defenses = DefenseComponent::init(Defenses::init(1 + dodge as u32, armor as u32, absorb as u32, 20 + health as u32));

    let resources = get_player_resources(ecs);

    let player = spawner::player(ecs, player_position, SkillResourceComponent::init(&resources[..]).with_focus(1.0), defenses);

    let templates = collect_attack_skills(ecs, gunslinger::get_base_skill);
    ecs.write_component::<SkillsComponent>().grab_mut(player).templates = templates.iter().map(|t| t.name.to_string()).collect();
    gunslinger::add_base_abilities(skills);

    gunslinger::process_attack_modes(ecs, player, collect_attack_modes(ecs), skills);

    gunslinger::add_active_skills(ecs, player)
}

fn get_player_resources(ecs: &World) -> Vec<(AmmoKind, u32, u32)> {
    let mut resources = gunslinger::base_resources();
    for delta in collect_resource_modifier(ecs) {
        let i = resources
            .iter()
            .position(|r| r.0 == delta.0)
            .expect(&format!("Unable to find base resource {:?}", delta.0));
        resources[i].1 = (resources[i].1 as i32 + delta.1) as u32;
        if resources[i].2 > 0 {
            resources[i].2 = (resources[i].2 as i32 + delta.1) as u32;
        }
    }
    resources
}

fn collect_attack_modes(ecs: &World) -> Vec<String> {
    let mut modes = vec![];
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

fn collect_attack_skills<F>(ecs: &World, get: F) -> Vec<SkillInfo>
where
    F: Fn(&str) -> SkillInfo,
{
    let mut base_attacks = vec![];
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
                        base_attacks.push(get(&kind));
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

    let default_attack_replacement = gunslinger::default_attack_replacement();
    if !base_attacks.iter().any(|a| a.name == default_attack_replacement) {
        base_attacks.push(get("Default"));
    }

    let mut final_attacks = vec![];
    for mut a in base_attacks {
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
        final_attacks.push(a);
    }

    final_attacks
}

fn collect_defense_modifier(ecs: &World) -> (i32, i32, i32, i32) {
    let mut dodge = 0;
    let mut armor = 0;
    let mut absorb = 0;
    let mut health = 0;

    for e in ecs.read_resource::<ProgressionComponent>().state.equipment.all() {
        if let Some(e) = e {
            for effect in e.effect {
                match effect {
                    EquipmentEffect::ModifiesDodge(delta) => dodge += delta,
                    EquipmentEffect::ModifiesArmor(delta) => armor += delta,
                    EquipmentEffect::ModifiesAbsorb(delta) => absorb += delta,
                    EquipmentEffect::ModifiesMaxHealth(delta) => health += delta,
                    _ => {}
                }
            }
        }
    }
    (dodge, armor, absorb, health)
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
        let mut state = ProgressionState::init_gunslinger();
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
        let (dodge, armor, absorb, life) = collect_defense_modifier(&ecs);
        assert_eq!(2, dodge);
        assert_eq!(1, armor);
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

        let skills = collect_attack_skills(&ecs, |name| match name {
            "Default" => SkillInfo::init("Basic Attack", None, TargetType::Any, SkillEffect::None),
            _ => panic!(),
        });
        assert_eq!(1, skills.len());
        assert_eq!("Basic Attack", skills[0].name);
    }

    #[test]
    fn attack_skills_skill_unlock() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[EquipmentEffect::UnlocksAbilityClass("Quick Shot".to_string())],
            0,
        )]);

        let skills = collect_attack_skills(&ecs, |name| match name {
            "Quick Shot" => SkillInfo::init("Quick Shot", None, TargetType::Any, SkillEffect::None),
            _ => panic!(),
        });
        assert_eq!(1, skills.len());
        assert_eq!("Quick Shot", skills[0].name);
    }

    #[test]
    fn attack_skills_weapon_range() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[
                EquipmentEffect::UnlocksAbilityClass("Quick Shot".to_string()),
                EquipmentEffect::ModifiesWeaponRange(-1),
            ],
            0,
        )]);

        let skills = collect_attack_skills(&ecs, |name| match name {
            "Quick Shot" => SkillInfo::init_with_distance("Quick Shot", None, TargetType::Any, SkillEffect::None, Some(5), true),
            _ => panic!(),
        });
        assert_eq!(1, skills.len());
        assert_eq!(Some(4), skills[0].range);
    }

    #[test]
    fn attack_skills_weapon_damage() {
        let ecs = equip_test_state(&[eq("a", EquipmentKinds::Weapon, &[EquipmentEffect::ModifiesWeaponStrength(1)], 0)]);

        let skills = collect_attack_skills(&ecs, |name| match name {
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
        assert_eq!(1, skills.len());
        match skills[0].effect {
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
                EquipmentEffect::UnlocksAbilityClass("Quick Shot".to_string()),
                EquipmentEffect::ModifiesSkillRange(-1, "Quick Shot".to_string()),
            ],
            0,
        )]);

        let skills = collect_attack_skills(&ecs, |name| match name {
            "Quick Shot" => SkillInfo::init_with_distance("Quick Shot", None, TargetType::Any, SkillEffect::None, Some(5), true),
            _ => panic!(),
        });
        assert_eq!(1, skills.len());
        assert_eq!(Some(4), skills[0].range);
    }

    #[test]
    fn attack_skills_additional_unlock() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[EquipmentEffect::UnlocksAbilityClass("Triple Shot".to_string())],
            0,
        )]);

        let skills = collect_attack_skills(&ecs, |name| match name {
            "Default" => SkillInfo::init_with_distance("Snap Shot", None, TargetType::Any, SkillEffect::None, Some(5), true),
            "Triple Shot" => SkillInfo::init_with_distance("Triple Shot", None, TargetType::Any, SkillEffect::None, Some(5), true),
            _ => panic!(),
        });
        assert_eq!(2, skills.len());
    }

    #[test]
    fn attack_skills_skill_damage() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[EquipmentEffect::ModifiesSkillStrength(1, "Basic Attack".to_string())],
            0,
        )]);

        let skills = collect_attack_skills(&ecs, |name| match name {
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
        assert_eq!(1, skills.len());
        match skills[0].effect {
            SkillEffect::MeleeAttack(damage, _) => assert_eq!(4, damage.dice()),
            _ => panic!(),
        }
    }

    #[test]
    fn attack_modes() {
        let ecs = equip_test_state(&[eq(
            "a",
            EquipmentKinds::Weapon,
            &[EquipmentEffect::UnlocksAbilityMode("Inferno".to_string())],
            0,
        )]);

        let modes = collect_attack_modes(&ecs);
        assert_eq!(1, modes.len());
        assert_eq!("Inferno", modes[0]);
    }

    #[test]
    fn gunslinger_smoke() {
        let mut ecs = create_test_state().with_map().build();
        let mut state = ProgressionState::init_gunslinger();
        state.equipment = Equipment::init(4, 4, 4, 4);
        state.equipment.add(
            EquipmentKinds::Weapon,
            EquipmentItem::init(
                "A",
                None,
                EquipmentKinds::Weapon,
                &[EquipmentEffect::ModifiesResourceTotal(-1, "Bullets".to_string())],
            ),
            0,
        );
        state.equipment.add(
            EquipmentKinds::Armor,
            EquipmentItem::init("B", None, EquipmentKinds::Armor, &[EquipmentEffect::ModifiesArmor(1)]),
            0,
        );

        ecs.insert(ProgressionComponent::init(state));

        let mut skills = SkillsResource::init();
        create_player(&mut ecs, &mut skills, Point::init(0, 0));
        assert_eq!("Snap Shot", skills.skills.get("Snap Shot").unwrap().name);
        let player = find_player(&ecs);
        assert_eq!(
            5,
            *ecs.read_storage::<SkillResourceComponent>().grab(player).ammo.get(&AmmoKind::Bullets).unwrap()
        );

        assert_eq!(1, ecs.read_storage::<SkillsComponent>().grab(player).skills.len());
        assert_eq!(1, ecs.get_defenses(player).armor);

        // Now equip an ability class unlock and create anew to change abilities
        {
            let mut state = ecs.write_resource::<ProgressionComponent>().state.clone();
            state.equipment.add(
                EquipmentKinds::Weapon,
                EquipmentItem::init(
                    "C",
                    None,
                    EquipmentKinds::Weapon,
                    &[EquipmentEffect::UnlocksAbilityClass("Quick Shot".to_string())],
                ),
                1,
            );

            let mut ecs = create_test_state().with_map().build();
            ecs.insert(ProgressionComponent::init(state));

            create_player(&mut ecs, &mut skills, Point::init(0, 0));
            assert_eq!(1, ecs.read_storage::<SkillsComponent>().grab(player).skills.len());
            assert_eq!("Quick Shot", skills.skills.get("Quick Shot").unwrap().name);
        }

        // Now equip an ability mode unlock and create anew to change abilities
        {
            let mut state = ecs.write_resource::<ProgressionComponent>().state.clone();
            state.equipment.add(
                EquipmentKinds::Accessory,
                EquipmentItem::init(
                    "D",
                    None,
                    EquipmentKinds::Accessory,
                    &[EquipmentEffect::UnlocksAbilityMode("Ignite".to_string())],
                ),
                0,
            );

            let mut ecs = create_test_state().with_map().build();
            ecs.insert(ProgressionComponent::init(state));

            create_player(&mut ecs, &mut skills, Point::init(0, 0));
            assert_eq!(2, ecs.read_storage::<GunslingerComponent>().grab(player).ammo_types.len());
        }
    }
}