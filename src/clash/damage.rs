use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::*;
use crate::atlas::{EasyMutECS, Point};
use crate::clash::{Direction, EventCoordinator};

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct DamageOptions: u32 {
        const RAISE_TEMPERATURE =       0b00000001;
        const LOWER_TEMPERATURE =       0b00000010;
        const LARGE_TEMPERATURE_DELTA = 0b00000100;
        const KNOCKBACK =               0b00001000;
        const ADD_CHARGE_STATUS =       0b00010000;
        const CONSUMES_CHARGE   =       0b00100000;
        const PIERCE_DEFENSES   =       0b01000000;
        const TRIPLE_SHOT       =       0b10000000;
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Damage {
    pub amount: Strength,
    pub options: DamageOptions,
}

impl Damage {
    pub fn init(dice: u32) -> Damage {
        Damage {
            amount: Strength::init(dice),
            options: DamageOptions::empty(),
        }
    }

    pub fn with_option(mut self, option: DamageOptions) -> Damage {
        self.options.insert(option);
        self
    }

    pub fn dice(&self) -> u32 {
        self.amount.dice
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct RolledDamage {
    pub amount: u32,
    pub options: DamageOptions,
}

// A representation of a Damage after final roll, with a fixed value
impl RolledDamage {
    pub fn init(amount: u32, initial_damage: &Damage) -> RolledDamage {
        RolledDamage {
            amount,
            options: initial_damage.options,
        }
    }
}

pub fn apply_damage_to_location(ecs: &mut World, target_position: Point, source_position: Option<Point>, damage: Damage) {
    if let Some(target) = find_character_at_location(ecs, target_position) {
        apply_damage_to_character(ecs, damage, &target, source_position);
    }
}

pub fn apply_damage_to_character(ecs: &mut World, damage: Damage, target: &Entity, source_position: Option<Point>) {
    let damage_count = {
        if damage.options.contains(DamageOptions::TRIPLE_SHOT) {
            3
        } else {
            1
        }
    };
    for _ in 0..damage_count {
        apply_damage_core(ecs, damage, target, source_position);
    }
}

fn apply_damage_core(ecs: &mut World, damage: Damage, target: &Entity, source_position: Option<Point>) {
    let rolled_damage = {
        let mut character_infos = ecs.write_storage::<CharacterInfoComponent>();
        let defenses = &mut character_infos.grab_mut(*target).character.defenses;
        defenses.apply_damage(damage, &mut ecs.fetch_mut::<RandomComponent>().rand)
    };
    ecs.log(format!(
        "{} took {} damage (Str {}).",
        ecs.get_name(&target).unwrap().as_str(),
        rolled_damage.amount,
        damage.dice()
    ));

    if rolled_damage.options.contains(DamageOptions::KNOCKBACK) {
        if let Some(source_position) = source_position {
            let current_position = ecs.get_position(target);
            let direction_of_impact = Direction::from_two_points(&source_position, &current_position.origin);
            if let Some(new_origin) = direction_of_impact.point_in_direction(&current_position.origin) {
                let new_position = current_position.move_to(new_origin);
                if is_area_clear(ecs, &new_position.all_positions(), target) {
                    begin_move(ecs, target, new_position, PostMoveAction::None);
                }
            }
        }
    }
    if rolled_damage.options.contains(DamageOptions::ADD_CHARGE_STATUS) {
        ecs.log(format!("{} crackles with static electricity", ecs.get_name(target).unwrap()));
        ecs.write_storage::<StatusComponent>()
            .grab_mut(*target)
            .status
            .add_status(StatusKind::StaticCharge, 300);
    }
    if rolled_damage.options.contains(DamageOptions::CONSUMES_CHARGE) && ecs.has_status(target, StatusKind::StaticCharge) {
        const STATIC_CHARGE_DAMAGE: u32 = 4;
        apply_damage_to_character(
            ecs,
            Damage::init(STATIC_CHARGE_DAMAGE).with_option(DamageOptions::PIERCE_DEFENSES),
            target,
            None,
        );
        ecs.write_storage::<StatusComponent>()
            .grab_mut(*target)
            .status
            .remove_status(StatusKind::StaticCharge);
    }

    ecs.raise_event(EventKind::Damage(rolled_damage), Some(*target));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::EasyMutECS;

    #[test]
    fn knockback() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&mut ecs, 2, 2);
        let target = find_at(&mut ecs, 2, 3);
        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 3),
            Damage::init(1).with_option(DamageOptions::KNOCKBACK),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &target, Point::init(2, 4));
    }

    #[test]
    fn knockback_against_a_wall() {
        let mut ecs = create_test_state().with_player(2, 1, 100).with_character(2, 0, 0).with_map().build();
        let player = find_at(&mut ecs, 2, 1);
        let target = find_at(&mut ecs, 2, 0);
        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 0),
            Damage::init(1).with_option(DamageOptions::KNOCKBACK),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &target, Point::init(2, 0));
    }

    #[test]
    fn knockback_against_another() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 100)
            .with_character(2, 3, 0)
            .with_character(2, 4, 0)
            .with_map()
            .build();
        let player = find_at(&mut ecs, 2, 2);
        let target = find_at(&mut ecs, 2, 3);
        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 3),
            Damage::init(1).with_option(DamageOptions::KNOCKBACK),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &target, Point::init(2, 3));
    }

    #[test]
    fn add_charge_on_hit() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&mut ecs, 2, 2);
        let target = find_at(&mut ecs, 2, 3);

        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 3),
            Damage::init(1).with_option(DamageOptions::ADD_CHARGE_STATUS),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert!(ecs.has_status(&target, StatusKind::StaticCharge));
        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("crackles with static electricity"));
    }

    #[test]
    fn consumes_charge_for_damage() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&mut ecs, 2, 2);
        let target = find_at(&mut ecs, 2, 3);

        ecs.write_storage::<StatusComponent>()
            .grab_mut(target)
            .status
            .add_status(StatusKind::StaticCharge, 100);

        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 3),
            Damage::init(0).with_option(DamageOptions::CONSUMES_CHARGE),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert_eq!(false, ecs.has_status(&target, StatusKind::StaticCharge));
        let health = &ecs.get_defenses(&target);
        assert_ne!(health.max_health, health.health);
    }

    #[test]
    fn consumes_no_status_for_no_damage() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&mut ecs, 2, 2);
        let target = find_at(&mut ecs, 2, 3);

        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 3),
            Damage::init(0).with_option(DamageOptions::CONSUMES_CHARGE),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert_eq!(false, ecs.has_status(&target, StatusKind::StaticCharge));

        let health = &ecs.get_defenses(&target);
        assert_eq!(health.max_health, health.health);
    }

    #[test]
    fn triple_shot_applies_three_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&mut ecs, 2, 2);

        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 3),
            Damage::init(3).with_option(DamageOptions::TRIPLE_SHOT),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);
        assert_eq!(3, ecs.read_resource::<LogComponent>().log.contains_count("took"));
    }

    #[test]
    fn triple_shot_applies_armor_each_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&mut ecs, 2, 2);
        let target = find_at(&mut ecs, 2, 3);

        ecs.write_storage::<CharacterInfoComponent>().grab_mut(target).character.defenses.armor = 6;

        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 3),
            Damage::init(3).with_option(DamageOptions::TRIPLE_SHOT),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        let health = &ecs.get_defenses(&target);
        assert_eq!(health.max_health, health.health);
    }
}