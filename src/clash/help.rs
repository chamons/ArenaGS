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
        HelpInfo { header, text }
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
            AmmoKind::Adrenaline => "Adrenaline",
            AmmoKind::Bullets => "Bullet(s)",
            AmmoKind::Charge => "Elemental Charge",
            AmmoKind::Eggs => "Eggs(s)",
            AmmoKind::Feathers => "Feather(s)",
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
        details.push(format!("[[Strength]]: {}", damage.dice()));
        let opt = &damage.options;
        let raises = opt.contains(DamageOptions::RAISE_TEMPERATURE);
        let lowers = opt.contains(DamageOptions::LOWER_TEMPERATURE);
        if raises || lowers {
            details.push(format!(
                "{} [[temperature]] by {}",
                if raises { "Raises" } else { "Lowers" },
                TEMPERATURE_PER_DICE_DAMAGE as u32
                    * damage.dice()
                    * if opt.contains(DamageOptions::LARGE_TEMPERATURE_DELTA) {
                        LARGE_TEMPERATURE_MULTIPLIER
                    } else {
                        1
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
                "Consumes [[static charge]] to do {} strength additional [[piercing]] damage.",
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
            SkillEffect::Move => details.push("Move to Location".to_string()),
            SkillEffect::RangedAttack(damage, _) => {
                details.push("Ranged Attack".to_string());
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::MeleeAttack(damage, _) => {
                details.push("Melee Attack".to_string());
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::ConeAttack(damage, _, size) => {
                details.push(format!("Cone of Width {}", size));
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::ChargeAttack(damage, _) => {
                details.push("Move towards location, attacking first character in path".to_string());
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::Reload(kind) => details.push(format!("Reload all {}", HelpInfo::get_ammo_name(*kind))),
            SkillEffect::ReloadSome(kind, amount) => details.push(format!("Reload {} {}", amount, HelpInfo::get_ammo_name(*kind))),
            SkillEffect::ReloadSomeRandom(kind, amount) => details.push(format!("Reload randomly between 2 and {} {}", amount, HelpInfo::get_ammo_name(*kind))),
            SkillEffect::Field(effect, _) => match effect {
                FieldEffect::Damage(damage, _) => {
                    details.push("Damage after 200 [[ticks]]".to_string());
                    HelpInfo::report_damage(details, &damage);
                }
                FieldEffect::Spawn(kind) => details.push(format!("Summon a {} after 100 [[ticks]]", HelpInfo::get_spawn_name(*kind))),
                FieldEffect::SustainedDamage(damage, duration) => {
                    details.push(format!("Damage every 100 [[ticks]] with {} duration", duration));
                    HelpInfo::report_damage(details, &damage);
                }
            },
            SkillEffect::MoveAndShoot(damage, shoot_range, _) => {
                details.push(format!(
                    "Move to target and shoot nearest enemy{}.",
                    &shoot_range.map_or("".to_string(), |r| format!(" within range {}", r))
                ));
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::ReloadAndRotateAmmo() => details.push("Reload Bullets and rotates to next ammo type.".to_string()),
            SkillEffect::Buff(kind, duration) => details.push(format!(
                "Applies {} effect for {} [[ticks]].",
                HelpInfo::get_status_effect_name(*kind),
                duration
            )),
            SkillEffect::Orb(damage, _, _, _) => {
                details.push("Fire a slow moving a orb along a path.".to_string());
                HelpInfo::report_damage(details, &damage);
            }
            SkillEffect::Spawn(kind) => details.push(format!("Summon a {}.", HelpInfo::get_spawn_name(*kind))),
            SkillEffect::SpawnReplace(kind) => details.push(format!("Summon a {} replacing itself.", HelpInfo::get_spawn_name(*kind))),
            SkillEffect::Sequence(first, second) => {
                HelpInfo::report_skill_effect(details, first);
                details.push("|tab|Then".to_string());
                HelpInfo::report_skill_effect(details, second);
            }
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
        HelpInfo::report_skill_effect(&mut details, &skill.effect);
        if let Some(range) = skill.range {
            details.push(format!("Range: {}", range));
        }

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
        HelpInfo::init(header, details)
    }

    pub fn find(mut word: &str) -> HelpInfo {
        word = match word {
            "Burning" | "Frozen" | "temperature" | "ignite" => "Temperature",
            "static charge" => "Static Charge",
            "ticks" => "Time",
            "Damage" | "Pierce" | "piercing" => "Damage & Defenses",
            "Strength" | "strength" => "Strength in Depth",
            "Defenses" => "Defenses in Depth",
            "armor" => "Armor",
            _ => word,
        };

        match word {
            "Top Level Help" => return HelpInfo::text_header("Help", top_level_topics().iter().map(|t| format!("[[{}]]", t)).collect::<Vec<String>>()),
            "Getting Started" => {
                return HelpInfo::text_header(
                    "Getting Started",
                    vec_of_strings![
                        "Welcome to Arena: Gunpowder And Sorcery!",
                        "Survive arena combat with all sorts of man, beast, and the arcane.",
                        "",
                        "- Right click adjacent tile or use arrow keys to move your character one square North/South/East/West.",
                        "- 1-5 keys or clicking on the skill bar will activate a skill. Most will require a target.",
                        "- Skill range is indicated by yellow highlighted squares.",
                        "- Press F1 at any time to bring up the global help.",
                        "- Middle clicking (or Ctrl + Left click) almost anywhere, specially on underlined text will bring up a context help.",
                        "- Middle click (or Ctrl + Left click) again to promote the tooltip into a full help window.",
                        "- Alt + Enter toggles full-screen mode.",
                        "",
                        "In the future, there will hopefully be a full featured tutorial.",
                        "",
                        "Good Luck!"
                    ],
                )
            }

            "Damage & Defenses" => {
                return HelpInfo::text_header(
                    "Damage & Defenses",
                    vec_of_strings![
                        "Combat in ArenaGS is based upon the interaction of skills Strength and character's Defenses.",
                        "",
                        "Strength: Each ability as a base strength rating, possibly modified by status effects and the situation.",
                        "",
                        "When a skill is resolved, the strength is rolled into a single value which impacts the damage/healing done.",
                        "Each point of strength converts to roughly 1.5 points of damage or healing.",
                        "",
                        "See [[Strength in Depth]] for details.",
                        "",
                        "That damage is then applied to the character's defenses.",
                        " - Dodge and Armor values are applied as strength and subtracted from the total damage.",
                        " - The remaining damage is applied first to any absorb barrier, and then to the health total.",
                        " - Some damage is 'piercing' and ignores Dodge and Armor completely.",
                        "",
                        "See [[Defenses in Depth]] for details."
                    ],
                )
            }
            "Strength in Depth" => {
                return HelpInfo::text_header(
                    "Strength in Depth",
                    vec_of_strings![
                        "Each point of strength is added to a pool of two sided dice (d2's).",
                        "Half of the pool is immediately set to the max value of 2, and the rest is rolled.",
                        "This gives a random result, but with a somewhat small range.",
                        "",
                        "Example:",
                        "8 Strength",
                        "- Create a pool of 8 two sided dice (8d2)",
                        "- 4 of them are immediately set to 2 and set aside. (Total 8)",
                        "- 4 of them are individually rolled, each of which can roll value 1 or 2 (4-8)",
                        "The total result will be in the range of 12 to 16 (8 + 4 to 8 + 8)"
                    ],
                )
            }
            "Defenses in Depth" => {
                return HelpInfo::text_header(
                    "Defenses in Depth",
                    vec_of_strings![
                        "Defenses in ArenaGS come in four forms (Dodge, Armor, Absorb, and Health)",
                        "",
                        "Dodge and Armor are strength values rolled to a value which reduces the raw damage taken.",
                        "- Dodge is a pool of dice which is consumed to reduce damage, and is replenished by movement.",
                        "- Armor tends to be lower, but applies equally to reduce attack damage.",
                        "",
                        "Absorb and Health are pools of health that are depleted by damage that makes it past.",
                        "- Absorb is a shield that goes is taken from first.",
                        "- Once a character's health is reduced to zero or below, they die.",
                        "",
                        "See [[Defense Example]] for an example worked out."
                    ],
                );
            }
            "Health" => return HelpInfo::no_header(vec_of_strings!["Once depleted by damage to zero or below the character dies."]),
            "Absorb" => return HelpInfo::no_header (vec_of_strings!["A pool of health that are depleted first by damage before health."]),
            "Dodge" => return HelpInfo::no_header(vec_of_strings!["A pool of dice which is consumed to reduce damage, and is replenished by movement."]),
            "Armor" => return HelpInfo::no_header(vec_of_strings!["A pool of dice which applies every attack to reduce incoming damage."]),
            "Exhaustion" => return HelpInfo::no_header(vec_of_strings!["How much intense activity a character cna do before being physically taxed."]),
            "Focus" => return HelpInfo::no_header (vec_of_strings!["How much concentration and grit a character can spent in an effort."]),
            "Adrenaline" => return HelpInfo::no_header(vec_of_strings!["The surge of excitement and will needed for some abilities."]),
            "Bullets" => return HelpInfo::no_header(vec_of_strings!["Ammunition used by ranged flashpowder weapons. Will need to be reloaded was expended."]),
            "Defense Example" => {
                return HelpInfo::text_header(
                    "Defense Example",
                    vec_of_strings![
                        "6 Strength attack",
                        "against",
                        "1 Armor/2 Dodge character with 5 Absorb and 20 Health",
                        "",
                        "- First the 6 strength is rolled into an attack damage as described in [[Strength in Depth]]. It rolls 10 total for this example.",
                        "- Dodge applies first, apply as many dodge dice as available (up to the total strength). Roll 2d2 (2) and subtract from 10. (8 incoming damage now).",
                        "- Dodge pool is now 0/2, and won't reduce damage until refilled (2 points per square traveled).",
                        "- Armor applies same, but it does not deplete so applies to every attack. 1d2 rolls a 2. (6 incoming damage now).",
                        "- Damage is now applied to Absorb/Health. Absorb is reduced by 5 to 0. Health is reduced from 20 to 19"
                    ],
                );
            }

            "Gunslinger" => {
                return HelpInfo::text_header(
                    "Gunslinger",
                    vec_of_strings![
                        "Flashpowder is the explosive marriage of fire and air, throwing lead at unbelievable speeds. Gunslingers harness this power along with elemental ammunition, to forge their will into the law.",
                        "",
                        "The gunslinger cycles through three kinds of ammo (Magnum, Ignite, Cyclone).",
                        "",
                        "- [[Magnum Bullets]] use a super heavyweight core packed into an extended bullet with custom grain flashpowder to pack the maximum damage into each shot.",
                        "- [[Ignite Bullets]] pack distilled dragon fire into the hollow shell along with a impact trigger to ignite it upon impact.",
                        "- [[Cyclone Bullets]] distill crystallized gales and use gravity runes to contain hurricane force winds that can knock back and trigger lightning strikes.",
                        "",
                        "Gunslingers are clothed in light armor better for the long trail than battle, and depend upon dexterity to [[Dodge]] incoming fire.",
                        "",
                        "As the battle rages gunslinger's [[Adrenaline]] increases until they are able to pull off almost superhuman feats of agility and speed."
                    ],
                )
            }
            "Magnum Bullets" => {
                use super::content::gunslinger::*;
                let mut text =vec_of_strings![
                    "Since they have the highest base strength, Magnum is the to-go choice when facing heavily armored foes.",
                    "However since Magnum's oversized design does pretty dramatically reduce range, in comparison to the other choices.",
                    "Their lack of elemental effects tends to make and overall one dimensional, though forceful, impact.",
                    "",
                    "Abilities:",
                    ""
                ];
                text.extend(get_weapon_skills (TargetAmmo::Magnum).iter().map(|x| format!("[[{}]]", x)));
                return HelpInfo::text_header(
                    "Magnum Bullets",
                    text
                )
            },
            "Ignite Bullets" => {
                use super::content::gunslinger::*;
                let mut text = vec_of_strings![
                    "Ignite bullets are relatively accurate to medium-long range, and thought they lack the raw stopping power of Magnum rounds they can ignite enemies in flame.",
                    "Since [[Burning]] enemies take constant [[piercing]] damage, Ignite rounds are a solid all-around solution to armored or agile opponents, assuming you can pour on the heat.",
                    "",
                    "Abilities:",
                    ""
                ];

                text.extend(get_weapon_skills (TargetAmmo::Ignite).iter().map(|x| format!("[[{}]]", x)));
                return HelpInfo::text_header(
                    "Ignite Bullets",
                    text
                )
            },
            "Cyclone Bullets" => {
                use super::content::gunslinger::*;
                let mut text = vec_of_strings![
                    "Though lacking some of the raw offensive power of the other ammo, Cyclone bullets provide superb flexibility both in offense and defense",
                    "Cyclone abilities place and consume [[Static Charge]] on enemies to knock back or strike with lightning.",
                    "",
                    "Abilities:",
                    ""
            ];

                text.extend(get_weapon_skills (TargetAmmo::Cyclone).iter().map(|x| format!("[[{}]]", x)));
                return HelpInfo::text_header(
                    "Cyclone Bullets",
                    text
                )
            },
            "Static Charge" => return HelpInfo::text_header("Static Charge", vec_of_strings![
                "Some abilities charge their targets with high levels of electric charge.",
                "",
                "Other abilities can benefit from this charge may (depending on the skill):" ,
                "- Strike for additional [[piercing]] damage",
                "- Knock the target back with gale force winds"
            ]),
            "Resources" => {
                return HelpInfo::text_header(
                    "Resources",
                    vec_of_strings![
                        "Many skills in ArenaGS require one or more resources to be used.",
                        "",
                        "The most common include:",
                        "",
                        "- Exhaustion: How physically taxing an action is.",
                        format!(
                            "|tab|- Starts at 0, with a maximum of {}. Reduces by {} every 100 [[ticks].",
                            MAX_EXHAUSTION, EXHAUSTION_PER_100_TICKS
                        ),
                        format!("|tab|- Standard movement costs {} exhaustion.", EXHAUSTION_COST_PER_MOVE),
                        "",
                        "- Focus: How much concentration and grit a character can spent in an effort.",
                        format!("|tab|- Starts at the maximum of 1.0 and increases by {} every turn.", FOCUS_PER_100_TICKS),
                        "",
                        "- Adrenaline: Some skills need the surge of excitement and will one gains as the battle continues.",
                        "|tab|- Starts at 0 and increases every turn based upon actions.",
                        "",
                        "- Bullets: Firearms can only hold so many rounds, and will require reloading when nearing empty.",
                        "|tab|- Some skills can fire more than one bullet."
                    ],
                )
            }
            "Status Effects" => return HelpInfo::text_header("Status Effects", vec_of_strings![]),
            "Temperature" => {
                return HelpInfo::text_header(
                    "Temperature",
                    vec_of_strings![
                        format!(
                            "All characters in ArenaGS have a temperature value which slowly reduces towards {}.",
                            TEMPERATURE_MIDPOINT
                        ),
                        "",
                        "Some skills can raise or lower it:",
                        "",
                        format!("- Temperatures above {} will ignite a character in flames", TEMPERATURE_BURN_POINT),
                        format!(
                            "|tab|- Burning does {} [[piercing]] [[strength]] of damage every {} [[ticks]].",
                            TEMPERATURE_DAMAGE_PER_TICK, BURN_DURATION
                        ),
                        format!("|tab|- Once the temperatures is below {} they will quit burning.", TEMPERATURE_BURN_POINT),
                        "",
                        format!("- Temperatures below {} will freeze a character.", TEMPERATURE_FREEZE_POINT),
                        "|tab|- Movements will cost an additional 50% time ([[ticks]]) to accomplish."
                    ],
                )
            }
            "Time" => {
                return HelpInfo::text_header(
                    "Time",
                    vec_of_strings![
                        "Time in ArenaGS is based upon 'ticks', which accumulate until a character can act",
                        "",
                        format!(
                            "By default most actions consume {} ticks and movement {} ticks",
                            BASE_ACTION_COST, MOVE_ACTION_COST
                        ),
                        "",
                        "Some status can reduce these costs, allowing more actions to be taken over a period of turns.",
                        "",
                        "A rare few skills are 'instant speed'  - they do not costs ticks to use, allowing the invoker to take another action."
                    ],
                )
            }
            "Aimed" => return HelpInfo::find_status(StatusKind::Aimed),
            _ => {}
        }
        if is_skill(word) {
            return HelpInfo::get_skill_help(word);
        }

        HelpInfo::get_error(word)
    }

    pub fn find_status(status: StatusKind) -> HelpInfo {
        match status {
            StatusKind::Burning => HelpInfo::text_header(
                HelpInfo::get_status_effect_name(status),
                vec_of_strings![format!(
                    "Burning: Take {} [[piercing]] damage every turn while [[temperature]] is above {}.",
                    TEMPERATURE_DAMAGE_PER_TICK, TEMPERATURE_BURN_POINT
                )],
            ),
            StatusKind::Frozen => HelpInfo::text_header(
                HelpInfo::get_status_effect_name(status),
                vec_of_strings![format!(
                    "Frozen: Movements take 150% longer while [[temperature]] is below {}.",
                    TEMPERATURE_FREEZE_POINT
                )],
            ),
            StatusKind::Magnum => HelpInfo::find("Magnum Bullets"),
            StatusKind::Ignite => HelpInfo::find("Ignite Bullets"),
            StatusKind::Cyclone => HelpInfo::find("Cyclone Bullets"),
            StatusKind::StaticCharge => HelpInfo::find("Static Charge"),
            StatusKind::Aimed => HelpInfo::text_header(
                HelpInfo::get_status_effect_name(status),
                vec![format!(
                    "Focused on nailing the next shot. Next attack [[strength]] increased by {}.",
                    AIMED_SHOT_ADDITIONAL_STRENGTH
                )],
            ),
            StatusKind::Armored => HelpInfo::text_header(
                HelpInfo::get_status_effect_name(status),
                vec![format!(
                    "Well prepared for the next blow. {} Additional [[armor]] through next attack.",
                    AIMED_SHOT_ADDITIONAL_STRENGTH
                )],
            ),
            StatusKind::Flying => HelpInfo::text_header(
                HelpInfo::get_status_effect_name(status),
                vec_of_strings!["High above the battleground, unreachable for now."],
            ),
            StatusKind::Regen => HelpInfo::text_header(
                HelpInfo::get_status_effect_name(status),
                vec![format!("Rapidly healing injuries. A {} [[strength]] heal every turn.", HEALTH_REGEN_PER_TICK)],
            ),
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

        HelpInfo::text_header(ecs.get_name(&entity).unwrap_or_else(|| "Unknown Entity".to_string()).as_str(), details)
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
    vec![
        "Getting Started",
        "Damage & Defenses",
        "Gunslinger",
        "Resources",
        "Status Effects",
        "Temperature",
        "Time",
    ]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::after_image::font_test_helpers::*;
    use crate::after_image::{layout_text, Font, LayoutChunkValue, LayoutRequest};

    fn check_links(link: &str, font: &Font) {
        let help = HelpInfo::find(link);
        assert!(!help.text.iter().any(|t| t.contains("Internal Help Error")));
        for chunk in help.text {
            let layout = layout_text(&chunk, font, LayoutRequest::init(0, 0, 500, 0)).unwrap();
            for l in layout.chunks {
                match l.value {
                    LayoutChunkValue::Link(new_link) => {
                        check_links(&new_link, font);
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn help_has_no_unresolved_links() {
        if !has_test_font() {
            return;
        }
        check_links("Top Level Help", &get_test_font());
    }
}
