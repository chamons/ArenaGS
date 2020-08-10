use std::collections::HashMap;
use std::slice::from_ref;

use enum_iterator::IntoEnumIterator;
use lazy_static::lazy_static;
use specs::prelude::*;
use specs_derive::Component;

use super::*;
use crate::atlas::{EasyECS, EasyMutECS, Point};

#[allow(dead_code)]
#[derive(is_enum_variant, Clone, Copy)]
pub enum TargetType {
    None,
    Tile,
    Enemy,
    Any,
}

#[allow(dead_code)]
pub enum SkillEffect {
    None,
    Move,
    RangedAttack(u32, BoltKind),
    MeleeAttack(u32, WeaponKind),
    Reload(AmmoKind),
    FieldEffect(u32, FieldKind),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, IntoEnumIterator, Debug)]
pub enum AmmoKind {
    Bullets,
}

pub struct AmmoInfo {
    pub kind: AmmoKind,
    pub usage: u32,
}

#[derive(Component)]
pub struct SkillResourceComponent {
    pub ammo: HashMap<AmmoKind, u32>,
    pub max: HashMap<AmmoKind, u32>,
    pub exhaustion: f64,
    pub focus: f64,
    pub max_focus: f64,
}

impl SkillResourceComponent {
    pub fn init(starting_ammo: &[(AmmoKind, u32)]) -> SkillResourceComponent {
        let ammo: HashMap<AmmoKind, u32> = starting_ammo.iter().map(|(k, a)| (*k, *a)).collect();
        SkillResourceComponent {
            max: ammo.clone(),
            ammo,
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
    pub image: &'static str,
    pub target: TargetType,
    pub effect: SkillEffect,
    pub distance: Option<u32>,
    pub must_be_clear: bool,
    pub ammo_info: Option<AmmoInfo>,
    pub alternate: Option<String>,
    pub exhaustion: Option<f64>,
    pub focus_use: Option<f64>,
}

impl SkillInfo {
    #[allow(dead_code)]
    pub fn init(image: &'static str, target: TargetType, effect: SkillEffect) -> SkillInfo {
        SkillInfo {
            image,
            target,
            effect,
            distance: None,
            must_be_clear: false,
            ammo_info: None,
            alternate: None,
            exhaustion: None,
            focus_use: None,
        }
    }

    pub fn init_with_distance(image: &'static str, target: TargetType, effect: SkillEffect, distance: Option<u32>, must_be_clear: bool) -> SkillInfo {
        SkillInfo {
            image,
            target,
            effect,
            distance,
            must_be_clear,
            ammo_info: None,
            alternate: None,
            exhaustion: None,
            focus_use: None,
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

    pub fn is_usable(&self, ecs: &World, entity: &Entity) -> bool {
        match self.get_remaining_usages(ecs, entity) {
            Some(amount) => amount > 0,
            None => true,
        }
    }
}

lazy_static! {
    static ref SKILLS: HashMap<&'static str, SkillInfo> = {
        let mut m = HashMap::new();
        #[cfg(test)]
        {
            m.insert("TestNone", SkillInfo::init("", TargetType::None, SkillEffect::None));
            m.insert("TestTile", SkillInfo::init("", TargetType::Tile, SkillEffect::None));
            m.insert("TestEnemy", SkillInfo::init("", TargetType::Enemy, SkillEffect::None));
            m.insert(
                "TestWithRange",
                SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::None, Some(2), false),
            );
            m.insert(
                "TestMove",
                SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::Move, Some(2), false),
            );
            m.insert(
                "TestRanged",
                SkillInfo::init_with_distance("", TargetType::Enemy, SkillEffect::RangedAttack(2, BoltKind::Fire), Some(2), false),
            );
            m.insert(
                "TestMelee",
                SkillInfo::init_with_distance("", TargetType::Enemy, SkillEffect::MeleeAttack(2, WeaponKind::Sword), Some(1), false),
            );
            m.insert(
                "TestAmmo",
                SkillInfo::init("", TargetType::None, SkillEffect::None).with_ammo(AmmoKind::Bullets, 1),
            );
            m.insert("TestReload", SkillInfo::init("", TargetType::None, SkillEffect::Reload(AmmoKind::Bullets)));
            m.insert("TestExhaustion", SkillInfo::init("", TargetType::None, SkillEffect::None).with_exhaustion(25.0));
            m.insert("TestFocus", SkillInfo::init("", TargetType::None, SkillEffect::None).with_focus_use(0.5));
            m.insert(
                "TestMultiple",
                SkillInfo::init("", TargetType::None, SkillEffect::None)
                    .with_ammo(AmmoKind::Bullets, 1)
                    .with_exhaustion(25.0),
            );
            m.insert("TestField", SkillInfo::init("", TargetType::Any, SkillEffect::FieldEffect(1, FieldKind::Fire)));
        }
        m.insert(
            "Dash",
            SkillInfo::init_with_distance("SpellBookPage09_39.png", TargetType::Tile, SkillEffect::Move, Some(3), true).with_exhaustion(50.0),
        );
        m.insert(
            "Reload",
            SkillInfo::init("SpellBook06_22.png", TargetType::None, SkillEffect::Reload(AmmoKind::Bullets)),
        );
        m.insert(
            "Strong Shot",
            SkillInfo::init_with_distance(
                "SpellBook01_37.png",
                TargetType::Enemy,
                SkillEffect::RangedAttack(10, BoltKind::Bullet),
                Some(10),
                true,
            )
            .with_ammo(AmmoKind::Bullets, 1)
            .with_alternate("Reload"),
        );
        m.insert(
            "Fire Bolt",
            SkillInfo::init_with_distance(
                "SpellBook06_117.png",
                TargetType::Enemy,
                SkillEffect::RangedAttack(5, BoltKind::Fire),
                Some(15),
                true,
            )
            .with_focus_use(0.5),
        );
        m.insert(
            "Slash",
            SkillInfo::init_with_distance(
                "SpellBook01_76.png",
                TargetType::Enemy,
                SkillEffect::MeleeAttack(5, WeaponKind::Sword),
                Some(1),
                true,
            ),
        );
        m.insert(
            "Delayed Blast",
            SkillInfo::init_with_distance("en_craft_96.png", TargetType::Any, SkillEffect::FieldEffect(1, FieldKind::Fire), Some(3), false),
        );

        m
    };
}

pub fn get_skill(name: &str) -> &'static SkillInfo {
    &SKILLS[name]
}

fn assert_correct_targeting(ecs: &mut World, invoker: &Entity, name: &str, target: Option<Point>) {
    let skill = get_skill(name);

    let requires_point = match skill.target {
        TargetType::None => false,
        TargetType::Tile | TargetType::Enemy | TargetType::Any => true,
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
        TargetType::Any => !initial.contains_point(&target),
        TargetType::None => false,
    } {
        return false;
    }

    if let Some(skill_range) = skill.distance {
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

    match &skill.effect {
        SkillEffect::Move => {
            // Targeting only gives us a point, so clone their position to get size as well
            let position = ecs.get_position(invoker).move_to(target.unwrap());
            begin_move(ecs, invoker, position);
        }
        SkillEffect::RangedAttack(strength, kind) => begin_bolt(ecs, &invoker, target.unwrap(), *strength, *kind),
        SkillEffect::MeleeAttack(strength, kind) => begin_melee(ecs, &invoker, target.unwrap(), *strength, *kind),
        SkillEffect::Reload(kind) => reload(ecs, &invoker, *kind),
        SkillEffect::FieldEffect(strength, kind) => begin_field(ecs, &invoker, target.unwrap(), *strength, *kind),
        SkillEffect::None => ecs.log(&format!("Invoking {}", name)),
    }

    spend_time(ecs, invoker, BASE_ACTION_COST);
    spend_ammo(ecs, invoker, skill);

    if let Some(exhaustion) = skill.exhaustion {
        spend_exhaustion(ecs, invoker, exhaustion);
    }
    if let Some(focus_use) = skill.focus_use {
        spend_focus(ecs, invoker, focus_use);
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
            set_ammo(ecs, invoker, kind, current_ammo - 1);
        }
        None => {}
    }
}

fn reload(ecs: &mut World, invoker: &Entity, kind: AmmoKind) {
    let max_ammo = { ecs.read_storage::<SkillResourceComponent>().grab(*invoker).max[&kind] };
    set_ammo(ecs, invoker, kind, max_ammo);
}

#[cfg(test)]
mod tests {
    use super::super::{
        add_ticks, create_test_state, find_at, find_first_entity, get_ticks, wait_for_animations, Character, CharacterInfoComponent, LogComponent,
        PositionComponent,
    };
    use super::*;
    use crate::atlas::{EasyMutWorld, SizedPoint};

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
        let info = SkillInfo::init("", TargetType::Tile, SkillEffect::None);
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

        let info = SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::None, Some(2), true);
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
        ecs.create_entity()
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        assert_eq!(false, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
    }

    #[test]
    fn skill_info_any_target() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        let info = SkillInfo::init_with_distance("", TargetType::Any, SkillEffect::None, Some(2), false);
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

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());
        invoke_skill(&mut ecs, &player, "TestRanged", Some(Point::init(4, 2)));
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    #[test]
    fn ranged_effect_multi() {
        let mut ecs = create_test_state()
            .with_character(2, 2, 100)
            .with_sized_character(SizedPoint::init_multi(2, 5, 2, 2), 100)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());
        invoke_skill(&mut ecs, &player, "TestRanged", Some(Point::init(2, 4)));
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    #[test]
    fn melee_effect() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());
        invoke_skill(&mut ecs, &player, "TestMelee", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    #[test]
    fn melee_effect_multi() {
        let mut ecs = create_test_state()
            .with_character(2, 2, 100)
            .with_sized_character(SizedPoint::init_multi(2, 4, 2, 2), 100)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());
        invoke_skill(&mut ecs, &player, "TestMelee", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    fn add_bullets(ecs: &mut World, player: &Entity, count: u32) {
        let resource = SkillResourceComponent::init(&[(AmmoKind::Bullets, count)]);
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
        assert_eq!(4, get_skill("TestExhaustion").get_remaining_usages(&ecs, &player).unwrap());
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

        for _ in 0..4 {
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
        invoke_skill(&mut ecs, &player, "TestField", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        add_ticks(&mut ecs, 100);
        wait(&mut ecs, player);
        assert_eq!(0, ecs.read_resource::<LogComponent>().count());
        add_ticks(&mut ecs, 100);
        wait(&mut ecs, player);
        tick_next_action(&mut ecs);
        tick_next_action(&mut ecs);
        wait_for_animations(&mut ecs);
        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }
}
