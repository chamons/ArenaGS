use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::super::progression::SkillTreeNode;
use super::super::*;
use super::weapon_pack::{add_skills_to_front, remove_skills};
use crate::atlas::prelude::*;
use crate::vec_of_strings;

pub fn get_skill_tree(equipment: &EquipmentResource) -> Vec<SkillTreeNode> {
    fn skill_pos(x: u32, y: u32) -> Point {
        let x = 60 + (100 * x);
        let y = 20 + 50 * y;
        Point::init(x, y)
    }

    vec![
        SkillTreeNode::with_equipment(equipment.get("Adjustable Sight"), skill_pos(0, 6), 5, &[]),
        SkillTreeNode::with_equipment(equipment.get("Recoil Spring"), skill_pos(1, 6), 10, &["Adjustable Sight"]),
        SkillTreeNode::with_equipment(equipment.get("Stippled Grip"), skill_pos(2, 5), 15, &["Recoil Spring"]),
        SkillTreeNode::with_expansion(EquipmentKinds::Weapon, 1, skill_pos(0, 8), 5, &[]),
        SkillTreeNode::with_expansion(EquipmentKinds::Weapon, 2, skill_pos(1, 8), 5, &["Weapon Expansion"]),
        SkillTreeNode::with_equipment(equipment.get("Ignite Ammo"), skill_pos(0, 4), 5, &[]),
    ]
}

pub fn get_equipment() -> Vec<EquipmentItem> {
    vec![
        EquipmentItem::init(
            "Adjustable Sight",
            Some("gun_06_b.png"),
            EquipmentKinds::Weapon,
            EquipmentRarity::Standard,
            &[EquipmentEffect::UnlocksAbilityClass("Aimed Shot".to_string())],
        ),
        EquipmentItem::init(
            "Recoil Spring",
            Some("SpellBook06_22.png"),
            EquipmentKinds::Weapon,
            EquipmentRarity::Standard,
            &[EquipmentEffect::UnlocksAbilityClass("Triple Shot".to_string())],
        ),
        EquipmentItem::init(
            "Stippled Grip",
            Some("SpellBook03_10.png"),
            EquipmentKinds::Weapon,
            EquipmentRarity::Standard,
            &[EquipmentEffect::UnlocksAbilityClass("Quick Shot".to_string())],
        ),
        EquipmentItem::init(
            "Ignite Ammo",
            Some("b_31_1.png"),
            EquipmentKinds::Weapon,
            EquipmentRarity::Standard,
            &[EquipmentEffect::UnlocksAbilityMode("Ignite".to_string())],
        ),
        EquipmentItem::init(
            "Oversized Chamber",
            Some("gun_12_b.png"),
            EquipmentKinds::Weapon,
            EquipmentRarity::Uncommon,
            &[
                EquipmentEffect::ModifiesResourceTotal(-3, "Bullets".to_string()),
                EquipmentEffect::ModifiesWeaponStrength(2),
            ],
        ),
        // summon a shadow that shoots a few times
        EquipmentItem::init(
            "Gunslinger's Regret",
            Some("artifact_01_b.png"),
            EquipmentKinds::Accessory,
            EquipmentRarity::Uncommon,
            &[],
        ),
        // Rotate ammo every shot but reloads after every shot and slightly more skill damage
        EquipmentItem::init("Luck of the Draw", Some("book_01_b.png"), EquipmentKinds::Mastery, EquipmentRarity::Rare, &[]),
    ]
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GunslingerAmmo {
    Magnum,
    Ignite,
    Cyclone,
}

pub fn get_current_weapon_trait(ecs: &World, invoker: Entity) -> GunslingerAmmo {
    if ecs.has_status(invoker, StatusKind::Magnum) {
        GunslingerAmmo::Magnum
    } else if ecs.has_status(invoker, StatusKind::Ignite) {
        GunslingerAmmo::Ignite
    } else {
        GunslingerAmmo::Cyclone
    }
}

pub fn get_image_for_status(kind: StatusKind) -> &'static str {
    match kind {
        StatusKind::Ignite => "b_31_1.png",
        StatusKind::Cyclone => "b_40_02.png",
        StatusKind::Magnum => "b_30.png",
        _ => panic!("Unknown status {:?} in get_image_for_status", kind),
    }
}

pub fn get_image_for_kind(ammo: GunslingerAmmo) -> &'static str {
    match ammo {
        GunslingerAmmo::Ignite => "b_31_1.png",
        GunslingerAmmo::Cyclone => "b_40_02.png",
        GunslingerAmmo::Magnum => "b_30.png",
    }
}

pub fn get_all_trait_images() -> Vec<&'static str> {
    vec!["b_31_1.png", "b_40_02.png", "b_30.png"]
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

pub fn get_equipped_ammos(ecs: &World, invoker: Entity) -> Vec<GunslingerAmmo> {
    ecs.read_storage::<GunslingerComponent>().grab(invoker).ammo_types.to_vec()
}

fn get_next_ammo(ecs: &mut World, invoker: Entity) -> GunslingerAmmo {
    let mut current = get_current_weapon_trait(ecs, invoker);
    loop {
        let next_ammo = match current {
            GunslingerAmmo::Magnum => GunslingerAmmo::Ignite,
            GunslingerAmmo::Ignite => GunslingerAmmo::Cyclone,
            GunslingerAmmo::Cyclone => GunslingerAmmo::Magnum,
        };

        if ecs.read_storage::<GunslingerComponent>().grab(invoker).ammo_types.contains(&next_ammo) {
            return next_ammo;
        } else {
            current = next_ammo;
        }
    }
}

pub fn rotate_ammo(ecs: &mut World, invoker: Entity) {
    let current_ammo = get_current_weapon_trait(ecs, invoker);
    let next_ammo = get_next_ammo(ecs, invoker);

    // The skill should not be available, but make it a no-op beyond reload
    if current_ammo == next_ammo {
        reload(ecs, invoker, AmmoKind::Bullets, None);
        return;
    }

    set_ammo_to(ecs, invoker, next_ammo);
}

pub fn set_ammo_to(ecs: &mut World, invoker: Entity, next_ammo: GunslingerAmmo) {
    let current_ammo = get_current_weapon_trait(ecs, invoker);

    remove_skills(ecs, invoker, get_weapon_skills(ecs, Some(invoker), current_ammo));
    add_skills_to_front(ecs, invoker, get_weapon_skills(ecs, Some(invoker), next_ammo));
    set_current_weapon_trait(ecs, invoker, next_ammo);

    reload(ecs, invoker, AmmoKind::Bullets, None);
}

pub fn default_attack() -> &'static str {
    "Snap Shot"
}

pub fn default_attack_replacement() -> &'static str {
    "Quick Shot"
}

pub fn get_weapon_skills(ecs: &World, player: Option<Entity>, ammo: GunslingerAmmo) -> Vec<String> {
    let mut skills = vec![];
    let templates = if let Some(player) = player {
        ecs.read_storage::<SkillsComponent>().grab(player).skills.clone()
    } else {
        get_all_bases()
    };

    for template_name in templates {
        let name = match ammo {
            GunslingerAmmo::Magnum => template_name, // The template name is the magnum name
            GunslingerAmmo::Ignite => match template_name.as_str() {
                "Snap Shot" => "Spark Shot".to_string(),
                "Aimed Shot" => "Explosive Blast".to_string(),
                "Triple Shot" => "Dragon's Breath".to_string(),
                "Quick Shot" => "Hot Hands".to_string(),
                _ => panic!("Unknown template {}", template_name),
            },
            GunslingerAmmo::Cyclone => match template_name.as_str() {
                "Snap Shot" => "Airburst Shot".to_string(),
                "Aimed Shot" => "Air Lance".to_string(),
                "Triple Shot" => "Tornado Shot".to_string(),
                "Quick Shot" => "Lightning Speed".to_string(),
                _ => panic!("Unknown template {}", template_name),
            },
        };

        skills.push(name.to_string());
    }

    skills
}

pub fn get_all_bases() -> Vec<String> {
    vec_of_strings!["Snap Shot", "Aimed Shot", "Triple Shot", "Quick Shot"]
}

// Syntax here gets ugly otherwise after autoformat
#[allow(clippy::needless_return)]
pub fn get_base_skill(name: &str) -> SkillInfo {
    match name {
        "Default" | "Snap Shot" => {
            return SkillInfo::init_with_distance(
                "Snap Shot",
                Some("gun_08_b.png"),
                TargetType::Enemy,
                SkillEffect::RangedAttack(Damage::init(4, DamageElement::PHYSICAL), BoltKind::Bullet),
                Some(7),
                true,
            )
            .with_ammo(AmmoKind::Bullets, 1)
            .with_alternate("Reload");
        }
        "Aimed Shot" => {
            return SkillInfo::init_with_distance(
                "Aimed Shot",
                Some("gun_06_b.png"),
                TargetType::Enemy,
                SkillEffect::RangedAttack(
                    Damage::init(6, DamageElement::PHYSICAL).with_option(DamageOptions::AIMED_SHOT),
                    BoltKind::Bullet,
                ),
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
                SkillEffect::RangedAttack(
                    Damage::init(4, DamageElement::PHYSICAL).with_option(DamageOptions::TRIPLE_SHOT),
                    BoltKind::Bullet,
                ),
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
                SkillEffect::MoveAndShoot(Damage::init(4, DamageElement::PHYSICAL), Some(5), BoltKind::Bullet),
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

fn get_concrete_skill(name: &str, ammo: GunslingerAmmo, templates: &Vec<SkillInfo>) -> SkillInfo {
    // Start with that base
    let find = |n| templates.iter().find(|&skill_name| n == skill_name.name).unwrap().clone();

    let base_name = match ammo {
        GunslingerAmmo::Magnum => {
            return find(name);
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

    let mut skill = find(base_name);
    skill.name = name.to_string();

    let get_damage = |e: &SkillEffect| match e {
        SkillEffect::RangedAttack(damage, _) => damage.dice(),
        SkillEffect::MoveAndShoot(damage, _, _) => damage.dice(),
        _ => panic!("get_concrete_skill processing damage of attack: {}", name),
    };

    let get_range = |e: &SkillEffect| match e {
        SkillEffect::MoveAndShoot(_, range, _) => *range,
        _ => panic!("get_concrete_skill processing range of attack: {}", name),
    };

    match name {
        "Spark Shot" => {
            skill.image = Some("gun_01_b.png".to_string());
            skill.range = skill.range.map(|r| r + 1);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1, DamageElement::PHYSICAL).with_option(DamageOptions::RAISE_TEMPERATURE),
                BoltKind::FireBullet,
            );
        }
        "Explosive Blast" => {
            skill.image = Some("SpellBook01_37.png".to_string());
            skill.range = skill.range.map(|r| r + 1);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1, DamageElement::PHYSICAL | DamageElement::FIRE)
                    .with_option(DamageOptions::RAISE_TEMPERATURE)
                    .with_option(DamageOptions::LARGE_TEMPERATURE_DELTA),
                BoltKind::FireBullet,
            );
        }
        "Dragon's Breath" => {
            skill.image = Some("r_16.png".to_string());
            skill.range = skill.range.map(|r| r + 2);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1, DamageElement::FIRE)
                    .with_option(DamageOptions::TRIPLE_SHOT)
                    .with_option(DamageOptions::RAISE_TEMPERATURE),
                BoltKind::FireBullet,
            );
        }
        "Hot Hands" => {
            skill.image = Some("SpellBook01_15.png".to_string());
            skill.effect = SkillEffect::MoveAndShoot(
                Damage::init(get_damage(&skill.effect) - 1, DamageElement::PHYSICAL).with_option(DamageOptions::RAISE_TEMPERATURE),
                get_range(&skill.effect),
                BoltKind::FireBullet,
            );
        }
        "Airburst Shot" => {
            skill.image = Some("gun_01_b.png".to_string());
            skill.range = skill.range.map(|r| r + 2);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1, DamageElement::PHYSICAL).with_option(DamageOptions::ADD_CHARGE_STATUS),
                BoltKind::AirBullet,
            );
        }
        "Air Lance" => {
            skill.image = Some("SpellBook06_46.png".to_string());
            skill.range = skill.range.map(|r| r + 3);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 2, DamageElement::PHYSICAL).with_option(DamageOptions::CONSUMES_CHARGE_KNOCKBACK),
                BoltKind::AirBullet,
            );
        }
        "Tornado Shot" => {
            skill.image = Some("SpellBookPage09_66.png".to_string());
            skill.range = skill.range.map(|r| r + 2);
            skill.effect = SkillEffect::RangedAttack(
                Damage::init(get_damage(&skill.effect) - 1, DamageElement::PHYSICAL | DamageElement::LIGHTNING)
                    .with_option(DamageOptions::TRIPLE_SHOT)
                    .with_option(DamageOptions::CONSUMES_CHARGE_DMG),
                BoltKind::AirBullet,
            );
        }
        "Lightning Speed" => {
            skill.image = Some("SpellBookPage09_39.png".to_string());
            skill.effect = SkillEffect::MoveAndShoot(
                Damage::init(get_damage(&skill.effect) - 1, DamageElement::PHYSICAL).with_option(DamageOptions::ADD_CHARGE_STATUS),
                get_range(&skill.effect).map(|r| r + 1),
                BoltKind::AirBullet,
            );
        }
        _ => {}
    }

    skill
}

pub fn base_resources() -> Vec<(AmmoKind, u32, u32)> {
    vec![(AmmoKind::Bullets, 6, 6), (AmmoKind::Adrenaline, 0, 100)]
}

pub fn instance_skills(ecs: &World, player: Option<Entity>, templates: &Vec<SkillInfo>, skills: &mut SkillsResource) {
    // We instance all, even those impossible to reach in game (because we haven't unlocked that ammo kind)
    // since you can reach them via help
    for m in &[GunslingerAmmo::Magnum, GunslingerAmmo::Ignite, GunslingerAmmo::Cyclone] {
        for s in get_weapon_skills(ecs, player, *m) {
            skills.add(get_concrete_skill(&s, *m, &templates));
        }
    }

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

pub fn add_active_skills(ecs: &mut World, player: Entity, modes: Vec<String>) {
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
    set_current_weapon_trait(ecs, player, GunslingerAmmo::Magnum);

    add_skills_to_front(ecs, player, get_weapon_skills(ecs, Some(player), GunslingerAmmo::Magnum));
    if ecs.read_storage::<GunslingerComponent>().grab(player).ammo_types.len() > 1 {
        ecs.write_storage::<SkillsComponent>().grab_mut(player).skills.push("Swap Ammo".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_ammo_no_holes() {
        let mut ecs = create_test_state()
            .with_gunslinger(
                2,
                2,
                &[test_eq(
                    "a",
                    EquipmentKinds::Armor,
                    &[
                        EquipmentEffect::UnlocksAbilityMode("Ignite".to_string()),
                        EquipmentEffect::UnlocksAbilityMode("Cyclone".to_string()),
                    ],
                    0,
                )],
            )
            .build();
        let player = find_at(&ecs, 2, 2);

        assert_eq!(GunslingerAmmo::Magnum, get_current_weapon_trait(&ecs, player));
        assert_eq!("Snap Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
        rotate_ammo(&mut ecs, player);
        assert_eq!(GunslingerAmmo::Ignite, get_current_weapon_trait(&ecs, player));
        assert_eq!("Spark Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
        rotate_ammo(&mut ecs, player);
        assert_eq!(GunslingerAmmo::Cyclone, get_current_weapon_trait(&ecs, player));
        assert_eq!("Airburst Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
    }

    #[test]
    fn rotate_ammo_with_holes() {
        let mut ecs = create_test_state()
            .with_gunslinger(
                2,
                2,
                &[test_eq(
                    "a",
                    EquipmentKinds::Armor,
                    &[EquipmentEffect::UnlocksAbilityMode("Cyclone".to_string())],
                    0,
                )],
            )
            .build();
        let player = find_at(&ecs, 2, 2);

        assert_eq!(GunslingerAmmo::Magnum, get_current_weapon_trait(&ecs, player));
        assert_eq!("Snap Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
        rotate_ammo(&mut ecs, player);
        assert_eq!(GunslingerAmmo::Cyclone, get_current_weapon_trait(&ecs, player));
        assert_eq!("Airburst Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
    }

    #[test]
    fn rotate_ammo_only_one() {
        let mut ecs = create_test_state().with_gunslinger(2, 2, &[]).build();
        let player = find_at(&ecs, 2, 2);

        assert_eq!(GunslingerAmmo::Magnum, get_current_weapon_trait(&ecs, player));
        assert_eq!("Snap Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
        rotate_ammo(&mut ecs, player);
        assert_eq!(GunslingerAmmo::Magnum, get_current_weapon_trait(&ecs, player));
        assert_eq!("Snap Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
    }

    #[test]
    fn rotate_ammo_reloads_as_well() {
        let mut ecs = create_test_state().with_gunslinger(2, 2, &[]).build();
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
        let mut ecs = create_test_state()
            .with_gunslinger(
                2,
                2,
                &[test_eq(
                    "a",
                    EquipmentKinds::Armor,
                    &[
                        EquipmentEffect::UnlocksAbilityMode("Ignite".to_string()),
                        EquipmentEffect::UnlocksAbilityMode("Cyclone".to_string()),
                    ],
                    0,
                )],
            )
            .build();
        let player = find_at(&ecs, 2, 2);
        assert!(ecs.has_status(player, StatusKind::Magnum));

        rotate_ammo(&mut ecs, player);
        assert!(ecs.has_status(player, StatusKind::Ignite));

        rotate_ammo(&mut ecs, player);
        assert!(ecs.has_status(player, StatusKind::Cyclone));
    }

    #[test]
    fn rotate_ammo_has_sets_correct_skills() {
        let mut ecs = create_test_state()
            .with_gunslinger(
                2,
                2,
                &[test_eq(
                    "a",
                    EquipmentKinds::Armor,
                    &[
                        EquipmentEffect::UnlocksAbilityMode("Ignite".to_string()),
                        EquipmentEffect::UnlocksAbilityMode("Cyclone".to_string()),
                    ],
                    0,
                )],
            )
            .build();
        let player = find_at(&ecs, 2, 2);
        assert_eq!("Snap Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);

        rotate_ammo(&mut ecs, player);
        assert_eq!("Spark Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);

        rotate_ammo(&mut ecs, player);
        assert_eq!("Airburst Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
    }
}
