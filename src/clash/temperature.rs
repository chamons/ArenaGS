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
    current_temperature: i32,
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

fn apply_temperature_effects(character_info: &mut CharacterInfoComponent, random: &mut RandomComponent) {
    let current_temperature = character_info.character.temperature.current_temperature;

    if current_temperature > TEMPERATURE_BURN_POINT {
        const TEMPERATURE_DAMAGE_PER_TICK: u32 = 2;
        apply_damage(Damage::heat(TEMPERATURE_DAMAGE_PER_TICK), character_info, random);
    } else if current_temperature < TEMPERATURE_FREEZE_POINT {
    }
}

pub fn tick_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Tick(ticks) => {
            let mut character_infos = ecs.write_storage::<CharacterInfoComponent>();
            if let Some(character_info) = &mut character_infos.get_mut(target.unwrap()) {
                if character_info.character.temperature.is_ready(ticks) {
                    apply_temperature_effects(character_info, &mut ecs.fetch_mut::<RandomComponent>());
                    character_info.character.temperature.reduce_temperature();
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
    use crate::atlas::EasyMutECS;

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

        let starting_temp = ecs
            .read_storage::<CharacterInfoComponent>()
            .grab(player)
            .character
            .temperature
            .current_temperature;
        add_ticks(&mut ecs, 100);
        assert!(
            ecs.read_storage::<CharacterInfoComponent>()
                .grab(player)
                .character
                .temperature
                .current_temperature
                < starting_temp
        );
    }

    #[test]
    fn ranged_attack_adds_temperature() {}
}
