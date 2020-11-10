use crate::clash::{CharacterWeaponKind, Equipment, EquipmentItem, EquipmentRarity, ProgressionComponent, ProgressionState};

pub fn selection_cost(equip: &EquipmentItem) -> u32 {
    match equip.rarity {
        EquipmentRarity::Standard => panic!("Standard should never be found in merchant"),
        EquipmentRarity::Common => 20,
        EquipmentRarity::Uncommon => 50,
        EquipmentRarity::Rare => 100,
    }
}

pub fn can_purchase_selection<'a>(progression: &ProgressionComponent, equipment: &EquipmentItem) -> bool {
    let cost = selection_cost(equipment);
    let influence = progression.state.influence;
    influence >= cost
}

pub fn purchase_selection<'a>(progression: &mut ProgressionComponent, equipment: &EquipmentItem) {
    if can_purchase_selection(progression, equipment) {
        progression.state.items.insert(equipment.name.to_string());
        progression.state.influence -= selection_cost(equipment);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clash::{test_helpers::*, EquipmentEffect, EquipmentItem, EquipmentKinds};

    #[test]
    fn get_equip_cost() {
        assert_eq!(
            20,
            selection_cost(&EquipmentItem::init("a", None, EquipmentKinds::Weapon, EquipmentRarity::Common, &vec![]))
        );
        assert_eq!(
            50,
            selection_cost(&EquipmentItem::init("a", None, EquipmentKinds::Weapon, EquipmentRarity::Uncommon, &vec![]))
        );
        assert_eq!(
            100,
            selection_cost(&EquipmentItem::init("a", None, EquipmentKinds::Weapon, EquipmentRarity::Rare, &vec![]))
        );
    }

    #[test]
    fn can_purchase() {
        let mut progression = ProgressionComponent::init(ProgressionState::init(0, 0, &[], CharacterWeaponKind::Gunslinger, Equipment::init_empty()));
        assert_eq!(
            false,
            can_purchase_selection(
                &mut progression,
                &EquipmentItem::init("a", None, EquipmentKinds::Weapon, EquipmentRarity::Uncommon, &vec![]),
            )
        );
        progression.state.influence = 60;
        assert!(can_purchase_selection(
            &mut progression,
            &EquipmentItem::init("a", None, EquipmentKinds::Weapon, EquipmentRarity::Uncommon, &vec![]),
        ));
    }

    #[test]
    fn purchase() {
        let mut progression = ProgressionComponent::init(ProgressionState::init(0, 60, &[], CharacterWeaponKind::Gunslinger, Equipment::init_empty()));
        purchase_selection(
            &mut progression,
            &EquipmentItem::init("a", None, EquipmentKinds::Weapon, EquipmentRarity::Uncommon, &vec![]),
        );
        assert!(progression.state.items.contains("a"));
        assert_eq!(10, progression.state.influence);
    }
}
