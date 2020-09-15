use specs::prelude::*;

use crate::clash::*;

pub fn take_player_action(ecs: &mut World) {
    // If I'm on a field, try to move & shoot or just move

    // Else do a strategy based upon gun kind

    wait(ecs, find_player(ecs));
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
