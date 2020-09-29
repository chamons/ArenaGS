use specs::prelude::*;

use super::{get_skill, is_skill, AmmoKind, Damage, ShortInfo, SkillEffect, TargetType};
use crate::after_image::LayoutChunkIcon;

pub enum HelpHeader {
    None,
    Image(String, String),
    Text(String),
}

pub struct HelpInfo {
    pub header: HelpHeader,
    pub text: Vec<String>,
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

impl HelpInfo {
    pub fn init(header: HelpHeader, text: Vec<String>) -> HelpInfo {
        HelpInfo { header: header, text }
    }

    pub fn no_header(text: Vec<String>) -> HelpInfo {
        HelpInfo::init(HelpHeader::None, text)
    }

    fn get_error(key: &str) -> HelpInfo {
        HelpInfo::no_header(vec_of_strings![
            "|tab| Internal Help Error:",
            "",
            "Please file an issue at:",
            "https://tinyurl.com/ArenaGS-Issue",
            "",
            &format!("Include '{}' in the description.", key)
        ])
    }

    fn get_ammo_name(kind: AmmoKind) -> &'static str {
        match kind {
            AmmoKind::Bullets => "Bullet(s)",
            AmmoKind::Adrenaline => "Adrenaline",
        }
    }

    fn get_skill_help(word: &str) -> HelpInfo {
        let skill = get_skill(word);
        let header = {
            if let Some(image) = skill.image {
                HelpHeader::Image(word.to_string(), image.to_string())
            } else {
                HelpHeader::Text(word.to_string())
            }
        };
        let mut details = vec![];
        match skill.target {
            TargetType::None => {}
            TargetType::Tile => details.push("Target: Any Tile".to_string()),
            TargetType::Player => details.push("Target: Player Directly".to_string()),
            TargetType::Enemy => details.push("Target: Enemy Directly".to_string()),
            TargetType::Any => details.push("Target: Any".to_string()),
            TargetType::AnyoneButSelf => details.push("Target: Any but Self".to_string()),
        }
        if let Some(range) = skill.range {
            details.push(format!("Range: {}", range));
        }

        if details.len() > 0 {
            details.push("".to_string());
        }

        // TODO - Need to get icon in white (?)
        match &skill.effect {
            SkillEffect::None => {}
            SkillEffect::Move => details.push("Effect: Move to Location".to_string()),
            SkillEffect::RangedAttack(damage, _) => details.push(format!("Effect: Ranged {{{{Sword}}}} {}", damage.dice())),
            SkillEffect::MeleeAttack(damage, _) => details.push(format!("Effect: Attack {{{{Sword}}}} {}", damage.dice())),
            SkillEffect::ConeAttack(damage, _, size) => details.push(format!("Effect: Cone of Width {} {{{{Sword}}}} {}", size, damage.dice())),
            SkillEffect::ChargeAttack(damage, _) => details.push(format!(
                "Effect: Move towards location, attacking first impacted character {{{{Sword}}}} {}",
                damage.dice()
            )),
            SkillEffect::Reload(kind) => details.push(format!("Reload all {}", HelpInfo::get_ammo_name(*kind))),
            SkillEffect::Field(effect, _) => {}
            SkillEffect::MoveAndShoot(damage, shoot_range, _) => {}
            SkillEffect::ReloadAndRotateAmmo() => {}
            SkillEffect::Buff(kind, duration) => {}
            SkillEffect::BuffThen(kind, duration, other_effect) => {}
            SkillEffect::ThenBuff(other_effect, kind, duration) => {}
            SkillEffect::Orb(damage, _, speed, duration) => {}
            SkillEffect::Spawn(kind) => {}
            SkillEffect::SpawnReplace(kind) => {}
        }

        if let Some(focus) = skill.focus_use {
            details.push(format!("Costs {} Focus", focus))
        }

        if let Some(exhaustion) = skill.exhaustion {
            details.push(format!("Costs {} Exhaustion", exhaustion))
        }

        if let Some(ammo) = &skill.ammo_info {
            details.push(format!("Costs {} {}", ammo.usage, HelpInfo::get_ammo_name(ammo.kind)));
        }

        if !skill.must_be_clear {
            details.push("Requires no Line of Sight.".to_string())
        }
        if skill.no_time {
            details.push("Requires no time to use.".to_string())
        }

        // pub effect: SkillEffect,
        return HelpInfo::init(header, details);
    }

    pub fn find(word: &str) -> HelpInfo {
        match word {
            // A 'fake' spell for gaining charge
            "Invoke the Elements" => {
                return HelpInfo::init(
                    HelpHeader::Text("Invoke the Elements".to_string()),
                    vec!["Internally focus to more quickly summon additional elements.".to_string()],
                )
            }
            _ => {}
        }
        if is_skill(word) {
            return HelpInfo::get_skill_help(word);
        }

        match word {
            _ => HelpInfo::get_error(word),
        }
    }

    pub fn find_icon(icon: LayoutChunkIcon) -> HelpInfo {
        HelpInfo::get_error(&format!("{:?}", icon))
    }

    pub fn find_entity(ecs: &World, entity: Entity) -> HelpInfo {
        HelpInfo::get_error(&ecs.get_name(&entity).unwrap_or("Unknown Entity".to_string()))
    }
}
