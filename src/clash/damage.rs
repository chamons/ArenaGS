use serde::{Deserialize, Serialize};

use super::Strength;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum DamageKind {
    Physical,
    Fire,
    Burning, // Fire, but doesn't change temperature
    Ice,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Damage {
    pub amount: Strength,
    pub kind: DamageKind,
}

#[allow(dead_code)]
impl Damage {
    pub fn init(amount: Strength, kind: DamageKind) -> Damage {
        Damage { amount, kind }
    }

    pub fn physical(amount: u32) -> Damage {
        Damage::init(Strength::init(amount), DamageKind::Physical)
    }

    pub fn fire(amount: u32) -> Damage {
        Damage::init(Strength::init(amount), DamageKind::Fire)
    }

    pub fn burning(amount: u32) -> Damage {
        Damage::init(Strength::init(amount), DamageKind::Burning)
    }

    pub fn ice(amount: u32) -> Damage {
        Damage::init(Strength::init(amount), DamageKind::Ice)
    }

    pub fn dice(&self) -> u32 {
        self.amount.dice
    }
}
