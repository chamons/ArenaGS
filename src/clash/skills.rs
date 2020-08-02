use std::collections::HashMap;
use std::slice::from_ref;

use lazy_static::lazy_static;
use specs::prelude::*;
use specs_derive::Component;

use super::{bolt, is_area_clear, melee, move_action, spend_time, BoltKind, Logger, Positions, WeaponKind, BASE_ACTION_COST};
use crate::atlas::{EasyECS, EasyMutECS, Point};

#[allow(dead_code)]
#[derive(is_enum_variant, Clone, Copy)]
pub enum TargetType {
    None,
    Tile,
    Enemy,
}

#[allow(dead_code)]
pub enum SkillEffect {
    None,
    Move,
    RangedAttack(u32, BoltKind),
    MeleeAttack(u32, WeaponKind),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl SkillResourceComponent {
    pub fn init(starting_ammo: &[(AmmoKind, u32)]) -> SkillResourceComponent {
        SkillResourceComponent {
            ammo: starting_ammo.iter().map(|(k, a)| (*k, *a)).collect(),
        }
    }
}

pub struct SkillInfo {
    pub image: &'static str,
    pub target: TargetType,
    pub effect: SkillEffect,
    pub distance: Option<u32>,
    pub must_be_clear: bool,
    pub ammo_info: Option<AmmoInfo>,
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
        }
    }

    pub fn show_trail(&self) -> bool {
        self.must_be_clear
    }

    pub fn with_ammo(mut self, kind: AmmoKind, usage: u32) -> SkillInfo {
        self.ammo_info = Some(AmmoInfo { kind, usage });
        self
    }

    pub fn get_remaining_usages(&self, ecs: &World, entity: &Entity) -> Option<u32> {
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
        }
        m.insert(
            "Dash",
            SkillInfo::init_with_distance("SpellBookPage09_39.png", TargetType::Tile, SkillEffect::Move, Some(3), true),
        );
        m.insert(
            "Fire Bolt",
            SkillInfo::init_with_distance(
                "SpellBook06_117.png",
                TargetType::Enemy,
                SkillEffect::RangedAttack(5, BoltKind::Fire),
                Some(15),
                true,
            ),
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
        TargetType::Tile => true,
        TargetType::Enemy => true,
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

    match skill.effect {
        SkillEffect::Move => {
            // Targeting only gives us a point, so clone their position to get size as well
            let position = ecs.get_position(invoker).move_to(target.unwrap());
            move_action(ecs, invoker, position);
        }
        SkillEffect::RangedAttack(strength, kind) => bolt(ecs, &invoker, target.unwrap(), strength, kind),
        SkillEffect::MeleeAttack(strength, kind) => melee(ecs, &invoker, target.unwrap(), strength, kind),
        SkillEffect::None => ecs.log(&format!("Invoking {}", name)),
    }

    spend_time(ecs, invoker, MOVE_ACTION_COST);
    spend_ammo(ecs, invoker, skill)
}

fn spend_ammo(ecs: &mut World, invoker: &Entity, skill: &SkillInfo) {
    match &skill.ammo_info {
        Some(ammo_info) => {
            let kind = &ammo_info.kind;
            let current_ammo = { ecs.read_storage::<SkillResourceComponent>().grab(*invoker).ammo[kind] };

            let mut skill_resources = ecs.write_storage::<SkillResourceComponent>();
            *skill_resources.grab_mut(*invoker).ammo.get_mut(kind).unwrap() = current_ammo - 1;
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        add_ticks, create_world, get_ticks, wait_for_animations, Character, CharacterInfoComponent, LogComponent, Map, MapComponent, PositionComponent,
        TimeComponent,
    };
    use super::*;
    use crate::atlas::SizedPoint;

    #[test]
    #[should_panic]
    fn panic_if_wrong_targeting() {
        let mut ecs = create_world();
        let entity = ecs.create_entity().with(TimeComponent::init(100)).build();
        invoke_skill(&mut ecs, &entity, "TestNone", Some(Point::init(2, 2)));
    }

    #[test]
    #[should_panic]
    fn panic_if_missing_targeting() {
        let mut ecs = create_world();
        let entity = ecs.create_entity().with(TimeComponent::init(100)).build();
        invoke_skill(&mut ecs, &entity, "TestTile", None);
    }

    #[test]
    fn invoker_spend_time() {
        let mut ecs = create_world();
        let entity = ecs.create_entity().with(TimeComponent::init(100)).build();
        invoke_skill(&mut ecs, &entity, "TestNone", None);
        assert_eq!(0, get_ticks(&ecs, &entity));
    }

    #[test]
    #[should_panic]
    fn target_must_be_in_range() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .build();
        invoke_skill(&mut ecs, &entity, "TestWithRange", Some(Point::init(2, 5)));
    }

    #[test]
    fn target_in_range() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));
        invoke_skill(&mut ecs, &entity, "TestWithRange", Some(Point::init(2, 4)));
    }

    #[test]
    fn skill_info_range() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let info = get_skill("TestWithRange");
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
        assert_eq!(false, is_good_target(&mut ecs, &entity, &info, Point::init(2, 5)));
        let info = SkillInfo::init("", TargetType::Tile, SkillEffect::None);
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 5)));
    }

    #[test]
    fn skill_info_correct_target_kind() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.create_entity()
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let info = get_skill("TestWithRange");
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
        assert_eq!(false, is_good_target(&mut ecs, &entity, &info, Point::init(2, 3)));
    }

    #[test]
    fn skill_info_must_be_clear() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let info = SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::None, Some(2), true);
        assert_eq!(true, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
        ecs.create_entity()
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        assert_eq!(false, is_good_target(&mut ecs, &entity, &info, Point::init(2, 4)));
    }

    #[test]
    fn movement_effect() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        invoke_skill(&mut ecs, &entity, "TestMove", Some(Point::init(3, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(Point::init(3, 3), ecs.get_position(&entity).origin);
    }

    #[test]
    fn movement_effect_multi() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init_multi(2, 2, 2, 1)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        invoke_skill(&mut ecs, &entity, "TestMove", Some(Point::init(3, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(Point::init(3, 3), ecs.get_position(&entity).origin);
    }

    #[test]
    fn ranged_effect() {
        let mut ecs = create_world();
        let player = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        ecs.create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(4, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());
        invoke_skill(&mut ecs, &player, "TestRanged", Some(Point::init(4, 2)));
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    #[test]
    fn ranged_effect_multi() {
        let mut ecs = create_world();
        let player = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        ecs.create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init_multi(2, 5, 2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());
        invoke_skill(&mut ecs, &player, "TestRanged", Some(Point::init(2, 4)));
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    #[test]
    fn melee_effect() {
        let mut ecs = create_world();
        let player = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        ecs.create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());
        invoke_skill(&mut ecs, &player, "TestMelee", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    #[test]
    fn melee_effect_multi() {
        let mut ecs = create_world();
        let player = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        ecs.create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init_multi(2, 4, 2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());
        invoke_skill(&mut ecs, &player, "TestMelee", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    #[test]
    fn get_remaining_usages_with_ammo() {
        let mut ecs = create_world();
        let player = ecs.create_entity().with(SkillResourceComponent::init(&[(AmmoKind::Bullets, 3)])).build();

        assert_eq!(3, get_skill("TestAmmo").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn get_remaining_usages_zero_ammo() {
        let mut ecs = create_world();
        let player = ecs.create_entity().with(SkillResourceComponent::init(&[(AmmoKind::Bullets, 0)])).build();

        assert_eq!(0, get_skill("TestAmmo").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn get_remaining_usages_non_existant_ammo() {
        let mut ecs = create_world();
        let player = ecs.create_entity().with(SkillResourceComponent::init(&[])).build();

        assert_eq!(0, get_skill("TestAmmo").get_remaining_usages(&ecs, &player).unwrap());
    }

    #[test]
    fn get_remaining_usages_skill_uses_no_ammo() {
        let mut ecs = create_world();
        let player = ecs.create_entity().with(SkillResourceComponent::init(&[])).build();

        assert_eq!(true, get_skill("TestMelee").get_remaining_usages(&ecs, &player).is_none());
    }

    #[test]
    fn skills_with_ammo() {
        let mut ecs = create_world();
        let player = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .with(SkillResourceComponent::init(&[(AmmoKind::Bullets, 3)]))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let skill = get_skill("TestAmmo");

        for _ in 0..3 {
            assert_eq!(true, can_invoke_skill(&mut ecs, &player, &skill, None));
            invoke_skill(&mut ecs, &player, "TestAmmo", None);
            add_ticks(&mut ecs, 100);
        }

        assert_eq!(false, can_invoke_skill(&mut ecs, &player, &skill, None));
    }
}
