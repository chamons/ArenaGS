use specs::prelude::*;

use crate::atlas::{EasyECS, Point, SizedPoint};
use crate::clash::*;

fn get_skill_at_index(ecs: &mut World, player: &Entity, index: usize) -> String {
    let skills_component = ecs.read_storage::<SkillsComponent>();
    let skills = &skills_component.grab(*player).skills;
    skills[index].to_string()
}

fn find_points_that_hit_an_enemy(ecs: &mut World, player: &Entity, all_enemies: &Vec<SizedPoint>, skill: &SkillInfo) -> Vec<Point> {
    let player_position = ecs.get_position(player);
    get_random_direction_list(ecs)
        .iter()
        .filter_map(|d| {
            if let Some(point) = d.point_in_direction(&player_position.origin) {
                // This is suboptimal (should check all points)
                if all_enemies
                    .iter()
                    .any(|e| in_possible_skill_range_for_secondary(&SizedPoint::from(point), skill, e.origin))
                {
                    Some(point)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn find_distance_to_closest_enemy(point: &Point, all_enemies: &Vec<SizedPoint>) -> Option<u32> {
    all_enemies.iter().filter_map(|x| SizedPoint::from(*point).distance_to_multi(*x)).min().take()
}

fn find_best_move_and_shoot_square(ecs: &mut World, player: &Entity, skill: &SkillInfo) -> Option<Point> {
    let all_enemies: Vec<SizedPoint> = find_all_characters(ecs).iter().filter(|&x| x != player).map(|x| ecs.get_position(x)).collect();
    let point_hit_enemy: Vec<Point> = find_points_that_hit_an_enemy(ecs, player, &all_enemies, skill);
    println!("{}", point_hit_enemy.len());
    point_hit_enemy
        .iter()
        .map(|x| (x, find_distance_to_closest_enemy(x, &all_enemies)))
        .filter(|(_, d)| d.is_some())
        .max_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .map(|(x, _)| *x)
}

pub fn take_player_action(ecs: &mut World) {
    let player = find_player(ecs);
    let player_position = ecs.get_position(&player);
    // Or orb path
    if find_field_at_location(ecs, &player_position).is_some() {
        let skill_name = get_skill_at_index(ecs, &player, 2);
        let move_shoot_skill = &get_skill(&skill_name);
        if let Some(best_move_and_shoot_square) = find_best_move_and_shoot_square(ecs, &player, &move_shoot_skill) {
            invoke_skill(ecs, &player, &skill_name, Some(best_move_and_shoot_square));
        }
    }

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
