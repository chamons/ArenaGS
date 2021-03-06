use specs::prelude::*;

use super::*;
use crate::atlas::prelude::*;

pub fn find_player(ecs: &World) -> Entity {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let players = ecs.read_storage::<PlayerComponent>();
    let (entity, _) = (&entities, &players).join().next().expect("No player in world?");
    entity
}

pub fn maybe_find_player(ecs: &World) -> Option<Entity> {
    let players = ecs.read_storage::<PlayerComponent>();
    if (&players).join().count() > 0 {
        Some(find_player(ecs))
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn is_enemy(ecs: &World, entity: Entity) -> bool {
    ecs.read_storage::<PlayerComponent>().get(entity).is_none() && ecs.read_storage::<PlayerAlly>().get(entity).is_none()
}

pub fn is_player_or_ally(ecs: &World, entity: Entity) -> bool {
    ecs.read_storage::<PlayerComponent>().get(entity).is_some() || ecs.read_storage::<PlayerAlly>().get(entity).is_some()
}

#[allow(dead_code)]
pub fn is_of_my_side(ecs: &World, entity: Entity, target: Entity) -> bool {
    is_player_or_ally(ecs, entity) == is_player_or_ally(ecs, target)
}

pub fn find_enemies(ecs: &World) -> Vec<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let is_characters = ecs.read_storage::<IsCharacterComponent>();
    let players = ecs.read_storage::<PlayerComponent>();
    let player_ally = ecs.read_storage::<PlayerAlly>();

    let mut enemies = vec![];
    for (entity, _, player, ally) in (&entities, &is_characters, (&players).maybe(), (&player_ally).maybe()).join() {
        if player.is_none() && ally.is_none() {
            enemies.push(entity);
        }
    }
    enemies
}

pub fn find_player_and_allies(ecs: &World) -> Vec<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let is_characters = ecs.read_storage::<IsCharacterComponent>();
    let players = ecs.read_storage::<PlayerComponent>();
    let player_ally = ecs.read_storage::<PlayerAlly>();

    let mut enemies = vec![];
    for (entity, _, player, ally) in (&entities, &is_characters, (&players).maybe(), (&player_ally).maybe()).join() {
        if player.is_some() || ally.is_some() {
            enemies.push(entity);
        }
    }
    enemies
}

pub enum Allegiance {
    Player,
    Opposition,
}

pub fn find_characters(ecs: &World, allegiance: Allegiance) -> Vec<Entity> {
    match allegiance {
        Allegiance::Player => find_player_and_allies(ecs),
        Allegiance::Opposition => find_enemies(ecs),
    }
}

pub fn find_allies_of(ecs: &World, entity: Entity) -> Vec<Entity> {
    find_characters(
        ecs,
        if is_player_or_ally(ecs, entity) {
            Allegiance::Player
        } else {
            Allegiance::Opposition
        },
    )
}

pub fn find_enemies_of(ecs: &World, entity: Entity) -> Vec<Entity> {
    find_characters(
        ecs,
        if is_player_or_ally(ecs, entity) {
            Allegiance::Opposition
        } else {
            Allegiance::Player
        },
    )
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

        assert!(!player_move(&mut ecs, Direction::North));
    }

    #[test]
    fn move_spends_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_map().build();
        let player = find_player(&ecs);

        assert!(player_move(&mut ecs, Direction::North));

        assert_eq!(0, get_ticks(&ecs, player));
    }

    #[test]
    fn use_skill_not_current_actor() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_timed(10).build();
        ecs.insert(LogComponent::init());

        assert!(!player_use_skill(&mut ecs, "TestNone", None));
    }

    #[test]
    fn use_skill_spends_time() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_timed(10).build();
        let player = find_player(&ecs);

        assert!(player_use_skill(&mut ecs, "TestNone", None));
        assert_eq!(0, get_ticks(&ecs, player));
    }
}
