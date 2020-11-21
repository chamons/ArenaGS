use std::fmt;

use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::super::progression::SkillTreeNode;
use super::super::*;
use super::weapon_pack::{add_skills_to_front, remove_skills};
use crate::atlas::prelude::*;
use crate::vec_of_strings;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GunslingerAmmo {
    Magnum,
    Ignite,
    Cyclone,
}

impl GunslingerAmmo {
    pub fn from_string(str: &str) -> GunslingerAmmo {
        match str {
            "Magnum" => GunslingerAmmo::Magnum,
            "Ignite" => GunslingerAmmo::Ignite,
            "Cyclone" => GunslingerAmmo::Cyclone,
            _ => panic!("Unknown GunslingerAmmo {}", str),
        }
    }
}

impl fmt::Display for GunslingerAmmo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GunslingerAmmo::Magnum => write!(f, "Magnum"),
            GunslingerAmmo::Ignite => write!(f, "Ignite"),
            GunslingerAmmo::Cyclone => write!(f, "Cyclone"),
        }
    }
}

fn set_current_ammo(ecs: &mut World, invoker: Entity, ammo: GunslingerAmmo) {
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Magnum);
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Ignite);
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Cyclone);
    match ammo {
        GunslingerAmmo::Magnum => ecs.add_trait(invoker, StatusKind::Magnum),
        GunslingerAmmo::Ignite => ecs.add_trait(invoker, StatusKind::Ignite),
        GunslingerAmmo::Cyclone => ecs.add_trait(invoker, StatusKind::Cyclone),
    }
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

    let current_skills = skills_for_ammo(ecs, Some(invoker), current_ammo);
    let new_skills = skills_for_ammo(ecs, Some(invoker), next_ammo);
    remove_skills(ecs, invoker, current_skills);
    add_skills_to_front(ecs, invoker, new_skills);
    set_current_ammo(ecs, invoker, next_ammo);

    reload(ecs, invoker, AmmoKind::Bullets, None);
}

pub fn skills_for_ammo(ecs: &World, player: Option<Entity>, ammo: GunslingerAmmo) -> Vec<String> {
    let weapon_skills = if let Some(player) = player {
        ecs.read_storage::<GunslingerComponent>().grab(player).weapon_skills.clone()
    } else {
        all_weapon_skill_classes()
    };

    weapon_skills.iter().map(|t| get_skill_name_under_ammo(t, ammo).to_string()).collect()
}

fn instance_skill_for_ammo(name: &str, ammo: GunslingerAmmo, templates: &[SkillInfo]) -> SkillInfo {
    let base_name = match ammo {
        GunslingerAmmo::Magnum => name,
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

    let mut skill = templates.iter().find(|&skill_name| base_name == skill_name.name).unwrap().clone();
    if ammo == GunslingerAmmo::Magnum {
        return skill;
    }

    skill.name = name.to_string();

    let get_damage = |e: &SkillEffect| match e {
        SkillEffect::RangedAttack(damage, _) => damage.dice(),
        SkillEffect::MoveAndShoot(damage, _, _) => damage.dice(),
        _ => panic!("instance_skill_for_ammo processing damage of attack: {}", name),
    };

    let get_range = |e: &SkillEffect| match e {
        SkillEffect::MoveAndShoot(_, range, _) => *range,
        _ => panic!("instance_skill_for_ammo processing range of attack: {}", name),
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

fn get_skill_name_under_ammo(base_name: &str, ammo: GunslingerAmmo) -> &str {
    match ammo {
        GunslingerAmmo::Magnum => base_name,
        GunslingerAmmo::Ignite => match base_name {
            "Snap Shot" => "Spark Shot",
            "Aimed Shot" => "Explosive Blast",
            "Triple Shot" => "Dragon's Breath",
            "Quick Shot" => "Hot Hands",
            _ => panic!("Unknown skill template {}", base_name),
        },
        GunslingerAmmo::Cyclone => match base_name {
            "Snap Shot" => "Airburst Shot",
            "Aimed Shot" => "Air Lance",
            "Triple Shot" => "Tornado Shot",
            "Quick Shot" => "Lightning Speed",
            _ => panic!("Unknown skill  template {}", base_name),
        },
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

fn get_equipped_ammo(ecs: &World, invoker: Entity) -> Vec<GunslingerAmmo> {
    ecs.read_storage::<GunslingerComponent>().grab(invoker).ammo_types.to_vec()
}

fn get_current_weapon_trait(ecs: &World, invoker: Entity) -> GunslingerAmmo {
    if ecs.has_status(invoker, StatusKind::Magnum) {
        GunslingerAmmo::Magnum
    } else if ecs.has_status(invoker, StatusKind::Ignite) {
        GunslingerAmmo::Ignite
    } else {
        GunslingerAmmo::Cyclone
    }
}

fn all_weapon_skill_classes() -> Vec<String> {
    vec_of_strings!["Snap Shot", "Aimed Shot", "Triple Shot", "Quick Shot"]
}

pub struct GunslingerWeaponPack {}

impl super::weapon_pack::WeaponPack for GunslingerWeaponPack {
    fn get_skill_tree(&self, equipment: &EquipmentResource) -> Vec<SkillTreeNode> {
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

    fn get_equipment(&self) -> Vec<EquipmentItem> {
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

    fn get_equipped_mode(&self, ecs: &World, invoker: Entity) -> Vec<String> {
        get_equipped_ammo(ecs, invoker).iter().map(|a| a.to_string()).collect()
    }

    fn get_current_weapon_mode(&self, ecs: &World, invoker: Entity) -> String {
        get_current_weapon_trait(ecs, invoker).to_string()
    }

    fn all_weapon_skill_classes(&self) -> Vec<String> {
        all_weapon_skill_classes()
    }

    fn set_mode_to(&self, ecs: &mut World, invoker: Entity, mode: &str) {
        set_ammo_to(ecs, invoker, GunslingerAmmo::from_string(mode));
    }

    fn get_image_for_weapon_mode(&self, mode: &str) -> &'static str {
        match mode {
            "Ignite" => "b_31_1.png",
            "Cyclone" => "b_40_02.png",
            "Magnum" => "b_30.png",
            _ => panic!("Unknown image in get_image_for_weapon_mode: {}", mode),
        }
    }

    fn get_all_mode_images(&self) -> Vec<&'static str> {
        vec!["b_31_1.png", "b_40_02.png", "b_30.png"]
    }

    fn default_attack(&self) -> SkillInfo {
        self.get_raw_skill("Snap Shot")
    }

    fn default_attack_replacement(&self) -> &'static str {
        "Quick Shot"
    }

    // Syntax here gets ugly otherwise after autoformat
    #[allow(clippy::needless_return)]
    fn get_raw_skill(&self, name: &str) -> SkillInfo {
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

    fn base_resources(&self) -> Vec<(AmmoKind, u32, u32)> {
        vec![(AmmoKind::Bullets, 6, 6), (AmmoKind::Adrenaline, 0, 100)]
    }

    fn instance_skills(&self, templates: &[SkillInfo], skills: &mut SkillsResource) {
        // We instance all, even those impossible to reach in game (because we haven't unlocked that ammo kind)
        // since you can reach them via help
        for template in templates {
            for ammo in &[GunslingerAmmo::Magnum, GunslingerAmmo::Ignite, GunslingerAmmo::Cyclone] {
                skills.add(instance_skill_for_ammo(get_skill_name_under_ammo(&template.name, *ammo), *ammo, &templates));
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

    fn add_active_skills(&self, ecs: &mut World, player: Entity, modes: Vec<String>, templates: Vec<String>) {
        let mut modes: Vec<GunslingerAmmo> = modes.iter().map(|m| GunslingerAmmo::from_string(m)).collect();

        if !modes.contains(&GunslingerAmmo::Magnum) {
            modes.insert(0, GunslingerAmmo::Magnum);
        }

        ecs.shovel(player, GunslingerComponent::init(&modes[..], &templates[..]));
        set_current_ammo(ecs, player, GunslingerAmmo::Magnum);

        // Since we start on Magnum, just add the templates directly
        add_skills_to_front(ecs, player, templates);
        if modes.len() > 1 {
            ecs.write_storage::<SkillsComponent>().grab_mut(player).skills.push("Swap Ammo".to_string());
        }
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
