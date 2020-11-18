use std::collections::HashMap;

use super::SkillInfo;

#[derive(Clone)] // NotConvertSaveload
pub struct SkillsResource {
    pub skills: HashMap<String, SkillInfo>,
}

impl SkillsResource {
    pub fn init() -> SkillsResource {
        SkillsResource { skills: HashMap::new() }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.skills.contains_key(name)
    }

    pub fn get(&self, name: &str) -> SkillInfo {
        self.skills[name].clone()
    }

    pub fn all_skill_image_files(&self) -> Vec<String> {
        self.skills.values().filter_map(|s| s.image.clone()).collect()
    }

    pub fn add(&mut self, skill: SkillInfo) {
        self.skills.insert(skill.name.to_string(), skill);
    }
}

use specs::prelude::*;

pub trait SkillLookup {
    fn get_skill(&self, name: &str) -> SkillInfo;
}

impl SkillLookup for World {
    fn get_skill(&self, name: &str) -> SkillInfo {
        self.read_resource::<SkillsResource>().get(name)
    }
}
