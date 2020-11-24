// The ai macros can add "unnecessary" returns occationally
#![allow(clippy::needless_return)]

use super::super::*;

pub fn get_equipment() -> Vec<EquipmentItem> {
    let mut equipment = get_armor();
    equipment.append(&mut get_accessories());
    equipment.append(&mut get_masteries());
    equipment
}

fn get_armor() -> Vec<EquipmentItem> {
    vec![
        // Move in straight line
        EquipmentItem::init("Jump Boots", Some("boot_b_01.png"), EquipmentKinds::Armor, EquipmentRarity::Common, &[]),
        // Absorb at start of battle
        EquipmentItem::init("Shield Pin", Some("rune_04_b.png"), EquipmentKinds::Armor, EquipmentRarity::Common, &[]),
        // 1 turn free action dodge CD + exhaustion
        EquipmentItem::init("Wellworn Boots", Some("boot_b_07.png"), EquipmentKinds::Armor, EquipmentRarity::Common, &[]),
        // Warp + 2 turn dodge
        EquipmentItem::init("Cloak of shadows", Some("cl_b_01.png"), EquipmentKinds::Armor, EquipmentRarity::Uncommon, &[]),
        // Ranged attack that has CD
        EquipmentItem::init("Blast Gloves", Some("gl_b_04.png"), EquipmentKinds::Armor, EquipmentRarity::Uncommon, &[]),
    ]
}

fn get_accessories() -> Vec<EquipmentItem> {
    vec![
        // Regen for duration
        EquipmentItem::init("Troll's Blood", Some("ptn_b_01.png"), EquipmentKinds::Accessory, EquipmentRarity::Common, &[]),
        // Short range blink
        EquipmentItem::init("Warp Stone", Some("rune_06_b.png"), EquipmentKinds::Accessory, EquipmentRarity::Common, &[]),
        // Armor for duration pot
        EquipmentItem::init(
            "Adamantine Skin",
            Some("pt_b_04.png"),
            EquipmentKinds::Accessory,
            EquipmentRarity::Uncommon,
            &[],
        ),
        // Single use absorb gain
        EquipmentItem::init("Crush Charm", Some("rune_12_b.png"), EquipmentKinds::Accessory, EquipmentRarity::Common, &[]),
        // AoE Knockback
        EquipmentItem::init("Gale Orb", Some("orb_12_b.png"), EquipmentKinds::Accessory, EquipmentRarity::Common, &[]),
        // Summon elemental baby
        EquipmentItem::init("Terra's Tear", Some("rune_01_b.png"), EquipmentKinds::Accessory, EquipmentRarity::Uncommon, &[]),
        // Summon ghost dog
        EquipmentItem::init(
            "Rover's Last",
            Some("collar_b_01.png"),
            EquipmentKinds::Accessory,
            EquipmentRarity::Uncommon,
            &[],
        ),
        // Cone Damage Focus + CD
        EquipmentItem::init("Magma Orb", Some("orb_02_b.png"), EquipmentKinds::Accessory, EquipmentRarity::Common, &[]),
        // Free action
        EquipmentItem::init("Chronowatch", Some("en_craft_2.png"), EquipmentKinds::Accessory, EquipmentRarity::Rare, &[]),
        // Teleport in line and attack first in line with piercing damage, CD
        EquipmentItem::init("Blink Dagger", Some("kn_b_02.png"), EquipmentKinds::Accessory, EquipmentRarity::Rare, &[]),
    ]
}

fn get_masteries() -> Vec<EquipmentItem> {
    vec![
        // Accessories with charges get +2 charge
        EquipmentItem::init(
            "Equipment Master",
            Some("SpellBook06_92.png"),
            EquipmentKinds::Mastery,
            EquipmentRarity::Rare,
            &[],
        ),
        // Replace all health but 1 to absorb. Armor set to zero. Trickle regen
        EquipmentItem::init("Fool's Reward", Some("coins_b_03.png"), EquipmentKinds::Mastery, EquipmentRarity::Rare, &[]),
        // Convert all dodge to armor. Improves skill damage but no regen
        EquipmentItem::init("The Final Crusade", Some("arm_b_08.png"), EquipmentKinds::Mastery, EquipmentRarity::Rare, &[]),
    ]
}

pub fn get_item_skills(m: &mut SkillsResource) {
    m.add(SkillInfo::init_with_distance(
        "Shadow Shot",
        None,
        TargetType::Enemy,
        SkillEffect::RangedAttack(Damage::init(4, DamageElement::DARKNESS), BoltKind::Bullet),
        Some(5),
        true,
    ));

    m.add(
        SkillInfo::init_with_distance(
            "Summon Shadow",
            Some("SpellBook03_76.png"),
            TargetType::Tile,
            SkillEffect::Spawn(SpawnKind::ShadowGunSlinger, Some(5)),
            Some(5),
            true,
        )
        .with_focus_use(0.5)
        .with_cooldown(1500),
    );
}

use crate::try_behavior;
use specs::prelude::*;

pub fn shadow_gunslinger_behavior(ecs: &mut World, enemy: Entity) {
    try_behavior!(use_skill_at_any_enemy_if_in_range(ecs, enemy, "Shadow Shot"));
    wait(ecs, enemy);
}
