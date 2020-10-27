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
    weapon: Vec<Option<String>>,
    armor: Vec<Option<String>>,
    accessory: Vec<Option<String>>,
    mastery: Vec<Option<String>>,
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

    fn get_store(&self, kind: EquipmentKinds) -> &Vec<Option<String>> {
        match kind {
            EquipmentKinds::Weapon => &self.weapon,
            EquipmentKinds::Armor => &self.armor,
            EquipmentKinds::Accessory => &self.accessory,
            EquipmentKinds::Mastery => &self.mastery,
        }
    }

    fn get_mut_store(&mut self, kind: EquipmentKinds) -> &mut Vec<Option<String>> {
        match kind {
            EquipmentKinds::Weapon => &mut self.weapon,
            EquipmentKinds::Armor => &mut self.armor,
            EquipmentKinds::Accessory => &mut self.accessory,
            EquipmentKinds::Mastery => &mut self.mastery,
        }
    }

    pub fn get(&self, kind: EquipmentKinds, index: usize) -> Option<String> {
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

    pub fn add(&mut self, kind: EquipmentKinds, name: &str, index: usize) -> bool {
        let store = self.get_mut_store(kind);
        if let Some(slot) = store.get_mut(index) {
            if slot.is_none() {
                *slot = Some(name.to_string());
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

    pub fn all(&self) -> Vec<Option<String>> {
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
            if let Some((i, _)) = store.iter().enumerate().find(|(_, e)| e.as_deref() == Some(name)) {
                return Some((*kind, i));
            }
        }
        None
    }

    pub fn has(&self, name: &str) -> bool {
        self.find(name).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_empty_space() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert!(equipment.add(EquipmentKinds::Weapon, "Test", 1));
        assert_eq!(Some("Test".to_string()), equipment.get(EquipmentKinds::Weapon, 1));
    }

    #[test]
    fn add_on_top() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert!(equipment.add(EquipmentKinds::Weapon, "Test", 1));
        assert_eq!(false, equipment.add(EquipmentKinds::Weapon, "Test2", 1));
        assert_eq!(Some("Test".to_string()), equipment.get(EquipmentKinds::Weapon, 1));
    }

    #[test]
    fn add_no_space() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert_eq!(false, equipment.add(EquipmentKinds::Weapon, "Test", 3));
    }

    #[test]
    fn add_zero_spaces() {
        let mut equipment = Equipment::init(0, 0, 0, 0);
        assert_eq!(false, equipment.add(EquipmentKinds::Weapon, "Test", 0));
    }

    #[test]
    fn remove_has_item() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        equipment.add(EquipmentKinds::Weapon, "Test", 0);
        equipment.add(EquipmentKinds::Weapon, "Test2", 1);

        assert!(equipment.remove(EquipmentKinds::Weapon, 0));
        assert_eq!(None, equipment.get(EquipmentKinds::Weapon, 0));
        assert_eq!(Some("Test2".to_string()), equipment.get(EquipmentKinds::Weapon, 1));
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
        equipment.add(EquipmentKinds::Weapon, "Weapon", 1);
        equipment.add(EquipmentKinds::Armor, "Armor", 2);
        let all = equipment.all();
        assert_eq!(None, all[0]);
        assert_eq!(Some("Weapon".to_string()), all[1]);
        assert_eq!(Some("Armor".to_string()), all[6]);
    }

    #[test]
    fn find() {
        let mut equipment = Equipment::init(4, 3, 2, 1);
        equipment.add(EquipmentKinds::Weapon, "Weapon", 2);
        equipment.add(EquipmentKinds::Armor, "Armor", 1);
        equipment.add(EquipmentKinds::Armor, "Armor2", 0);

        assert_eq!((EquipmentKinds::Weapon, 2), equipment.find("Weapon").unwrap());
        assert_eq!((EquipmentKinds::Armor, 0), equipment.find("Armor2").unwrap());
    }

    #[test]
    fn has() {
        let mut equipment = Equipment::init(4, 3, 2, 1);
        equipment.add(EquipmentKinds::Weapon, "Weapon", 2);
        equipment.add(EquipmentKinds::Armor, "Armor", 1);
        equipment.add(EquipmentKinds::Armor, "Armor2", 0);

        assert!(equipment.has("Weapon"));
        assert!(equipment.has("Armor2"));
        assert_eq!(false, equipment.has("Foo"));
    }
}
