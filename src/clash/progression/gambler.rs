use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use rand::Rng;
use specs::prelude::*;

use super::super::*;

pub fn get_random_items(ecs: &World, player: Entity, requests: Vec<(EquipmentRarity, u32)>) -> Vec<EquipmentItem> {
    let equipment = ecs.read_resource::<EquipmentResource>();
    let progression = ecs.read_resource::<ProgressionComponent>();

    let available: Vec<&EquipmentItem> = equipment.all().filter(|e| !progression.state.items.contains(&e.name)).collect();

    let common: Vec<&EquipmentItem> = available.iter().filter(|&e| e.rarity == EquipmentRarity::Common).map(|&e| e).collect();
    let uncommon: Vec<&EquipmentItem> = available.iter().filter(|e| e.rarity == EquipmentRarity::Uncommon).map(|&e| e).collect();
    let rare: Vec<&EquipmentItem> = available.iter().filter(|e| e.rarity == EquipmentRarity::Rare).map(|&e| e).collect();

    let mut rare_request_count: u32 = requests.iter().filter(|r| r.0 == EquipmentRarity::Rare).map(|r| r.1).sum();
    let mut uncommon_request_count: u32 = requests.iter().filter(|r| r.0 == EquipmentRarity::Uncommon).map(|r| r.1).sum();
    let mut common_request_count: u32 = requests.iter().filter(|r| r.0 == EquipmentRarity::Common).map(|r| r.1).sum();
    // Sort requests by highest rarity first
    // Draw from each bucket, if we run out downgrade to a lower rarity and continue

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_items() {}

    #[test]
    fn downgrades_when_too_few() {}

    #[test]
    fn too_few_total_items() {}
}
