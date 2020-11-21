use specs::prelude::*;

use super::gunslinger::GunslingerAmmo;
use crate::atlas::prelude::*;
use crate::clash::*;

pub trait WeaponPack {
    // Setup new game state
    fn get_skill_tree(&self, equipment: &EquipmentResource) -> Vec<SkillTreeNode>;
    fn get_equipment(&self) -> Vec<EquipmentItem>;
    fn base_resources(&self) -> Vec<(AmmoKind, u32, u32)>;

    // Setup new game skills
    fn all_weapon_skill_classes(&self) -> Vec<String>;
    fn get_raw_skill(&self, name: &str) -> SkillInfo;
    fn instance_skills(&self, templates: &Vec<SkillInfo>, skills: &mut SkillsResource);
    fn add_active_skills(&self, ecs: &mut World, player: Entity, modes: Vec<String>, templates: Vec<String>);
    fn default_attack(&self) -> SkillInfo;
    fn default_attack_replacement(&self) -> &'static str;

    // Next battle preview buttons
    fn get_image_for_kind(&self, ammo: GunslingerAmmo) -> &'static str;
    fn get_all_ammo_images(&self) -> Vec<&'static str>;
    fn get_equipped_ammo(&self, ecs: &World, invoker: Entity) -> Vec<GunslingerAmmo>;
    fn get_current_weapon_trait(&self, ecs: &World, invoker: Entity) -> GunslingerAmmo;
    fn set_ammo_to(&self, ecs: &mut World, invoker: Entity, next_ammo: GunslingerAmmo);
}

pub fn get_weapon_pack(ecs: &World) -> Box<dyn WeaponPack> {
    get_weapon_pack_for(ecs.read_resource::<ProgressionComponent>().state.weapon)
}

pub fn get_weapon_pack_for(weapon: CharacterWeaponKind) -> Box<dyn WeaponPack> {
    match weapon {
        CharacterWeaponKind::Gunslinger => Box::new(super::gunslinger::GunslingerWeaponPack {}),
    }
}

pub fn add_skills_to_front(ecs: &mut World, invoker: Entity, skills_to_add: Vec<String>) {
    let mut skills = ecs.write_storage::<SkillsComponent>();
    let skill_list = &mut skills.grab_mut(invoker).skills;

    // Backwards since we insert one at a time in front
    for s in skills_to_add.iter().rev() {
        skill_list.insert(0, s.to_string());
    }
}

pub fn remove_skills(ecs: &mut World, invoker: Entity, skills_to_remove: Vec<String>) {
    let mut skills = ecs.write_storage::<SkillsComponent>();
    let skill_list = &mut skills.grab_mut(invoker).skills;

    for s in skills_to_remove.iter() {
        skill_list.remove(skill_list.iter().position(|x| *x == *s).unwrap());
    }
}
