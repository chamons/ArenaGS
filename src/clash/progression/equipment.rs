use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[allow(dead_code)]
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

    pub fn all(&self) -> Vec<String> {
        let mut all = self.weapon.clone();
        all.extend(self.armor.clone());
        all.extend(self.accessory.clone());
        all.extend(self.mastery.clone());
        all
    }

    pub fn find(&self, name: &str) -> Option<(EquipmentKinds, usize)> {
        if let Some((i, _)) = self.weapon.iter().enumerate().find(|(_, w)| *w == name) {
            return Some((EquipmentKinds::Weapon, i));
        }
        if let Some((i, _)) = self.armor.iter().enumerate().find(|(_, w)| *w == name) {
            return Some((EquipmentKinds::Armor, i));
        }
        if let Some((i, _)) = self.accessory.iter().enumerate().find(|(_, w)| *w == name) {
            return Some((EquipmentKinds::Accessory, i));
        }
        if let Some((i, _)) = self.mastery.iter().enumerate().find(|(_, w)| *w == name) {
            return Some((EquipmentKinds::Mastery, i));
        }

        None
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
    fn all() {
        let mut equipment = Equipment::init(4, 3, 2, 1);
        equipment.add(EquipmentKinds::Weapon, "Weapon");
        equipment.add(EquipmentKinds::Armor, "Armor");
        let all = equipment.all();
        assert_eq!("Weapon", all[0]);
        assert_eq!("Armor", all[1]);
    }

    #[test]
    fn find() {
        let mut equipment = Equipment::init(4, 3, 2, 1);
        equipment.add(EquipmentKinds::Weapon, "Weapon");
        equipment.add(EquipmentKinds::Armor, "Armor");
        equipment.add(EquipmentKinds::Armor, "Armor2");

        assert_eq!((EquipmentKinds::Weapon, 0), equipment.find("Weapon").unwrap());
        assert_eq!((EquipmentKinds::Armor, 1), equipment.find("Armor2").unwrap());
    }
}
