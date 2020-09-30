use std::cmp;

use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::*;
use crate::atlas::{EasyECS, EasyMutECS};

pub const TEMPERATURE_MIDPOINT: i32 = 0;
pub const TEMPERATURE_BURN_POINT: i32 = 100;
pub const TEMPERATURE_FREEZE_POINT: i32 = -100;
pub const BURN_DURATION: i32 = 100;
pub const TEMPERATURE_DAMAGE_PER_TICK: u32 = 2;

#[derive(Serialize, Deserialize, Clone)]
pub struct Temperature {
    current_temperature: i32,
    timer: TickTimer,
}

pub enum TemperatureDirection {
    Heat,
    #[allow(dead_code)]
    Cool,
}

impl Temperature {
    pub fn init() -> Temperature {
        Temperature {
            current_temperature: TEMPERATURE_MIDPOINT,
            timer: TickTimer::init(),
        }
    }

    pub fn change_from_incoming_damage(&mut self, damage: u32, direction: TemperatureDirection) {
        // 4 strength 10 shots half resisted should tip us over (20 dice)
        // Default range 100 / 20 = 5 temperature per dice
        const TEMPERATURE_PER_DICE_DAMAGE: i32 = 5;
        let dice: i32 = (damage / STRENGTH_DICE_SIDES) as i32;

        let delta = match direction {
            TemperatureDirection::Heat => dice * TEMPERATURE_PER_DICE_DAMAGE,
            TemperatureDirection::Cool => -dice * TEMPERATURE_PER_DICE_DAMAGE,
        };
        self.current_temperature += delta;
    }

    pub fn reduce_temperature(&mut self) {
        // 8 turns should be enough to go from 100 -> 0 or -100 -> 0
        // 100 / 8 = 13
        const TEMPERATURE_LOST_PER_TURN: i32 = 13;

        if self.current_temperature > TEMPERATURE_MIDPOINT {
            self.current_temperature = cmp::max(self.current_temperature - TEMPERATURE_LOST_PER_TURN, 0);
        } else {
            self.current_temperature = cmp::min(self.current_temperature + TEMPERATURE_LOST_PER_TURN, 0);
        }
    }

    pub fn is_burning(&self) -> bool {
        self.current_temperature > TEMPERATURE_BURN_POINT
    }

    pub fn is_freezing(&self) -> bool {
        self.current_temperature < TEMPERATURE_FREEZE_POINT
    }

    pub fn is_ready(&mut self, ticks_to_add: i32) -> bool {
        self.timer.apply_ticks(ticks_to_add)
    }

    pub fn current_temperature(&self) -> i32 {
        self.current_temperature
    }

    #[cfg(test)]
    pub fn set_temperature(&mut self, temperature: i32) {
        self.current_temperature = temperature;
    }
}

fn check_temperature_state(ecs: &mut World, target: &Entity) {
    let did_freeze = StatusStore::toggle_trait_from(ecs, target, StatusKind::Frozen, ecs.get_temperature(target).is_freezing());

    if did_freeze {
        ecs.log(format!("{} freezes over.", ecs.get_name(target).unwrap()));
    }

    if ecs.get_temperature(target).is_burning() && !ecs.has_status(target, StatusKind::Burning) {
        ecs.log(format!("{} begins to burn.", ecs.get_name(target).unwrap()));
        ecs.add_status(target, StatusKind::Burning, BURN_DURATION);
    }
}

fn reduce_temperature(ecs: &mut World, target: &Entity, ticks: i32) {
    if ecs.read_storage::<CharacterInfoComponent>().has(*target) {
        {
            let mut character_infos = ecs.write_storage::<CharacterInfoComponent>();
            let character_info = character_infos.grab_mut(*target);
            let temperature = &mut character_info.character.temperature;
            if temperature.is_ready(ticks) {
                temperature.reduce_temperature();
            }
        }

        // reduce_temperature could have changed burning/frozen
        check_temperature_state(ecs, target);
    }
}

fn apply_temperature_damage_delta(ecs: &mut World, target: &Entity, rolled_damage: RolledDamage) {
    if ecs.read_storage::<CharacterInfoComponent>().has(*target) {
        {
            let mut character_infos = ecs.write_storage::<CharacterInfoComponent>();
            let character_info = character_infos.grab_mut(*target);
            let direction = {
                if rolled_damage.options.contains(DamageOptions::RAISE_TEMPERATURE) {
                    Some(TemperatureDirection::Heat)
                } else if rolled_damage.options.contains(DamageOptions::LOWER_TEMPERATURE) {
                    Some(TemperatureDirection::Cool)
                } else {
                    None
                }
            };

            let mut amount = rolled_damage.amount;
            if rolled_damage.options.contains(DamageOptions::LARGE_TEMPERATURE_DELTA) {
                amount *= 4;
            }

            if let Some(direction) = direction {
                character_info.character.temperature.change_from_incoming_damage(amount, direction);
            }
        }
        // change_from_incoming_damage could have changed burning/frozen
        check_temperature_state(ecs, target);
    }
}

pub fn temp_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Tick(ticks) => reduce_temperature(ecs, &target.unwrap(), ticks),
        EventKind::Damage(rolled_damage) => apply_temperature_damage_delta(ecs, &target.unwrap(), rolled_damage),
        EventKind::StatusExpired(kind) => {
            if matches!(kind, StatusKind::Burning) {
                // Order matter here - must re-apply burning status before apply damage,
                // else we'll repeat "began burning" log
                if ecs.get_temperature(&target.unwrap()).is_burning() {
                    ecs.add_status(&target.unwrap(), StatusKind::Burning, BURN_DURATION);
                } else {
                    ecs.log(format!("{} stops burning.", ecs.get_name(&target.unwrap()).unwrap()));
                }

                apply_damage_to_character(
                    ecs,
                    Damage::init(TEMPERATURE_DAMAGE_PER_TICK).with_option(DamageOptions::PIERCE_DEFENSES),
                    &target.unwrap(),
                    None,
                );
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::atlas::Point;

    #[test]
    fn apply_temperature_based_upon_damage_dice() {
        let mut temperature = Temperature::init();
        temperature.change_from_incoming_damage(50, TemperatureDirection::Heat);
        assert!(temperature.current_temperature > TEMPERATURE_MIDPOINT);
    }

    #[test]
    fn apply_temperature_can_reverse_others() {
        let mut temperature = Temperature::init();
        temperature.change_from_incoming_damage(50, TemperatureDirection::Heat);
        temperature.change_from_incoming_damage(100, TemperatureDirection::Cool);
        assert!(temperature.current_temperature < TEMPERATURE_MIDPOINT);
    }

    #[test]
    fn temperature_reduction_towards_average() {
        let mut temperature = Temperature::init();
        temperature.change_from_incoming_damage(50, TemperatureDirection::Heat);
        let initial = temperature.current_temperature;
        temperature.reduce_temperature();
        assert!(temperature.current_temperature < initial);
    }

    #[test]
    fn temperature_reduction_does_not_overshoot() {
        let mut temperature = Temperature {
            current_temperature: 5,
            timer: TickTimer::init(),
        };
        temperature.reduce_temperature();
        assert_eq!(0, temperature.current_temperature);
    }

    #[test]
    fn temperature_can_cause_burns() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        set_temperature(&mut ecs, player, TEMPERATURE_BURN_POINT + 20);

        // Set armor so high burning must pierce to do actual damage
        ecs.write_storage::<CharacterInfoComponent>().grab_mut(player).character.defenses.armor = 100;

        let starting_health = ecs.get_defenses(&player).health;
        add_ticks(&mut ecs, 100);
        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("begins to burn"));
        assert!(ecs.has_status(&player, StatusKind::Burning));
        assert!(ecs.get_defenses(&player).health < starting_health);
    }

    #[test]
    fn burns_go_out_over_time() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        set_temperature(&mut ecs, player, TEMPERATURE_BURN_POINT + 20);

        for _ in 0..10 {
            add_ticks(&mut ecs, 100);
        }
        assert!(!ecs.has_status(&player, StatusKind::Burning));
        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("begins to burn"));
        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("stops burning"));
    }

    #[test]
    fn temperature_can_cause_frost() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        set_temperature(&mut ecs, player, TEMPERATURE_FREEZE_POINT - 20);

        add_ticks(&mut ecs, 100);
        assert!(ecs.has_status(&player, StatusKind::Frozen));
        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("freezes over"));
    }

    #[test]
    fn reductions_happen_over_game_turns() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        set_temperature(&mut ecs, player, TEMPERATURE_BURN_POINT + 10);

        let starting_temp = ecs.get_temperature(&player).current_temperature;
        add_ticks(&mut ecs, 100);
        assert!(ecs.get_temperature(&player).current_temperature < starting_temp);
    }

    #[test]
    fn ranged_attack_adds_temperature() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(3, 2, 0).build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 3, 2);
        // Prevent target from dying
        set_health(&mut ecs, target, 200);

        for _ in 0..4 {
            begin_bolt(
                &mut ecs,
                &player,
                Point::init(3, 2),
                Damage::init(10).with_option(DamageOptions::RAISE_TEMPERATURE),
                BoltKind::Fire,
            );
            wait_for_animations(&mut ecs);
        }
        assert!(ecs.get_temperature(&target).current_temperature > TEMPERATURE_BURN_POINT);
        assert_eq!(1, ecs.read_resource::<LogComponent>().log.contains_count("begins to burn"));
    }

    #[test]
    fn large_delta_raises_more() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(3, 2, 0).build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 3, 2);
        // Prevent target from dying
        set_health(&mut ecs, target, 200);

        begin_bolt(
            &mut ecs,
            &player,
            Point::init(3, 2),
            Damage::init(10).with_option(DamageOptions::RAISE_TEMPERATURE),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        let basic_delta = ecs.get_temperature(&target).current_temperature;
        ecs.write_storage::<CharacterInfoComponent>()
            .grab_mut(target)
            .character
            .temperature
            .current_temperature = 0;

        begin_bolt(
            &mut ecs,
            &player,
            Point::init(3, 2),
            Damage::init(10)
                .with_option(DamageOptions::RAISE_TEMPERATURE)
                .with_option(DamageOptions::LARGE_TEMPERATURE_DELTA),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);
        let large_delta = ecs.get_temperature(&target).current_temperature;
        assert!(large_delta > basic_delta * 2);
    }
}
