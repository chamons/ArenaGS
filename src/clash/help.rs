use enum_iterator::IntoEnumIterator;
use specs::prelude::*;

use super::*;
use crate::after_image::LayoutChunkIcon;
use crate::atlas::EasyECS;

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

    pub fn text_header(header: &str, text: Vec<String>) -> HelpInfo {
        HelpInfo::init(HelpHeader::Text(header.to_string()), text)
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

    fn get_status_effect_name(kind: StatusKind) -> &'static str {
        match kind {
            StatusKind::Burning => "Burning",
            StatusKind::Frozen => "Frozen",
            StatusKind::Magnum => "Magnum",
            StatusKind::Ignite => "Ignite",
            StatusKind::Cyclone => "Cyclone",
            StatusKind::StaticCharge => "Static Charge",
            StatusKind::Aimed => "Aimed",
            StatusKind::Armored => "Armored",
            StatusKind::Flying => "Flying",
            StatusKind::Regen => "Regen",
            StatusKind::RegenTick => panic!("RegenTick should not be visible to help"),
            #[cfg(test)]
            StatusKind::TestStatus | StatusKind::TestTrait => "",
        }
    }

    fn report_damage(details: &mut Vec<String>, damage: &Damage) {
        details.push(format!("Strength: {}", damage.dice()));
        let opt = &damage.options;
        let raises = opt.contains(DamageOptions::RAISE_TEMPERATURE);
        let lowers = opt.contains(DamageOptions::LOWER_TEMPERATURE);
        if raises || lowers {
            details.push(format!(
                "{} target's [[temperature]]{}",
                if raises { "Raises" } else { "Lowers" },
                if opt.contains(DamageOptions::LARGE_TEMPERATURE_DELTA) {
                    " by a large amount."
                } else {
                    "."
                }
            ));
        }
        if opt.contains(DamageOptions::KNOCKBACK) {
            details.push("Knocks target back.".to_string());
        }
        if opt.contains(DamageOptions::ADD_CHARGE_STATUS) {
            details.push("Applies [[static charge]].".to_string());
        }
        if opt.contains(DamageOptions::CONSUMES_CHARGE_DMG) {
            details.push(format!(
                "Consumes [[static charge]] to do {} {{Sword}} additional [[piercing]] damage.",
                STATIC_CHARGE_DAMAGE
            ));
        }
        if opt.contains(DamageOptions::CONSUMES_CHARGE_KNOCKBACK) {
            details.push("Consumes [[static charge]] to knockback target.".to_string());
        }
        if opt.contains(DamageOptions::PIERCE_DEFENSES) {
            details.push("[[Pierce]] target's [[Armor]] and [[Dodge]].".to_string());
        }
        if opt.contains(DamageOptions::TRIPLE_SHOT) {
            details.push("Applies three instances of damage".to_string());
        }
        if opt.contains(DamageOptions::AIMED_SHOT) {
            details.push("Grants '[[Aimed]]' effect.".to_string());
        }
    }

    fn report_skill_effect(details: &mut Vec<String>, effect: &SkillEffect) {
        match effect {
            SkillEffect::None => {}
            SkillEffect::Move => details.push("Effect: Move to Location".to_string()),
            SkillEffect::RangedAttack(damage, _) => {
                details.push("Effect: Ranged Attack".to_string());
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::MeleeAttack(damage, _) => {
                details.push("Effect: Melee Attack".to_string());
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::ConeAttack(damage, _, size) => {
                details.push(format!("Effect: Cone of Width {}", size));
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::ChargeAttack(damage, _) => {
                details.push("Effect: Move towards location, attacking first character in path".to_string());
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::Reload(kind) => details.push(format!("Effect: Reload all {}", HelpInfo::get_ammo_name(*kind))),
            SkillEffect::Field(effect, _) => match effect {
                FieldEffect::Damage(damage, _) => {
                    details.push("Effect: Damage after 200 [[ticks]]".to_string());
                    HelpInfo::report_damage(details, &damage);
                }
                FieldEffect::Spawn(kind) => details.push(format!("Effect: Summon a {} after 100 [[ticks]]", HelpInfo::get_spawn_name(*kind))),
                FieldEffect::SustainedDamage(damage, duration) => {
                    details.push(format!("Effect: Damage every 100 [[ticks]] with {} duration", duration));
                    HelpInfo::report_damage(details, &damage);
                }
            },
            SkillEffect::MoveAndShoot(damage, shoot_range, _) => {
                details.push(format!(
                    "Effect: Move to targt location and shoot nearest enemy{}.",
                    &shoot_range.map_or("".to_string(), |r| format!(" within range {}", r))
                ));
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::ReloadAndRotateAmmo() => details.push("Effect: Reload Bullets and rotates to next ammo type.".to_string()),
            SkillEffect::Buff(kind, duration) => details.push(format!(
                "Effect: Applies {} effect for {} [[ticks]].",
                HelpInfo::get_status_effect_name(*kind),
                duration
            )),
            SkillEffect::BuffThen(kind, duration, other_effect) => {
                details.push(format!(
                    "Effect: Applies {} effect for {} [[ticks]].",
                    HelpInfo::get_status_effect_name(*kind),
                    duration
                ));
                details.push("|tab|Then".to_string());
                HelpInfo::report_skill_effect(details, other_effect);
            }
            SkillEffect::ThenBuff(other_effect, kind, duration) => {
                HelpInfo::report_skill_effect(details, other_effect);
                details.push("|tab|Then".to_string());
                details.push(format!(
                    "Effect: Applies {} effect for {} [[ticks]].",
                    HelpInfo::get_status_effect_name(*kind),
                    duration
                ));
            }
            SkillEffect::Orb(damage, _, _, _) => {
                details.push("Effect: Fire a slow moving a orb along a path.".to_string());
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::Spawn(kind) => details.push(format!("Effect: Summon a {}.", HelpInfo::get_spawn_name(*kind))),
            SkillEffect::SpawnReplace(kind) => details.push(format!("Effect: Summon a {} replacing itself.", HelpInfo::get_spawn_name(*kind))),
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

        HelpInfo::report_skill_effect(&mut details, &skill.effect);

        details.push("".to_string());

        if let Some(focus) = skill.focus_use {
            details.push(format!("Costs {} [[Focus]]", focus))
        }

        if let Some(exhaustion) = skill.exhaustion {
            details.push(format!("Costs {} [[Exhaustion]]", exhaustion))
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
            "Top Level Help" => return HelpInfo::text_header("Help", top_level_topics().iter().map(|t| format!("[[{}]]", t)).collect::<Vec<String>>()),
            "Getting Started" => return HelpInfo::text_header("Getting Started", vec![]),
            "Defenses" => return HelpInfo::text_header("Defenses", vec![]),
            "Status Effects" => return HelpInfo::text_header("Status Effects", vec![]),
            "Strength" => return HelpInfo::text_header("Status Effects", vec![]),
            "Temperature" => return HelpInfo::text_header("Temperature", vec![]),
            "Static Charge" => return HelpInfo::text_header("Static Charge", vec![]),
            "Piercing" => return HelpInfo::text_header("Piercing", vec![]),
            "Ticks" => return HelpInfo::text_header("Ticks", vec![]),
            "Focus" => return HelpInfo::text_header("Focus", vec![]),
            "Exhaustion" => return HelpInfo::text_header("Exhaustion", vec![]),
            "Ignite" => return HelpInfo::text_header("Ignite", vec![]),
            // A 'fake' spell for gaining charge
            "Invoke the Elements" => {
                return HelpInfo::init(
                    HelpHeader::Text("Invoke the Elements".to_string()),
                    vec!["Internally focus to more quickly summon additional elementals.".to_string()],
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

    pub fn find_status(status: StatusKind) -> HelpInfo {
        match status {
            StatusKind::Burning => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec_of_strings![format!(
                        "Burning: Take {} [[piercing]] damage every turn while [[temperature]] is above {}.",
                        TEMPERATURE_DAMAGE_PER_TICK, TEMPERATURE_BURN_POINT
                    )],
                )
            }
            StatusKind::Frozen => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec_of_strings![format!(
                        "Frozen: Movements take 150% longer while [[temperature]] is below {}.",
                        TEMPERATURE_FREEZE_POINT
                    )],
                )
            }
            StatusKind::Magnum => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec_of_strings!["Enable shorter range high power gunslinger abilities."],
                )
            }
            StatusKind::Ignite => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec_of_strings!["Enable average range gunslinger abilities which can [[ignite]] foes."],
                )
            }
            StatusKind::Cyclone => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec_of_strings!["Enable long range shocking gunslinger abilities."],
                )
            }
            StatusKind::StaticCharge => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec_of_strings!["Charged with electricity - a lightning rod in the storm. Beware!"],
                )
            }
            StatusKind::Aimed => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec![format!(
                        "Focued on nailing the next shot. Next ranged attack's [[strength]] incrased by {}.",
                        AIMED_SHOT_ADDITIONAL_STRENGTH
                    )],
                )
            }
            StatusKind::Armored => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec![format!(
                        "Well prepared for the next blow. {} Additional [[armor]] through next attack.",
                        AIMED_SHOT_ADDITIONAL_STRENGTH
                    )],
                )
            }
            StatusKind::Flying => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec_of_strings!["High above the battleground, unreachable for now."],
                )
            }
            StatusKind::Regen => {
                return HelpInfo::text_header(
                    HelpInfo::get_status_effect_name(status),
                    vec![format!("Rapidly healing injuries. A {} {{Sword}} heal every turn.", HEALTH_REGEN_PER_TICK)],
                )
            }
            StatusKind::RegenTick => panic!("{:?} should not be visible to help", status),
            #[cfg(test)]
            StatusKind::TestStatus | StatusKind::TestTrait => panic!("{:?} should not be visible to help", status),
        }
    }

    pub fn find_icon(icon: LayoutChunkIcon) -> HelpInfo {
        HelpInfo::get_error(&format!("{:?}", icon))
    }

    pub fn find_entity(ecs: &World, entity: Entity) -> HelpInfo {
        let mut details = vec![];

        summarize_character(ecs, entity, true, false, |t| details.push(t.to_string()));

        HelpInfo::text_header(ecs.get_name(&entity).unwrap_or("Unknown Entity".to_string()).as_str(), details)
    }

    pub fn find_field(ecs: &World, entity: Entity) -> HelpInfo {
        let mut details = vec![];

        let attacks = ecs.read_storage::<AttackComponent>();

        HelpInfo::report_damage(&mut details, &attacks.grab(entity).attack.damage);

        HelpInfo::text_header("Field", details)
    }

    pub fn find_orb(ecs: &World, entity: Entity) -> HelpInfo {
        let attacks = ecs.read_storage::<AttackComponent>();
        let attack = attacks.grab(entity).attack;

        let mut details = vec![];

        HelpInfo::report_damage(&mut details, &attack.damage);

        HelpInfo::text_header("Orb Projectile", details)
    }
}

fn top_level_topics() -> Vec<&'static str> {
    vec!["Getting Started", "Defenses", "Status Effects", "Strength", "Temperature"]
}

pub fn summarize_character<'a>(ecs: &'a World, entity: Entity, show_status_effect: bool, use_links: bool, mut on_text: impl FnMut(&str) + 'a) {
    let char_infos = &ecs.read_storage::<CharacterInfoComponent>();
    let char_info = char_infos.grab(entity);
    let defenses = &char_info.character.defenses;

    let linkify = |s: &str| -> String {
        if use_links {
            format!("[[{}]]", s)
        } else {
            s.to_string()
        }
    };

    let health_text = {
        if defenses.absorb != 0 {
            format!(
                "{}: (+{:.2}) {:.2}/{:.2}",
                linkify("Health"),
                defenses.absorb,
                defenses.health,
                defenses.max_health
            )
        } else {
            format!("{}: {:.2}/{:.2}", linkify("Health"), defenses.health, defenses.max_health)
        }
    };
    on_text(&health_text);

    if defenses.max_dodge != 0 {
        on_text(&format!("{}: {:.2}/{:.2}", linkify("Dodge"), defenses.dodge, defenses.max_dodge));
    }
    if defenses.armor != 0 {
        on_text(&format!("{}: {:.2}", linkify("Armor"), defenses.armor));
    }

    let resources = &ecs.read_storage::<SkillResourceComponent>();
    if let Some(resource) = resources.get(entity) {
        on_text(&format!("{}: {:.2}", linkify("Exhaustion"), resource.exhaustion));

        on_text(&format!("{}: {:.2}", linkify("Focus"), resource.focus).as_str());

        for kind in AmmoKind::into_enum_iter() {
            match resource.max.get(&kind) {
                Some(value) => {
                    on_text(&format!("{}: {:.2}/{:.2}", linkify(&format!("{:?}", kind)), resource.ammo[&kind], value));
                }
                None => {}
            }
        }
    }

    let temperature = char_info.character.temperature.current_temperature();
    if temperature != 0 {
        on_text(&format!("{}: {:.2}", linkify("Temperature"), temperature));
    }

    if show_status_effect {
        let statuses = &ecs.read_storage::<StatusComponent>();
        if let Some(status) = statuses.get(entity) {
            let all = status.status.get_all_display_status();
            if !all.is_empty() {
                on_text(&format!("Status: {}", all.iter().map(|a| linkify(a)).collect::<Vec<String>>().join(" ")));
            }
        }
    }
}
