use specs::prelude::*;

use super::components::*;
use crate::clash::*;

fn player_can_act(ecs: &World) -> bool {
    let animations = ecs.read_storage::<AnimationComponent>();
    for a in (&animations).join() {
        match &a.animation {
            Animation::Position { .. } => {
                return false;
            }
            Animation::CharacterState { .. } => {}
        }
    }
    true
}

pub fn select_skill(ecs: &mut World, name: &str) {
    if !player_can_act(ecs) {
        return;
    }

    let target_required = get_target_for_skill(name);
    if target_required.is_none() {
        invoke_skill(ecs, name, None);
    } else {
        match target_required {
            TargetType::Enemy | TargetType::Tile => set_state(ecs, BattleSceneState::Targeting(BattleTargetSource::Skill(name.to_string()), target_required)),
            TargetType::None => panic!("TargetType::None should not have reached here in select_skill"),
        }
    }
}

pub fn read_state(ecs: &World) -> BattleSceneState {
    ecs.read_resource::<BattleSceneStateComponent>().state.clone()
}

pub fn set_state(ecs: &mut World, state: BattleSceneState) {
    ecs.write_resource::<BattleSceneStateComponent>().state = state;
}

pub fn reset_state(ecs: &mut World) {
    let mut state = ecs.write_resource::<BattleSceneStateComponent>();
    state.state = BattleSceneState::Default();
}

pub fn select_skill_with_target(ecs: &mut World, name: &str, position: &Point) {
    if !player_can_act(ecs) {
        return;
    }

    // Selection has been made, drop out of targeting state
    reset_state(ecs);

    let target_required = get_target_for_skill(name);

    match target_required {
        TargetType::Enemy | TargetType::Tile => invoke_skill(ecs, name, Some(position)),
        TargetType::None => panic!("TargetType::None should not have reached select_skill_with_target"),
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

fn find_player(ecs: &World) -> Option<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let players = ecs.read_storage::<PlayerComponent>();

    if let Some((entity, _)) = (&entities, &players).join().next() {
        return Some(entity);
    }
    None
}

fn point_in_direction(initial: &Point, direction: Direction) -> Option<Point> {
    let is_valid = match direction {
        Direction::North => initial.y > 0,
        Direction::South => initial.y < MAX_MAP_TILES - 1,
        Direction::East => initial.x < MAX_MAP_TILES - 1,
        Direction::West => initial.x > 0,
    };

    if is_valid {
        match direction {
            Direction::North => Some(Point::init(initial.x, initial.y - 1)),
            Direction::South => Some(Point::init(initial.x, initial.y + 1)),
            Direction::East => Some(Point::init(initial.x + 1, initial.y)),
            Direction::West => Some(Point::init(initial.x - 1, initial.y)),
        }
    } else {
        None
    }
}

pub fn move_player(ecs: &mut World, direction: Direction) {
    if !player_can_act(ecs) {
        return;
    }
    let player = find_player(ecs).unwrap();
    let new_position = {
        let positions = ecs.read_storage::<PositionComponent>();
        let position_component = positions.get(player).unwrap();
        point_in_direction(&position_component.single_position(), direction)
    };
    if let Some(new_position) = new_position {
        move_character(ecs, player, new_position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_off_map() {
        assert_eq!(None, point_in_direction(&Point::init(5, 0), Direction::North));
        assert_eq!(None, point_in_direction(&Point::init(5, 12), Direction::South));
        assert_eq!(None, point_in_direction(&Point::init(0, 5), Direction::West));
        assert_eq!(None, point_in_direction(&Point::init(12, 5), Direction::East));
    }
}
