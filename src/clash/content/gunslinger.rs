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

#[derive(Copy, Clone)]
pub enum TargetAmmo {
    Magnum,
    Ignite,
    Cyclone,
}

fn add_skills_to_front(ecs: &mut World, invoker: Entity, skills_to_add: &[&str]) {
    let mut skills = ecs.write_storage::<SkillsComponent>();
    let skill_list = &mut skills.grab_mut(invoker).skills;

    // Backwards since we insert one at a time in front
    for s in skills_to_add.iter().rev() {
        skill_list.insert(0, s.to_string());
    }
}

fn remove_skills(ecs: &mut World, invoker: Entity, skills_to_remove: &[&str]) {
    let mut skills = ecs.write_storage::<SkillsComponent>();
    let skill_list = &mut skills.grab_mut(invoker).skills;

    for s in skills_to_remove.iter() {
        skill_list.remove(skill_list.iter().position(|x| *x == *s).unwrap());
    }
}

fn set_weapon_trait(ecs: &mut World, invoker: Entity, ammo: TargetAmmo) {
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Magnum);
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Ignite);
    StatusStore::remove_trait_if_found_from(ecs, invoker, StatusKind::Cyclone);
    match ammo {
        TargetAmmo::Magnum => ecs.add_trait(invoker, StatusKind::Magnum),
        TargetAmmo::Ignite => ecs.add_trait(invoker, StatusKind::Ignite),
        TargetAmmo::Cyclone => ecs.add_trait(invoker, StatusKind::Cyclone),
    }
}

pub fn rotate_ammo(ecs: &mut World, invoker: Entity) {
    let (current_ammo, next_ammo) = {
        if ecs.has_status(invoker, StatusKind::Magnum) {
            (TargetAmmo::Magnum, TargetAmmo::Ignite)
        } else if ecs.has_status(invoker, StatusKind::Ignite) {
            (TargetAmmo::Ignite, TargetAmmo::Cyclone)
        } else {
            (TargetAmmo::Cyclone, TargetAmmo::Magnum)
        }
    };

    // FIXME
    // remove_skills(ecs, invoker, &get_weapon_skills(current_ammo));
    // add_skills_to_front(ecs, invoker, &get_weapon_skills(next_ammo));
    set_weapon_trait(ecs, invoker, next_ammo);

    reload(ecs, invoker, AmmoKind::Bullets, None);
}

pub fn gunslinger_modes(name: &str) -> String {
    match name {
        "Magnum" | "Default" => "Magnum".to_string(),
        "Ignite" => "Ignite".to_string(),
        "Cyclone" => "Cyclone".to_string(),
        _ => panic!("Unknown gunslinger mode {}", name),
    }
}

pub fn gunslinger_attack_skill_base(name: &str) -> SkillInfo {
    match name {
        "Default" => {
            return SkillInfo::init_with_distance(
                "Snap Shot",
                Some("gun_06_b.PNG"),
                TargetType::Enemy,
                SkillEffect::RangedAttack(Damage::init(5), BoltKind::Bullet),
                Some(7),
                true,
            )
            .with_ammo(AmmoKind::Bullets, 1)
            .with_focus_use(0.5)
            .with_alternate("Reload");
        }

        _ => panic!("Unknown gunslinger attack {}", name),
    }
}

pub fn gunslinger_base_abilities() -> Vec<SkillInfo> {
    vec![
        SkillInfo::init("Reload", Some("b_45.png"), TargetType::None, SkillEffect::Reload(AmmoKind::Bullets)),
        SkillInfo::init("Swap Ammo", Some("b_28.png"), TargetType::None, SkillEffect::ReloadAndRotateAmmo()),
    ]
}

pub fn gunslinger_base_resources() -> Vec<(AmmoKind, u32, u32)> {
    vec![(AmmoKind::Bullets, 6, 6), (AmmoKind::Adrenaline, 0, 100)]
}

pub fn gunslinger_final_setup(ecs: &mut World, invoker: Entity) {
    set_weapon_trait(ecs, invoker, TargetAmmo::Magnum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gunslinger_starts_correctly() {
        let ecs = create_test_state().with_gunslinger(2, 2).build();
        let player = find_at(&ecs, 2, 2);

        assert!(ecs.has_status(player, StatusKind::Magnum));
        assert_eq!(5, ecs.read_storage::<SkillsComponent>().grab(player).skills.len());
    }

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
        assert_eq!("Aimed Shot", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);

        rotate_ammo(&mut ecs, player);
        assert_eq!("Explosive Blast", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);

        rotate_ammo(&mut ecs, player);
        assert_eq!("Air Lance", ecs.read_storage::<SkillsComponent>().grab(player).skills[0]);
    }
}
