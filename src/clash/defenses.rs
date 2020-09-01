use std::cmp;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::{CharacterInfoComponent, Damage, DamageOptions, EventKind, RolledDamage, Strength};

#[derive(Serialize, Deserialize, Clone)]
pub struct Defenses {
    pub dodge: u32,
    pub max_dodge: u32,
    pub armor: u32,
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
        }
    }

    fn apply_defenses<R: Rng + ?Sized>(&mut self, damage_value: u32, damage: Damage, rng: &mut R) -> u32 {
        if !damage.options.contains(DamageOptions::PIERCE_DEFENSES) {
            // Apply dodge first, burning charges up to matching dice
            let dodge_to_apply = cmp::min(self.dodge, damage.dice());
            self.dodge -= dodge_to_apply;
            let dodge_value = Strength::init(dodge_to_apply).roll(rng);
            let (_, damage_value) = apply_with_remain(damage_value, dodge_value);

            // Then soak with armor, all of it applies
            let armor_value = Strength::init(self.armor).roll(rng);
            let (_, damage_value) = apply_with_remain(damage_value, armor_value);
            damage_value
        } else {
            damage_value
        }
    }

    pub fn apply_damage<R: Rng + ?Sized>(&mut self, damage: Damage, rng: &mut R) -> RolledDamage {
        let damage_value = damage.amount.roll(rng);

        let damage_value = self.apply_defenses(damage_value, damage, rng);

        // Report damage after mitigation applied
        let total_applied_damage = damage_value;

        // Any absorb is burned first
        let (absorb_damage, damage_value) = apply_with_remain(damage_value, self.absorb);
        self.absorb -= absorb_damage;

        // Rest fall to health, ignore any overkill (since they are already dead)
        let (health_damage, _) = apply_with_remain(damage_value, self.health);
        self.health -= health_damage;

        RolledDamage::init(total_applied_damage, &damage)
    }

    pub fn regain_dodge(&mut self, regain: u32) {
        self.dodge = cmp::min(self.dodge + regain, self.max_dodge);
    }
}

pub fn defense_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if let Some(target) = target {
        match kind {
            EventKind::MoveComplete(distance) => {
                if let Some(char_info) = ecs.write_storage::<CharacterInfoComponent>().get_mut(target) {
                    char_info.character.defenses.regain_dodge(2 * distance);
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
    use crate::atlas::{EasyECS, EasyMutECS, SizedPoint};

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
        def.apply_damage(Damage::init(4), &mut rng);
        // (2 * 2) + [2,4] = 6-8 damage
        assert!(def.health > 1);
        assert!(def.health < 5);
    }

    #[test]
    fn no_defense_overkill_leaves_at_zero() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::just_health(10);
        def.apply_damage(Damage::init(10), &mut rng);
        assert_eq!(0, def.health);
    }

    #[test]
    fn dodge_reduces_damage_up_to_strength() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(3, 0, 0, 10);
        def.apply_damage(Damage::init(4), &mut rng);

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
        def.apply_damage(Damage::init(4), &mut rng);
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
        def.apply_damage(Damage::init(4), &mut rng);

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
        def.apply_damage(Damage::init(4), &mut rng);
        assert_eq!(0, def.dodge);
    }

    #[test]
    fn absorb_takes_all_damage_before_life() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::just_health(10);
        def.absorb = 10;
        def.apply_damage(Damage::init(4), &mut rng);
        // (2 * 2) + [2,4] = 6-8 damage
        assert!(def.absorb > 1);
        assert!(def.absorb < 5);
        assert_eq!(10, def.health);
    }

    #[test]
    fn reports_damage_after_mitigation() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(1, 1, 10, 10);
        let applied_damage = def.apply_damage(Damage::init(4), &mut rng);
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
        ecs.write_storage::<CharacterInfoComponent>().grab_mut(player).character.defenses = defenses;
        move_character_action(&mut ecs, player, SizedPoint::init(2, 3));
        wait_for_animations(&mut ecs);
        let dodge = ecs.read_storage::<CharacterInfoComponent>().grab(player).character.defenses.dodge;
        assert_eq!(2, dodge);
    }
}
