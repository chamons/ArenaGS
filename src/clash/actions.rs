use specs::prelude::*;

use super::{
    get_next_actor, invoke_skill, move_character, point_in_direction, spend_time, take_enemy_action, wait_for_next, PlayerComponent, Point, PositionComponent,
    TimeComponent, BASE_ACTION_COST, MAX_MAP_TILES, MOVE_ACTION_COST,
};

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub fn find_player(ecs: &World) -> Option<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let players = ecs.read_storage::<PlayerComponent>();

    if let Some((entity, _)) = (&entities, &players).join().next() {
        return Some(entity);
    }
    None
}

fn can_act(ecs: &World) -> bool {
    let player = find_player(ecs).unwrap();
    let is_player = if let Some(actor) = get_next_actor(ecs) { actor == player } else { false };
    let is_ready = ecs.read_storage::<TimeComponent>().get(player).unwrap().ticks == MOVE_ACTION_COST;
    is_player && is_ready
}

pub fn player_move(ecs: &mut World, direction: Direction) -> bool {
    if !can_act(ecs) {
        return false;
    }

    let player = find_player(ecs).unwrap();
    let new_position = {
        let positions = ecs.read_storage::<PositionComponent>();
        let position_component = positions.get(player).unwrap();
        point_in_direction(&position_component.single_position(), direction)
    };
    if let Some(new_position) = new_position {
        move_character(ecs, player, new_position);
        true
    } else {
        false
    }
}

pub fn player_use_skill(ecs: &mut World, name: &str, target: Option<Point>) -> bool {
    if !can_act(ecs) {
        return false;
    }

    let player = find_player(ecs).unwrap();
    invoke_skill(ecs, &player, name, target);
    true
}

pub fn tick_next_action(ecs: &mut World) {
    if let Some(next) = wait_for_next(ecs) {
        if find_player(ecs).unwrap() != next {
            take_enemy_action(ecs, &next);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{create_world, LogComponent, Map, MapComponent, TimeComponent};
    use super::*;

    #[test]
    fn move_not_current_actor() {
        let mut ecs = create_world();
        ecs.create_entity()
            .with(TimeComponent::init(0))
            .with(PositionComponent::init(2, 2))
            .with(PlayerComponent::init())
            .build();
        ecs.create_entity().with(TimeComponent::init(10)).build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let did_move = player_move(&mut ecs, Direction::North);
        assert_eq!(false, did_move);
    }

    #[test]
    fn move_spends_time() {
        let mut ecs = create_world();
        let player = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(2, 2))
            .with(PlayerComponent::init())
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let did_move = player_move(&mut ecs, Direction::North);
        assert_eq!(true, did_move);

        assert_eq!(0, ecs.read_storage::<TimeComponent>().get(player).unwrap().ticks);
    }

    #[test]
    fn use_skill_not_current_actor() {
        let mut ecs = create_world();
        ecs.create_entity()
            .with(TimeComponent::init(0))
            .with(PositionComponent::init(2, 2))
            .with(PlayerComponent::init())
            .build();
        ecs.create_entity().with(TimeComponent::init(10)).build();
        ecs.insert(LogComponent::init());

        let did_act = player_use_skill(&mut ecs, "TestNone", None);
        assert_eq!(false, did_act);
    }

    #[test]
    fn use_skill_spends_time() {
        let mut ecs = create_world();
        let player = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(2, 2))
            .with(PlayerComponent::init())
            .build();
        ecs.create_entity().with(TimeComponent::init(10)).build();
        ecs.insert(LogComponent::init());

        let did_act = player_use_skill(&mut ecs, "TestNone", None);
        assert_eq!(true, did_act);
        assert_eq!(0, ecs.read_storage::<TimeComponent>().get(player).unwrap().ticks);
    }
}
