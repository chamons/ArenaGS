use std::cmp;

use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::*;
use crate::atlas::prelude::*;
use crate::clash::EventCoordinator;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct DamageOptions: u32 {
        const RAISE_TEMPERATURE =         0b00000000_00000001;
        const LOWER_TEMPERATURE =         0b00000000_00000010;
        const LARGE_TEMPERATURE_DELTA =   0b00000000_00000100;
        const KNOCKBACK =                 0b00000000_00001000;
        const ADD_CHARGE_STATUS =         0b00000000_00010000;
        const CONSUMES_CHARGE_DMG =       0b00000000_00100000;
        const CONSUMES_CHARGE_KNOCKBACK = 0b00000000_01000000;
        const PIERCE_DEFENSES   =         0b00000000_10000000;
        const TRIPLE_SHOT       =         0b00000001_00000000;
        const AIMED_SHOT        =         0b00000010_00000000;
    }
}

pub const STATIC_CHARGE_DAMAGE: u32 = 4;

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

    pub fn more_strength(&self, increase: u32) -> Damage {
        Damage {
            amount: Strength::init(self.amount.dice + increase),
            options: self.options,
        }
    }

    pub fn dice(&self) -> u32 {
        self.amount.dice
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct RolledDamage {
    pub absorbed_by_dodge: u32,
    pub absorbed_by_armor: u32,
    pub amount: u32,
    pub options: DamageOptions,
}

// A representation of a Damage after final roll, with a fixed value
impl RolledDamage {
    pub fn init(absorbed_by_dodge: u32, absorbed_by_armor: u32, amount: u32, options: &DamageOptions) -> RolledDamage {
        RolledDamage {
            absorbed_by_dodge,
            absorbed_by_armor,
            amount,
            options: *options,
        }
    }
}

pub fn apply_healing_to_character(ecs: &mut World, amount: Strength, target: Entity) {
    let healing_total = {
        let amount_to_heal = amount.roll(&mut ecs.fetch_mut::<RandomComponent>().rand);
        let mut defenses = ecs.write_storage::<CharacterInfoComponent>();
        let mut defense = &mut defenses.grab_mut(target).character.defenses;

        let initial_health = defense.health;
        defense.health = cmp::min(defense.health + amount_to_heal, defense.max_health);
        defense.health - initial_health
    };
    ecs.raise_event(EventKind::Healing(healing_total), Some(target));
}

pub fn apply_damage_to_location(ecs: &mut World, target_position: Point, source_position: Option<Point>, damage: Damage) {
    if let Some(target) = find_character_at_location(ecs, target_position) {
        apply_damage_to_character(ecs, damage, target, source_position);
    }
}

pub fn apply_damage_to_character(ecs: &mut World, damage: Damage, target: Entity, source_position: Option<Point>) {
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

const ARMORED_ADDITIONAL_ARMOR: u32 = 3;
fn apply_damage_core(ecs: &mut World, damage: Damage, target: Entity, source_position: Option<Point>) {
    // Flying should not be visible, and are immune to all damage
    if ecs.has_status(target, StatusKind::Flying) {
        return;
    }

    let rolled_damage = {
        let mut character_infos = ecs.write_storage::<CharacterInfoComponent>();
        let defenses = &mut character_infos.grab_mut(target).character.defenses;

        if ecs.has_status(target, StatusKind::Armored) {
            defenses.apply_damage_with_additional_armor(damage, ARMORED_ADDITIONAL_ARMOR, &mut ecs.fetch_mut::<RandomComponent>().rand)
        } else {
            defenses.apply_damage(damage, &mut ecs.fetch_mut::<RandomComponent>().rand)
        }
    };
    ecs.log(format!(
        "{} took {} damage ({} {{{{Sword}}}})",
        ecs.get_name(target).unwrap().as_str(),
        rolled_damage.amount,
        damage.dice()
    ));

    let should_knockback = {
        if rolled_damage.options.contains(DamageOptions::KNOCKBACK) {
            true
        } else if rolled_damage.options.contains(DamageOptions::CONSUMES_CHARGE_KNOCKBACK) && ecs.has_status(target, StatusKind::StaticCharge) {
            ecs.remove_status(target, StatusKind::StaticCharge);
            true
        } else {
            false
        }
    };

    if should_knockback {
        if let Some(source_position) = source_position {
            let current_position = ecs.get_position(target);
            let direction_of_impact = Direction::from_two_points(&source_position, &current_position.origin);
            if let Some(new_origin) = direction_of_impact.point_in_direction(&current_position.origin) {
                let new_position = current_position.move_to(new_origin);
                if is_area_clear_of_others(ecs, &new_position.all_positions(), target) {
                    ecs.log(format!("{} is knocked back.", ecs.get_name(target).unwrap()));
                    begin_move(ecs, target, new_position, PostMoveAction::CheckNewLocationDamage);
                }
            }
        }
    }
    if rolled_damage.options.contains(DamageOptions::ADD_CHARGE_STATUS) && !ecs.has_status(target, StatusKind::StaticCharge) {
        ecs.log(format!("{} crackles with static electricity.", ecs.get_name(target).unwrap()));
        ecs.add_status(target, StatusKind::StaticCharge, 300);
    }
    if rolled_damage.options.contains(DamageOptions::CONSUMES_CHARGE_DMG) && ecs.has_status(target, StatusKind::StaticCharge) {
        apply_damage_to_character(
            ecs,
            Damage::init(STATIC_CHARGE_DAMAGE).with_option(DamageOptions::PIERCE_DEFENSES),
            target,
            None,
        );
        ecs.remove_status(target, StatusKind::StaticCharge);
    }

    if rolled_damage.options.contains(DamageOptions::AIMED_SHOT) {
        if let Some(source_position) = source_position {
            if let Some(source) = find_character_at_location(ecs, source_position) {
                if !ecs.has_status(source, StatusKind::Aimed) {
                    ecs.log(format!("{} takes aim.", ecs.get_name(source).unwrap()));
                    ecs.add_status(source, StatusKind::Aimed, 300);
                }
            }
        }
    }

    ecs.raise_event(EventKind::Damage(rolled_damage), Some(target));
}

pub const REGEN_DURATION: i32 = 100;
pub const HEALTH_REGEN_PER_TICK: u32 = 1;

pub fn regen_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::StatusAdded(kind) => {
            if matches!(kind, StatusKind::Regen) {
                ecs.add_status(target.unwrap(), StatusKind::RegenTick, REGEN_DURATION);
            }
        }
        EventKind::StatusExpired(kind) => {
            if matches!(kind, StatusKind::RegenTick) {
                if ecs.has_status(target.unwrap(), StatusKind::Regen) {
                    ecs.add_status(target.unwrap(), StatusKind::RegenTick, REGEN_DURATION);
                } else {
                    ecs.log(format!("{} stops regenerating.", ecs.get_name(target.unwrap()).unwrap()));
                }

                apply_healing_to_character(ecs, Strength::init(HEALTH_REGEN_PER_TICK), target.unwrap());
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knockback() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);
        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 3),
            Damage::init(1).with_option(DamageOptions::KNOCKBACK),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);
        assert_position(&ecs, target, Point::init(2, 4));
        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("is knocked back"));
    }

    #[test]
    fn knockback_against_a_wall() {
        let mut ecs = create_test_state().with_player(2, 1, 100).with_character(2, 0, 0).with_map().build();
        let player = find_at(&ecs, 2, 1);
        let target = find_at(&ecs, 2, 0);
        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 0),
            Damage::init(1).with_option(DamageOptions::KNOCKBACK),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);
        assert_position(&ecs, target, Point::init(2, 0));
    }

    #[test]
    fn knockback_against_another() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 100)
            .with_character(2, 3, 0)
            .with_character(2, 4, 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);
        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 3),
            Damage::init(1).with_option(DamageOptions::KNOCKBACK),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);
        assert_position(&ecs, target, Point::init(2, 3));
    }

    #[test]
    fn add_charge_on_hit() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);

        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 3),
            Damage::init(1).with_option(DamageOptions::ADD_CHARGE_STATUS),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert!(ecs.has_status(target, StatusKind::StaticCharge));
        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("crackles with static electricity"));

        // Don't repeat the message if already has static charge
        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 3),
            Damage::init(1).with_option(DamageOptions::ADD_CHARGE_STATUS),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);
        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("crackles with static electricity"));
    }

    #[test]
    fn consumes_charge_for_damage() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);

        ecs.add_status(target, StatusKind::StaticCharge, 300);

        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 3),
            Damage::init(0).with_option(DamageOptions::CONSUMES_CHARGE_DMG),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert!(!ecs.has_status(target, StatusKind::StaticCharge));
        let health = &ecs.get_defenses(target);
        assert_ne!(health.max_health, health.health);
    }

    #[test]
    fn consumes_charge_for_knockback() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);

        ecs.add_status(target, StatusKind::StaticCharge, 300);

        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 3),
            Damage::init(0).with_option(DamageOptions::CONSUMES_CHARGE_KNOCKBACK),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("is knocked back"));
        assert!(!ecs.has_status(target, StatusKind::StaticCharge));
        assert_eq!(target, find_at(&ecs, 2, 4));
    }

    #[test]
    fn consumes_no_status_for_no_damage() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);

        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 3),
            Damage::init(0).with_option(DamageOptions::CONSUMES_CHARGE_DMG),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert!(!ecs.has_status(target, StatusKind::StaticCharge));

        let health = &ecs.get_defenses(target);
        assert_eq!(health.max_health, health.health);
    }

    #[test]
    fn triple_shot_applies_three_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_bolt(
            &mut ecs,
            player,
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
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);

        ecs.write_storage::<CharacterInfoComponent>().grab_mut(target).character.defenses.armor = 6;

        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 3),
            Damage::init(3).with_option(DamageOptions::TRIPLE_SHOT),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        let health = &ecs.get_defenses(target);
        assert_eq!(health.max_health, health.health);
    }

    #[test]
    fn aimed_shot_applies_buff() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_bolt(
            &mut ecs,
            player,
            Point::init(2, 3),
            Damage::init(3).with_option(DamageOptions::AIMED_SHOT),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("takes aim"));
        assert!(ecs.has_status(player, StatusKind::Aimed));
    }

    #[test]
    fn aimed_removed_after_shot() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        ecs.add_status(player, StatusKind::Aimed, 300);

        begin_bolt(&mut ecs, player, Point::init(2, 3), Damage::init(3), BoltKind::Fire);
        wait_for_animations(&mut ecs);

        // We assume removal = more damage, since it's a bit tricky to test due to RNG
        assert!(!ecs.has_status(player, StatusKind::Aimed));
    }

    #[test]
    fn armored_adds_armor_one_hit() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);

        ecs.add_status(target, StatusKind::Armored, 300);

        begin_bolt(&mut ecs, player, Point::init(2, 3), Damage::init(3), BoltKind::Fire);
        wait_for_animations(&mut ecs);

        // 3 armor, 3 damage
        // Damage: 2 + [2,4] = 4-6
        // Armor: 2 + [2,4] = 4-6
        // 0 - 2 damage
        let health = &ecs.get_defenses(target);
        assert!(health.health > 7);
    }

    #[test]
    fn flying_prevents_damage() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);
        let starting_health = ecs.get_defenses(target).health;

        ecs.add_status(target, StatusKind::Flying, 300);

        begin_bolt(&mut ecs, player, Point::init(2, 3), Damage::init(3), BoltKind::Fire);
        wait_for_animations(&mut ecs);

        assert_eq!(ecs.get_defenses(target).health, starting_health);
    }

    #[test]
    fn regen_adds_health_over_turns() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        ecs.write_storage::<CharacterInfoComponent>().grab_mut(player).character.defenses.health = 5;
        ecs.add_status(player, StatusKind::Regen, 300);

        add_ticks(&mut ecs, 100);
        let first_tick = ecs.get_defenses(player).health;
        assert!(first_tick > 5);

        add_ticks(&mut ecs, 100);
        let second_tick = ecs.get_defenses(player).health;
        assert!(second_tick > first_tick);

        add_ticks(&mut ecs, 100);
        let third_tick = ecs.get_defenses(player).health;
        assert!(third_tick > second_tick);
    }

    fn test_event(ecs: &mut World, kind: EventKind, _target: Option<Entity>) {
        match kind {
            EventKind::Healing(_) => ecs.increment_test_data("Healing".to_string()),
            _ => {}
        };
    }

    #[test]
    fn regen_total_ticks() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        ecs.subscribe(test_event);
        let player = find_at(&ecs, 2, 2);

        ecs.add_status(player, StatusKind::Regen, 300);
        for _ in 0..5 {
            add_ticks(&mut ecs, 100);
        }
        assert_eq!(3, ecs.get_test_data("Healing"));
    }

    #[test]
    fn apply_healing_raises_health() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        ecs.write_storage::<CharacterInfoComponent>().grab_mut(player).character.defenses.health = 5;
        apply_healing_to_character(&mut ecs, Strength::init(2), player);
        assert!(ecs.get_defenses(player).health > 5);
    }

    #[test]
    fn apply_healing_does_not_exceed_max() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);

        ecs.write_storage::<CharacterInfoComponent>().grab_mut(player).character.defenses.health = 5;
        apply_healing_to_character(&mut ecs, Strength::init(20), player);
        assert_eq!(ecs.get_defenses(player).max_health, ecs.get_defenses(player).health);
    }
}
