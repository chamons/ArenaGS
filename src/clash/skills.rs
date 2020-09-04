use std::cmp;
use std::collections::HashMap;
use std::slice::from_ref;

use enum_iterator::IntoEnumIterator;
use lazy_static::lazy_static;
use ordered_float::*;
use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::*;
use crate::atlas::{EasyECS, EasyMutECS, Point};

#[allow(dead_code)]
#[derive(is_enum_variant, Clone, Copy)]
pub enum TargetType {
    None,
    Tile,
    Player,
    Enemy,
    Any,
}

#[allow(dead_code)]
pub enum SkillEffect {
    None,
    Move,
    RangedAttack(Damage, BoltKind),
    MeleeAttack(Damage, WeaponKind),
    Reload(AmmoKind),
    FieldEffect(Damage, FieldKind),
    MoveAndShoot(Damage, Option<u32>, BoltKind),
    RotateAmmo(),
    BuffThen(StatusKind, i32, Box<SkillEffect>),
    ThenBuff(Box<SkillEffect>, StatusKind, i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, IntoEnumIterator, Debug, Deserialize, Serialize)]
pub enum AmmoKind {
    Bullets,
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
    pub image: Option<&'static str>,
    pub target: TargetType,
    pub effect: SkillEffect,
    pub range: Option<u32>,
    pub must_be_clear: bool,
    pub ammo_info: Option<AmmoInfo>,
    pub alternate: Option<String>,
    pub exhaustion: Option<f64>,
    pub focus_use: Option<f64>,
    pub no_time: bool,
}

#[allow(dead_code)]
impl SkillInfo {
    pub fn init(image: Option<&'static str>, target: TargetType, effect: SkillEffect) -> SkillInfo {
        SkillInfo {
            image,
            target,
            effect,
            range: None,
            must_be_clear: false,
            ammo_info: None,
            alternate: None,
            exhaustion: None,
            focus_use: None,
            no_time: false,
        }
    }

    pub fn init_with_distance(image: Option<&'static str>, target: TargetType, effect: SkillEffect, range: Option<u32>, must_be_clear: bool) -> SkillInfo {
        SkillInfo {
            image,
            target,
            effect,
            range,
            must_be_clear,
            ammo_info: None,
            alternate: None,
            exhaustion: None,
            focus_use: None,
            no_time: false,
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

    pub fn show_trail(&self) -> bool {
        self.must_be_clear
    }

    fn get_ammo_usage(&self, ecs: &World, entity: &Entity) -> Option<u32> {
        match &self.ammo_info {
            Some(ammo_info) => {
                let skill_resources = ecs.read_storage::<SkillResourceComponent>();
                let current_state = &skill_resources.grab(*entity).ammo;
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

    fn get_focus_usage(&self, ecs: &World, entity: &Entity) -> Option<u32> {
        match &self.focus_use {
            Some(usage) => {
                let current = ecs.read_storage::<SkillResourceComponent>().grab(*entity).focus;
                Some((current / usage).floor() as u32)
            }
            None => None,
        }
    }

    fn get_exhaustion_usage(&self, ecs: &World, entity: &Entity) -> Option<u32> {
        match &self.exhaustion {
            Some(usage) => {
                let current = ecs.read_storage::<SkillResourceComponent>().grab(*entity).exhaustion;
                let remaining = MAX_EXHAUSTION - current;
                Some((remaining / usage).floor() as u32)
            }
            None => None,
        }
    }

    pub fn get_remaining_usages(&self, ecs: &World, entity: &Entity) -> Option<u32> {
        let usages = vec![
            self.get_ammo_usage(ecs, entity),
            self.get_exhaustion_usage(ecs, entity),
            self.get_focus_usage(ecs, entity),
        ];
        usages.iter().filter_map(|x| *x).min()
    }

    pub fn is_usable(&self, ecs: &World, entity: &Entity) -> UsableResults {
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
}

pub enum UsableResults {
    Usable,
    LacksAmmo,
    Exhaustion,
    LacksFocus,
}

lazy_static! {
    static ref SKILLS: HashMap<&'static str, SkillInfo> = {
        let mut m = HashMap::new();

        #[cfg(test)]
        super::content::test::add_test_skills(&mut m);

        super::content::gunslinger::gunslinger_skills(&mut m);
        super::content::bird::bird_skills(&mut m);

        m.insert(
            "Dash",
            SkillInfo::init_with_distance(Some("SpellBookPage09_39.png"), TargetType::Tile, SkillEffect::Move, Some(3), true).with_exhaustion(50.0),
        );

        m
    };
}

pub fn get_skill(name: &str) -> &'static SkillInfo {
    &SKILLS[name]
}

pub fn all_skill_image_filesnames() -> Vec<&'static str> {
    SKILLS.values().filter_map(|s| s.image).collect()
}

fn assert_correct_targeting(ecs: &mut World, invoker: &Entity, name: &str, target: Option<Point>) {
    let skill = get_skill(name);

    let requires_point = match skill.target {
        TargetType::None => false,
        TargetType::Tile | TargetType::Enemy | TargetType::Player | TargetType::Any => true,
    };

    if requires_point != target.is_some() {
        panic!("invoke_skill for {} called with wrong targeting param state.", name);
    }

    if let Some(target) = target {
        assert!(is_good_target(ecs, invoker, &skill, target));
    }
}

pub fn is_good_target(ecs: &World, invoker: &Entity, skill: &SkillInfo, target: Point) -> bool {
    let initial = ecs.get_position(invoker);

    if !match skill.target {
        TargetType::Tile => is_area_clear(ecs, from_ref(&target), invoker),
        TargetType::Enemy => !is_area_clear(ecs, from_ref(&target), invoker),
        TargetType::Player => ecs.get_position(&find_player(ecs)).contains_point(&target),
        TargetType::Any => !initial.contains_point(&target),
        TargetType::None => false,
    } {
        return false;
    }

    if let Some(skill_range) = skill.range {
        if let Some(range_to_target) = initial.distance_to(target) {
            if range_to_target > skill_range {
                return false;
            }
        }
    }

    if skill.must_be_clear {
        if let Some(mut path) = initial.line_to(target) {
            // If we are targeting an enemy we can safely
            // ignore the last square, since we know that it must
            // have the target (from checks above)
            if skill.target.is_enemy() {
                path.pop();
            }
            if !is_area_clear(ecs, &path, invoker) {
                return false;
            }
        }
    }
    true
}

pub fn can_invoke_skill(ecs: &mut World, invoker: &Entity, skill: &SkillInfo, target: Option<Point>) -> bool {
    let has_needed_ammo = skill.get_remaining_usages(ecs, invoker).map_or(true, |x| x > 0);
    let has_valid_target = target.map_or(true, |x| is_good_target(ecs, invoker, skill, x));

    has_needed_ammo && has_valid_target
}

pub fn invoke_skill(ecs: &mut World, invoker: &Entity, name: &str, target: Option<Point>) {
    assert_correct_targeting(ecs, invoker, name, target);
    let skill = get_skill(name);
    assert!(can_invoke_skill(ecs, invoker, skill, target));

    if let Some(player_name) = ecs.get_name(&invoker) {
        ecs.log(format!("{} used {}.", player_name.as_str(), name));
    }

    process_skill(ecs, invoker, &skill.effect, target);

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

    gain_adrenaline(ecs, invoker, skill);
}

fn process_skill(ecs: &mut World, invoker: &Entity, effect: &SkillEffect, target: Option<Point>) {
    match effect {
        SkillEffect::Move => {
            // Targeting only gives us a point, so clone their position to get size as well
            let position = ecs.get_position(invoker).move_to(target.unwrap());
            begin_move(ecs, invoker, position, PostMoveAction::None);
        }
        SkillEffect::MoveAndShoot(damage, range, kind) => {
            // Targeting only gives us a point, so clone their position to get size as well
            let position = ecs.get_position(invoker).move_to(target.unwrap());
            begin_shoot_and_move(ecs, invoker, position, *range, *damage, *kind)
        }
        SkillEffect::RangedAttack(damage, kind) => begin_bolt(ecs, &invoker, target.unwrap(), *damage, *kind),
        SkillEffect::MeleeAttack(damage, kind) => begin_melee(ecs, &invoker, target.unwrap(), *damage, *kind),
        SkillEffect::Reload(kind) => reload(ecs, &invoker, *kind),
        SkillEffect::FieldEffect(damage, kind) => begin_field(ecs, &invoker, target.unwrap(), *damage, *kind),
        SkillEffect::RotateAmmo() => content::gunslinger::rotate_ammo(ecs, &invoker),
        SkillEffect::BuffThen(buff, duration, next_skill) => {
            ecs.add_status(invoker, *buff, *duration);
            process_skill(ecs, invoker, next_skill, target);
        }
        SkillEffect::ThenBuff(next_skill, buff, duration) => {
            process_skill(ecs, invoker, next_skill, target);
            ecs.add_status(invoker, *buff, *duration);
        }
        SkillEffect::None => {}
    }
}

fn gain_adrenaline(ecs: &mut World, invoker: &Entity, skill: &SkillInfo) {
    let amount = match &skill.effect {
        SkillEffect::Move => 1,
        SkillEffect::MoveAndShoot(_, _, _) => 3,
        SkillEffect::RangedAttack(_, _) => 3,
        SkillEffect::MeleeAttack(_, _) => 3,
        SkillEffect::Reload(_) => 2,
        SkillEffect::FieldEffect(_, _) => 2,
        SkillEffect::RotateAmmo() => 1,
        SkillEffect::None => 0,
        SkillEffect::BuffThen(_, _, _) => 1,
        SkillEffect::ThenBuff(_, _, _) => 1,
    };

    let mut skill_resources = ecs.write_storage::<SkillResourceComponent>();
    if let Some(resources) = &mut skill_resources.get_mut(*invoker) {
        if resources.ammo.contains_key(&AmmoKind::Adrenaline) {
            let new_total = cmp::min(resources.ammo[&AmmoKind::Adrenaline] + amount, resources.max[&AmmoKind::Adrenaline]);
            *resources.ammo.get_mut(&AmmoKind::Adrenaline).unwrap() = new_total;
        }
    }
}

fn set_ammo(ecs: &mut World, invoker: &Entity, kind: AmmoKind, amount: u32) {
    let mut skill_resources = ecs.write_storage::<SkillResourceComponent>();
    *skill_resources.grab_mut(*invoker).ammo.get_mut(&kind).unwrap() = amount;
}

fn spend_ammo(ecs: &mut World, invoker: &Entity, skill: &SkillInfo) {
    match &skill.ammo_info {
        Some(ammo_info) => {
            let kind = ammo_info.kind;
            let current_ammo = { ecs.read_storage::<SkillResourceComponent>().grab(*invoker).ammo[&kind] };
            set_ammo(ecs, invoker, kind, current_ammo - ammo_info.usage);
        }
        None => {}
    }
}

fn reload(ecs: &mut World, invoker: &Entity, kind: AmmoKind) {
    let max_ammo = { ecs.read_storage::<SkillResourceComponent>().grab(*invoker).max[&kind] };
    set_ammo(ecs, invoker, kind, max_ammo);
}

fn add_ticks_for_skill(skill: &mut SkillResourceComponent, ticks_to_add: i32) {
    let exhaustion_to_remove = EXHAUSTION_PER_100_TICKS as f64 * (ticks_to_add as f64 / 100.0);

    let focus_to_add = FOCUS_PER_100_TICKS as f64 * (ticks_to_add as f64 / 100.0);
    // Ordering f64 is hard _tm_
    skill.exhaustion = *cmp::max(NotNan::new(0.0).unwrap(), NotNan::new(skill.exhaustion - exhaustion_to_remove).unwrap());
    skill.focus = *cmp::min(NotNan::new(skill.max_focus).unwrap(), NotNan::new(skill.focus + focus_to_add).unwrap());
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
    use crate::atlas::{EasyMutWorld, SizedPoint};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    #[should_panic]
    fn panic_if_wrong_targeting() {
        let mut ecs = create_test_state().with_timed(100).build();
        let entity = find_first_entity(&ecs);
        invoke_skill(&mut ecs, &entity, "TestNone", Some(Point::init(2, 2)));
    }

    #[test]
    #[should_panic]
    fn panic_if_missing_targeting() {
        let mut ecs = create_test_state().with_timed(100).build();
        let entity = find_first_entity(&ecs);
        invoke_skill(&mut ecs, &entity, "TestTile", None);
    }

    #[test]
    fn invoker_spend_time() {
        let mut ecs = create_test_state().with_timed(100).build();
        let entity = find_first_entity(&ecs);
        invoke_skill(&mut ecs, &entity, "TestNone", None);
        assert_eq!(0, get_ticks(&ecs, &entity));
    }

    #[test]
    #[should_panic]
    fn target_must_be_in_range() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let entity = find_at(&ecs, 2, 2);
        invoke_skill(&mut ecs, &entity, "TestWithRange", Some(Point::init(2, 5)));
    }

    #[test]
    fn target_in_range() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);
        invoke_skill(&mut ecs, &entity, "TestWithRange", Some(Point::init(2, 4)));
    }

    #[test]
    fn skill_info_range() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = get_skill("TestWithRange");
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
        assert_eq!(false, is_good_target(&mut ecs, &entity, &info, Point::init(2, 5)));
        let info = SkillInfo::init(None, TargetType::Tile, SkillEffect::None);
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 5)));
    }

    #[test]
    fn skill_info_correct_target_kind() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = get_skill("TestWithRange");
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
        assert_eq!(false, is_good_target(&mut ecs, &entity, &info, Point::init(2, 3)));
    }

    #[test]
    fn skill_info_must_be_clear() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = SkillInfo::init_with_distance(None, TargetType::Tile, SkillEffect::None, Some(2), true);
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
        make_test_character(&mut ecs, SizedPoint::init(2, 3), 0);

        assert_eq!(false, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
    }

    #[test]
    fn skill_info_any_target() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = SkillInfo::init_with_distance(None, TargetType::Any, SkillEffect::None, Some(2), false);
        assert_eq!(false, is_good_target(&mut ecs, &entity, &info, Point::init(2, 2)));
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 3)));
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
    }

    #[test]
    fn movement_effect() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, &entity, "TestMove", Some(Point::init(3, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(Point::init(3, 3), ecs.get_position(&entity).origin);
    }

    #[test]
    fn movement_effect_multi() {
        let mut ecs = create_test_state()
            .with_sized_character(SizedPoint::init_multi(2, 2, 2, 1), 100)
            .with_map()
            .build();
        let entity = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, &entity, "TestMove", Some(Point::init(3, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(Point::init(3, 3), ecs.get_position(&entity).origin);
    }

    #[test]
    fn ranged_effect() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(4, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 4, 2);
        let starting_health = ecs.get_defenses(&target).health;

        invoke_skill(&mut ecs, &player, "TestRanged", Some(Point::init(4, 2)));
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(&target).health < starting_health);
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
        let starting_health = ecs.get_defenses(&target).health;
        invoke_skill(&mut ecs, &player, "TestRanged", Some(Point::init(2, 4)));
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(&target).health < starting_health);
    }

    #[test]
    fn melee_effect() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);
        let starting_health = ecs.get_defenses(&target).health;

        invoke_skill(&mut ecs, &player, "TestMelee", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(&target).health < starting_health);
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
        let starting_health = ecs.get_defenses(&target).health;

        invoke_skill(&mut ecs, &player, "TestMelee", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(&target).health < starting_health);
    }

    fn add_bullets(ecs: &mut World, player: &Entity, count: u32) {
        let resource = SkillResourceComponent::init(&[(AmmoKind::Bullets, count, count)]);
        ecs.shovel(*player, resource);
    }

    #[test]
    fn get_remaining_usages_with_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        add_bullets(&mut ecs, &player, 3);

        assert_eq!(3, get_skill("TestAmmo").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn get_remaining_usages_zero_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        add_bullets(&mut ecs, &player, 0);

        assert_eq!(0, get_skill("TestAmmo").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn get_remaining_usages_non_existent_ammo() {
        let ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);

        assert_eq!(0, get_skill("TestAmmo").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn get_remaining_usages_skill_uses_no_ammo() {
        let ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);

        assert_eq!(true, get_skill("TestMelee").get_remaining_usages(&ecs, &player).is_none());
    }
    #[test]
    fn skills_with_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, &player, 3);

        let skill = get_skill("TestAmmo");

        for _ in 0..3 {
            assert_eq!(true, can_invoke_skill(&mut ecs, &player, &skill, None));
            invoke_skill(&mut ecs, &player, "TestAmmo", None);
            add_ticks(&mut ecs, 100);
        }

        assert_eq!(false, can_invoke_skill(&mut ecs, &player, &skill, None));
    }

    #[test]
    fn skills_with_multiple_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, &player, 6);

        invoke_skill(&mut ecs, &player, "TestMultiAmmo", None);
        assert_eq!(1, get_skill("TestMultiAmmo").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn reload_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, &player, 3);

        for _ in 0..3 {
            invoke_skill(&mut ecs, &player, "TestAmmo", None);
            add_ticks(&mut ecs, 100);
        }

        invoke_skill(&mut ecs, &player, "TestReload", None);
        assert_eq!(3, get_skill("TestAmmo").get_remaining_usages(&ecs, &player).unwrap());
        assert_eq!(0, get_ticks(&ecs, &player));
    }

    #[test]
    fn get_exhaustion_usage() {
        let ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        assert_eq!(2, get_skill("TestExhaustion").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn get_multiple_usage() {
        let mut ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        add_bullets(&mut ecs, &player, 3);
        assert_eq!(3, get_skill("TestMultiple").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn skills_with_exhaustion_up_to_max() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_bullets(&mut ecs, &player, 3);

        for _ in 0..2 {
            invoke_skill(&mut ecs, &player, "TestExhaustion", None);
            add_ticks(&mut ecs, 100);
        }

        assert_eq!(0, get_skill("TestExhaustion").get_remaining_usages(&ecs, &player).unwrap());
        for _ in 0..10 {
            add_ticks(&mut ecs, 100);
        }
        assert_eq!(true, get_skill("TestExhaustion").get_remaining_usages(&ecs, &player).unwrap() > 0);
    }

    fn add_focus(ecs: &mut World, player: &Entity, focus: f64) {
        ecs.shovel(*player, SkillResourceComponent::init(&[]).with_focus(focus));
    }

    #[test]
    fn get_focus_usage() {
        let mut ecs = create_test_state().with_character(2, 2, 0).build();
        let player = find_first_entity(&ecs);
        add_focus(&mut ecs, &player, 1.0);
        assert_eq!(2, get_skill("TestFocus").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn skills_with_focus_up_to_max() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        add_focus(&mut ecs, &player, 1.0);

        for _ in 0..2 {
            invoke_skill(&mut ecs, &player, "TestFocus", None);
            add_ticks(&mut ecs, 100);
        }

        assert_eq!(0, get_skill("TestFocus").get_remaining_usages(&ecs, &player).unwrap());
        for _ in 0..10 {
            add_ticks(&mut ecs, 100);
        }
        assert_eq!(true, get_skill("TestFocus").get_remaining_usages(&ecs, &player).unwrap() > 0);
    }

    #[test]
    fn skill_with_field_explodes() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let other = find_at(&ecs, 2, 3);
        ecs.shovel(other, BehaviorComponent::init(BehaviorKind::None));
        let starting_health = ecs.get_defenses(&other).health;
        invoke_skill(&mut ecs, &player, "TestField", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        add_ticks(&mut ecs, 100);
        wait(&mut ecs, player);
        tick_next_action(&mut ecs);
        wait_for_animations(&mut ecs);

        add_ticks(&mut ecs, 100);
        wait(&mut ecs, player);
        tick_next_action(&mut ecs);
        wait_for_animations(&mut ecs);
        assert!(ecs.get_defenses(&other).health < starting_health);
    }

    #[test]
    fn dodge_restored_by_skill_movement() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);
        let mut defenses = Defenses::just_health(10);
        defenses.max_dodge = 5;
        ecs.write_storage::<CharacterInfoComponent>().grab_mut(entity).character.defenses = defenses;

        invoke_skill(&mut ecs, &entity, "TestMove", Some(Point::init(3, 3)));
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
        let starting_health = ecs.get_defenses(&target).health;

        invoke_skill(&mut ecs, &player, "TestMoveAndShoot", Some(Point::init(2, 1)));
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &player, Point::init(2, 1));
        assert!(ecs.get_defenses(&target).health < starting_health);
        assert_eq!(ecs.get_defenses(&other).health, starting_health);
    }

    #[test]
    fn move_and_shoot_out_of_range() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 100)
            .with_character(2, 6, 0)
            .with_character(2, 7, 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 6);
        let other = find_at(&ecs, 2, 7);
        let starting_health = ecs.get_defenses(&target).health;

        invoke_skill(&mut ecs, &player, "TestMoveAndShoot", Some(Point::init(2, 1)));
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &player, Point::init(2, 1));
        assert_eq!(ecs.get_defenses(&target).health, starting_health);
        assert_eq!(ecs.get_defenses(&other).health, starting_health);
    }

    #[test]
    fn gain_adrenaline_when_has_resource() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        {
            let resource = SkillResourceComponent::init(&[(AmmoKind::Adrenaline, 0, 100)]);
            ecs.shovel(player, resource);
        }

        invoke_skill(&mut ecs, &player, "TestMoveAndShoot", Some(Point::init(2, 1)));
        wait_for_animations(&mut ecs);

        assert!(ecs.read_storage::<SkillResourceComponent>().grab(player).ammo[&AmmoKind::Adrenaline] > 0);
    }

    #[test]
    fn buff_then_move() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, &player, "BuffAndMove", Some(Point::init(2, 1)));
        wait_for_animations(&mut ecs);

        assert!(ecs.has_status(&player, StatusKind::Aimed));
        assert_position(&ecs, &player, Point::init(2, 1));
    }

    #[test]
    fn shoot_and_buff() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);

        invoke_skill(&mut ecs, &player, "ShootThenBuff", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        let health = ecs.get_defenses(&target);
        assert_ne!(health.health, health.max_health);
        assert!(ecs.has_status(&player, StatusKind::Aimed));
    }

    #[test]
    fn no_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        invoke_skill(&mut ecs, &player, "TestNoTime", None);
        wait_for_animations(&mut ecs);
        assert_eq!(100, get_ticks(&ecs, &player));
    }
}
