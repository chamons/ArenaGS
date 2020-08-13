use super::Strength;
use rand::prelude::*;

enum DamageKind {
    Physical,
}

struct Damage {
    amount: Strength,
    kind: DamageKind,
}

impl Damage {
    pub fn init(amount: Strength, kind: DamageKind) -> Damage {
        Damage { amount, kind }
    }
}

struct Defenses {
    dodge: Strength,
    armor: Strength,
    absorb: u32,
    health: u32,
}

impl Defenses {
    pub fn init(dodge: u32, armor: u32, absorb: u32, health: u32) -> Defenses {
        Defenses {
            dodge: Strength::init(dodge),
            armor: Strength::init(armor),
            absorb,
            health,
        }
    }

    pub fn apply_damage<R: Rng + ?Sized>(&mut self, damage: Damage, rng: &mut R) {
        let damage = damage.amount.roll(rng);

        let (health_damage, _) = apply_with_remain(damage, self.health);
        self.health -= health_damage;
    }
}

fn apply_with_remain(to_apply: u32, pool: u32) -> (u32, u32) {
    if to_apply <= pool {
        (to_apply, 0)
    } else {
        (pool, to_apply - pool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_remain() {
        assert_eq!((2, 0), apply_with_remain(2, 5));
        assert_eq!((5, 0), apply_with_remain(5, 5));
        assert_eq!((5, 5), apply_with_remain(10, 5));
    }

    #[test]
    fn no_defense_full_damage() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 0, 0, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);
        assert!(def.health > 0);
        assert!(def.health < 5);
    }

    #[test]
    fn no_defense_overkill_leaves_at_zero() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 0, 0, 10);
        def.apply_damage(Damage::init(Strength::init(10), DamageKind::Physical), &mut rng);
        assert_eq!(0, def.health);
    }

    #[test]
    fn dodge_reduces_damage_up_to_strength() {}

    #[test]
    fn extra_dodge_saved_after_roll() {}

    #[test]
    fn regain_dodge() {}

    #[test]
    fn armor_reduces_every_hit() {}

    #[test]
    fn absorb_takes_all_damage_before_life() {}
}
