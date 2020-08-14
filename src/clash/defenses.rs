use std::cmp;

use rand::prelude::*;

use super::Strength;

enum DamageKind {
    Physical,
}

struct Damage {
    pub amount: Strength,
    pub kind: DamageKind,
}

impl Damage {
    pub fn init(amount: Strength, kind: DamageKind) -> Damage {
        Damage { amount, kind }
    }

    pub fn dice(&self) -> u32 {
        self.amount.dice
    }
}

struct Defenses {
    dodge: u32,
    armor: u32,
    absorb: u32,
    health: u32,
}

impl Defenses {
    pub fn init(dodge: u32, armor: u32, absorb: u32, health: u32) -> Defenses {
        Defenses {
            dodge: dodge,
            armor: armor,
            absorb,
            health,
        }
    }

    pub fn apply_damage<R: Rng + ?Sized>(&mut self, damage: Damage, rng: &mut R) {
        let damage_value = damage.amount.roll(rng);

        // Apply dodge first, burning charges up to matching dice
        let dodge_to_apply = cmp::min(self.dodge, damage.dice());
        self.dodge -= dodge_to_apply;
        let dodge_value = Strength::init(dodge_to_apply).roll(rng);
        let (_, damage_value) = apply_with_remain(damage_value, dodge_value);

        // Then soak with armor, all of it applies
        let armor_value = Strength::init(self.armor).roll(rng);
        let (_, damage_value) = apply_with_remain(damage_value, armor_value);

        // Any absorb is burned first
        let (absorb_damage, damage_value) = apply_with_remain(damage_value, self.absorb);
        self.absorb -= absorb_damage;

        // Rest fall to health, ignore any overkill (since they are already dead)
        let (health_damage, _) = apply_with_remain(damage_value, self.health);
        self.health -= health_damage;
    }

    pub fn regain_dodge(&mut self, regain: u32, max: u32) {
        self.dodge = cmp::min(self.dodge + regain, max);
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
        // (2 * 2) + [2,4] = 6-8 damage
        assert!(def.health > 1);
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
    fn dodge_reduces_damage_up_to_strength() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(3, 0, 0, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);

        // (2 * 2) + [2,4] = 6-8 damage
        // 2 + [2,4] = 4-6 dodge
        // 0 - 4 damage taken
        assert!(def.health > 5);
        assert_eq!(0, def.dodge);
    }

    #[test]
    fn extra_dodge_saved_after_roll() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(5, 0, 0, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);
        assert_eq!(1, def.dodge);
    }

    #[test]
    fn regain_dodge() {
        let mut def = Defenses::init(5, 0, 0, 10);
        def.regain_dodge(3, 2);
        assert_eq!(2, def.dodge);
    }

    #[test]
    fn armor_reduces_every_hit() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 1, 0, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);

        // (2 * 2) + [2,4] = 6-8 damage
        // 1-2 armor
        // 4 - 7 damage taken
        assert!(def.health > 2);
        assert_eq!(1, def.armor);
    }

    #[test]
    fn dodge_used_even_if_armor_would_cover() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(4, 4, 0, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);
        assert_eq!(0, def.dodge);
    }

    #[test]
    fn absorb_takes_all_damage_before_life() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 0, 10, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);
        // (2 * 2) + [2,4] = 6-8 damage
        assert!(def.absorb > 1);
        assert!(def.absorb < 5);
        assert_eq!(10, def.health);
    }
}
