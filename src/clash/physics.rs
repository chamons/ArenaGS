use specs::prelude::*;

use super::*;
use crate::atlas::prelude::*;

// Begin the move itself, does not spend time
pub fn begin_move(ecs: &mut World, entity: Entity, new_position: SizedPoint, action: PostMoveAction) {
    ecs.shovel(entity, MovementComponent::init(new_position));
    ecs.raise_event(EventKind::Move(MoveState::BeginAnimation, action), Some(entity));
}

pub fn move_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Move(state, action) => {
            if state.is_complete_animation() {
                complete_move(ecs, target.unwrap());
                match action {
                    PostMoveAction::Shoot(damage, range, kind) => begin_bolt_nearest_in_range(ecs, target.unwrap(), range, damage, kind),
                    PostMoveAction::CheckNewLocationDamage => check_new_location_for_damage(ecs, target.unwrap()),
                    PostMoveAction::Attack => {
                        let a = ecs.read_storage::<AttackComponent>().grab(target.unwrap()).clone();
                        ecs.write_storage::<AttackComponent>().remove(target.unwrap());
                        begin_melee(ecs, target.unwrap(), a.attack.target, a.attack.damage, a.attack.melee_kind());
                    }
                    PostMoveAction::None => {}
                }
            }
        }
        _ => {}
    }
}

pub fn complete_move(ecs: &mut World, entity: Entity) {
    let current_position = ecs.get_position(entity);
    let new_position = {
        let mut movements = ecs.write_storage::<MovementComponent>();
        let new_position = movements.grab(entity).new_position;
        movements.remove(entity);
        new_position.origin
    };
    let distance = current_position.distance_to(new_position).unwrap();

    {
        let mut positions = ecs.write_storage::<PositionComponent>();
        let position = &mut positions.grab_mut(entity);
        position.move_to(new_position);
    }

    ecs.raise_event(EventKind::MoveComplete(distance), Some(entity));
}

pub fn point_in_direction(initial: &SizedPoint, direction: Direction) -> Option<SizedPoint> {
    match direction {
        Direction::North | Direction::South | Direction::East | Direction::West => direction.sized_point_in_direction(initial),
        _ => None,
    }
}

// Is an area clear of all elements with PositionComponent and CharacterInfoComponent _except_ the invoker (if)
pub fn is_area_clear_of_others(ecs: &World, area: &[Point], invoker: Entity) -> bool {
    is_area_clear(ecs, area, Some(invoker))
}

pub fn is_area_clear(ecs: &World, area: &[Point], invoker: Option<Entity>) -> bool {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let positions = ecs.read_storage::<PositionComponent>();
    let char_info = ecs.read_storage::<CharacterInfoComponent>();
    let map = &ecs.read_resource::<MapComponent>().map;

    for (entity, position, _) in (&entities, &positions, &char_info).join() {
        for p in area.iter() {
            if !p.is_in_bounds() || !map.is_walkable(&p) {
                return false;
            }
            if invoker != Some(entity) && position.position.contains_point(&p) {
                return false;
            }
        }
    }
    true
}

pub fn find_entity_at_location(ecs: &World, area: Point) -> Option<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let positions = ecs.read_storage::<PositionComponent>();

    for (entity, position) in (&entities, &positions).join() {
        if position.position.contains_point(&area) {
            return Some(entity);
        }
    }
    None
}

pub fn find_character_at_location(ecs: &World, area: Point) -> Option<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let positions = ecs.read_storage::<PositionComponent>();
    let char_info = ecs.read_storage::<CharacterInfoComponent>();

    for (entity, position, _) in (&entities, &positions, &char_info).join() {
        if position.position.contains_point(&area) {
            return Some(entity);
        }
    }
    None
}

pub fn find_orb_at_location(ecs: &World, target: &SizedPoint) -> Option<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let orbs = ecs.read_storage::<OrbComponent>();
    let positions = ecs.read_storage::<PositionComponent>();
    for (entity, _, position) in (&entities, &orbs, &positions).join() {
        if target.contains_point(&position.position.single_position()) {
            return Some(entity);
        }
    }
    None
}

pub fn find_field_at_location(ecs: &World, target: &SizedPoint) -> Option<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let fields = ecs.read_storage::<FieldComponent>();
    let positions = ecs.read_storage::<PositionComponent>();
    for (entity, field, position) in (&entities, &fields, &positions).join() {
        for (p, _) in &field.fields {
            if let Some(p) = p {
                if target.contains_point(p) {
                    return Some(entity);
                }
            } else {
                for p in position.position.all_positions() {
                    if target.contains_point(&p) {
                        return Some(entity);
                    }
                }
            }
        }
    }
    None
}

pub fn find_all_characters(ecs: &World) -> Vec<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let char_infos = ecs.read_storage::<CharacterInfoComponent>();

    let mut all = vec![];
    for (entity, _) in (&entities, &char_infos).join() {
        all.push(entity);
    }
    all
}

pub fn can_move_character(ecs: &World, mover: Entity, new: SizedPoint) -> bool {
    let has_exhaustion = {
        if let Some(skill_resource) = ecs.read_storage::<SkillResourceComponent>().get(mover) {
            skill_resource.exhaustion + EXHAUSTION_COST_PER_MOVE <= MAX_EXHAUSTION
        } else {
            true
        }
    };

    // A 2x2 character can't move their origin to the 0'th row, as their 'head' would poke off the map
    // Same goes for one of the 13th column
    let top_x = new.origin.x + (new.width - 1);
    let top_y = new.origin.y as i32 - (new.height as i32 - 1);
    if top_x >= MAX_MAP_TILES || top_y < 0 {
        return false;
    }

    is_area_clear_of_others(ecs, &new.all_positions(), mover) && has_exhaustion
}

// Move a character, spending standard time and exhaustion
pub fn move_character_action(ecs: &mut World, entity: Entity, new: SizedPoint) -> bool {
    if !can_move_character(ecs, entity, new) {
        return false;
    }

    begin_move(ecs, entity, new, PostMoveAction::None);

    let time_cost = if ecs.has_status(entity, StatusKind::Frozen) {
        MOVE_ACTION_COST + (MOVE_ACTION_COST / 2)
    } else {
        MOVE_ACTION_COST
    };

    spend_time(ecs, entity, time_cost);
    if ecs.read_storage::<SkillResourceComponent>().get(entity).is_some() {
        spend_exhaustion(ecs, entity, EXHAUSTION_COST_PER_MOVE);
    }
    true
}

pub fn wait(ecs: &mut World, entity: Entity) {
    spend_time(ecs, entity, BASE_ACTION_COST);
}

pub const MAX_EXHAUSTION: f64 = 100.0;
pub fn spend_exhaustion(ecs: &mut World, invoker: Entity, cost: f64) {
    ecs.write_storage::<SkillResourceComponent>().grab_mut(invoker).exhaustion += cost;
    assert!(ecs.read_storage::<SkillResourceComponent>().grab(invoker).exhaustion <= MAX_EXHAUSTION);
}

#[cfg(test)]
pub fn wait_for_animations(ecs: &mut World) {
    crate::arena::force_complete_animations(ecs);
}

pub fn find_clear_landing(ecs: &mut World, initial: &SizedPoint, entity: Option<Entity>) -> SizedPoint {
    if is_area_clear(ecs, &initial.all_positions(), entity) {
        return *initial;
    }

    for distance in 1..3 {
        for direction in get_random_full_direction_list(ecs) {
            let mut attempt = *initial;
            for _ in 0..distance {
                if let Some(p) = direction.sized_point_in_direction(&attempt) {
                    attempt = p;
                }
            }
            if is_area_clear(ecs, &attempt.all_positions(), entity) {
                return attempt;
            }
        }
    }
    // This seems very unlikely, we check every single possibility within 3 of the source point
    panic!("Unable to find clear landing at {}", initial.origin);
}

#[cfg(test)]
mod tests {
    use super::create_test_state;
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn point_off_map() {
        assert_eq!(None, point_in_direction(&SizedPoint::init(5, 0), Direction::North));
        assert_eq!(None, point_in_direction(&SizedPoint::init(5, MAX_MAP_TILES - 1), Direction::South));
        assert_eq!(None, point_in_direction(&SizedPoint::init(0, 5), Direction::West));
        assert_eq!(None, point_in_direction(&SizedPoint::init(MAX_MAP_TILES - 1, 5), Direction::East));
    }

    #[test]
    fn walk_into_clear() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);
        assert_position(&ecs, entity, Point::init(2, 2));

        let success = move_character_action(&mut ecs, entity, SizedPoint::init(2, 3));
        assert!(success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, entity, Point::init(2, 3));
        assert_eq!(0, get_ticks(&ecs, entity));
    }

    #[test]
    fn walk_into_non_characters() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let entity = find_at(&ecs, 2, 2);
        ecs.create_entity()
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(FieldComponent::init_single(255, 0, 0))
            .build();

        assert_position(&ecs, entity, Point::init(2, 2));

        let success = move_character_action(&mut ecs, entity, SizedPoint::init(2, 3));
        assert!(success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, entity, Point::init(2, 3));
    }

    #[test]
    fn unable_to_walk_into_unwalkable() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let mut map = Map::init_empty();
        map.set_walkable(&Point::init(2, 3), false);
        ecs.insert(MapComponent::init(map));
        let entity = find_at(&ecs, 2, 2);

        assert_position(&ecs, entity, Point::init(2, 2));

        let success = move_character_action(&mut ecs, entity, SizedPoint::init(2, 3));
        assert!(!success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, entity, Point::init(2, 2))
    }

    #[test]
    fn unable_to_walk_into_another() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 0).with_map().build();
        let entity = find_at(&ecs, 2, 2);

        assert_position(&ecs, entity, Point::init(2, 2));

        let success = move_character_action(&mut ecs, entity, SizedPoint::init(2, 3));
        assert!(!success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, entity, Point::init(2, 2));
    }

    #[test]
    fn walk_off_map() {
        let mut ecs = create_test_state().with_character(13, 13, 100).with_map().build();
        let entity = find_at(&ecs, 13, 13);
        assert_position(&ecs, entity, Point::init(13, 13));

        let success = move_character_action(&mut ecs, entity, SizedPoint::init(13, 14));
        assert!(!success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, entity, Point::init(13, 13));
    }

    #[test]
    fn multi_walks_into_single() {
        let mut ecs = create_test_state()
            .with_character(2, 2, 100)
            .with_sized_character(SizedPoint::init_multi(2, 4, 2, 2), 100)
            .with_map()
            .build();
        let bottom = find_at(&ecs, 2, 4);

        assert!(!move_character_action(&mut ecs, bottom, SizedPoint::init_multi(2, 3, 2, 2)));
        wait_for_animations(&mut ecs);
    }

    #[test]
    fn entity_with_resources_spend_exhaustion_to_move() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        // All of these tests by default do not include an SkillResourceComponent, so they get no exhaustion
        ecs.shovel(player, SkillResourceComponent::init(&[]));

        assert!(move_character_action(&mut ecs, player, SizedPoint::init(2, 3)));
        wait_for_animations(&mut ecs);

        let skills = ecs.read_storage::<SkillResourceComponent>();
        assert_approx_eq!(EXHAUSTION_COST_PER_MOVE, skills.grab(player).exhaustion);
    }

    #[test]
    fn entity_with_max_resources_can_not_move() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        // All of these tests by default do not include an SkillResourceComponent, so they get no exhaustion
        let skill_resource = SkillResourceComponent {
            exhaustion: MAX_EXHAUSTION,
            ..SkillResourceComponent::init(&[])
        };
        ecs.shovel(player, skill_resource);

        assert!(!move_character_action(&mut ecs, player, SizedPoint::init(2, 3)));
        let skills = ecs.read_storage::<SkillResourceComponent>();
        assert_approx_eq!(100.0, skills.grab(player).exhaustion);
    }

    #[test]
    fn frozen_slows_movement() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let player = find_at(&ecs, 2, 2);
        ecs.add_trait(player, StatusKind::Frozen);

        // All of these tests by default do not include an SkillResourceComponent, so they get no exhaustion
        let skill_resource = SkillResourceComponent {
            exhaustion: 0.0,
            ..SkillResourceComponent::init(&[])
        };
        ecs.shovel(player, skill_resource);

        assert!(move_character_action(&mut ecs, player, SizedPoint::init(2, 3)));
        assert_eq!(MOVE_ACTION_COST / -2, get_ticks(&ecs, player));
    }

    #[test]
    fn can_move_boundary_crash() {
        let ecs = create_test_state()
            .with_sized_character(SizedPoint::init_multi(8, 1, 2, 2), 100)
            .with_map()
            .build();
        let enemy = find_at(&ecs, 8, 1);
        can_move_character(&ecs, enemy, SizedPoint::init_multi(8, 0, 2, 2));
    }
}
