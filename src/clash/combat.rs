use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::content::spawner;
use super::*;
use crate::atlas::{extend_line_along_path, Direction, EasyECS, EasyMutWorld, Point, SizedPoint};
use crate::clash::EventCoordinator;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum FieldEffect {
    Damage(Damage, u32),
    Spawn(SpawnKind),
    SustainedDamage(Damage, u32),
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum FieldKind {
    Fire,
    Hail,
    Lightning,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum ConeKind {
    Fire,
    Water,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum BoltKind {
    Fire,
    Water,
    Lightning,
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
pub enum ExplosionKind {
    Fire,
    Bomb,
    Lightning,
    Cloud,
    Water,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum AttackKind {
    Ranged(BoltKind),
    Melee(WeaponKind),
    Cone(ConeKind, u32),
    Explode(ExplosionKind, u32),
    DamageTick,
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
    pub fn melee_kind(&self) -> WeaponKind {
        match self.kind {
            AttackKind::Melee(kind) => kind,
            _ => panic!("Wrong type in melee_kind"),
        }
    }

    pub fn ranged_kind(&self) -> BoltKind {
        match self.kind {
            AttackKind::Ranged(kind) => kind,
            _ => panic!("Wrong type in ranged_kind"),
        }
    }

    pub fn cone_kind(&self) -> ConeKind {
        match self.kind {
            AttackKind::Cone(kind, _) => kind,
            _ => panic!("Wrong type in cone_kind"),
        }
    }

    pub fn orb_kind(&self) -> OrbKind {
        match self.kind {
            AttackKind::Orb(kind) => kind,
            _ => panic!("Wrong type in orb_kind"),
        }
    }

    pub fn explode_kind(&self) -> ExplosionKind {
        match self.kind {
            AttackKind::Explode(kind, _) => kind,
            _ => panic!("Wrong type in explode_kind"),
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
            // ALLIES_TODO -  https://github.com/chamons/ArenaGS/issues/201
            vec![find_player(&ecs)]
        }
    };
    let source_position = ecs.get_position(source);
    let target = targets.iter().filter(|t| !ecs.has_status(t, StatusKind::Flying)).min_by(|first, second| {
        let first = source_position.distance_to_multi(ecs.get_position(first));
        let second = source_position.distance_to_multi(ecs.get_position(second));
        first.cmp(&second)
    });
    if let Some(target) = target {
        if let Some((_, target_position, distance)) = source_position.distance_to_multi_with_endpoints(ecs.get_position(target)) {
            if range.is_none() || range.unwrap() >= distance {
                begin_bolt(ecs, source, target_position, strength, kind);
            }
        }
    }
}

pub fn begin_bolt(ecs: &mut World, source: &Entity, target_position: Point, mut strength: Damage, kind: BoltKind) {
    if ecs.has_status(source, StatusKind::Aimed) {
        ecs.remove_status(source, StatusKind::Aimed);
        strength = strength.more_strength(2);
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

    // NotConvertSaveload - Bolts only last during an animation
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
    // NotConvertSaveload - These entities only last the duration of the animation
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
        FieldEffect::Damage(damage, explosion_size) => {
            let ((r, g, b), explosion_kind) = match cast.kind {
                FieldKind::Fire => ((255, 0, 0), ExplosionKind::Fire),
                FieldKind::Hail => ((0, 43, 102), ExplosionKind::Water),
                FieldKind::Lightning => ((166, 171, 35), ExplosionKind::Lightning),
            };

            let attack = AttackComponent::init(cast.target.origin, damage, AttackKind::Explode(explosion_kind, explosion_size), None);
            let fields = cast
                .target
                .origin
                .get_burst(explosion_size)
                .iter()
                .map(|p| (Some(*p), (r, g, b, 140)))
                .collect();
            spawner::create_damage_field(ecs, cast.target, attack, FieldComponent::init_group(fields));
        }
        FieldEffect::Spawn(kind) => spawn(ecs, cast.target, kind),
        FieldEffect::SustainedDamage(damage, duration) => {
            let (r, g, b) = match cast.kind {
                FieldKind::Fire => (255, 140, 0),
                FieldKind::Hail => (0, 42, 102),
                FieldKind::Lightning => (166, 171, 35),
            };

            let attack = AttackComponent::init(cast.target.origin, damage, AttackKind::DamageTick, Some(cast.target.origin));
            let fields = FieldComponent::init_single(r, g, b);
            let field = spawner::create_sustained_damage_field(ecs, cast.target, attack, fields, duration);
            tick_damage(ecs, &field);
        }
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
            AttackKind::Explode(_, range) => (attack_info.damage, range, attack_info.source),
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
    let path = source_position.line_to(target_position).unwrap();
    let path = extend_line_along_path(&path, duration);
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

// Apply orb can damage enties not on it's exact location
// if it will run into them this turn
pub fn apply_orb(ecs: &mut World, orb: Entity, point: Point) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.grab(orb).attack
    };
    apply_damage_to_location(ecs, point, attack.source, attack.damage);
    ecs.delete_entity(orb).unwrap();
}

pub fn check_new_location_for_damage(ecs: &mut World, entity: Entity) {
    if let Some(orb) = find_orb_at_location(ecs, &ecs.get_position(&entity)) {
        apply_orb(ecs, orb, ecs.get_position(&orb).single_position());
    }
    if let Some(field) = find_field_at_location(ecs, &ecs.get_position(&entity)) {
        let should_tick = {
            if let Some(b) = ecs.read_storage::<BehaviorComponent>().get(field) {
                b.behavior.is_tick_damage()
            } else {
                false
            }
        };
        if should_tick {
            tick_damage(ecs, &field);
        }
    }
}

pub fn tick_damage(ecs: &mut World, entity: &Entity) {
    let attack = ecs.read_storage::<AttackComponent>().grab(*entity).attack;
    for p in ecs.get_position(entity).all_positions() {
        if let Some(target) = find_character_at_location(ecs, p) {
            apply_damage_to_character(ecs, attack.damage, &target, Some(p));
        }
    }
}

pub fn begin_cone(ecs: &mut World, source: &Entity, target: Point, strength: Damage, kind: ConeKind, size: u32) {
    let source_position = Some(ecs.get_position(source).origin);
    ecs.shovel(*source, AttackComponent::init(target, strength, AttackKind::Cone(kind, size), source_position));
    ecs.raise_event(EventKind::Cone(ConeState::BeginSwingAnimation), Some(*source));
}

pub fn cone_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Cone(state) if state.is_complete_swing_animation()) {
        apply_cone(ecs, target.unwrap());
    } else if matches!(kind, EventKind::Cone(state) if state.is_complete_hit_animation()) {
        cone_hits(ecs, target.unwrap());
    }
}

pub fn apply_cone(ecs: &mut World, character: Entity) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.grab(character).attack
    };
    let size = match attack.kind {
        AttackKind::Cone(_, size) => size,
        _ => panic!("Unexpected kind in apply_cone"),
    };
    let cone_direction = Direction::from_two_points(&attack.source.unwrap(), &attack.target);
    for p in attack.source.unwrap().get_cone(cone_direction, size) {
        // NotConvertSaveload - Hits only last during an animation
        let hit = ecs
            .create_entity()
            .with(PositionComponent::init(SizedPoint::from(p)))
            .with(AttackComponent { attack })
            .build();
        ecs.raise_event(EventKind::Cone(ConeState::BeginHitAnimation), Some(hit));
    }

    ecs.write_storage::<AttackComponent>().remove(character);
}

pub fn cone_hits(ecs: &mut World, entity: Entity) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.grab(entity).attack
    };
    let position = ecs.get_position(&entity);
    apply_damage_to_location(ecs, position.single_position(), attack.source, attack.damage);
    ecs.delete_entity(entity).unwrap();
}

pub fn begin_charge(ecs: &mut World, entity: &Entity, target: Point, damage: Damage, kind: WeaponKind) {
    let initial_position = ecs.get_position(entity);
    // This code does not correctly handle wide charges, that will take some thinking
    assert!(initial_position.width == 1 && initial_position.height == 1);

    if let Some(path) = initial_position.line_to(target) {
        // First element on path is entity's position
        if let Some((index, _)) = path.iter().skip(1).enumerate().find(|(_, &p)| find_character_at_location(ecs, p).is_some()) {
            // Index is target's position -1 (from the skip), but this matches the last free square
            ecs.shovel(
                *entity,
                AttackComponent::init(path[index + 1], damage, AttackKind::Melee(kind), Some(path[index])),
            );
            begin_move(ecs, &entity, initial_position.move_to(path[index]), PostMoveAction::Attack);
        } else {
            begin_move(ecs, &entity, initial_position.move_to(*path.last().unwrap()), PostMoveAction::None);
        }
    }
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
            AttackComponent::init(
                Point::init(2, 3),
                Damage::init(2),
                AttackKind::Explode(ExplosionKind::Bomb, 1),
                Some(Point::init(2, 3)),
            ),
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

        begin_field(&mut ecs, &player, Point::init(2, 3), FieldEffect::Damage(Damage::init(1), 0), FieldKind::Fire);
        wait_for_animations(&mut ecs);

        assert_eq!(true, get_field_at(&ecs, &Point::init(2, 3)).is_some());
    }

    #[test]
    fn field_is_placed_without_position() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_character(2, 3, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        // Some conditions, like flying can remove position temporarly. They should still be able to make fields
        ecs.write_storage::<PositionComponent>().remove(player);

        begin_field(&mut ecs, &player, Point::init(2, 3), FieldEffect::Damage(Damage::init(1), 0), FieldKind::Fire);
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
    fn move_and_shoot_max_range() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 8, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 8);
        let starting_health = ecs.get_defenses(&target).health;

        begin_shoot_and_move(&mut ecs, &player, SizedPoint::init(2, 3), Some(5), Damage::init(1), BoltKind::Bullet);
        wait_for_animations(&mut ecs);
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

        begin_shoot_and_move(&mut ecs, &player, SizedPoint::init(2, 1), Some(4), Damage::init(1), BoltKind::Bullet);
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &player, Point::init(2, 1));
        assert_eq!(ecs.get_defenses(&target).health, starting_health);
        assert_eq!(ecs.get_defenses(&other).health, starting_health);
    }

    pub fn assert_orb_at_position(ecs: &World, expected: Point) {
        let orb_components = ecs.read_storage::<OrbComponent>();
        let attack_components = ecs.read_storage::<AttackComponent>();
        let position_components = ecs.read_storage::<PositionComponent>();

        for (_, _, position) in (&orb_components, &attack_components, &position_components).join() {
            if position.position.contains_point(&expected) {
                return;
            }
        }
        panic!("Unable to find orb at point {:?}");
    }

    #[test]
    fn orb_hits_target_on_time() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 6);
        let starting_health = ecs.get_defenses(&target).health;

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);

        new_turn_wait_characters(&mut ecs);
        assert_orb_at_position(&ecs, Point::init(2, 5));

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
        assert_orb_at_position(&ecs, Point::init(2, 3));

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

        new_turn_wait_characters(&mut ecs);
        assert_orb_at_position(&ecs, Point::init(2, 5));

        begin_move(&mut ecs, &target, SizedPoint::init(3, 6), PostMoveAction::None);
        wait_for_animations(&mut ecs);

        new_turn_wait_characters(&mut ecs);
        assert_eq!(ecs.get_defenses(&target).health, starting_health);
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

        new_turn_wait_characters(&mut ecs);
        assert_orb_at_position(&ecs, Point::init(2, 5));

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

        new_turn_wait_characters(&mut ecs);
        assert_orb_at_position(&ecs, Point::init(2, 5));

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

        new_turn_wait_characters(&mut ecs);
        assert_orb_at_position(&ecs, Point::init(2, 5));
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

    // Being knocked back into an active orb should explode it, even if you go first
    #[test]
    fn knockback_into_orb() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 0)
            .with_character(2, 6, 100)
            .with_character(3, 7, 0)
            .with_map()
            .build();

        // . . . .
        // . . . .
        // . . P .
        // . . . .
        // . . . .
        // . . . .
        // . . T .
        // . . . O
        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 6);
        let orb_caster = find_at(&ecs, 3, 7);

        let target_starting_health = ecs.get_defenses(&target).health;

        begin_orb(&mut ecs, &orb_caster, Point::init(1, 7), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);

        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 6),
            Damage::init(0).with_option(DamageOptions::KNOCKBACK),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert_character_at(&ecs, 2, 7);
        assert_ne!(ecs.get_defenses(&target).health, target_starting_health);
    }

    #[test]
    fn move_and_shoot_one_flying() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 0)
            .with_character(2, 6, 100)
            .with_character(2, 8, 100)
            .with_map()
            .build();
        let player = find_at(&ecs, 2, 2);
        let flyer = find_at(&ecs, 2, 8);

        ecs.add_status(&flyer, StatusKind::Flying, 300);

        begin_shoot_and_move(&mut ecs, &player, SizedPoint::init(2, 3), Some(5), Damage::init(1), BoltKind::Bullet);
        wait_for_animations(&mut ecs);
        assert_position(&ecs, &player, Point::init(2, 3));
    }

    #[test]
    fn sustained_damage_field_damages_over_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(2, 6, 100).with_map().build();

        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 6);
        ecs.shovel(player, BehaviorComponent::init(BehaviorKind::None));
        ecs.shovel(target, BehaviorComponent::init(BehaviorKind::None));

        let target_starting_health = ecs.get_defenses(&target).health;

        begin_field(
            &mut ecs,
            &player,
            Point::init(2, 6),
            FieldEffect::SustainedDamage(Damage::init(1), 3),
            FieldKind::Fire,
        );
        wait_for_animations(&mut ecs);

        // Triggering does a tick of damage
        let first_tick_health = ecs.get_defenses(&target).health;
        assert_ne!(first_tick_health, target_starting_health);

        new_turn_wait_characters(&mut ecs);
        // Second tick as well
        assert_ne!(ecs.get_defenses(&target).health, target_starting_health);
    }

    #[test]
    fn knockback_into_sustained_damage_does_damage() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 4, 100).with_map().build();

        let player = find_at(&ecs, 2, 2);
        let target = find_at(&ecs, 2, 4);

        let target_starting_health = ecs.get_defenses(&target).health;

        begin_field(
            &mut ecs,
            &player,
            Point::init(2, 5),
            FieldEffect::SustainedDamage(Damage::init(1), 3),
            FieldKind::Fire,
        );
        wait_for_animations(&mut ecs);
        begin_bolt(
            &mut ecs,
            &player,
            Point::init(2, 4),
            Damage::init(0).with_option(DamageOptions::KNOCKBACK),
            BoltKind::Fire,
        );
        wait_for_animations(&mut ecs);

        assert_character_at(&ecs, 2, 5);
        assert_ne!(ecs.get_defenses(&target).health, target_starting_health);
    }

    #[test]
    fn cone_hits() {
        // Cone of size 2 from (1,2) to (1,3) hits:
        //      (0,3) (1,3) (2,3)
        //(-1,4) (0,4) (1,4) (2,4) (3,4)
        let mut ecs = create_test_state()
            .with_player(1, 2, 100)
            .with_character(2, 3, 10)
            .with_character(0, 4, 10)
            .with_character(1, 5, 10)
            .build();
        let player = find_player(&ecs);
        let player_health = ecs.get_defenses(&player).health;
        let target_one = find_at(&ecs, 2, 3);
        let target_one_health = ecs.get_defenses(&target_one).health;
        let target_two = find_at(&ecs, 0, 4);
        let target_two_health = ecs.get_defenses(&target_two).health;
        let bystander = find_at(&ecs, 1, 5);
        let bystander_health = ecs.get_defenses(&bystander).health;

        begin_cone(&mut ecs, &player, Point::init(1, 3), Damage::init(1), ConeKind::Fire, 2);
        wait_for_animations(&mut ecs);

        assert_eq!(ecs.get_defenses(&player).health, player_health);
        assert!(ecs.get_defenses(&target_one).health < target_one_health);
        assert!(ecs.get_defenses(&target_two).health < target_two_health);
        assert_eq!(ecs.get_defenses(&bystander).health, bystander_health);
    }

    fn test_event(ecs: &mut World, kind: EventKind, _target: Option<Entity>) {
        match kind {
            EventKind::Damage(_) => ecs.increment_test_data("Damage".to_string()),
            _ => {}
        };
    }

    #[test]
    fn cone_hits_wide_multiple_time() {
        let mut ecs = create_test_state()
            .with_player(2, 2, 100)
            .with_sized_character(SizedPoint::init_multi(2, 3, 2, 2), 0)
            .build();
        let player = find_at(&mut ecs, 2, 2);
        ecs.subscribe(test_event);

        begin_cone(&mut ecs, &player, Point::init(2, 3), Damage::init(1), ConeKind::Fire, 2);
        wait_for_animations(&mut ecs);
        assert_eq!(2, ecs.get_test_data("Damage"));
    }
}
