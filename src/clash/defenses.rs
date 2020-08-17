use std::cmp;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::{CharacterInfoComponent, Damage, EventKind, MoveState, Strength};

#[derive(Serialize, Deserialize, Clone)]
pub struct Defenses {
    pub dodge: u32,
    pub max_dodge: u32,
    pub armor: u32,
    pub absorb: u32,
    pub health: u32,
}

impl Defenses {
    pub fn init(dodge: u32, max_dodge: u32, armor: u32, absorb: u32, health: u32) -> Defenses {
        Defenses {
            dodge,
            max_dodge,
            armor,
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

    pub fn regain_dodge(&mut self, regain: u32) {
        self.dodge = cmp::min(self.dodge + regain, self.max_dodge);
    }
}

pub fn defense_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if let Some(target) = target {
        match kind {
            EventKind::Move(state) => match state {
                MoveState::Complete(distance) => {
                    if let Some(char_info) = ecs.write_storage::<CharacterInfoComponent>().get_mut(target) {
                        char_info.character.defenses.regain_dodge(2 * distance);
                    }
                }
                _ => {}
            },
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
        let mut def = Defenses::init(0, 0, 0, 0, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);
        // (2 * 2) + [2,4] = 6-8 damage
        assert!(def.health > 1);
        assert!(def.health < 5);
    }

    #[test]
    fn no_defense_overkill_leaves_at_zero() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 0, 0, 0, 10);
        def.apply_damage(Damage::init(Strength::init(10), DamageKind::Physical), &mut rng);
        assert_eq!(0, def.health);
    }

    #[test]
    fn dodge_reduces_damage_up_to_strength() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(3, 3, 0, 0, 10);
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
        let mut def = Defenses::init(5, 5, 0, 0, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);
        assert_eq!(1, def.dodge);
    }

    #[test]
    fn regain_dodge() {
        let mut def = Defenses::init(0, 2, 0, 0, 10);
        def.regain_dodge(3);
        assert_eq!(2, def.dodge);
    }

    #[test]
    fn armor_reduces_every_hit() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 0, 1, 0, 10);
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
        let mut def = Defenses::init(4, 4, 4, 0, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);
        assert_eq!(0, def.dodge);
    }

    #[test]
    fn absorb_takes_all_damage_before_life() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut def = Defenses::init(0, 0, 0, 10, 10);
        def.apply_damage(Damage::init(Strength::init(4), DamageKind::Physical), &mut rng);
        // (2 * 2) + [2,4] = 6-8 damage
        assert!(def.absorb > 1);
        assert!(def.absorb < 5);
        assert_eq!(10, def.health);
    }

    #[test]
    fn dodge_restored_by_movement() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        ecs.write_storage::<CharacterInfoComponent>().grab_mut(player).character.defenses = Defenses::init(0, 10, 0, 0, 10);
        move_character_action(&mut ecs, player, SizedPoint::init(2, 3));
        wait_for_animations(&mut ecs);
        let dodge = ecs.read_storage::<CharacterInfoComponent>().grab(player).character.defenses.dodge;
        assert_eq!(2, dodge);
    }
}
