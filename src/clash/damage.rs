use serde::{Deserialize, Serialize};

use super::Strength;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct DamageOptions: u32 {
        const RAISE_TEMPERATURE = 0b00000001;
        const LOWER_TEMPERATURE = 0b00000010;
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

    pub fn with_raise_temp(mut self) -> Damage {
        self.options.insert(DamageOptions::RAISE_TEMPERATURE);
        self
    }

    #[allow(dead_code)]
    pub fn with_lower_temp(mut self) -> Damage {
        self.options.insert(DamageOptions::LOWER_TEMPERATURE);
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
