use specs::prelude::*;

use super::{CharacterInfoComponent, FieldComponent, PlayerComponent, Point, PositionComponent};

pub enum MapHitTestResult {
    None(),
    Enemy(),
    Player(),
    Field(),
}

pub fn element_at_location(ecs: &World, map_position: &Point) -> MapHitTestResult {
    let positions = ecs.read_storage::<PositionComponent>();
    let fields = ecs.read_storage::<FieldComponent>();
    let character_infos = ecs.read_storage::<CharacterInfoComponent>();
    let player = ecs.read_storage::<PlayerComponent>();

    for (position, field, character, player) in (&positions, (&fields).maybe(), (&character_infos).maybe(), (&player).maybe()).join() {
        if position.x == map_position.x as u32 && position.y == map_position.y as u32 {
            if let Some(_character) = character {
                if player.is_none() {
                    return MapHitTestResult::Enemy();
                } else {
                    return MapHitTestResult::Player();
                }
            } else if let Some(_field) = field {
                return MapHitTestResult::Field();
            }
        }
    }
    MapHitTestResult::None()
}
