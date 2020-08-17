use std::cmp;

use super::{BASE_ACTION_COST, STRENGTH_DICE_SIDES};

const TEMPERATURE_MIDPOINT: i32 = 0;
const TEMPERATURE_BURN_POINT: i32 = 100;
const TEMPERATURE_BASE_POINT: i32 = -100;

struct Temperature {
    current_temperature: i32,
}

pub enum TemperatureDirection {
    Heat,
    Cool,
}

impl Temperature {
    pub fn init() -> Temperature {
        Temperature {
            current_temperature: TEMPERATURE_MIDPOINT,
        }
    }

    pub fn apply_damage(&mut self, damage: u32, direction: TemperatureDirection) {
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

    pub fn reduce_temperature(&mut self, ticks: u32) {
        // 8 turns should be enough to go from 100 -> 0 or -100 -> 0
        // 100 / 8 = 13
        const TEMPERATURE_LOST_PER_TURN: i32 = 13;
        let delta = ((TEMPERATURE_LOST_PER_TURN as f32 * ticks as f32) / (BASE_ACTION_COST as f32)).round();

        if self.current_temperature > TEMPERATURE_MIDPOINT {
            self.current_temperature = cmp::max(self.current_temperature - (delta as i32), 0);
        } else {
            self.current_temperature = cmp::min(self.current_temperature + (delta as i32), 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_temperature_based_upon_damage_dice() {
        let mut temperature = Temperature::init();
        temperature.apply_damage(50, TemperatureDirection::Heat);
        assert!(temperature.current_temperature > TEMPERATURE_MIDPOINT);
    }

    #[test]
    fn apply_temperature_can_reverse_others() {
        let mut temperature = Temperature::init();
        temperature.apply_damage(50, TemperatureDirection::Heat);
        temperature.apply_damage(100, TemperatureDirection::Cool);
        assert!(temperature.current_temperature < TEMPERATURE_MIDPOINT);
    }

    #[test]
    fn temperature_reduction_towards_average() {
        let mut temperature = Temperature::init();
        temperature.apply_damage(50, TemperatureDirection::Heat);
        let initial = temperature.current_temperature;
        temperature.reduce_temperature(100);
        assert!(temperature.current_temperature < initial);
    }

    #[test]
    fn temperature_reduction_does_not_overshoot() {
        let mut temperature = Temperature { current_temperature: 5 };
        temperature.reduce_temperature(100);
        assert_eq!(0, temperature.current_temperature);
    }

    #[test]
    fn temperature_can_cause_burns() {}

    #[test]
    fn temperature_can_cause_frost() {}

    #[test]
    fn reductions_happen_over_game_turns() {}
}
