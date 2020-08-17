use std::cmp;

use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::*;
use crate::atlas::{EasyECS, EasyMutECS};

const TEMPERATURE_MIDPOINT: i32 = 0;
const TEMPERATURE_BURN_POINT: i32 = 100;
const TEMPERATURE_FREEZE_POINT: i32 = -100;

#[derive(Serialize, Deserialize, Clone)]
pub struct Temperature {
    pub current_temperature: i32,
    timer: TickTimer,
}

pub enum TemperatureDirection {
    Heat,
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
            TemperatureDirection::Cool => -1 * dice * TEMPERATURE_PER_DICE_DAMAGE,
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

    pub fn is_ready(&mut self, ticks_to_add: i32) -> bool {
        self.timer.apply_ticks(ticks_to_add)
    }
}

fn apply_temperature_effects(ecs: &mut World, target: &Entity, ticks: i32) {
    let mut apply_burn = false;
    let mut apply_freeze = false;
    {
        let mut character_infos = ecs.write_storage::<CharacterInfoComponent>();
        if let Some(character_info) = character_infos.get_mut(*target) {
            let temperature = &mut character_info.character.temperature;
            if temperature.is_ready(ticks) {
                apply_burn = temperature.current_temperature > TEMPERATURE_BURN_POINT;
                apply_freeze = temperature.current_temperature < TEMPERATURE_FREEZE_POINT;
                temperature.reduce_temperature();
            }
        }
    }
    if apply_burn {
        // If this is >= 5 we'll need to prevent burning from adding heat itself
        const TEMPERATURE_DAMAGE_PER_TICK: u32 = 2;
        apply_damage_to_character(ecs, Damage::fire(TEMPERATURE_DAMAGE_PER_TICK), target);
    }
    if apply_freeze {}
}

pub fn temp_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Tick(ticks) => apply_temperature_effects(ecs, &target.unwrap(), ticks),
        EventKind::Damage(damage, kind) => {
            let mut character_infos = ecs.write_storage::<CharacterInfoComponent>();
            if let Some(character_info) = character_infos.get_mut(target.unwrap()) {
                match kind {
                    DamageKind::Fire => character_info
                        .character
                        .temperature
                        .change_from_incoming_damage(damage, TemperatureDirection::Heat),
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::atlas::{EasyMutECS, Point};

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
        ecs.write_storage::<CharacterInfoComponent>()
            .grab_mut(player)
            .character
            .temperature
            .current_temperature = TEMPERATURE_BURN_POINT + 10;

        let starting_health = ecs.get_defenses(&player).health;
        add_ticks(&mut ecs, 100);
        assert!(ecs.get_defenses(&player).health < starting_health);
    }

    #[test]
    fn temperature_can_cause_frost() {}

    #[test]
    fn reductions_happen_over_game_turns() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        ecs.write_storage::<CharacterInfoComponent>()
            .grab_mut(player)
            .character
            .temperature
            .current_temperature = TEMPERATURE_BURN_POINT + 10;

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
        ecs.write_storage::<CharacterInfoComponent>().grab_mut(target).character.defenses = Defenses::just_health(200);

        for _ in 0..4 {
            begin_bolt(&mut ecs, &player, Point::init(3, 2), Damage::fire(10), BoltKind::Fire);
            wait_for_animations(&mut ecs);
        }
        assert!(ecs.get_temperature(&target).current_temperature > TEMPERATURE_BURN_POINT);
    }
}
