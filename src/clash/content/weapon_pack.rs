use specs::prelude::*;

use super::gunslinger::GunslingerAmmo;
use crate::atlas::prelude::*;
use crate::clash::*;
pub trait WeaponPack {
    fn get_skill_tree(equipment: &EquipmentResource) -> Vec<SkillTreeNode>;
    fn get_equipment() -> Vec<EquipmentItem>;
    fn get_image_for_status(kind: StatusKind) -> &'static str;
    fn get_all_trait_images() -> Vec<&'static str>;
    fn base_resources() -> Vec<(AmmoKind, u32, u32)>;

    fn default_attack() -> &'static str;
    fn default_attack_replacement() -> &'static str;
    fn get_all_bases() -> Vec<String>;
    fn get_base_skill(name: &str) -> SkillInfo;
    fn add_base_abilities(skills: &mut SkillsResource);
    fn process_attack_modes(ecs: &mut World, player: Entity, modes: Vec<String>, templates: &Vec<SkillInfo>, skills: &mut SkillsResource);
    fn instance_skills(ecs: &World, player: Option<Entity>, templates: &Vec<SkillInfo>, skills: &mut SkillsResource);
    fn add_active_skills(ecs: &mut World, player: Entity);

    fn get_weapon_skills(ecs: &World, player: Option<Entity>, ammo: GunslingerAmmo) -> Vec<String>;
    fn get_equipped_ammos(ecs: &World, invoker: Entity) -> Vec<GunslingerAmmo>;
    fn get_image_for_kind(ammo: GunslingerAmmo) -> &'static str;
    fn get_current_weapon_trait(ecs: &World, invoker: Entity) -> GunslingerAmmo;
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
