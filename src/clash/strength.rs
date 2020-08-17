use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub const STRENGTH_DICE_SIDES: u32 = 2;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Strength {
    pub dice: u32,
}

impl Strength {
    pub fn init(dice: u32) -> Strength {
        Strength { dice }
    }

    pub fn roll<R: Rng + ?Sized>(&self, rng: &mut R) -> u32 {
        let fixed_count = self.dice / 2;
        let open_count = self.dice - fixed_count;
        let fixed_value = fixed_count * STRENGTH_DICE_SIDES;
        let open_value = rng.gen_range(open_count, (open_count * STRENGTH_DICE_SIDES) + 1);
        fixed_value + open_value
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum DamageKind {
    Physical,
    Fire,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Damage {
    pub amount: Strength,
    pub kind: DamageKind,
}

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

    pub fn dice(&self) -> u32 {
        self.amount.dice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_value() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = Strength::init(14);
        let v = d.roll(&mut rng);
        // 7 * 2 + 7 * (1,2) = 21 - 28
        assert_eq!(true, v >= 21 && v <= 28);
    }
}
