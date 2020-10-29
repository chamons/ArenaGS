use std::collections::HashMap;
use std::iter;

use serde::{Deserialize, Serialize};

#[derive(Hash, Deserialize, Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum EquipmentKinds {
    Weapon,
    Armor,
    Accessory,
    Mastery,
}

#[derive(Hash, PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
pub struct EquipmentItem {
    pub name: String,
    pub image: Option<String>,
    pub kind: EquipmentKinds,
}

impl EquipmentItem {
    pub fn init(name: &str, image: Option<&str>, kind: EquipmentKinds) -> EquipmentItem {
        EquipmentItem {
            name: name.to_string(),
            image: image.map(|i| i.to_string()),
            kind,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Equipment {
    weapon: Vec<Option<EquipmentItem>>,
    armor: Vec<Option<EquipmentItem>>,
    accessory: Vec<Option<EquipmentItem>>,
    mastery: Vec<Option<EquipmentItem>>,
}

#[allow(dead_code)]
impl Equipment {
    pub fn init_empty() -> Equipment {
        Equipment::init(0, 0, 0, 0)
    }

    pub fn init(weapon_slots: u32, armor_slots: u32, accessory_slots: u32, mastery_slots: u32) -> Equipment {
        Equipment {
            weapon: iter::repeat(None).take(weapon_slots as usize).collect(),
            armor: iter::repeat(None).take(armor_slots as usize).collect(),
            accessory: iter::repeat(None).take(accessory_slots as usize).collect(),
            mastery: iter::repeat(None).take(mastery_slots as usize).collect(),
        }
    }

    fn get_store(&self, kind: EquipmentKinds) -> &Vec<Option<EquipmentItem>> {
        match kind {
            EquipmentKinds::Weapon => &self.weapon,
            EquipmentKinds::Armor => &self.armor,
            EquipmentKinds::Accessory => &self.accessory,
            EquipmentKinds::Mastery => &self.mastery,
        }
    }

    fn get_mut_store(&mut self, kind: EquipmentKinds) -> &mut Vec<Option<EquipmentItem>> {
        match kind {
            EquipmentKinds::Weapon => &mut self.weapon,
            EquipmentKinds::Armor => &mut self.armor,
            EquipmentKinds::Accessory => &mut self.accessory,
            EquipmentKinds::Mastery => &mut self.mastery,
        }
    }

    pub fn get(&self, kind: EquipmentKinds, index: usize) -> Option<EquipmentItem> {
        let store = self.get_store(kind);
        if let Some(slot) = store.get(index) {
            slot.clone()
        } else {
            None
        }
    }

    pub fn count(&self, kind: EquipmentKinds) -> usize {
        let store = self.get_store(kind);
        store.len()
    }

    pub fn add(&mut self, kind: EquipmentKinds, item: EquipmentItem, index: usize) -> bool {
        let store = self.get_mut_store(kind);
        if let Some(slot) = store.get_mut(index) {
            if slot.is_none() {
                *slot = Some(item);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn remove(&mut self, kind: EquipmentKinds, index: usize) -> bool {
        let store = self.get_mut_store(kind);
        if let Some(slot) = store.get_mut(index) {
            if slot.is_some() {
                *slot = None;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn all(&self) -> Vec<Option<EquipmentItem>> {
        let mut all = self.weapon.clone();
        all.extend(self.armor.clone());
        all.extend(self.accessory.clone());
        all.extend(self.mastery.clone());
        all
    }

    pub fn find(&self, name: &str) -> Option<(EquipmentKinds, usize)> {
        for kind in &[
            EquipmentKinds::Weapon,
            EquipmentKinds::Armor,
            EquipmentKinds::Accessory,
            EquipmentKinds::Mastery,
        ] {
            let store = self.get_store(*kind);
            if let Some((i, _)) = store.iter().enumerate().find(|(_, e)| e.as_ref().map(|x| &x.name).map(|x| &**x) == Some(name)) {
                return Some((*kind, i));
            }
        }
        None
    }

    pub fn has(&self, name: &str) -> bool {
        self.find(name).is_some()
    }
}

#[derive(Clone)] // NotConvertSaveload
pub struct EquipmentResource {
    pub equipment: HashMap<String, EquipmentItem>,
}

#[allow(dead_code)]
impl EquipmentResource {
    pub fn init() -> EquipmentResource {
        EquipmentResource { equipment: HashMap::new() }
    }

    pub fn init_with(items: &[EquipmentItem]) -> EquipmentResource {
        EquipmentResource {
            equipment: items.iter().map(|e| (e.name.to_string(), e.clone())).collect(),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.equipment.contains_key(name)
    }

    pub fn get(&self, name: &str) -> EquipmentItem {
        self.equipment[name].clone()
    }

    pub fn add(&mut self, equipment: EquipmentItem) {
        self.equipment.insert(equipment.name.to_string(), equipment);
    }
}

use specs::prelude::*;

pub trait EquipmentLookup {
    fn get_equipment(&self, name: &str) -> EquipmentItem;
}

impl EquipmentLookup for World {
    fn get_equipment(&self, name: &str) -> EquipmentItem {
        self.read_resource::<EquipmentResource>().get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eq(name: &str) -> EquipmentItem {
        EquipmentItem::init(name, None, EquipmentKinds::Weapon)
    }

    #[test]
    fn add_empty_space() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert!(equipment.add(EquipmentKinds::Weapon, eq("Test"), 1));
        assert_eq!("Test", equipment.get(EquipmentKinds::Weapon, 1).unwrap().name);
    }

    #[test]
    fn add_on_top() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert!(equipment.add(EquipmentKinds::Weapon, eq("Test"), 1));
        assert_eq!(false, equipment.add(EquipmentKinds::Weapon, eq("Test2"), 1));
        assert_eq!("Test", equipment.get(EquipmentKinds::Weapon, 1).unwrap().name);
    }

    #[test]
    fn add_no_space() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert_eq!(false, equipment.add(EquipmentKinds::Weapon, eq("Test"), 3));
    }

    #[test]
    fn add_zero_spaces() {
        let mut equipment = Equipment::init(0, 0, 0, 0);
        assert_eq!(false, equipment.add(EquipmentKinds::Weapon, eq("Test"), 0));
    }

    #[test]
    fn remove_has_item() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        equipment.add(EquipmentKinds::Weapon, eq("Test"), 0);
        equipment.add(EquipmentKinds::Weapon, eq("Test2"), 1);

        assert!(equipment.remove(EquipmentKinds::Weapon, 0));
        assert_eq!(None, equipment.get(EquipmentKinds::Weapon, 0));
        assert_eq!("Test2", equipment.get(EquipmentKinds::Weapon, 1).unwrap().name);
    }

    #[test]
    fn remove_empty() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert_eq!(false, equipment.remove(EquipmentKinds::Weapon, 1));
    }

    #[test]
    fn count() {
        let equipment = Equipment::init(4, 3, 0, 0);
        assert_eq!(3, equipment.count(EquipmentKinds::Armor));
    }

    #[test]
    fn all() {
        let mut equipment = Equipment::init(4, 3, 2, 1);
        equipment.add(EquipmentKinds::Weapon, eq("Weapon"), 1);
        equipment.add(EquipmentKinds::Armor, eq("Armor"), 2);
        let all = equipment.all();
        assert_eq!(None, all[0]);
        assert_eq!("Weapon", all[1].as_ref().unwrap().name);
        assert_eq!("Armor", all[6].as_ref().unwrap().name);
    }

    #[test]
    fn find() {
        let mut equipment = Equipment::init(4, 3, 2, 1);
        equipment.add(EquipmentKinds::Weapon, eq("Weapon"), 2);
        equipment.add(EquipmentKinds::Armor, eq("Armor"), 1);
        equipment.add(EquipmentKinds::Armor, eq("Armor2"), 0);

        assert_eq!((EquipmentKinds::Weapon, 2), equipment.find("Weapon").unwrap());
        assert_eq!((EquipmentKinds::Armor, 0), equipment.find("Armor2").unwrap());
    }

    #[test]
    fn has() {
        let mut equipment = Equipment::init(4, 3, 2, 1);
        equipment.add(EquipmentKinds::Weapon, eq("Weapon"), 2);
        equipment.add(EquipmentKinds::Armor, eq("Armor"), 1);
        equipment.add(EquipmentKinds::Armor, eq("Armor2"), 0);

        assert!(equipment.has("Weapon"));
        assert!(equipment.has("Armor2"));
        assert_eq!(false, equipment.has("Foo"));
    }
}
