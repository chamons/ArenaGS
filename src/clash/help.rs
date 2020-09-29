use specs::prelude::*;

use super::{get_skill, is_skill, AmmoKind, Damage, FieldEffect, ShortInfo, SkillEffect, SpawnKind, TargetType};
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

    fn get_spawn_name(kind: SpawnKind) -> &'static str {
        match kind {
            SpawnKind::Player => "Player",
            SpawnKind::Bird => "Giant Bird",
            SpawnKind::BirdSpawn => "Bird",
            SpawnKind::Egg => "Egg",
            SpawnKind::WaterElemental => "Water Elemental",
            SpawnKind::FireElemental => "Fire Elemental",
            SpawnKind::WindElemental => "Wind Elemental",
            SpawnKind::EarthElemental => "Earth Elemental",
            SpawnKind::Elementalist => "Elementalist",
            SpawnKind::SimpleGolem => "Simple Golem",
        }
    }

    fn report_damage(details: &mut Vec<String>, damage: &Damage) {
        details.push(format!("Strength: {}", damage.dice()));
        // TODO - Report damage attributes
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
            SkillEffect::RangedAttack(damage, _) => {
                details.push("Effect: Ranged Attack".to_string());
                HelpInfo::report_damage(&mut details, &damage);
            }
            SkillEffect::MeleeAttack(damage, _) => {
                details.push("Effect: Melee Attack".to_string());
                HelpInfo::report_damage(&mut details, &damage);
            }
            SkillEffect::ConeAttack(damage, _, size) => {
                details.push(format!("Effect: Cone of Width {}", size));
                HelpInfo::report_damage(&mut details, &damage);
            }
            SkillEffect::ChargeAttack(damage, _) => {
                details.push("Effect: Move towards location, attacking first character in path".to_string());
                HelpInfo::report_damage(&mut details, &damage);
            }
            SkillEffect::Reload(kind) => details.push(format!("Effect: Reload all {}", HelpInfo::get_ammo_name(*kind))),
            SkillEffect::Field(effect, _) => match effect {
                FieldEffect::Damage(damage, duration) => {
                    details.push(format!("Effect: Damage after {} ticks", duration));
                    HelpInfo::report_damage(&mut details, &damage);
                }
                FieldEffect::Spawn(kind) => details.push(format!("Effect: Summon a new {} after 100 ticks", HelpInfo::get_spawn_name(*kind))),
                FieldEffect::SustainedDamage(damage, duration) => {
                    details.push(format!("Effect: Damage every 100 ticks with {} duration", duration));
                    HelpInfo::report_damage(&mut details, &damage);
                }
            },
            SkillEffect::MoveAndShoot(damage, shoot_range, _) => {
                details.push(format!(
                    "Effect: Move to targt location and shoot nearest enemy{}.",
                    &shoot_range.map_or("".to_string(), |r| format!(" within range {}", r))
                ));
                HelpInfo::report_damage(&mut details, &damage);
            }
            // TODO - Finish up
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
