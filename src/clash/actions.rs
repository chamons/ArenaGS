use specs::prelude::*;

use super::*;
use crate::atlas::prelude::*;

pub fn find_player(ecs: &World) -> Entity {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let players = ecs.read_storage::<PlayerComponent>();
    let (entity, _) = (&entities, &players).join().next().expect("No player in world?");
    entity
}

pub fn find_enemies(ecs: &World) -> Vec<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let char_infos = ecs.read_storage::<CharacterInfoComponent>();
    let players = ecs.read_storage::<PlayerComponent>();

    let mut enemies = vec![];
    for (entity, _, player) in (&entities, &char_infos, (&players).maybe()).join() {
        if player.is_none() {
            enemies.push(entity);
        }
    }
    enemies
}

pub fn can_act(ecs: &World) -> bool {
    let player = find_player(ecs);
    let is_player = if let Some(actor) = get_next_actor(ecs) { actor == player } else { false };
    let is_ready = get_ticks(ecs, player) == BASE_ACTION_COST;
    is_player && is_ready
}

pub fn player_move(ecs: &mut World, direction: Direction) -> bool {
    if !can_act(ecs) {
        return false;
    }

    let player = find_player(ecs);
    let new_position = {
        let position = ecs.get_position(player);
        point_in_direction(&position, direction)
    };
    if let Some(new_position) = new_position {
        move_character_action(ecs, player, new_position);
        true
    } else {
        false
    }
}

pub fn player_use_skill(ecs: &mut World, name: &str, target: Option<Point>) -> bool {
    if !can_act(ecs) {
        return false;
    }

    let player = find_player(ecs);
    invoke_skill(ecs, player, name, target);
    true
}

// Returns if player can act
pub fn tick_next_action(ecs: &mut World) -> bool {
    if let Some(next) = wait_for_next(ecs) {
        if find_player(ecs) != next {
            take_enemy_action(ecs, next);
            false
        } else {
            true
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_not_current_actor() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_timed(10).with_map().build();

        assert_eq!(false, player_move(&mut ecs, Direction::North));
    }

    #[test]
    fn move_spends_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_player(&ecs);

        assert_eq!(true, player_move(&mut ecs, Direction::North));

        assert_eq!(0, get_ticks(&ecs, player));
    }

    #[test]
    fn use_skill_not_current_actor() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_timed(10).build();
        ecs.insert(LogComponent::init());

        assert_eq!(false, player_use_skill(&mut ecs, "TestNone", None));
    }

    #[test]
    fn use_skill_spends_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_timed(10).build();
        let player = find_player(&ecs);

        assert_eq!(true, player_use_skill(&mut ecs, "TestNone", None));
        assert_eq!(0, get_ticks(&ecs, player));
    }
}
