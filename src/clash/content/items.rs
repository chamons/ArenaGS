use super::super::*;

pub fn get_equipment() -> Vec<EquipmentItem> {
    vec![
        EquipmentItem::init("A", Some("gun_06_b.PNG"), EquipmentKinds::Accessory, EquipmentRarity::Common, &[]),
        EquipmentItem::init("B", Some("gun_06_b.PNG"), EquipmentKinds::Accessory, EquipmentRarity::Common, &[]),
        EquipmentItem::init("C", Some("gun_06_b.PNG"), EquipmentKinds::Accessory, EquipmentRarity::Common, &[]),
        EquipmentItem::init("UA", Some("gun_06_b.PNG"), EquipmentKinds::Accessory, EquipmentRarity::Uncommon, &[]),
        EquipmentItem::init("UB", Some("gun_06_b.PNG"), EquipmentKinds::Accessory, EquipmentRarity::Uncommon, &[]),
        EquipmentItem::init("RA", Some("gun_06_b.PNG"), EquipmentKinds::Accessory, EquipmentRarity::Rare, &[]),
        EquipmentItem::init("RB", Some("gun_06_b.PNG"), EquipmentKinds::Accessory, EquipmentRarity::Rare, &[]),
    ]
}
