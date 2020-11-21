use specs::prelude::*;

use super::gunslinger::GunslingerAmmo;
use crate::atlas::prelude::*;
use crate::clash::*;

pub trait WeaponPack {}

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
