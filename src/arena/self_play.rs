use specs::prelude::*;

use crate::atlas::{EasyECS, Point, SizedPoint};
use crate::clash::*;

pub fn take_player_action(ecs: &mut World) {
    let player = find_player(ecs);
    for d in get_random_direction_list(ecs) {
        if let Some(potential) = d.point_in_direction(&ecs.get_position(&player).origin) {
            if can_move_character(ecs, &player, SizedPoint::from(potential)) {
                move_character_action(ecs, player, SizedPoint::from(potential));
                return;
            }
        }
    }
    wait(ecs, player);
}

#[cfg(feature = "profile_self_play")]
pub mod tests {
    use std::time::Instant;

    use crate::conductor::StageDirection;

    pub fn self_play_10000_games() {
        let start = Instant::now();

        let mut deaths = 0;
        let mut wins = 0;

        for _ in 0..10000 {
            let mut ecs = super::super::new_game::random_new_world(0).unwrap();
            let mut frame: u64 = 0;
            loop {
                super::super::battle_scene::battle_tick(&mut ecs, frame);
                frame += 1;

                match super::super::battle_scene::battle_stage_direction(&ecs) {
                    StageDirection::BattleEnemyDefeated(_) => {
                        wins += 1;
                        break;
                    }
                    StageDirection::BattlePlayerDeath(_) => {
                        deaths += 1;
                        break;
                    }
                    _ => {}
                }
            }
        }

        let duration = start.elapsed();
        println!("That took {} ms", duration.as_millis());
        println!("Won: {}", wins);
        println!("Deaths: {}", deaths);
    }
}
