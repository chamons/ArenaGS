use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EquipmentKinds {
    Weapon,
    Armor,
    Accessory,
    Mastery,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Equipment {
    pub weapon: Vec<String>,
    pub weapon_count: u32,

    pub armor: Vec<String>,
    pub armor_count: u32,

    pub accessory: Vec<String>,
    pub accessory_count: u32,

    pub mastery: Vec<String>,
    pub mastery_count: u32,
}

impl Equipment {
    pub fn init_empty() -> Equipment {
        Equipment::init(0, 0, 0, 0)
    }

    pub fn init(weapon_count: u32, armor_count: u32, accessory_count: u32, mastery_count: u32) -> Equipment {
        Equipment {
            weapon_count,
            armor_count,
            accessory_count,
            mastery_count,
            weapon: vec![],
            armor: vec![],
            accessory: vec![],
            mastery: vec![],
        }
    }

    fn get(&self, kind: EquipmentKinds) -> (u32, &Vec<String>) {
        match kind {
            EquipmentKinds::Weapon => (self.weapon_count, &self.weapon),
            EquipmentKinds::Armor => (self.armor_count, &self.armor),
            EquipmentKinds::Accessory => (self.accessory_count, &self.accessory),
            EquipmentKinds::Mastery => (self.mastery_count, &self.mastery),
        }
    }

    fn get_mut(&mut self, kind: EquipmentKinds) -> (u32, &mut Vec<String>) {
        match kind {
            EquipmentKinds::Weapon => (self.weapon_count, &mut self.weapon),
            EquipmentKinds::Armor => (self.armor_count, &mut self.armor),
            EquipmentKinds::Accessory => (self.accessory_count, &mut self.accessory),
            EquipmentKinds::Mastery => (self.mastery_count, &mut self.mastery),
        }
    }

    pub fn add(&mut self, kind: EquipmentKinds, name: &str) -> bool {
        let (max, store) = self.get_mut(kind);
        if store.len() + 1 <= max as usize {
            store.push(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, kind: EquipmentKinds, index: usize) -> bool {
        let (_, store) = self.get_mut(kind);
        if index < store.len() {
            store.remove(index);
            true
        } else {
            false
        }
    }

    pub fn swap(&mut self, kind: EquipmentKinds, name: &str, index: usize) -> bool {
        let (_, store) = self.get_mut(kind);
        if index < store.len() {
            store.remove(index);
            store.insert(index, name.to_string());
            true
        } else {
            false
        }
    }

    pub fn count(&self) -> Vec<(EquipmentKinds, u32)> {
        vec![
            (EquipmentKinds::Weapon, self.get(EquipmentKinds::Weapon).0),
            (EquipmentKinds::Armor, self.get(EquipmentKinds::Armor).0),
            (EquipmentKinds::Accessory, self.get(EquipmentKinds::Accessory).0),
            (EquipmentKinds::Mastery, self.get(EquipmentKinds::Mastery).0),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_empty_space() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert!(equipment.add(EquipmentKinds::Weapon, "Test"));
        assert_eq!("Test", equipment.weapon.get(0).unwrap());
    }

    #[test]
    fn add_no_space() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        equipment.weapon = vec!["a".to_string(), "b".to_string()];
        assert_eq!(false, equipment.add(EquipmentKinds::Weapon, "Test"));
    }

    #[test]
    fn add_zero_spaces() {
        let mut equipment = Equipment::init(0, 0, 0, 0);
        assert_eq!(false, equipment.add(EquipmentKinds::Weapon, "Test"));
    }

    #[test]
    fn remove_has_item() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        equipment.add(EquipmentKinds::Weapon, "Test");
        equipment.add(EquipmentKinds::Weapon, "Test2");
        assert!(equipment.remove(EquipmentKinds::Weapon, 1));
        assert_eq!("Test", equipment.weapon.get(0).unwrap());
    }

    #[test]
    fn remove_empty() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert_eq!(false, equipment.remove(EquipmentKinds::Weapon, 1));
    }

    #[test]
    fn swap_full() {
        let mut equipment = Equipment::init(3, 0, 0, 0);
        equipment.add(EquipmentKinds::Weapon, "Test");
        equipment.add(EquipmentKinds::Weapon, "Test2");
        equipment.add(EquipmentKinds::Weapon, "Test3");
        assert!(equipment.swap(EquipmentKinds::Weapon, "Test4", 1));
        assert_eq!("Test", equipment.weapon.get(0).unwrap());
        assert_eq!("Test4", equipment.weapon.get(1).unwrap());
        assert_eq!("Test3", equipment.weapon.get(2).unwrap());
    }

    #[test]
    fn swap_empty() {
        let mut equipment = Equipment::init(2, 0, 0, 0);
        assert_eq!(false, equipment.swap(EquipmentKinds::Weapon, "Test2", 0));
    }

    #[test]
    fn swap_no_space() {
        let mut equipment = Equipment::init(0, 0, 0, 0);
        assert_eq!(false, equipment.swap(EquipmentKinds::Weapon, "Test2", 0));
    }

    #[test]
    fn count() {
        let equipment = Equipment::init(1, 2, 3, 4);
        let count = equipment.count();
        assert_eq!(EquipmentKinds::Weapon, count[0].0);
        assert_eq!(1, count[0].1);
        assert_eq!(EquipmentKinds::Armor, count[1].0);
        assert_eq!(2, count[1].1);
        assert_eq!(EquipmentKinds::Accessory, count[2].0);
        assert_eq!(3, count[2].1);
        assert_eq!(EquipmentKinds::Mastery, count[3].0);
        assert_eq!(4, count[3].1);
    }
}
