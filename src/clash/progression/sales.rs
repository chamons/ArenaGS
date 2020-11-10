use specs::prelude::*;

use crate::clash::{EquipmentItem, EquipmentRarity, ProgressionComponent};

fn selection_cost(equip: &EquipmentItem) -> u32 {
    match equip.rarity {
        EquipmentRarity::Standard => panic!("Standard should never be found in merchant"),
        EquipmentRarity::Common => 20,
        EquipmentRarity::Uncommon => 50,
        EquipmentRarity::Rare => 100,
    }
}

fn can_purchase_selection<'a>(ecs: &'a World, selection: Option<u32>, get_equipment: &impl Fn(u32) -> &'a EquipmentItem) -> bool {
    if let Some(selection) = selection {
        let cost = selection_cost(get_equipment(selection));
        let influence = (*ecs.read_resource::<ProgressionComponent>()).state.influence;
        influence >= cost
    } else {
        false
    }
}

fn purchase_selection<'a>(ecs: &'a World, selection: Option<u32>, get_equipment: &impl Fn(u32) -> &'a EquipmentItem) {
    if can_purchase_selection(ecs, selection, get_equipment) {
        let progression = &mut ecs.write_resource::<ProgressionComponent>();
        let equip = get_equipment(selection.unwrap());
        progression.state.items.insert(equip.name.to_string());
        progression.state.influence -= selection_cost(equip);
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
    fn can_purchase() {}

    #[test]
    fn purchase() {}
}
