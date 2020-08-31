use serde::{Deserialize, Serialize};

use super::Strength;

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
