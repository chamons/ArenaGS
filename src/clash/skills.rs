use std::cmp;
use std::collections::HashMap;
use std::slice::from_ref;

use enum_iterator::IntoEnumIterator;
use lazy_static::lazy_static;
use ordered_float::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::*;
use crate::atlas::prelude::*;

#[allow(dead_code)]
#[derive(is_enum_variant, Clone, Copy)]
pub enum TargetType {
    None,
    Tile,
    Player,
    Enemy,
    Any,
    AnyoneButSelf,
}

#[allow(dead_code)]
pub enum SkillEffect {
    None,
    Move,
    RangedAttack(Damage, BoltKind),
    MeleeAttack(Damage, WeaponKind),
    ConeAttack(Damage, ConeKind, u32),
    ChargeAttack(Damage, WeaponKind),
    Reload(AmmoKind),
    ReloadSome(AmmoKind, u32),
    ReloadSomeRandom(AmmoKind, u32),
    Field(FieldEffect, FieldKind),
    MoveAndShoot(Damage, Option<u32>, BoltKind),
    ReloadAndRotateAmmo(),
    Buff(StatusKind, i32),
    Orb(Damage, OrbKind, u32, u32),
    Spawn(SpawnKind),
    SpawnReplace(SpawnKind),
    Sequence(Box<SkillEffect>, Box<SkillEffect>),
}

#[macro_export]
macro_rules! sequence {
    ($x:expr, $y:expr) => {
        SkillEffect::Sequence(Box::new($x), Box::new($y))
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, IntoEnumIterator, Debug, Deserialize, Serialize)]
pub enum AmmoKind {
    Bullets,
    Eggs,
    Feathers,
    Charge,
    Adrenaline,
}

pub struct AmmoInfo {
    pub kind: AmmoKind,
    pub usage: u32,
}

impl SkillResourceComponent {
    pub fn init(starting_ammo: &[(AmmoKind, u32, u32)]) -> SkillResourceComponent {
        let ammo: HashMap<AmmoKind, u32> = starting_ammo.iter().map(|(kind, starting, _)| (*kind, *starting)).collect();
        let max: HashMap<AmmoKind, u32> = starting_ammo.iter().map(|(kind, _, max)| (*kind, *max)).collect();
        SkillResourceComponent {
            ammo,
            max,
            cooldown: HashMap::new(),
            exhaustion: 0.0,
            focus: 0.0,
            max_focus: 0.0,
        }
    }

    pub fn with_focus(mut self, focus: f64) -> SkillResourceComponent {
        self.focus = focus;
        self.max_focus = focus;
        self
    }
}

pub struct SkillInfo {
    pub name: &'static str,
    pub image: Option<&'static str>,
    pub target: TargetType,
    pub effect: SkillEffect,
    pub range: Option<u32>,
    pub must_be_clear: bool,
    pub ammo_info: Option<AmmoInfo>,
    pub alternate: Option<String>,
    pub exhaustion: Option<f64>,
    pub focus_use: Option<f64>,
    pub cooldown: Option<u32>,
    pub start_cooldown_spent: bool,
    pub no_time: bool,
}

#[allow(dead_code)]
impl SkillInfo {
    pub fn init(name: &'static str, image: Option<&'static str>, target: TargetType, effect: SkillEffect) -> SkillInfo {
        SkillInfo::init_with_distance(name, image, target, effect, None, false)
    }

    pub fn init_with_distance(
        name: &'static str,
        image: Option<&'static str>,
        target: TargetType,
        effect: SkillEffect,
        range: Option<u32>,
        must_be_clear: bool,
    ) -> SkillInfo {
        SkillInfo {
            name,
            image,
            target,
            effect,
            range,
            must_be_clear,
            ammo_info: None,
            alternate: None,
            exhaustion: None,
            focus_use: None,
            cooldown: None,
            no_time: false,
            start_cooldown_spent: false,
        }
    }

    pub fn with_ammo(mut self, kind: AmmoKind, usage: u32) -> SkillInfo {
        self.ammo_info = Some(AmmoInfo { kind, usage });
        self
    }

    pub fn with_alternate(mut self, skill_name: &str) -> SkillInfo {
        self.alternate = Some(skill_name.to_string());
        self
    }

    pub fn with_exhaustion(mut self, exhaustion: f64) -> SkillInfo {
        self.exhaustion = Some(exhaustion);
        self
    }

    pub fn with_focus_use(mut self, focus: f64) -> SkillInfo {
        self.focus_use = Some(focus);
        self
    }

    pub fn with_no_time(mut self) -> SkillInfo {
        self.no_time = true;
        self
    }

    pub fn with_cooldown(mut self, cooldown: u32) -> SkillInfo {
        self.cooldown = Some(cooldown);
        self
    }

    pub fn with_cooldown_spent(mut self) -> SkillInfo {
        self.start_cooldown_spent = true;
        self
    }

    pub fn show_trail(&self) -> bool {
        self.must_be_clear
    }

    fn get_ammo_usage(&self, ecs: &World, entity: Entity) -> Option<u32> {
        match &self.ammo_info {
            Some(ammo_info) => {
                let skill_resources = ecs.read_storage::<SkillResourceComponent>();
                let current_state = &skill_resources.grab(entity).ammo;
                match current_state.get(&ammo_info.kind) {
                    Some(current) => {
                        if *current >= ammo_info.usage {
                            Some(current / ammo_info.usage)
                        } else {
                            Some(0)
                        }
                    }
                    None => Some(0),
                }
            }
            None => None,
        }
    }

    fn get_focus_usage(&self, ecs: &World, entity: Entity) -> Option<u32> {
        match &self.focus_use {
            Some(usage) => {
                let current = ecs.read_storage::<SkillResourceComponent>().grab(entity).focus;
                Some((current / usage).floor() as u32)
            }
            None => None,
        }
    }

    fn get_exhaustion_usage(&self, ecs: &World, entity: Entity) -> Option<u32> {
        match &self.exhaustion {
            Some(usage) => {
                let current = ecs.read_storage::<SkillResourceComponent>().grab(entity).exhaustion;
                let remaining = MAX_EXHAUSTION - current;
                Some((remaining / usage).floor() as u32)
            }
            None => None,
        }
    }

    pub fn get_remaining_usages(&self, ecs: &World, entity: Entity) -> Option<u32> {
        let usages = vec![
            self.get_ammo_usage(ecs, entity),
            self.get_exhaustion_usage(ecs, entity),
            self.get_focus_usage(ecs, entity),
        ];
        usages.iter().filter_map(|x| *x).min()
    }

    pub fn is_usable(&self, ecs: &World, entity: Entity) -> UsableResults {
        if self.get_focus_usage(ecs, entity).unwrap_or(1) == 0 {
            return UsableResults::LacksFocus;
        }
        if self.get_exhaustion_usage(ecs, entity).unwrap_or(1) == 0 {
            return UsableResults::Exhaustion;
        }
        if self.get_ammo_usage(ecs, entity).unwrap_or(1) == 0 {
            return UsableResults::LacksAmmo;
        }
        UsableResults::Usable
    }

    pub fn get_cooldown(&self, ecs: &mut World, entity: Entity) -> u32 {
        let mut skill_resources = ecs.write_storage::<SkillResourceComponent>();
        if let Some(skill_resource) = skill_resources.get_mut(entity) {
            *skill_resource
                .cooldown
                .entry(self.name.to_string())
                .or_insert_with(|| if self.start_cooldown_spent { self.cooldown.unwrap() } else { 0 })
        } else {
            0
        }
    }
}

pub enum UsableResults {
    Usable,
    LacksAmmo,
    Exhaustion,
    LacksFocus,
}

pub trait SkillMap {
    fn add_skill(&mut self, info: SkillInfo);
}

impl SkillMap for HashMap<&'static str, SkillInfo> {
    fn add_skill(&mut self, info: SkillInfo) {
        self.insert(&info.name, info);
    }
}

lazy_static! {
    static ref SKILLS: HashMap<&'static str, SkillInfo> = {
        let mut m = HashMap::new();

        #[cfg(test)]
        super::content::test::add_test_skills(&mut m);

        super::content::gunslinger::gunslinger_skills(&mut m);
        super::content::bird::bird_skills(&mut m);
        super::content::elementalist::elementalist_skills(&mut m);
        super::content::tutorial::golem_skills(&mut m);

        m.add_skill(
            SkillInfo::init_with_distance("Dash", Some("SpellBookPage09_39.png"), TargetType::Tile, SkillEffect::Move, Some(3), true).with_exhaustion(50.0),
        );

        m
    };
}

pub fn is_skill(name: &str) -> bool {
    SKILLS.contains_key(name)
}

pub fn get_skill(name: &str) -> &'static SkillInfo {
    &SKILLS[name]
}

pub fn all_skill_image_filesnames() -> Vec<&'static str> {
    SKILLS.values().filter_map(|s| s.image).collect()
}

fn assert_correct_targeting(ecs: &mut World, invoker: Entity, name: &str, target: Option<Point>) {
    let skill = get_skill(name);

    let requires_point = match skill.target {
        TargetType::None => false,
        TargetType::AnyoneButSelf | TargetType::Tile | TargetType::Enemy | TargetType::Player | TargetType::Any => true,
    };

    if requires_point != target.is_some() {
        panic!("invoke_skill for {} called with wrong targeting param state.", name);
    }

    if let Some(target) = target {
        assert!(is_good_target(ecs, invoker, &skill, target));
    }
}

pub fn is_good_target(ecs: &World, invoker: Entity, skill: &SkillInfo, target: Point) -> bool {
    if !match skill.target {
        TargetType::Tile => is_area_clear_of_others(ecs, from_ref(&target), invoker),
        TargetType::Enemy => !is_area_clear_of_others(ecs, from_ref(&target), invoker),
        TargetType::Player => ecs.get_position(find_player(ecs)).contains_point(&target),
        TargetType::AnyoneButSelf => {
            if let Some(initial) = ecs.read_storage::<PositionComponent>().get(invoker) {
                !initial.position.contains_point(&target)
            } else {
                true
            }
        }
        TargetType::Any => true,
        TargetType::None => false,
    } {
        return false;
    }

    if !in_possible_skill_range(ecs, invoker, skill, target) {
        return false;
    }

    true
}

pub fn in_possible_skill_range(ecs: &World, invoker: Entity, skill: &SkillInfo, target: Point) -> bool {
    if let Some(skill_range) = skill.range {
        if let Some(range_to_target) = ecs.get_position(invoker).distance_to(target) {
            if range_to_target > skill_range {
                return false;
            }
        }
    }

    if skill.must_be_clear {
        if let Some(mut path) = ecs.get_position(invoker).line_to(target) {
            // If we are targeting an enemy/player we can safely
            // ignore the last square, since we know that it must
            // have the target (from checks above)
            if skill.target.is_enemy() || skill.target.is_player() {
                path.pop();
            }
            if !is_area_clear_of_others(ecs, &path, invoker) {
                return false;
            }
        }
    }
    true
}

pub fn skill_secondary_range(skill: &SkillInfo) -> Option<u32> {
    match skill.effect {
        SkillEffect::MoveAndShoot(_, range, _) => range,
        _ => None,
    }
}

pub fn has_resources_for_skill(ecs: &mut World, invoker: Entity, skill: &SkillInfo) -> bool {
    let has_needed_ammo = skill.get_remaining_usages(ecs, invoker).map_or(true, |x| x > 0);
    let has_no_cooldown = skill.get_cooldown(ecs, invoker) == 0;
    has_needed_ammo && has_no_cooldown
}

pub fn can_invoke_skill(ecs: &mut World, invoker: Entity, skill: &SkillInfo, target: Option<Point>) -> bool {
    let has_valid_target = target.map_or(true, |x| is_good_target(ecs, invoker, skill, x));
    has_resources_for_skill(ecs, invoker, skill) && has_valid_target
}

pub fn spend_focus(ecs: &mut World, invoker: Entity, cost: f64) {
    ecs.write_storage::<SkillResourceComponent>().grab_mut(invoker).focus -= cost;
    assert!(ecs.read_storage::<SkillResourceComponent>().grab(invoker).focus >= 0.0);
}

pub fn spend_cooldown(ecs: &mut World, invoker: Entity, skill: &SkillInfo) {
    ecs.write_storage::<SkillResourceComponent>()
        .grab_mut(invoker)
        .cooldown
        .insert(skill.name.to_string(), skill.cooldown.unwrap());
}

pub fn invoke_skill(ecs: &mut World, invoker: Entity, name: &str, target: Option<Point>) {
    assert_correct_targeting(ecs, invoker, name, target);
    let skill = get_skill(name);
    assert!(can_invoke_skill(ecs, invoker, skill, target));

    if let Some(invoker_name) = ecs.get_name(invoker) {
        ecs.log(format!("{} used [[{}]]", invoker_name.as_str(), name));
    }

    if !skill.no_time {
        spend_time(ecs, invoker, BASE_ACTION_COST);
    }
    spend_ammo(ecs, invoker, skill);

    if let Some(exhaustion) = skill.exhaustion {
        spend_exhaustion(ecs, invoker, exhaustion);
    }
    if let Some(focus_use) = skill.focus_use {
        spend_focus(ecs, invoker, focus_use);
    }
    if skill.cooldown.is_some() {
        spend_cooldown(ecs, invoker, skill);
    }

    gain_adrenaline(ecs, invoker, skill);

    process_skill(ecs, invoker, &skill.effect, target);
}

fn process_skill(ecs: &mut World, invoker: Entity, effect: &SkillEffect, target: Option<Point>) {
    let skill_power = ecs
        .read_storage::<CharacterInfoComponent>()
        .get(invoker)
        .map(|c| c.character.skill_power)
        .unwrap_or(0);

    match effect {
        SkillEffect::Move => {
            // Targeting only gives us a point, so clone their position to get size as well
            let position = ecs.get_position(invoker).move_to(target.unwrap());
            begin_move(ecs, invoker, position, PostMoveAction::None);
        }
        SkillEffect::MoveAndShoot(damage, range, kind) => {
            // Targeting only gives us a point, so clone their position to get size as well
            let position = ecs.get_position(invoker).move_to(target.unwrap());
            begin_shoot_and_move(ecs, invoker, position, *range, damage.more_strength(skill_power), *kind)
        }
        SkillEffect::RangedAttack(damage, kind) => begin_bolt(ecs, invoker, target.unwrap(), damage.more_strength(skill_power), *kind),
        SkillEffect::MeleeAttack(damage, kind) => begin_melee(ecs, invoker, target.unwrap(), damage.more_strength(skill_power), *kind),
        SkillEffect::ConeAttack(damage, kind, size) => begin_cone(ecs, invoker, target.unwrap(), *damage, *kind, *size),
        SkillEffect::ChargeAttack(damage, kind) => begin_charge(ecs, invoker, target.unwrap(), *damage, *kind),
        SkillEffect::Reload(kind) => reload(ecs, invoker, *kind, None),
        SkillEffect::ReloadSome(kind, amount) => reload(ecs, invoker, *kind, Some(*amount)),
        SkillEffect::ReloadSomeRandom(kind, amount) => reload_random(ecs, invoker, *kind, *amount),
        SkillEffect::Field(effect, kind) => begin_field(ecs, invoker, target.unwrap(), *effect, *kind),
        SkillEffect::ReloadAndRotateAmmo() => content::gunslinger::rotate_ammo(ecs, invoker),
        SkillEffect::Buff(buff, duration) => {
            ecs.add_status(find_valid_buff_target(ecs, invoker, target), *buff, *duration);
        }
        SkillEffect::Orb(damage, kind, speed, duration) => {
            begin_orb(ecs, invoker, target.unwrap(), damage.more_strength(skill_power), *kind, *speed, *duration)
        }
        SkillEffect::Spawn(kind) => spawn(ecs, SizedPoint::from(target.unwrap()), *kind),
        SkillEffect::SpawnReplace(kind) => spawn_replace(ecs, invoker, *kind),
        SkillEffect::Sequence(first, second) => {
            process_skill(ecs, invoker, first, target);
            process_skill(ecs, invoker, second, target);
        }
        SkillEffect::None => {}
    }
}

fn find_valid_buff_target(ecs: &World, invoker: Entity, target: Option<Point>) -> Entity {
    if let Some(target) = target {
        if let Some(target) = find_character_at_location(ecs, target) {
            // ALLIES_TODO -  https://github.com/chamons/ArenaGS/issues/201
            let player = find_player(ecs);
            if invoker != player && target != player {
                return target;
            }
        }
    }
    invoker
}

fn gain_adrenaline(ecs: &mut World, invoker: Entity, skill: &SkillInfo) {
    let amount = match &skill.effect {
        SkillEffect::Move => 1,
        SkillEffect::MoveAndShoot(_, _, _) => 6,
        SkillEffect::RangedAttack(_, _) => 6,
        SkillEffect::MeleeAttack(_, _) => 6,
        SkillEffect::ConeAttack(_, _, _) => 6,
        SkillEffect::ChargeAttack(_, _) => 6,
        SkillEffect::Orb(_, _, _, _) => 6,
        SkillEffect::Reload(_) => 3,
        SkillEffect::ReloadSome(_, _) => 3,
        SkillEffect::Field(_, _) => 3,
        SkillEffect::Spawn(_) => 3,
        SkillEffect::ReloadSomeRandom(_, _) => 3,
        SkillEffect::ReloadAndRotateAmmo() => 3,
        SkillEffect::Buff(_, _) => 3,
        SkillEffect::None => 0,
        SkillEffect::SpawnReplace(_) => 0,
        SkillEffect::Sequence(_, _) => 0,
    };

    let mut skill_resources = ecs.write_storage::<SkillResourceComponent>();
    if let Some(resources) = &mut skill_resources.get_mut(invoker) {
        if resources.ammo.contains_key(&AmmoKind::Adrenaline) {
            let new_total = cmp::min(resources.ammo[&AmmoKind::Adrenaline] + amount, resources.max[&AmmoKind::Adrenaline]);
            *resources.ammo.get_mut(&AmmoKind::Adrenaline).unwrap() = new_total;
        }
    }
}

fn set_ammo(ecs: &mut World, invoker: Entity, kind: AmmoKind, amount: u32) {
    let mut skill_resources = ecs.write_storage::<SkillResourceComponent>();
    *skill_resources.grab_mut(invoker).ammo.get_mut(&kind).unwrap() = amount;
}

fn spend_ammo(ecs: &mut World, invoker: Entity, skill: &SkillInfo) {
    match &skill.ammo_info {
        Some(ammo_info) => {
            let kind = ammo_info.kind;
            let current_ammo = { ecs.read_storage::<SkillResourceComponent>().grab(invoker).ammo[&kind] };
            set_ammo(ecs, invoker, kind, current_ammo - ammo_info.usage);
        }
        None => {}
    }
}

pub fn reload_core(ecs: &mut World, invoker: Entity, kind: AmmoKind, amount: u32) {
    let (current, max) = {
        let skill_resources = ecs.read_storage::<SkillResourceComponent>();
        let resources = skill_resources.grab(invoker);
        (resources.ammo[&kind], resources.max[&kind])
    };
    let total = amount + current;
    let amount = cmp::min(total, max);
    set_ammo(ecs, invoker, kind, amount);
}

pub fn reload(ecs: &mut World, invoker: Entity, kind: AmmoKind, amount: Option<u32>) {
    let amount = amount.unwrap_or_else(|| ecs.read_storage::<SkillResourceComponent>().grab(invoker).max[&kind]);
    reload_core(ecs, invoker, kind, amount);
}

pub fn reload_random(ecs: &mut World, invoker: Entity, kind: AmmoKind, amount: u32) {
    let amount = ecs.fetch_mut::<RandomComponent>().rand.gen_range(2, amount);
    let amount = cmp::min(amount, ecs.read_storage::<SkillResourceComponent>().grab(invoker).max[&kind]);
    reload_core(ecs, invoker, kind, amount);
}

fn add_ticks_for_skill(skill: &mut SkillResourceComponent, ticks_to_add: i32) {
    let exhaustion_to_remove = EXHAUSTION_PER_100_TICKS as f64 * (ticks_to_add as f64 / 100.0);

    let focus_to_add = FOCUS_PER_100_TICKS as f64 * (ticks_to_add as f64 / 100.0);
    // Ordering f64 is hard _tm_
    skill.exhaustion = *cmp::max(NotNan::new(0.0).unwrap(), NotNan::new(skill.exhaustion - exhaustion_to_remove).unwrap());
    skill.focus = *cmp::min(NotNan::new(skill.max_focus).unwrap(), NotNan::new(skill.focus + focus_to_add).unwrap());

    for (_, cooldown) in skill.cooldown.iter_mut() {
        *cooldown = cmp::max(*cooldown as i32 - ticks_to_add as i32, 0) as u32;
    }
}

pub fn tick_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Tick(ticks) => {
            let mut skills = ecs.write_storage::<SkillResourceComponent>();
            if let Some(skill_resource) = skills.get_mut(target.unwrap()) {
                add_ticks_for_skill(skill_resource, ticks);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::{add_ticks, create_test_state, find_at, find_first_entity, get_ticks, wait_for_animations};
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    #[should_panic]
    fn panic_if_wrong_targeting() {
        let mut ecs = create_test_state().with_timed(100).build();
        let entity = find_first_entity(&ecs);
        invoke_skill(&mut ecs, entity, "TestNone", Some(Point::init(2, 2)));
    }

    #[test]
    #[should_panic]
    fn panic_if_missing_targeting() {
        let mut ecs = create_test_state().with_timed(100).build();
        let entity = find_first_entity(&ecs);
        invoke_skill(&mut ecs, entity, "TestTile", None);
    }

    #[test]
    fn invoker_spend_time() {
        let mut ecs = create_test_state().with_timed(100).build();
        let entity = find_first_entity(&ecs);
        invoke_skill(&mut ecs, entity, "TestNone", None);
        assert_eq!(0, get_ticks(&ecs, entity));
    }

    #[test]
    #[should_panic]
    fn target_must_be_in_range() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let entity = find_at(&ecs, 2, 2);
        invoke_skill(&mut ecs, entity, "TestWithRange", Some(Point::init(2, 5)));
    }

    #[test]
    fn target_in_range() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);
        invoke_skill(&mut ecs, entity, "TestWithRange", Some(Point::init(2, 4)));
    }

    #[test]
    fn skill_info_range() {
        let ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = get_skill("TestWithRange");
        assert_eq!(true, is_good_target(&ecs, entity, &info, Point::init(2, 4)));
        assert_eq!(false, is_good_target(&ecs, entity, &info, Point::init(2, 5)));
        let info = SkillInfo::init("TestInfo", None, TargetType::Tile, SkillEffect::None);
        assert_eq!(true, is_good_target(&ecs, entity, &info, Point::init(2, 5)));
    }

    #[test]
    fn skill_info_correct_target_kind() {
        let ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = get_skill("TestWithRange");
        assert_eq!(true, is_good_target(&ecs, entity, &info, Point::init(2, 4)));
        assert_eq!(false, is_good_target(&ecs, entity, &info, Point::init(2, 3)));
    }

    #[test]
    fn skill_info_must_be_clear() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = SkillInfo::init_with_distance("TestInfo", None, TargetType::Tile, SkillEffect::None, Some(2), true);
        assert_eq!(true, is_good_target(&ecs, entity, &info, Point::init(2, 4)));
        make_test_character(&mut ecs, SizedPoint::init(2, 3), 0);

        assert_eq!(false, is_good_target(&ecs, entity, &info, Point::init(2, 4)));
    }

    #[test]
    fn skill_info_any_target() {
        let ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = SkillInfo::init_with_distance("TestInfo", None, TargetType::Any, SkillEffect::None, Some(2), false);
        assert!(is_good_target(&ecs, entity, &info, Point::init(2, 2)));
        assert!(is_good_target(&ecs, entity, &info, Point::init(2, 3)));
        assert!(is_good_target(&ecs, entity, &info, Point::init(2, 4)));
    }

    #[test]
    fn skill_info_any_but_self_target() {
        let ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = SkillInfo::init_with_distance("TestInfo", None, TargetType::AnyoneButSelf, SkillEffect::None, Some(2), false);
        assert_eq!(false, is_good_target(&ecs, entity, &info, Point::init(2, 2)));
        assert_eq!(true, is_good_target(&ecs, entity, &info, Point::init(2, 3)));
        assert_eq!(true, is_good_target(&ecs, entity, &info, Point::init(2, 4)));
    }

    #[test]
    fn movement_effect() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, entity, "TestMove", Some(Point::init(3, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(Point::init(3, 3), ecs.get_position(entity).origin);
    }

    #[test]
    fn movement_effect_multi() {
        let mut ecs = create_test_state()
            .with_sized_character(SizedPoint::init_multi(2, 2, 2, 1), 100)
            .with_map()
            .build();
        let entity = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, entity, "TestMove", Some(Point::init(3, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(Point::init(3, 3), ecs.get_position(entity).origin);
    }

    #[test]
    fn ranged_effect() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(4, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 4, 2);
        let starting_health = ecs.get_defenses(target).health;

        invoke_skill(&mut ecs, player, "TestRanged", Some(Point::init(4, 2)));
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(target).health < starting_health);
    }

    #[test]
    fn ranged_effect_multi() {
        let mut ecs = create_test_state()
            .with_character(2, 2, 100)
            .with_sized_character(SizedPoint::init_multi(2, 5, 2, 2), 100)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 5);
        let starting_health = ecs.get_defenses(target).health;
        invoke_skill(&mut ecs, player, "TestRanged", Some(Point::init(2, 4)));
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(target).health < starting_health);
    }

    #[test]
    fn melee_effect() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);
        let starting_health = ecs.get_defenses(target).health;

        invoke_skill(&mut ecs, player, "TestMelee", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(target).health < starting_health);
    }

    #[test]
    fn melee_effect_multi() {
        let mut ecs = create_test_state()
            .with_character(2, 2, 100)
            .with_sized_character(SizedPoint::init_multi(2, 4, 2, 2), 100)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 4);
        let starting_health = ecs.get_defenses(target).health;

        invoke_skill(&mut ecs, player, "TestMelee", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(target).health < starting_health);
    }

    fn add_bullets(ecs: &mut World, player: Entity, count: u32) {
        let resource = SkillResourceComponent::init(&[(AmmoKind::Bullets, count, count)]);
        ecs.shovel(player, resource);
    }

    #[test]
    fn get_remaining_usages_with_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        add_bullets(&mut ecs, player, 3);

        assert_eq!(3, get_skill("TestAmmo").get_remaining_usages(&ecs, player).unwrap());
    }

    #[test]
    fn get_remaining_usages_zero_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        add_bullets(&mut ecs, player, 0);

        assert_eq!(0, get_skill("TestAmmo").get_remaining_usages(&ecs, player).unwrap());
    }

    #[test]
    fn get_remaining_usages_non_existent_ammo() {
        let ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);

        assert_eq!(0, get_skill("TestAmmo").get_remaining_usages(&ecs, player).unwrap());
    }

    #[test]
    fn get_remaining_usages_skill_uses_no_ammo() {
        let ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);

        assert_eq!(true, get_skill("TestMelee").get_remaining_usages(&ecs, player).is_none());
    }

    #[test]
    fn skills_with_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, player, 3);

        let skill = get_skill("TestAmmo");

        for _ in 0..3 {
            assert_eq!(true, can_invoke_skill(&mut ecs, player, &skill, None));
            invoke_skill(&mut ecs, player, "TestAmmo", None);
            add_ticks(&mut ecs, 100);
        }

        assert_eq!(false, can_invoke_skill(&mut ecs, player, &skill, None));
    }

    #[test]
    fn skills_with_multiple_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, player, 6);

        invoke_skill(&mut ecs, player, "TestMultiAmmo", None);
        assert_eq!(1, get_skill("TestMultiAmmo").get_remaining_usages(&ecs, player).unwrap());
    }

    #[test]
    fn reload_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, player, 3);

        for _ in 0..3 {
            invoke_skill(&mut ecs, player, "TestAmmo", None);
            add_ticks(&mut ecs, 100);
        }

        invoke_skill(&mut ecs, player, "TestReload", None);
        assert_eq!(3, get_skill("TestAmmo").get_remaining_usages(&ecs, player).unwrap());
        assert_eq!(0, get_ticks(&ecs, player));
    }

    #[test]
    fn reload_some_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, player, 3);

        for _ in 0..3 {
            invoke_skill(&mut ecs, player, "TestAmmo", None);
            add_ticks(&mut ecs, 100);
        }

        invoke_skill(&mut ecs, player, "TestReloadOne", None);
        add_ticks(&mut ecs, 100);
        invoke_skill(&mut ecs, player, "TestReloadOne", None);
        assert_eq!(2, get_skill("TestAmmo").get_remaining_usages(&ecs, player).unwrap());
        assert_eq!(0, get_ticks(&ecs, player));
    }

    #[test]
    fn reload_some_random_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, player, 3);

        for _ in 0..3 {
            invoke_skill(&mut ecs, player, "TestAmmo", None);
            add_ticks(&mut ecs, 100);
        }

        invoke_skill(&mut ecs, player, "TestReloadSomeRandom", None);
        let remaining = get_skill("TestAmmo").get_remaining_usages(&ecs, player).unwrap();
        assert!(remaining > 0);
        assert!(remaining <= 3);
        assert_eq!(0, get_ticks(&ecs, player));
    }

    #[test]
    fn get_exhaustion_usage() {
        let ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        assert_eq!(2, get_skill("TestExhaustion").get_remaining_usages(&ecs, player).unwrap());
    }

    #[test]
    fn get_multiple_usage() {
        let mut ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        add_bullets(&mut ecs, player, 3);
        assert_eq!(3, get_skill("TestMultiple").get_remaining_usages(&ecs, player).unwrap());
    }

    #[test]
    fn skills_with_exhaustion_up_to_max() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, player, 3);

        for _ in 0..2 {
            invoke_skill(&mut ecs, player, "TestExhaustion", None);
            add_ticks(&mut ecs, 100);
        }

        assert_eq!(0, get_skill("TestExhaustion").get_remaining_usages(&ecs, player).unwrap());
        for _ in 0..10 {
            add_ticks(&mut ecs, 100);
        }
        assert_eq!(true, get_skill("TestExhaustion").get_remaining_usages(&ecs, player).unwrap() > 0);
    }

    fn add_focus(ecs: &mut World, player: Entity, focus: f64) {
        ecs.shovel(player, SkillResourceComponent::init(&[]).with_focus(focus));
    }

    #[test]
    fn get_focus_usage() {
        let mut ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        add_focus(&mut ecs, player, 1.0);
        assert_eq!(2, get_skill("TestFocus").get_remaining_usages(&ecs, player).unwrap());
    }

    #[test]
    fn skills_with_focus_up_to_max() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_focus(&mut ecs, player, 1.0);

        for _ in 0..2 {
            invoke_skill(&mut ecs, player, "TestFocus", None);
            add_ticks(&mut ecs, 100);
        }

        assert_eq!(0, get_skill("TestFocus").get_remaining_usages(&ecs, player).unwrap());
        for _ in 0..10 {
            add_ticks(&mut ecs, 100);
        }
        assert_eq!(true, get_skill("TestFocus").get_remaining_usages(&ecs, player).unwrap() > 0);
    }

    #[test]
    fn skill_with_field_without_position() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        // Some conditions, like flying can remove position temporarly. They should still be able to make fields
        ecs.write_storage::<PositionComponent>().remove(player);

        invoke_skill(&mut ecs, player, "TestField", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);
    }

    #[test]
    fn skill_with_field_explodes() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let other = find_at(&ecs, 2, 3);
        ecs.shovel(other, BehaviorComponent::init(BehaviorKind::None));
        let starting_health = ecs.get_defenses(other).health;
        invoke_skill(&mut ecs, player, "TestField", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        add_ticks(&mut ecs, 100);
        wait(&mut ecs, player);
        tick_next_action(&mut ecs);
        wait_for_animations(&mut ecs);

        add_ticks(&mut ecs, 100);
        wait(&mut ecs, player);
        tick_next_action(&mut ecs);
        wait_for_animations(&mut ecs);
        assert!(ecs.get_defenses(other).health < starting_health);
    }

    #[test]
    fn skill_with_large_field_explodes() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 5, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let other = find_at(&ecs, 2, 5);
        ecs.shovel(other, BehaviorComponent::init(BehaviorKind::None));
        let starting_health = ecs.get_defenses(other).health;
        invoke_skill(&mut ecs, player, "TestLargeField", Some(Point::init(2, 4)));
        wait_for_animations(&mut ecs);
        assert_field_exists(&ecs, 2, 5);

        for _ in 0..2 {
            add_ticks(&mut ecs, 100);
            wait(&mut ecs, player);
            tick_next_action(&mut ecs);
            wait_for_animations(&mut ecs);
        }

        assert!(ecs.get_defenses(other).health < starting_health);
    }

    #[test]
    fn dodge_restored_by_skill_movement() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);
        let mut defenses = Defenses::just_health(10);
        defenses.max_dodge = 5;
        ecs.write_storage::<CharacterInfoComponent>().grab_mut(entity).character.defenses = defenses;

        invoke_skill(&mut ecs, entity, "TestMove", Some(Point::init(3, 3)));
        wait_for_animations(&mut ecs);

        let dodge = ecs.read_storage::<CharacterInfoComponent>().grab(entity).character.defenses.dodge;
        assert_eq!(4, dodge);
    }

    #[test]
    fn exhaustion_reduced_by_time() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        ecs.write_storage::<SkillResourceComponent>().grab_mut(player).exhaustion = 50.0;
        // This works as long as there is no rounding, as 20 *(5 *.5) = 50.0
        for _ in 0..20 {
            add_ticks(&mut ecs, 50);
        }

        {
            let skills = ecs.read_storage::<SkillResourceComponent>();
            assert_approx_eq!(skills.grab(player).exhaustion, 0.0);
        }

        // Keep going, make sure it doesn't drop below zero
        for _ in 0..10 {
            add_ticks(&mut ecs, 100);
        }

        {
            let skills = ecs.read_storage::<SkillResourceComponent>();
            assert_approx_eq!(skills.grab(player).exhaustion, 0.0);
        }
    }

    #[test]
    fn focus_restored_by_time() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        ecs.write_storage::<SkillResourceComponent>().grab_mut(player).focus = 0.0;
        ecs.write_storage::<SkillResourceComponent>().grab_mut(player).max_focus = 1.0;
        for _ in 0..20 {
            add_ticks(&mut ecs, 50);
        }

        {
            let skills = ecs.read_storage::<SkillResourceComponent>();
            assert_approx_eq!(skills.grab(player).focus, 1.0);
        }

        // Keep going, make sure it doesn't go above max
        for _ in 0..10 {
            add_ticks(&mut ecs, 100);
        }

        {
            let skills = ecs.read_storage::<SkillResourceComponent>();
            assert_approx_eq!(skills.grab(player).focus, 1.0);
        }
    }

    #[test]
    fn move_and_shoot() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 100)
            .with_character(2, 3, 0)
            .with_character(2, 4, 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);
        let other = find_at(&ecs, 2, 4);
        let starting_health = ecs.get_defenses(target).health;

        invoke_skill(&mut ecs, player, "TestMoveAndShoot", Some(Point::init(2, 1)));
        wait_for_animations(&mut ecs);
        assert_position(&ecs, player, Point::init(2, 1));
        assert!(ecs.get_defenses(target).health < starting_health);
        assert_eq!(ecs.get_defenses(other).health, starting_health);
    }

    #[test]
    fn move_and_shoot_out_of_range() {
        let mut ecs = create_test_state()
            .with_player(2, 0, 100)
            .with_character(2, 7, 0)
            .with_character(2, 8, 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 0);
        let target = find_at(&ecs, 2, 7);
        let other = find_at(&ecs, 2, 8);
        let starting_health = ecs.get_defenses(target).health;

        invoke_skill(&mut ecs, player, "TestMoveAndShoot", Some(Point::init(2, 1)));
        wait_for_animations(&mut ecs);
        assert_position(&ecs, player, Point::init(2, 1));
        assert_eq!(ecs.get_defenses(target).health, starting_health);
        assert_eq!(ecs.get_defenses(other).health, starting_health);
    }

    #[test]
    fn gain_adrenaline_when_has_resource() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        {
            let resource = SkillResourceComponent::init(&[(AmmoKind::Adrenaline, 0, 100)]);
            ecs.shovel(player, resource);
        }

        invoke_skill(&mut ecs, player, "TestMoveAndShoot", Some(Point::init(2, 1)));
        wait_for_animations(&mut ecs);

        assert!(ecs.read_storage::<SkillResourceComponent>().grab(player).ammo[&AmmoKind::Adrenaline] > 0);
    }

    #[test]
    fn buff() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, player, "Buff", None);
        wait_for_animations(&mut ecs);

        assert!(ecs.has_status(player, StatusKind::Aimed));
    }

    #[test]
    fn buff_others_same_size() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 100)
            .with_character(2, 3, 100)
            .with_character(2, 4, 100)
            .with_map()
            .build();
        let caster = find_at(&ecs, 2, 3);
        let target = find_at(&ecs, 2, 4);

        invoke_skill(&mut ecs, caster, "BuffOthers", Some(Point::init(2, 4)));
        wait_for_animations(&mut ecs);

        assert!(ecs.has_status(target, StatusKind::Aimed));
    }

    #[test]
    fn buff_self_and_attack_still_buffs_caster() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, player, "BuffAndSwing", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        assert!(ecs.has_status(player, StatusKind::Armored));
    }

    #[test]
    fn buff_then_move() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, player, "BuffAndMove", Some(Point::init(2, 1)));
        wait_for_animations(&mut ecs);

        assert!(ecs.has_status(player, StatusKind::Aimed));
        assert_position(&ecs, player, Point::init(2, 1));
    }

    #[test]
    fn shoot_and_buff() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);

        invoke_skill(&mut ecs, player, "ShootThenBuff", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        let health = ecs.get_defenses(target);
        assert_ne!(health.health, health.max_health);
        assert!(ecs.has_status(player, StatusKind::Aimed));
    }

    #[test]
    fn no_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, player, "TestNoTime", None);
        wait_for_animations(&mut ecs);
        assert_eq!(100, get_ticks(&ecs, player));
    }

    #[test]
    fn spawn_add() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        assert_eq!(1, find_all_characters(&ecs).len());
        invoke_skill(&mut ecs, player, "TestSpawn", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);
        assert_eq!(2, find_all_characters(&ecs).len());
    }

    #[test]
    fn spawn_add_via_field() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        assert_eq!(1, find_all_characters(&ecs).len());
        invoke_skill(&mut ecs, player, "TestSpawnField", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);
        assert_eq!(2, find_all_characters(&ecs).len());
    }

    #[test]
    fn spawn_add_via_field_collision() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 4, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let bystander = find_at(&ecs, 2, 4);

        assert_eq!(2, find_all_characters(&ecs).len());
        invoke_skill(&mut ecs, player, "TestSpawnField", Some(Point::init(2, 3)));
        begin_move(&mut ecs, bystander, SizedPoint::init(2, 3), PostMoveAction::None);
        wait_for_animations(&mut ecs);
        assert_eq!(3, find_all_characters(&ecs).len());
        assert_eq!(
            1,
            find_all_characters(&ecs)
                .iter()
                .filter(|&&x| ecs.get_position(x).origin == Point::init(2, 3))
                .count()
        );
    }

    #[test]
    fn spawn_replace() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let health = ecs.get_defenses(player).health;

        assert_eq!(1, find_all_characters(&ecs).len());
        invoke_skill(&mut ecs, player, "TestReplaceSpawn", None);
        wait_for_animations(&mut ecs);
        assert_eq!(1, find_all_characters(&ecs).len());

        let bird = find_at(&ecs, 2, 2);
        assert_ne!(health, ecs.get_defenses(bird).health);
    }

    #[test]
    fn skill_power() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(4, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        ecs.write_storage::<CharacterInfoComponent>().grab_mut(player).character.skill_power = 1;

        let target = find_at(&ecs, 4, 2);
        let starting_health = ecs.get_defenses(target).health;

        // This does no damage unless you add skill_power
        invoke_skill(&mut ecs, player, "TestTap", Some(Point::init(4, 2)));
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(target).health < starting_health);
    }

    #[test]
    fn charge_moves_and_swings() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(2, 5, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 5);
        let starting_health = ecs.get_defenses(target).health;

        invoke_skill(&mut ecs, player, "TestCharge", Some(Point::init(2, 5)));
        wait_for_animations(&mut ecs);

        assert_position(&ecs, player, Point::init(2, 4));
        assert!(ecs.get_defenses(target).health < starting_health);
    }

    #[test]
    fn charge_only_moves_no_target() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, player, "TestCharge", Some(Point::init(2, 5)));
        wait_for_animations(&mut ecs);
        assert_position(&ecs, player, Point::init(2, 5));
    }

    #[test]
    fn skill_with_cooldown_starts_usable() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        assert!(can_invoke_skill(&mut ecs, player, get_skill("TestCooldown"), None));
        invoke_skill(&mut ecs, player, "TestCooldown", None);
        assert!(!can_invoke_skill(&mut ecs, player, get_skill("TestCooldown"), None));
        add_ticks(&mut ecs, 100);
        add_ticks(&mut ecs, 100);
        assert!(can_invoke_skill(&mut ecs, player, get_skill("TestCooldown"), None));
    }

    #[test]
    fn skill_with_cooldown_starts_usable_unless_first_check() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        assert!(!can_invoke_skill(&mut ecs, player, get_skill("TestCooldownStartSpent"), None));
        add_ticks(&mut ecs, 100);
        add_ticks(&mut ecs, 100);
        assert!(can_invoke_skill(&mut ecs, player, get_skill("TestCooldownStartSpent"), None));
        invoke_skill(&mut ecs, player, "TestCooldownStartSpent", None);
        assert!(!can_invoke_skill(&mut ecs, player, get_skill("TestCooldownStartSpent"), None));
    }

    #[test]
    fn skill_sequence() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, player, "TestSequence", None);
        assert!(ecs.has_status(player, StatusKind::Armored));
        assert!(ecs.has_status(player, StatusKind::Aimed));
    }
}
