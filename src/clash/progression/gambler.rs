use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use rand::Rng;
use specs::prelude::*;

use super::super::*;

pub fn get_random_items(ecs: &World, player: Entity, requests: Vec<(EquipmentRarity, u32)>) -> Vec<EquipmentItem> {
    let equipment = ecs.read_resource::<EquipmentResource>();
    let progression = ecs.read_resource::<ProgressionComponent>();

    let available: Vec<&EquipmentItem> = equipment.all().filter(|e| !progression.state.items.contains(&e.name)).collect();

    // Bucket into rarity
    // Sort requests by highest rarity first
    // Draw from each bucket, if we run out downgrade to a lower rarity and continue

    vec![]
}
