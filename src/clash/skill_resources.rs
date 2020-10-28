use std::collections::HashMap;

use super::{SkillEffect, SkillInfo, TargetType};

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

    pub fn all_skill_image_files(&self) -> Vec<&'static str> {
        self.skills.values().filter_map(|s| s.image).collect()
    }

    pub fn add(&mut self, skill: SkillInfo) {
        self.skills.insert(skill.name.to_string(), skill);
    }
}

pub fn init_skills() -> SkillsResource {
    let mut m = SkillsResource::init();

    #[cfg(test)]
    super::content::test::add_test_skills(&mut m);

    super::content::gunslinger::gunslinger_skills(&mut m);
    super::content::bird::bird_skills(&mut m);
    super::content::elementalist::elementalist_skills(&mut m);
    super::content::tutorial::golem_skills(&mut m);

    m.add(SkillInfo::init_with_distance("Dash", Some("SpellBookPage09_39.png"), TargetType::Tile, SkillEffect::Move, Some(3), true).with_exhaustion(50.0));

    m
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
