use specs::prelude::*;
use std::collections::HashMap;

use super::super::*;
use crate::atlas::{EasyMutECS, EasyMutWorld};
use crate::sequence;

pub fn setup_gunslinger(ecs: &mut World, invoker: &Entity) {
    ecs.shovel(
        *invoker,
        SkillResourceComponent::init(&[(AmmoKind::Bullets, 6, 6), (AmmoKind::Adrenaline, 0, 100)]).with_focus(1.0),
    );

    add_skills_to_front(ecs, invoker, &get_weapon_skills(TargetAmmo::Magnum));
    set_weapon_trait(ecs, invoker, TargetAmmo::Magnum);
}

pub fn get_weapon_skills(ammo: TargetAmmo) -> Vec<&'static str> {
    match ammo {
        TargetAmmo::Magnum => vec!["Aimed Shot", "Triple Shot", "Quick Shot", "Blink Shot", "Swap Ammo"],
        TargetAmmo::Ignite => vec!["Explosive Blast", "Dragon's Breath", "Hot Hands", "Showdown", "Swap Ammo"],
        TargetAmmo::Cyclone => vec!["Air Lance", "Tornado Shot", "Lightning Speed", "Dive", "Swap Ammo"],
    }
}

#[derive(Copy, Clone)]
pub enum TargetAmmo {
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
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Magnum);
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Ignite);
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Cyclone);
    match ammo {
        TargetAmmo::Magnum => ecs.add_trait(invoker, StatusKind::Magnum),
        TargetAmmo::Ignite => ecs.add_trait(invoker, StatusKind::Ignite),
        TargetAmmo::Cyclone => ecs.add_trait(invoker, StatusKind::Cyclone),
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

    reload(ecs, &invoker, AmmoKind::Bullets, None);
}

pub fn gunslinger_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    add_aimed_skills(m);
    add_triple_shot_skills(m);
    add_move_and_shoot_skills(m);
    add_utility_skills(m);

    m.add_skill(SkillInfo::init(
        "Reload",
        Some("b_45.png"),
        TargetType::None,
        SkillEffect::Reload(AmmoKind::Bullets),
    ));
    m.add_skill(SkillInfo::init(
        "Swap Ammo",
        Some("b_28.png"),
        TargetType::None,
        SkillEffect::ReloadAndRotateAmmo(),
    ));
}

fn add_aimed_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    const AIMED_SHOT_BASE_RANGE: u32 = 7;
    const AIMED_SHOT_BASE_STRENGTH: u32 = 5;

    m.add_skill(
        SkillInfo::init_with_distance(
            "Aimed Shot",
            Some("gun_06_b.PNG"),
            TargetType::Enemy,
            SkillEffect::RangedAttack(
                Damage::init(AIMED_SHOT_BASE_STRENGTH + 1).with_option(DamageOptions::AIMED_SHOT),
                BoltKind::Bullet,
            ),
            Some(AIMED_SHOT_BASE_RANGE - 1),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 1)
        .with_focus_use(0.5)
        .with_alternate("Reload"),
    );

    m.add_skill(
        SkillInfo::init_with_distance(
            "Explosive Blast",
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
        .with_ammo(AmmoKind::Bullets, 1)
        .with_focus_use(0.5)
        .with_alternate("Reload"),
    );

    m.add_skill(
        SkillInfo::init_with_distance(
            "Air Lance",
            Some("SpellBook06_46.png"),
            TargetType::Enemy,
            SkillEffect::RangedAttack(
                Damage::init(AIMED_SHOT_BASE_STRENGTH - 1).with_option(DamageOptions::CONSUMES_CHARGE_KNOCKBACK),
                BoltKind::AirBullet,
            ),
            Some(AIMED_SHOT_BASE_RANGE + 2),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 1)
        .with_focus_use(0.5)
        .with_alternate("Reload"),
    );
}

fn add_triple_shot_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    const TRIPLE_SHOT_BASE_RANGE: u32 = 5;
    const TRIPLE_SHOT_BASE_STRENGTH: u32 = 3;

    m.add_skill(
        SkillInfo::init_with_distance(
            "Triple Shot",
            Some("SpellBook06_22.png"),
            TargetType::Enemy,
            SkillEffect::RangedAttack(
                Damage::init(TRIPLE_SHOT_BASE_STRENGTH + 1).with_option(DamageOptions::TRIPLE_SHOT),
                BoltKind::Bullet,
            ),
            Some(TRIPLE_SHOT_BASE_RANGE - 1),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 3)
        .with_alternate("Reload"),
    );

    m.add_skill(
        SkillInfo::init_with_distance(
            "Dragon's Breath",
            Some("r_16.png"),
            TargetType::Enemy,
            SkillEffect::RangedAttack(
                Damage::init(TRIPLE_SHOT_BASE_STRENGTH)
                    .with_option(DamageOptions::TRIPLE_SHOT)
                    .with_option(DamageOptions::RAISE_TEMPERATURE),
                BoltKind::FireBullet,
            ),
            Some(TRIPLE_SHOT_BASE_RANGE + 1),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 3)
        .with_alternate("Reload"),
    );

    m.add_skill(
        SkillInfo::init_with_distance(
            "Tornado Shot",
            Some("SpellBookPage09_66.png"),
            TargetType::Enemy,
            SkillEffect::RangedAttack(
                Damage::init(TRIPLE_SHOT_BASE_RANGE)
                    .with_option(DamageOptions::TRIPLE_SHOT)
                    .with_option(DamageOptions::CONSUMES_CHARGE_DMG),
                BoltKind::AirBullet,
            ),
            Some(TRIPLE_SHOT_BASE_RANGE + 1),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 3)
        .with_alternate("Reload"),
    );
}

fn add_move_and_shoot_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    const MOVE_AND_SHOOT_BASE_RANGE: u32 = 5;
    const MOVE_AND_SHOOT_BASE_STRENGTH: u32 = 3;
    m.add_skill(
        SkillInfo::init_with_distance(
            "Quick Shot",
            Some("SpellBook03_10.png"),
            TargetType::Tile,
            SkillEffect::MoveAndShoot(
                Damage::init(MOVE_AND_SHOOT_BASE_STRENGTH + 1),
                Some(MOVE_AND_SHOOT_BASE_RANGE),
                BoltKind::Bullet,
            ),
            Some(1),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 1)
        .with_exhaustion(25.0)
        .with_alternate("Reload"),
    );

    m.add_skill(
        SkillInfo::init_with_distance(
            "Hot Hands",
            Some("SpellBook01_15.png"),
            TargetType::Tile,
            SkillEffect::MoveAndShoot(
                Damage::init(MOVE_AND_SHOOT_BASE_STRENGTH).with_option(DamageOptions::RAISE_TEMPERATURE),
                Some(MOVE_AND_SHOOT_BASE_RANGE),
                BoltKind::Bullet,
            ),
            Some(1),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 1)
        .with_exhaustion(25.0)
        .with_alternate("Reload"),
    );

    m.add_skill(
        SkillInfo::init_with_distance(
            "Lightning Speed",
            Some("SpellBookPage09_39.png"),
            TargetType::Tile,
            SkillEffect::MoveAndShoot(
                Damage::init(MOVE_AND_SHOOT_BASE_STRENGTH).with_option(DamageOptions::ADD_CHARGE_STATUS),
                Some(MOVE_AND_SHOOT_BASE_RANGE + 1),
                BoltKind::Bullet,
            ),
            Some(1),
            true,
        )
        .with_ammo(AmmoKind::Bullets, 1)
        .with_exhaustion(25.0)
        .with_alternate("Reload"),
    );
}

fn add_utility_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.add_skill(
        SkillInfo::init_with_distance(
            "Blink Shot",
            Some("SpellBook06_118.png"),
            TargetType::Enemy,
            sequence!(
                SkillEffect::RangedAttack(Damage::init(5), BoltKind::Bullet),
                SkillEffect::Buff(StatusKind::Aimed, 300)
            ),
            Some(7),
            true,
        )
        .with_no_time()
        .with_ammo(AmmoKind::Adrenaline, 50),
    );
    m.add_skill(
        SkillInfo::init_with_distance(
            "Showdown",
            Some("SpellBook03_54.png"),
            TargetType::None,
            sequence!(
                SkillEffect::Buff(StatusKind::Aimed, 300),
                sequence!(
                    SkillEffect::Buff(StatusKind::Armored, 300),
                    SkillEffect::RangedAttack(Damage::init(5), BoltKind::Bullet)
                )
            ),
            Some(3),
            true,
        )
        .with_ammo(AmmoKind::Adrenaline, 50),
    );
    m.add_skill(
        SkillInfo::init_with_distance(
            "Dive",
            Some("SpellBook08_121.png"),
            TargetType::Tile,
            sequence!(SkillEffect::Buff(StatusKind::Armored, 300), SkillEffect::Move),
            Some(3),
            true,
        )
        .with_ammo(AmmoKind::Adrenaline, 50),
    );
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
        assert_eq!(5, ecs.read_storage::<SkillsComponent>().grab(player).skills.len());
    }

    #[test]
    fn rotate_ammo_reloads_as_well() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let player = find_at(&ecs, 2, 2);
        setup_gunslinger(&mut ecs, &player);

        *ecs.write_storage::<SkillResourceComponent>()
            .grab_mut(player)
            .ammo
            .get_mut(&AmmoKind::Bullets)
            .unwrap() = 5;

        assert_eq!(5, ecs.read_storage::<SkillResourceComponent>().grab(player).ammo[&AmmoKind::Bullets]);
        rotate_ammo(&mut ecs, &player);
        assert_eq!(6, ecs.read_storage::<SkillResourceComponent>().grab(player).ammo[&AmmoKind::Bullets]);
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
