use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::*;
use crate::atlas::{EasyECS, EasyMutWorld, Point, SizedPoint};
use crate::clash::{EventCoordinator, FieldComponent};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum FieldEffect {
    Damage(Damage),
    Spawn(SpawnKind),
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum FieldKind {
    Fire,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum BoltKind {
    Fire,
    Bullet,
    FireBullet,
    AirBullet,
    Smoke,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum WeaponKind {
    Sword,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum OrbKind {
    Feather,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum AttackKind {
    Ranged(BoltKind),
    Melee(WeaponKind),
    Explode(u32),
    Orb(OrbKind),
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct AttackInfo {
    pub damage: Damage,
    pub target: Point,
    pub kind: AttackKind,
    pub source: Option<Point>,
}

impl AttackInfo {
    pub fn ranged_kind(&self) -> BoltKind {
        match self.kind {
            AttackKind::Ranged(kind) => kind,
            _ => panic!("Wrong type in ranged_kind"),
        }
    }

    pub fn melee_kind(&self) -> WeaponKind {
        match self.kind {
            AttackKind::Melee(kind) => kind,
            _ => panic!("Wrong type in melee_kind"),
        }
    }

    pub fn orb_kind(&self) -> OrbKind {
        match self.kind {
            AttackKind::Orb(kind) => kind,
            _ => panic!("Wrong type in orb_kind"),
        }
    }
}

impl AttackComponent {
    pub fn init(target: Point, damage: Damage, kind: AttackKind, source: Option<Point>) -> AttackComponent {
        AttackComponent {
            attack: AttackInfo { target, damage, kind, source },
        }
    }
}

pub fn begin_bolt_nearest_in_range(ecs: &mut World, source: &Entity, range: Option<u32>, strength: Damage, kind: BoltKind) {
    let targets = {
        if find_player(ecs) == *source {
            find_enemies(&ecs)
        } else {
            // ALLIES_TODO
            vec![find_player(&ecs)]
        }
    };
    let source_position = ecs.get_position(source);
    let target = targets.iter().min_by(|first, second| {
        let first = source_position.distance_to_multi(ecs.get_position(first));
        let second = source_position.distance_to_multi(ecs.get_position(second));
        first.cmp(&second)
    });
    if let Some(target) = target {
        if let Some((_, target_position, distance)) = source_position.distance_to_multi_with_endpoints(ecs.get_position(target)) {
            if range.is_none() || range.unwrap() > distance {
                begin_bolt(ecs, source, target_position, strength, kind);
            }
        }
    }
}

pub fn begin_bolt(ecs: &mut World, source: &Entity, target_position: Point, mut strength: Damage, kind: BoltKind) {
    if ecs.has_status(source, StatusKind::Aimed) {
        ecs.remove_status(source, StatusKind::Aimed);
        strength = strength.copy_more_strength(2);
    }

    let source_position = Some(ecs.get_position(source).origin);
    ecs.shovel(
        *source,
        AttackComponent::init(target_position, strength, AttackKind::Ranged(kind), source_position),
    );
    ecs.raise_event(EventKind::Bolt(BoltState::BeginCastAnimation), Some(*source));
}

pub fn bolt_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Bolt(state) => match state {
            BoltState::CompleteCastAnimation => start_bolt(ecs, target.unwrap()),
            BoltState::CompleteFlyingAnimation => apply_bolt(ecs, target.unwrap()),
            _ => {}
        },
        _ => {}
    }
}

pub fn start_bolt(ecs: &mut World, source: Entity) {
    // We must re-create a position using the origin so multi-sized
    // monsters don't create bolts with large widths
    let caster_origin = ecs.get_position(&source).origin;
    let source_position = SizedPoint::from(caster_origin);

    let attack = ecs.read_storage::<AttackComponent>().grab(source).attack;

    let bolt = ecs
        .create_entity()
        .with(PositionComponent::init(source_position))
        .with(AttackComponent { attack })
        .build();

    ecs.write_storage::<AttackComponent>().remove(source);
    ecs.raise_event(EventKind::Bolt(BoltState::BeginFlyingAnimation), Some(bolt));
}

pub fn apply_bolt(ecs: &mut World, bolt: Entity) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.grab(bolt).attack
    };
    apply_damage_to_location(ecs, attack.target, attack.source, attack.damage);
    ecs.delete_entity(bolt).unwrap();
}

pub fn begin_melee(ecs: &mut World, source: &Entity, target: Point, strength: Damage, kind: WeaponKind) {
    let source_position = Some(ecs.get_position(source).origin);
    ecs.shovel(*source, AttackComponent::init(target, strength, AttackKind::Melee(kind), source_position));
    ecs.raise_event(EventKind::Melee(MeleeState::BeginAnimation), Some(*source));
}

pub fn melee_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Melee(state) if state.is_complete_animation()) {
        apply_melee(ecs, target.unwrap());
    }
}

pub fn apply_melee(ecs: &mut World, character: Entity) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.grab(character).attack
    };
    apply_damage_to_location(ecs, attack.target, attack.source, attack.damage);

    ecs.write_storage::<AttackComponent>().remove(character);
}

pub fn begin_field(ecs: &mut World, source: &Entity, target: Point, effect: FieldEffect, kind: FieldKind) {
    ecs.shovel(*source, FieldCastComponent::init(effect, kind, SizedPoint::from(target)));
    ecs.raise_event(EventKind::Field(FieldState::BeginCastAnimation), Some(*source));
}

pub fn field_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Field(state) => match state {
            FieldState::CompleteCastAnimation => start_field(ecs, target.unwrap()),
            FieldState::CompleteFlyingAnimation => apply_field(ecs, target.unwrap()),
            _ => {}
        },
        _ => {}
    }
}

pub fn start_field(ecs: &mut World, source: Entity) {
    let cast = ecs.read_storage::<FieldCastComponent>().grab(source).clone();
    ecs.write_storage::<FieldCastComponent>().remove(source);

    // Fields can be fired by flying entities, skip animation if there is no Position
    if ecs.read_storage::<PositionComponent>().get(source).is_none() {
        let field_projectile = ecs.create_entity().with(cast).build();
        apply_field(ecs, field_projectile);
    } else {
        let source_position = ecs.get_position(&source);
        let field_projectile = ecs.create_entity().with(PositionComponent::init(source_position)).with(cast).build();
        ecs.raise_event(EventKind::Field(FieldState::BeginFlyingAnimation), Some(field_projectile));
    }
}

pub fn apply_field(ecs: &mut World, projectile: Entity) {
    let cast = ecs.read_storage::<FieldCastComponent>().grab(projectile).clone();
    ecs.delete_entity(projectile).unwrap();

    match cast.effect {
        FieldEffect::Damage(damage) => {
            let (r, g, b) = match cast.kind {
                FieldKind::Fire => (255, 0, 0),
            };

            ecs.create_entity()
                .with(PositionComponent::init(cast.target))
                .with(AttackComponent::init(cast.target.origin, damage, AttackKind::Explode(0), None))
                .with(BehaviorComponent::init(BehaviorKind::Explode))
                .with(FieldComponent::init(r, g, b))
                .with(TimeComponent::init(-BASE_ACTION_COST))
                .build();
        }
        FieldEffect::Spawn(kind) => spawn(ecs, cast.target, kind),
    }
}

pub fn begin_explode(ecs: &mut World, source: &Entity) {
    ecs.raise_event(EventKind::Explode(ExplodeState::BeginAnimation), Some(*source));
}

pub fn explode_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Explode(state) if state.is_complete_animation()) {
        apply_explode(ecs, target.unwrap());
    }
}

pub fn apply_explode(ecs: &mut World, source: Entity) {
    let (damage, range, source_position) = {
        let attack_info = ecs.read_storage::<AttackComponent>().grab(source).attack;
        match attack_info.kind {
            AttackKind::Explode(range) => (attack_info.damage, range, attack_info.source),
            _ => panic!("Explode with wrong AttackKind"),
        }
    };

    for in_blast in ecs.get_position(&source).origin.get_burst(range) {
        if let Some(target) = find_character_at_location(ecs, in_blast) {
            if target != source {
                apply_damage_to_location(ecs, in_blast, source_position, damage);
            }
        }
    }

    ecs.delete_entity(source).unwrap();
}

pub fn begin_shoot_and_move(ecs: &mut World, source: &Entity, new_position: SizedPoint, range: Option<u32>, strength: Damage, kind: BoltKind) {
    begin_move(ecs, source, new_position, PostMoveAction::Shoot(strength, range, kind));
}

pub fn reap_killed(ecs: &mut World) {
    let mut dead = vec![];
    let mut player_dead = false;
    {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let character_infos = ecs.read_storage::<CharacterInfoComponent>();
        let players = ecs.read_storage::<PlayerComponent>();

        for (entity, character_info, player) in (&entities, &character_infos, (&players).maybe()).join() {
            if character_info.character.defenses.health == 0 {
                // We do not remove the player on death, as the UI assumes existance (and may paint before tick)
                if player.is_some() {
                    player_dead = true;
                } else {
                    dead.push(entity);
                }
            }
        }
    }

    if player_dead {
        ecs.insert(PlayerDeadComponent::init());
    }
    for d in dead {
        ecs.delete_entity(d).unwrap();
    }
}

pub fn begin_orb(ecs: &mut World, source: &Entity, target_position: Point, strength: Damage, kind: OrbKind, speed: u32, duration: u32) {
    let source_position = ecs.get_position(source);
    // Need to use extend line to extend to duration if too little
    let path = source_position.line_to(target_position).unwrap();
    ecs.shovel(*source, OrbComponent::init(path, speed, duration));
    ecs.shovel(
        *source,
        AttackComponent::init(target_position, strength, AttackKind::Orb(kind), Some(source_position.origin)),
    );
    ecs.raise_event(EventKind::Orb(OrbState::BeginCastAnimation), Some(*source));
}

pub fn orb_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Orb(state) => match state {
            OrbState::CompleteCastAnimation => start_orb(ecs, target.unwrap()),
            _ => {}
        },
        _ => {}
    }
}

pub fn start_orb(ecs: &mut World, source: Entity) {
    let orb = create_orb(ecs, &source);
    ecs.write_storage::<AttackComponent>().remove(source);
    ecs.write_storage::<OrbComponent>().remove(source);
    ecs.raise_event(EventKind::Orb(OrbState::Created), Some(orb));
}

pub fn apply_orb(ecs: &mut World, orb: Entity, point: Point) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.grab(orb).attack
    };
    apply_damage_to_location(ecs, point, attack.source, attack.damage);
    ecs.delete_entity(orb).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn melee_hits() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(3, 2, 10).build();
        let player = find_player(&ecs);
        let target = find_at(&ecs, 3, 2);
        let starting_health = ecs.get_defenses(&target).health;

        begin_melee(&mut ecs, &player, Point::init(3, 2), Damage::init(1), WeaponKind::Sword);
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(&target).health < starting_health);
    }

    #[test]
    fn ranged_hits() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(4, 2, 10).build();
        let player = find_player(&ecs);
        let target = find_at(&ecs, 4, 2);
        let starting_health = ecs.get_defenses(&target).health;

        begin_bolt(&mut ecs, &player, Point::init(4, 2), Damage::init(1), BoltKind::Fire);
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(&target).health < starting_health);
    }

    #[test]
    fn explode_hits() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_character(2, 3, 0).with_map().build();
        let target = find_at(&ecs, 2, 2);
        let exploder = find_at(&ecs, 2, 3);
        let starting_health = ecs.get_defenses(&target).health;
        ecs.shovel(
            exploder,
            AttackComponent::init(Point::init(2, 3), Damage::init(2), AttackKind::Explode(1), Some(Point::init(2, 3))),
        );
        begin_explode(&mut ecs, &exploder);
        wait_for_animations(&mut ecs);

        assert!(ecs.get_defenses(&target).health < starting_health);
    }

    fn get_field_at(ecs: &World, target: &Point) -> Option<Entity> {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let positions = ecs.read_storage::<PositionComponent>();
        let fields = ecs.read_storage::<FieldComponent>();

        for (entity, position, _) in (&entities, &positions, &fields).join() {
            if *target == position.position.origin {
                return Some(entity);
            }
        }
        None
    }

    #[test]
    fn field_is_placed() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_field(&mut ecs, &player, Point::init(2, 3), FieldEffect::Damage(Damage::init(1)), FieldKind::Fire);
        wait_for_animations(&mut ecs);

        assert_eq!(true, get_field_at(&ecs, &Point::init(2, 3)).is_some());
    }

    #[test]
    fn field_is_placed_without_position() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        // Some conditions, like flying can remove position temporarly. They should still be able to make fields
        ecs.write_storage::<PositionComponent>().remove(player);

        begin_field(&mut ecs, &player, Point::init(2, 3), FieldEffect::Damage(Damage::init(1)), FieldKind::Fire);
        wait_for_animations(&mut ecs);

        assert_eq!(true, get_field_at(&ecs, &Point::init(2, 3)).is_some());
    }

    #[test]
    fn killed_enemies_removed() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        begin_bolt(&mut ecs, &player, Point::init(2, 3), Damage::init(10), BoltKind::Fire);
        wait_for_animations(&mut ecs);

        assert_eq!(1, find_all_entities(&ecs).len());
    }

    #[test]
    fn killed_player() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 3, 0).with_map().build();
        let enemy = find_at(&ecs, 2, 3);
        begin_bolt(&mut ecs, &enemy, Point::init(2, 2), Damage::init(10), BoltKind::Fire);
        wait_for_animations(&mut ecs);

        // We do not remove the player on death, as the UI assumes existance (and may paint before tick)
        assert_eq!(2, find_all_entities(&ecs).len());
        let _ = ecs.fetch::<PlayerDeadComponent>();
    }

    #[test]
    fn move_and_shoot() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);
        let starting_health = ecs.get_defenses(&target).health;

        begin_shoot_and_move(&mut ecs, &player, SizedPoint::init(2, 1), Some(5), Damage::init(1), BoltKind::Bullet);
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &player, Point::init(2, 1));
        assert!(ecs.get_defenses(&target).health < starting_health);
    }

    #[test]
    fn move_and_shoot_multiple_targets() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 100)
            .with_character(2, 3, 0)
            .with_character(2, 4, 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 3);
        let other = find_at(&ecs, 2, 4);
        let starting_health = ecs.get_defenses(&target).health;

        begin_shoot_and_move(&mut ecs, &player, SizedPoint::init(2, 1), None, Damage::init(1), BoltKind::Bullet);
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &player, Point::init(2, 1));
        assert!(ecs.get_defenses(&target).health < starting_health);
        assert_eq!(ecs.get_defenses(&other).health, starting_health);
    }

    #[test]
    fn move_and_shoot_out_of_range() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 100)
            .with_character(2, 6, 0)
            .with_character(2, 7, 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 6);
        let other = find_at(&ecs, 2, 7);
        let starting_health = ecs.get_defenses(&target).health;

        begin_shoot_and_move(&mut ecs, &player, SizedPoint::init(2, 1), Some(5), Damage::init(1), BoltKind::Bullet);
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &player, Point::init(2, 1));
        assert_eq!(ecs.get_defenses(&target).health, starting_health);
        assert_eq!(ecs.get_defenses(&other).health, starting_health);
    }

    #[test]
    fn orb_hits_target_on_time() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 6);
        let starting_health = ecs.get_defenses(&target).health;

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);
        let orb = find_entity_at(&ecs, 2, 3);

        new_turn_wait_characters(&mut ecs);
        assert_position(&ecs, &orb, Point::init(2, 5));

        new_turn_wait_characters(&mut ecs);
        assert!(ecs.get_defenses(&target).health < starting_health);
        assert_eq!(0, ecs.read_storage::<OrbComponent>().count());
    }

    #[test]
    fn fast_orb_does_not_overshoot() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 6);
        let starting_health = ecs.get_defenses(&target).health;

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 10, 12);
        wait_for_animations(&mut ecs);
        find_entity_at(&ecs, 2, 3); // Crashes if not in expected positions

        new_turn_wait_characters(&mut ecs);
        assert!(ecs.get_defenses(&target).health < starting_health);
        assert_eq!(0, ecs.read_storage::<OrbComponent>().count());
    }

    #[test]
    fn orb_misses_moving_target() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 6);
        let starting_health = ecs.get_defenses(&target).health;

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);
        let orb = find_entity_at(&ecs, 2, 3);

        new_turn_wait_characters(&mut ecs);
        assert_position(&ecs, &orb, Point::init(2, 5));

        begin_move(&mut ecs, &target, SizedPoint::init(2, 7), PostMoveAction::None);
        wait_for_animations(&mut ecs);

        new_turn_wait_characters(&mut ecs);
        assert_eq!(ecs.get_defenses(&target).health, starting_health);
        assert_eq!(0, ecs.read_storage::<OrbComponent>().count());
    }

    #[test]
    fn orb_hits_target_en_route() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 0)
            .with_character(3, 6, 0)
            .with_character(2, 6, 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let bystander = find_at(&ecs, 3, 6);
        let target = find_at(&ecs, 2, 6);
        let target_starting_health = ecs.get_defenses(&target).health;
        let bystander_starting_health = ecs.get_defenses(&bystander).health;

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);
        let orb = find_entity_at(&ecs, 2, 3);

        new_turn_wait_characters(&mut ecs);
        assert_position(&ecs, &orb, Point::init(2, 5));

        begin_move(&mut ecs, &target, SizedPoint::init(1, 6), PostMoveAction::None);
        begin_move(&mut ecs, &bystander, SizedPoint::init(2, 6), PostMoveAction::None);
        wait_for_animations(&mut ecs);

        new_turn_wait_characters(&mut ecs);
        assert!(ecs.get_defenses(&bystander).health < bystander_starting_health);
        assert_eq!(ecs.get_defenses(&target).health, target_starting_health);
    }

    #[test]
    fn orb_hit_first_target_only() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 0)
            .with_character(2, 6, 0)
            .with_character(2, 7, 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let bystander = find_at(&ecs, 2, 6);
        let target = find_at(&ecs, 2, 7);
        let target_starting_health = ecs.get_defenses(&target).health;
        let bystander_starting_health = ecs.get_defenses(&bystander).health;

        begin_orb(&mut ecs, &player, Point::init(2, 7), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);
        let orb = find_entity_at(&ecs, 2, 3);

        new_turn_wait_characters(&mut ecs);
        assert_position(&ecs, &orb, Point::init(2, 5));

        new_turn_wait_characters(&mut ecs);
        assert!(ecs.get_defenses(&bystander).health < bystander_starting_health);
        assert_eq!(ecs.get_defenses(&target).health, target_starting_health);
    }

    #[test]
    fn orb_walk_into_orb() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 0)
            .with_character(2, 6, 0)
            .with_character(1, 5, 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 6);
        let bystander = find_at(&ecs, 1, 5);
        let target_starting_health = ecs.get_defenses(&target).health;
        let bystander_starting_health = ecs.get_defenses(&bystander).health;

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);
        let orb = find_entity_at(&ecs, 2, 3);

        new_turn_wait_characters(&mut ecs);
        assert_position(&ecs, &orb, Point::init(2, 5));
        begin_move(&mut ecs, &bystander, SizedPoint::init(2, 5), PostMoveAction::None);
        wait_for_animations(&mut ecs);

        new_turn_wait_characters(&mut ecs);
        assert!(ecs.get_defenses(&bystander).health < bystander_starting_health);
        assert_eq!(ecs.get_defenses(&target).health, target_starting_health);
    }

    #[test]
    fn orb_from_multi_square_sourced() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 0)
            .with_sized_character(SizedPoint::init_multi(2, 6, 2, 2), 0)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let enemy = find_at(&ecs, 2, 6);
        let player_starting_health = ecs.get_defenses(&player).health;

        begin_orb(&mut ecs, &enemy, Point::init(2, 2), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);

        new_turn_wait_characters(&mut ecs);
        assert!(ecs.get_defenses(&player).health < player_starting_health);
    }
}
