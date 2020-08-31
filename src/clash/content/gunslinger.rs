use specs::prelude::*;
use std::collections::HashMap;

use super::super::*;
use crate::atlas::{EasyMutECS, EasyMutWorld};

pub fn setup_gunslinger(ecs: &mut World, invoker: &Entity) {
    ecs.shovel(*invoker, SkillResourceComponent::init(&[(AmmoKind::Bullets, 6)]).with_focus(1.0));

    add_skills_to_front(ecs, invoker, &get_weapon_skills(TargetAmmo::Magnum));
    set_weapon_trait(ecs, invoker, TargetAmmo::Magnum);
}

fn get_weapon_skills(ammo: TargetAmmo) -> Vec<&'static str> {
    match ammo {
        TargetAmmo::Magnum => vec!["Aimed Shot", "Swap Ammo Kind"],
        TargetAmmo::Ignite => vec!["Explosive Blast", "Swap Ammo Kind"],
        TargetAmmo::Cyclone => vec!["Air Lance", "Swap Ammo Kind"],
    }
}

#[derive(Copy, Clone)]
enum TargetAmmo {
    Magnum,
    Ignite,
    Cyclone,
}

fn add_skills_to_front(ecs: &mut World, invoker: &Entity, skills_to_add: &[&str]) {
    let mut skills = ecs.write_storage::<SkillsComponent>();
    let skill_list = &mut skills.grab_mut(*invoker).skills;

    // Backwards since we insert one at a time in front
    for s in skills_to_add.iter().rev() {
        skill_list.insert(0, s.to_string());
    }
}

fn remove_skills(ecs: &mut World, invoker: &Entity, skills_to_remove: &[&str]) {
    let mut skills = ecs.write_storage::<SkillsComponent>();
    let skill_list = &mut skills.grab_mut(*invoker).skills;

    for s in skills_to_remove.iter() {
        skill_list.remove(skill_list.iter().position(|x| *x == *s).unwrap());
    }
}

fn set_weapon_trait(ecs: &mut World, invoker: &Entity, ammo: TargetAmmo) {
    let mut statuses = ecs.write_storage::<StatusComponent>();
    let status = &mut statuses.grab_mut(*invoker).status;
    status.remove_trait_if_found(StatusKind::Magnum);
    status.remove_trait_if_found(StatusKind::Ignite);
    status.remove_trait_if_found(StatusKind::Cyclone);
    match ammo {
        TargetAmmo::Magnum => status.add_trait(StatusKind::Magnum),
        TargetAmmo::Ignite => status.add_trait(StatusKind::Ignite),
        TargetAmmo::Cyclone => status.add_trait(StatusKind::Cyclone),
    }
}

pub fn rotate_ammo(ecs: &mut World, invoker: &Entity) {
    let (current_ammo, next_ammo) = {
        if ecs.has_status(invoker, StatusKind::Magnum) {
            (TargetAmmo::Magnum, TargetAmmo::Ignite)
        } else if ecs.has_status(invoker, StatusKind::Ignite) {
            (TargetAmmo::Ignite, TargetAmmo::Cyclone)
        } else {
            (TargetAmmo::Cyclone, TargetAmmo::Magnum)
        }
    };

    remove_skills(ecs, invoker, &get_weapon_skills(current_ammo));
    add_skills_to_front(ecs, invoker, &get_weapon_skills(next_ammo));
    set_weapon_trait(ecs, invoker, next_ammo);
}

pub fn gunslinger_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    const AIMED_SHOT_BASE_RANGE: u32 = 4;
    const AIMED_SHOT_BASE_STRENGTH: u32 = 5;

    m.insert(
        "Aimed Shot",
        SkillInfo::init_with_distance(
            Some("SpellBook01_63.png"),
            TargetType::Enemy,
            SkillEffect::RangedAttack(Damage::init(AIMED_SHOT_BASE_STRENGTH + 2), BoltKind::Bullet),
            Some(AIMED_SHOT_BASE_RANGE - 2),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 1),
    );

    m.insert(
        "Explosive Blast",
        SkillInfo::init_with_distance(
            Some("SpellBook01_37.png"),
            TargetType::Enemy,
            SkillEffect::RangedAttack(
                Damage::init(AIMED_SHOT_BASE_STRENGTH)
                    .with_option(DamageOptions::RAISE_TEMPERATURE)
                    .with_option(DamageOptions::LARGE_TEMPERATURE_DELTA),
                BoltKind::FireBullet,
            ),
            Some(AIMED_SHOT_BASE_RANGE),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 1),
    );

    m.insert(
        "Air Lance",
        SkillInfo::init_with_distance(
            Some("SpellBook06_46.png"),
            TargetType::Enemy,
            SkillEffect::RangedAttack(
                Damage::init(AIMED_SHOT_BASE_STRENGTH - 1).with_option(DamageOptions::ADD_CHARGE_STATUS),
                BoltKind::AirBullet,
            ),
            Some(AIMED_SHOT_BASE_RANGE + 2),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 1),
    );

    m.insert("Swap Ammo Kind", SkillInfo::init(Some("b_28.png"), TargetType::None, SkillEffect::RotateAmmo()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::EasyECS;

    #[test]
    fn gunslinger_starts_correctly() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let player = find_at(&ecs, 2, 2);
        setup_gunslinger(&mut ecs, &player);

        assert!(ecs.has_status(&player, StatusKind::Magnum));
        assert_eq!(2, ecs.read_storage::<SkillsComponent>().grab(player).skills.len());
    }

    #[test]
    fn rotate_ammo_has_correct_buff() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let player = find_at(&ecs, 2, 2);
        setup_gunslinger(&mut ecs, &player);
        assert!(ecs.has_status(&player, StatusKind::Magnum));

        rotate_ammo(&mut ecs, &player);
        assert!(ecs.has_status(&player, StatusKind::Ignite));

        rotate_ammo(&mut ecs, &player);
        assert!(ecs.has_status(&player, StatusKind::Cyclone));
    }

    #[test]
    fn rotate_ammo_has_sets_correct_skills() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let player = find_at(&ecs, 2, 2);
        setup_gunslinger(&mut ecs, &player);
        assert_eq!("Aimed Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);

        rotate_ammo(&mut ecs, &player);
        assert_eq!("Explosive Blast", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);

        rotate_ammo(&mut ecs, &player);
        assert_eq!("Air Lance", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
    }
}
