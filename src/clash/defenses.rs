use std::cmp;
use std::collections::HashMap;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::{Damage, DamageElement, DamageOptions, DefenseComponent, EventKind, RolledDamage, Strength};

#[derive(Serialize, Deserialize, Clone)]
pub struct Resistances {
    resistances: HashMap<DamageElement, u32>,
}

impl Resistances {
    pub fn empty() -> Resistances {
        Resistances { resistances: HashMap::new() }
    }

    pub fn init(resistances: &[(DamageElement, u32)]) -> Resistances {
        Resistances {
            resistances: resistances.iter().map(|e| (e.0, e.1)).collect(),
        }
    }

    pub fn get(&self, kind: DamageElement) -> u32 {
        *self.resistances.get(&kind).unwrap_or(&0)
    }

    pub fn all(&self) -> Vec<(DamageElement, u32)> {
        self.resistances.iter().map(|r| (*r.0, *r.1)).collect()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Defenses {
    pub dodge: u32,
    pub max_dodge: u32,
    pub armor: u32,
    pub resistances: Resistances,
    pub absorb: u32,
    pub health: u32,
    pub max_health: u32,
}

impl Defenses {
    #[allow(dead_code)]
    pub fn init(dodge: u32, armor: u32, absorb: u32, health: u32) -> Defenses {
        Defenses {
            dodge,
            max_dodge: dodge,
            armor,
            absorb,
            health,
            max_health: health,
            resistances: Resistances::empty(),
        }
    }

    pub fn just_health(health: u32) -> Defenses {
        Defenses {
            dodge: 0,
            max_dodge: 0,
            armor: 0,
            absorb: 0,
            health,
            max_health: health,
            resistances: Resistances::empty(),
        }
    }

    pub fn with_resistances(mut self, resistances: Resistances) -> Defenses {
        self.resistances = resistances;
        self
    }

    fn apply_defenses<R: Rng + ?Sized>(&mut self, damage_value: u32, damage: Damage, additional_armor: u32, rng: &mut R) -> (u32, u32, u32, u32) {
        if !damage.options.contains(DamageOptions::PIERCE_DEFENSES) {
            // Apply dodge first, burning charges up to matching dice
            let dodge_to_apply = cmp::min(self.dodge, damage.dice());
            self.dodge -= dodge_to_apply;
            let dodge_value = Strength::init(dodge_to_apply).roll(rng);
            let (_, damage_value) = apply_with_remain(damage_value, dodge_value);

            // Apply resistance or armor to that section of damage
            let (armor_value, resist_value, damage_value) = self.apply_resistances(damage, damage_value, additional_armor, rng);

            (dodge_value, armor_value, resist_value, damage_value)
        } else {
            (0, 0, 0, damage_value)
        }
    }

    fn apply_resistances<R: Rng + ?Sized>(&self, damage: Damage, mut damage_value: u32, additional_armor: u32, rng: &mut R) -> (u32, u32, u32) {
        // After applying dodge, divide the damage by the number of DamageElements
        let total_damage_types = damage.element.count();
        let damage_to_soak_per_type = (damage_value as f64 / total_damage_types as f64).floor() as u32;

        let mut resist_value = 0;
        let mut armor_value = 0;

        for damage_element in damage.element.components() {
            let is_physical = damage_element == DamageElement::PHYSICAL;
            let resistance_strength = if is_physical {
                self.armor + additional_armor
            } else {
                self.resistances.get(damage_element)
            };
            let resistance_amount = Strength::init(resistance_strength).roll(rng);
            let resistance_amount = cmp::min(resistance_amount, damage_to_soak_per_type);
            if is_physical {
                armor_value += resistance_amount;
            } else {
                resist_value += resistance_amount;
            }
            damage_value = apply_with_remain(damage_value, resistance_amount).1;
        }
        (armor_value, resist_value, damage_value)
    }

    pub fn apply_damage<R: Rng + ?Sized>(&mut self, damage: Damage, rng: &mut R) -> RolledDamage {
        self.apply_damage_with_additional_armor(damage, 0, rng)
    }

    pub fn apply_damage_with_additional_armor<R: Rng + ?Sized>(&mut self, damage: Damage, additional_armor: u32, rng: &mut R) -> RolledDamage {
        let damage_value = damage.amount.roll(rng);

        let (absorbed_by_dodge, absorbed_by_armor, absorbed_by_resist, damage_value) = self.apply_defenses(damage_value, damage, additional_armor, rng);

        // Report damage after mitigation applied
        let total_applied_damage = damage_value;

        // Any absorb is burned first
        let (absorb_damage, damage_value) = apply_with_remain(damage_value, self.absorb);
        self.absorb -= absorb_damage;

        // Rest fall to health, ignore any overkill (since they are already dead)
        let (health_damage, _) = apply_with_remain(damage_value, self.health);
        self.health -= health_damage;

        RolledDamage::init(absorbed_by_dodge, absorbed_by_armor, absorbed_by_resist, total_applied_damage, &damage.options)
    }

    pub fn regain_dodge(&mut self, regain: u32) {
        self.dodge = cmp::min(self.dodge + regain, self.max_dodge);
    }
}

pub fn defense_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if let Some(target) = target {
        match kind {
            EventKind::MoveComplete(distance) => {
                if let Some(defense) = ecs.write_storage::<DefenseComponent>().get_mut(target) {
                    defense.defenses.regain_dodge(2 * distance);
                }
            }
            _ => {}
        }
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
    use super::super::*;
    use super::*;
    use crate::atlas::prelude::*;

    #[test]
    fn apply_remain() {
        assert_eq!((2, 0), apply_with_remain(2, 5));
        assert_eq!((5, 0), apply_with_remain(5, 5));
        assert_eq!((5, 5), apply_with_remain(10, 5));
    }

    #[test]
    fn no_defense_full_damage() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::just_health(10);
        def.apply_damage(Damage::init(4, DamageElement::PHYSICAL), &mut rng);
        // (2 * 2) + [2,4] = 6-8 damage
        assert!(def.health > 1);
        assert!(def.health < 5);
    }

    #[test]
    fn no_defense_overkill_leaves_at_zero() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::just_health(10);
        def.apply_damage(Damage::init(10, DamageElement::PHYSICAL), &mut rng);
        assert_eq!(0, def.health);
    }

    #[test]
    fn dodge_reduces_damage_up_to_strength() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(3, 0, 0, 10);
        def.apply_damage(Damage::init(4, DamageElement::PHYSICAL), &mut rng);

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
        def.apply_damage(Damage::init(4, DamageElement::PHYSICAL), &mut rng);
        assert_eq!(1, def.dodge);
    }

    #[test]
    fn regain_dodge() {
        let mut def = Defenses::just_health(10);
        def.max_dodge = 2;
        def.regain_dodge(3);
        assert_eq!(2, def.dodge);
    }

    #[test]
    fn armor_reduces_every_hit() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 1, 0, 10);
        def.apply_damage(Damage::init(4, DamageElement::PHYSICAL), &mut rng);

        // (2 * 2) + [2,4] = 6-8 damage
        // 1-2 armor
        // 4 - 7 damage taken
        assert!(def.health > 2);
        assert_eq!(1, def.armor);
    }

    #[test]
    fn additional_armor() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 0, 0, 10);
        def.apply_damage_with_additional_armor(Damage::init(4, DamageElement::PHYSICAL), 4, &mut rng);

        // (2 * 2) + [2,4] = 6-8 damage
        // 4-8 armor
        // 0 - 4 damage taken
        assert!(def.health > 5);
        assert_eq!(0, def.armor);
    }

    #[test]
    fn dodge_used_even_if_armor_would_cover() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(4, 4, 0, 10);
        def.apply_damage(Damage::init(4, DamageElement::PHYSICAL), &mut rng);
        assert_eq!(0, def.dodge);
    }

    #[test]
    fn absorb_takes_all_damage_before_life() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::just_health(10);
        def.absorb = 10;
        def.apply_damage(Damage::init(4, DamageElement::PHYSICAL), &mut rng);
        // (2 * 2) + [2,4] = 6-8 damage
        assert!(def.absorb > 1);
        assert!(def.absorb < 5);
        assert_eq!(10, def.health);
    }

    #[test]
    fn reports_damage_after_mitigation() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(1, 1, 10, 10);
        let applied_damage = def.apply_damage(Damage::init(4, DamageElement::PHYSICAL), &mut rng);
        // (2 * 2) + [2,4] = 6-8 damage
        // Minus 2-4 Dodge/armor
        // 2-6 damage
        assert!(applied_damage.amount >= 2);
        assert!(applied_damage.amount <= 6);
    }

    #[test]
    fn dodge_restored_by_movement() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let mut defenses = Defenses::just_health(10);
        defenses.max_dodge = 5;
        ecs.write_storage::<DefenseComponent>().grab_mut(player).defenses = defenses;
        move_character_action(&mut ecs, player, SizedPoint::init(2, 3));
        wait_for_animations(&mut ecs);
        let dodge = ecs.read_storage::<DefenseComponent>().grab(player).defenses.dodge;
        assert_eq!(2, dodge);
    }

    #[test]
    fn resistance_applies() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 0, 0, 20).with_resistances(Resistances::init(&[(DamageElement::FIRE, 10)]));
        let applied_damage = def.apply_damage(Damage::init(4, DamageElement::FIRE), &mut rng);
        assert_eq!(0, applied_damage.amount);
        assert!(applied_damage.absorbed_by_resist > 0);
    }

    #[test]
    fn mixed_resistance_applied_both() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 1, 0, 20).with_resistances(Resistances::init(&[(DamageElement::FIRE, 10)]));
        let applied_damage = def.apply_damage(Damage::init(5, DamageElement::PHYSICAL | DamageElement::FIRE), &mut rng);
        assert!(applied_damage.amount > 0);
        assert!(applied_damage.absorbed_by_resist > 0);
        assert!(applied_damage.absorbed_by_armor > 0);
    }
}
