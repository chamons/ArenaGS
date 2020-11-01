use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::super::progression::SkillTreeNode;
use super::super::*;
use crate::atlas::prelude::*;

pub fn get_skill_tree(equipment: &EquipmentResource) -> Vec<SkillTreeNode> {
    fn skill_pos(x: u32, y: u32) -> Point {
        let x = 60 + (100 * x);
        let y = 20 + 50 * y;
        Point::init(x, y)
    }

    vec![
        SkillTreeNode::init(equipment.get("First"), skill_pos(0, 6), 10, &[]),
        SkillTreeNode::init(equipment.get("Second"), skill_pos(1, 6), 10, &["First"]),
        SkillTreeNode::init(equipment.get("Third"), skill_pos(2, 5), 10, &["Second"]),
        SkillTreeNode::init(equipment.get("Fourth"), skill_pos(2, 7), 10, &["Second"]),
        SkillTreeNode::init(equipment.get("1"), skill_pos(0, 1), 10, &[]),
        SkillTreeNode::init(equipment.get("2"), skill_pos(1, 1), 10, &["1"]),
        SkillTreeNode::init(equipment.get("3"), skill_pos(2, 1), 10, &["2"]),
        SkillTreeNode::init(equipment.get("4"), skill_pos(3, 1), 10, &["3"]),
        SkillTreeNode::init(equipment.get("5"), skill_pos(4, 1), 10, &["4"]),
        SkillTreeNode::init(equipment.get("6"), skill_pos(5, 1), 10, &["5"]),
        SkillTreeNode::init(equipment.get("7"), skill_pos(6, 1), 10, &["6"]),
        SkillTreeNode::init(equipment.get("A"), skill_pos(0, 10), 10, &[]),
        SkillTreeNode::init(equipment.get("B"), skill_pos(0, 12), 10, &[]),
        SkillTreeNode::init(equipment.get("C"), skill_pos(1, 11), 10, &["A", "B"]),
    ]
}

pub fn get_equipment() -> Vec<EquipmentItem> {
    vec![
        EquipmentItem::init("First", Some("ar_b_04.png"), EquipmentKinds::Weapon, &[EquipmentEffect::None]),
        EquipmentItem::init("Second", Some("ar_b_04.PNG"), EquipmentKinds::Weapon, &[EquipmentEffect::None]),
        EquipmentItem::init("Third", Some("ar_b_04.PNG"), EquipmentKinds::Weapon, &[EquipmentEffect::None]),
        EquipmentItem::init("Fourth", Some("ar_b_04.PNG"), EquipmentKinds::Mastery, &[EquipmentEffect::None]),
        EquipmentItem::init("1", Some("artifact_12_b.png"), EquipmentKinds::Armor, &[EquipmentEffect::None]),
        EquipmentItem::init("2", Some("artifact_12_b.png"), EquipmentKinds::Armor, &[EquipmentEffect::None]),
        EquipmentItem::init("3", Some("artifact_12_b.png"), EquipmentKinds::Armor, &[EquipmentEffect::None]),
        EquipmentItem::init("4", Some("artifact_12_b.png"), EquipmentKinds::Armor, &[EquipmentEffect::None]),
        EquipmentItem::init("5", Some("artifact_12_b.png"), EquipmentKinds::Armor, &[EquipmentEffect::None]),
        EquipmentItem::init("6", Some("artifact_12_b.png"), EquipmentKinds::Armor, &[EquipmentEffect::None]),
        EquipmentItem::init("7", Some("artifact_12_b.png"), EquipmentKinds::Armor, &[EquipmentEffect::None]),
        EquipmentItem::init("A", Some("book_13_b.png"), EquipmentKinds::Accessory, &[EquipmentEffect::None]),
        EquipmentItem::init("B", Some("book_13_b.png"), EquipmentKinds::Accessory, &[EquipmentEffect::None]),
        EquipmentItem::init("C", Some("book_13_b.png"), EquipmentKinds::Accessory, &[EquipmentEffect::None]),
    ]
}

#[derive(Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum GunslingerAmmo {
    Magnum,
    Ignite,
    Cyclone,
}

fn add_skills_to_front(ecs: &mut World, invoker: Entity, skills_to_add: Vec<String>) {
    let mut skills = ecs.write_storage::<SkillsComponent>();
    let skill_list = &mut skills.grab_mut(invoker).skills;

    // Backwards since we insert one at a time in front
    for s in skills_to_add.iter().rev() {
        skill_list.insert(0, s.to_string());
    }
}

fn remove_skills(ecs: &mut World, invoker: Entity, skills_to_remove: Vec<String>) {
    let mut skills = ecs.write_storage::<SkillsComponent>();
    let skill_list = &mut skills.grab_mut(invoker).skills;

    for s in skills_to_remove.iter() {
        skill_list.remove(skill_list.iter().position(|x| *x == *s).unwrap());
    }
}

fn get_current_weapon_trait(ecs: &mut World, invoker: Entity) -> GunslingerAmmo {
    if ecs.has_status(invoker, StatusKind::Magnum) {
        GunslingerAmmo::Magnum
    } else if ecs.has_status(invoker, StatusKind::Ignite) {
        GunslingerAmmo::Ignite
    } else {
        GunslingerAmmo::Cyclone
    }
}

fn set_current_weapon_trait(ecs: &mut World, invoker: Entity, ammo: GunslingerAmmo) {
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Magnum);
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Ignite);
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Cyclone);
    match ammo {
        GunslingerAmmo::Magnum => ecs.add_trait(invoker, StatusKind::Magnum),
        GunslingerAmmo::Ignite => ecs.add_trait(invoker, StatusKind::Ignite),
        GunslingerAmmo::Cyclone => ecs.add_trait(invoker, StatusKind::Cyclone),
    }
}

pub fn rotate_ammo(ecs: &mut World, invoker: Entity) {
    let (current_ammo, next_ammo) = match get_current_weapon_trait(ecs, invoker) {
        GunslingerAmmo::Magnum => (GunslingerAmmo::Magnum, GunslingerAmmo::Ignite),
        GunslingerAmmo::Ignite => (GunslingerAmmo::Ignite, GunslingerAmmo::Cyclone),
        GunslingerAmmo::Cyclone => (GunslingerAmmo::Cyclone, GunslingerAmmo::Magnum),
    };

    remove_skills(ecs, invoker, get_weapon_skills(ecs, invoker, current_ammo));
    add_skills_to_front(ecs, invoker, get_weapon_skills(ecs, invoker, next_ammo));
    set_current_weapon_trait(ecs, invoker, next_ammo);

    reload(ecs, invoker, AmmoKind::Bullets, None);
}

pub fn default_attack_replacement() -> &'static str {
    "Quick Shot"
}

pub fn get_weapon_skills(ecs: &World, player: Entity, ammo: GunslingerAmmo) -> Vec<String> {
    let mut skills = vec![];
    for template_name in &ecs.read_storage::<SkillsComponent>().grab(player).templates {
        let name = match ammo {
            GunslingerAmmo::Magnum => template_name, // The template name is the magnum name
            GunslingerAmmo::Ignite => match template_name.as_str() {
                "Snap Shot" => "Spark Shot",
                "Aimed Shot" => "Explosive Blast",
                "Triple Shot" => "Dragon's Breath",
                "Quick Shot" => "Hot Hands",
                _ => panic!("Unknown template {}", template_name),
            },
            GunslingerAmmo::Cyclone => match template_name.as_str() {
                "Snap Shot" => "Airburst Shot",
                "Aimed Shot" => "Air Lance",
                "Triple Shot" => "Tornado Shot",
                "Quick Shot" => "Lightning Speed",
                _ => panic!("Unknown template {}", template_name),
            },
        };

        skills.push(name.to_string());
    }

    skills
}

pub fn get_base_skill(name: &str) -> SkillInfo {
    match name {
        "Default" | "Snap Shot" => {
            return SkillInfo::init_with_distance(
                "Snap Shot",
                Some("gun_06_b.PNG"),
                TargetType::Enemy,
                SkillEffect::RangedAttack(Damage::init(4), BoltKind::Bullet),
                Some(7),
                true,
            )
            .with_ammo(AmmoKind::Bullets, 1)
            .with_alternate("Reload");
        }
        "Aimed Shot" => {
            return SkillInfo::init_with_distance(
                "Aimed Shot",
                Some("gun_06_b.PNG"),
                TargetType::Enemy,
                SkillEffect::RangedAttack(Damage::init(6).with_option(DamageOptions::AIMED_SHOT), BoltKind::Bullet),
                Some(6),
                true,
            )
            .with_ammo(AmmoKind::Bullets, 1)
            .with_focus_use(0.5)
            .with_alternate("Reload");
        }
        "Triple Shot" => {
            return SkillInfo::init_with_distance(
                "Triple Shot",
                Some("SpellBook06_22.png"),
                TargetType::Enemy,
                SkillEffect::RangedAttack(Damage::init(4).with_option(DamageOptions::TRIPLE_SHOT), BoltKind::Bullet),
                Some(4),
                true,
            )
            .with_ammo(AmmoKind::Bullets, 3)
            .with_alternate("Reload");
        }
        "Quick Shot" => {
            return SkillInfo::init_with_distance(
                "Quick Shot",
                Some("SpellBook03_10.png"),
                TargetType::Tile,
                SkillEffect::MoveAndShoot(Damage::init(4), Some(5), BoltKind::Bullet),
                Some(1),
                true,
            )
            .with_ammo(AmmoKind::Bullets, 1)
            .with_exhaustion(40.0)
            .with_alternate("Reload");
        }

        _ => panic!("Unknown gunslinger attack {}", name),
    }
}

fn get_concrete_skill(name: &str, ammo: GunslingerAmmo) -> SkillInfo {
    let base_name = match ammo {
        GunslingerAmmo::Magnum => {
            return get_base_skill(name);
        }
        GunslingerAmmo::Ignite => match name {
            "Spark Shot" => "Snap Shot",
            "Explosive Blast" => "Aimed Shot",
            "Dragon's Breath" => "Triple Shot",
            "Hot Hands" => "Quick Shot",
            _ => panic!("Unknown concrete skill {}", name),
        },
        GunslingerAmmo::Cyclone => match name {
            "Airburst Shot" => "Snap Shot",
            "Air Lance" => "Aimed Shot",
            "Tornado Shot" => "Triple Shot",
            "Lightning Speed" => "Quick Shot",
            _ => panic!("Unknown concrete skill {}", name),
        },
    };

    // Start with that base
    let mut skill = get_base_skill(base_name);
    skill.name = name.to_string();

    let get_damage = |e: &SkillEffect| match e {
        SkillEffect::RangedAttack(damage, _) => damage.dice(),
        SkillEffect::MoveAndShoot(damage, _, _) => damage.dice(),
        _ => panic!("get_concrete_skill processing damage of attack: {}", name),
    };

    let get_range = |e: &SkillEffect| match e {
        SkillEffect::MoveAndShoot(_, range, _) => range.clone(),
        _ => panic!("get_concrete_skill processing range of attack: {}", name),
    };

    match name {
        "Spark Shot" => {
            skill.image = Some("gun_01_b.png".to_string());
            skill.range = skill.range.map(|r| r + 1);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1).with_option(DamageOptions::RAISE_TEMPERATURE),
                BoltKind::FireBullet,
            );
        }
        "Explosive Blast" => {
            skill.image = Some("SpellBook01_37.png".to_string());
            skill.range = skill.range.map(|r| r + 1);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1)
                    .with_option(DamageOptions::RAISE_TEMPERATURE)
                    .with_option(DamageOptions::LARGE_TEMPERATURE_DELTA),
                BoltKind::FireBullet,
            );
        }
        "Dragon's Breath" => {
            skill.image = Some("r_16.png".to_string());
            skill.range = skill.range.map(|r| r + 2);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1)
                    .with_option(DamageOptions::TRIPLE_SHOT)
                    .with_option(DamageOptions::RAISE_TEMPERATURE),
                BoltKind::FireBullet,
            );
        }
        "Hot Hands" => {
            skill.image = Some("SpellBook01_15.png".to_string());
            skill.effect = SkillEffect::MoveAndShoot(
                Damage::init(get_damage(&skill.effect) - 1).with_option(DamageOptions::RAISE_TEMPERATURE),
                get_range(&skill.effect),
                BoltKind::FireBullet,
            );
        }
        "Airburst Shot" => {
            skill.image = Some("gun_01_b.png".to_string());
            skill.range = skill.range.map(|r| r + 2);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1).with_option(DamageOptions::ADD_CHARGE_STATUS),
                BoltKind::FireBullet,
            );
        }
        "Air Lance" => {
            skill.image = Some("SpellBook06_46.png".to_string());
            skill.range = skill.range.map(|r| r + 3);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 2).with_option(DamageOptions::CONSUMES_CHARGE_KNOCKBACK),
                BoltKind::AirBullet,
            );
        }
        "Tornado Shot" => {
            skill.image = Some("SpellBookPage09_66.png".to_string());
            skill.range = skill.range.map(|r| r + 2);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1)
                    .with_option(DamageOptions::TRIPLE_SHOT)
                    .with_option(DamageOptions::CONSUMES_CHARGE_DMG),
                BoltKind::AirBullet,
            );
        }
        "Lightning Speed" => {
            skill.image = Some("SpellBookPage09_39.png".to_string());
            skill.effect = SkillEffect::MoveAndShoot(
                Damage::init(get_damage(&skill.effect) - 1).with_option(DamageOptions::ADD_CHARGE_STATUS),
                get_range(&skill.effect).map(|r| r + 1),
                BoltKind::AirBullet,
            );
        }
        _ => {}
    }

    return skill;
}

pub fn add_base_abilities(skills: &mut SkillsResource) {
    skills.add(SkillInfo::init(
        "Reload",
        Some("b_45.png"),
        TargetType::None,
        SkillEffect::Reload(AmmoKind::Bullets),
    ));

    skills.add(SkillInfo::init(
        "Swap Ammo",
        Some("b_28.png"),
        TargetType::None,
        SkillEffect::ReloadAndRotateAmmo(),
    ));
}

pub fn base_resources() -> Vec<(AmmoKind, u32, u32)> {
    vec![(AmmoKind::Bullets, 6, 6), (AmmoKind::Adrenaline, 0, 100)]
}

pub fn process_attack_modes(ecs: &mut World, player: Entity, modes: Vec<String>, skills: &mut SkillsResource) {
    let mut modes: Vec<GunslingerAmmo> = modes
        .iter()
        .map(|m| match m.as_str() {
            "Magnum" => GunslingerAmmo::Magnum,
            "Ignite" => GunslingerAmmo::Ignite,
            "Cyclone" => GunslingerAmmo::Cyclone,
            _ => panic!("Unknown gunslinger mode {}", m),
        })
        .collect();

    if !modes.contains(&GunslingerAmmo::Magnum) {
        modes.insert(0, GunslingerAmmo::Magnum);
    }

    ecs.shovel(player, GunslingerComponent::init(&modes[..]));

    for m in modes {
        for s in get_weapon_skills(ecs, player, m) {
            skills.add(get_concrete_skill(&s, m));
        }
    }
}

pub fn add_active_skills(ecs: &mut World, player: Entity) {
    set_current_weapon_trait(ecs, player, GunslingerAmmo::Magnum);

    add_skills_to_front(ecs, player, get_weapon_skills(ecs, player, GunslingerAmmo::Magnum));
    if ecs.read_storage::<GunslingerComponent>().grab(player).ammo_types.len() > 1 {
        ecs.write_storage::<SkillsComponent>().grab_mut(player).skills.push("Swap Ammo".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_ammo_reloads_as_well() {
        let mut ecs = create_test_state().with_gunslinger(2, 2).build();
        let player = find_at(&ecs, 2, 2);

        *ecs.write_storage::<SkillResourceComponent>()
            .grab_mut(player)
            .ammo
            .get_mut(&AmmoKind::Bullets)
            .unwrap() = 5;

        assert_eq!(5, ecs.read_storage::<SkillResourceComponent>().grab(player).ammo[&AmmoKind::Bullets]);
        rotate_ammo(&mut ecs, player);
        assert_eq!(6, ecs.read_storage::<SkillResourceComponent>().grab(player).ammo[&AmmoKind::Bullets]);
    }

    #[test]
    fn rotate_ammo_has_correct_buff() {
        let mut ecs = create_test_state().with_gunslinger(2, 2).build();
        let player = find_at(&ecs, 2, 2);
        assert!(ecs.has_status(player, StatusKind::Magnum));

        rotate_ammo(&mut ecs, player);
        assert!(ecs.has_status(player, StatusKind::Ignite));

        rotate_ammo(&mut ecs, player);
        assert!(ecs.has_status(player, StatusKind::Cyclone));
    }

    #[test]
    fn rotate_ammo_has_sets_correct_skills() {
        let mut ecs = create_test_state().with_gunslinger(2, 2).build();
        let player = find_at(&ecs, 2, 2);
        assert_eq!("Snap Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);

        rotate_ammo(&mut ecs, player);
        assert_eq!("Spark Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);

        rotate_ammo(&mut ecs, player);
        assert_eq!("Airburst Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
    }
}
