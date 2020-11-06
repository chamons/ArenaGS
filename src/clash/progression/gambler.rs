use std::cmp;
use std::collections::HashMap;

use rand::prelude::*;
use specs::prelude::*;

use super::super::*;

pub fn get_reward_request(ecs: &World, count: u32) -> Vec<(EquipmentRarity, u32)> {
    let mut rng = ecs.write_resource::<RandomComponent>();
    let choices = vec![EquipmentRarity::Common, EquipmentRarity::Uncommon, EquipmentRarity::Rare];

    let mut requests = HashMap::new();
    let mut add_request = |kind: EquipmentRarity| *requests.entry(kind).or_insert(0) += 1;

    for _ in 0..count {
        add_request(
            choices
                .choose_weighted(&mut rng.rand, |i| match i {
                    EquipmentRarity::Common => 75,
                    EquipmentRarity::Uncommon => 15,
                    EquipmentRarity::Rare => 10,
                    EquipmentRarity::Standard => 0,
                })
                .unwrap()
                .clone(),
        );
    }

    requests.iter().map(|x| (*x.0, *x.1)).collect()
}

pub fn get_random_items(ecs: &World, requests: Vec<(EquipmentRarity, u32)>) -> Vec<EquipmentItem> {
    let equipment = ecs.read_resource::<EquipmentResource>();
    let progression = ecs.read_resource::<ProgressionComponent>();

    let available: Vec<&EquipmentItem> = equipment.all().filter(|e| !progression.state.items.contains(&e.name)).collect();

    let rare: Vec<&EquipmentItem> = available.iter().filter(|e| e.rarity == EquipmentRarity::Rare).map(|&e| e).collect();
    let uncommon: Vec<&EquipmentItem> = available.iter().filter(|e| e.rarity == EquipmentRarity::Uncommon).map(|&e| e).collect();
    let common: Vec<&EquipmentItem> = available.iter().filter(|&e| e.rarity == EquipmentRarity::Common).map(|&e| e).collect();

    let rare_request_count: u32 = requests.iter().filter(|r| r.0 == EquipmentRarity::Rare).map(|r| r.1).sum();
    let mut uncommon_request_count: u32 = requests.iter().filter(|r| r.0 == EquipmentRarity::Uncommon).map(|r| r.1).sum();
    let mut common_request_count: u32 = requests.iter().filter(|r| r.0 == EquipmentRarity::Common).map(|r| r.1).sum();

    let rare_count = cmp::min(rare_request_count, rare.len() as u32);
    if rare_count < rare_request_count {
        uncommon_request_count += rare_request_count - rare_count;
    }

    let uncommon_count = cmp::min(uncommon_request_count, uncommon.len() as u32);
    if uncommon_count < uncommon_request_count {
        common_request_count += uncommon_request_count - uncommon_count;
    }

    let common_count = cmp::min(common_request_count, common.len() as u32);

    let mut rng = ecs.write_resource::<RandomComponent>();

    let mut chosen = Vec::with_capacity((rare_request_count + uncommon_request_count + common_request_count) as usize);
    chosen.extend(rare.choose_multiple(&mut rng.rand, rare_count as usize).map(|&e| e.clone()));
    chosen.extend(uncommon.choose_multiple(&mut rng.rand, uncommon_count as usize).map(|&e| e.clone()));
    chosen.extend(common.choose_multiple(&mut rng.rand, common_count as usize).map(|&e| e.clone()));

    // Reverse so rare at end
    chosen.reverse();
    chosen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_items() {
        let mut ecs = World::new();

        let equipments = EquipmentResource::init_with(&[
            EquipmentItem::init("a", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
            EquipmentItem::init("b", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
            EquipmentItem::init("c", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
            EquipmentItem::init("d", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
        ]);

        let progression = ProgressionComponent::init(ProgressionState::init(0, 0, &["a"], CharacterWeaponKind::Gunslinger, Equipment::init_empty()));

        ecs.insert(RandomComponent::init());
        ecs.insert(progression);
        ecs.insert(equipments);

        for _ in 0..10 {
            let chosen = get_random_items(&ecs, vec![(EquipmentRarity::Common, 2)]);
            assert_eq!(2, chosen.len());
            assert!(chosen.iter().all(|c| c.name == "b" || c.name == "c" || c.name == "d"));
        }
    }

    #[test]
    fn downgrades_when_too_few() {
        let mut ecs = World::new();

        let equipments = EquipmentResource::init_with(&[
            EquipmentItem::init("a", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
            EquipmentItem::init("b", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
            EquipmentItem::init("c", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
            EquipmentItem::init("d", None, EquipmentKinds::Accessory, EquipmentRarity::Uncommon, &[EquipmentEffect::None]),
            EquipmentItem::init("e", None, EquipmentKinds::Accessory, EquipmentRarity::Uncommon, &[EquipmentEffect::None]),
            EquipmentItem::init("f", None, EquipmentKinds::Accessory, EquipmentRarity::Rare, &[EquipmentEffect::None]),
        ]);

        let progression = ProgressionComponent::init(ProgressionState::init(0, 0, &["a"], CharacterWeaponKind::Gunslinger, Equipment::init_empty()));

        ecs.insert(RandomComponent::init());
        ecs.insert(progression);
        ecs.insert(equipments);

        for _ in 0..10 {
            let chosen = get_random_items(
                &ecs,
                vec![(EquipmentRarity::Common, 2), (EquipmentRarity::Uncommon, 2), (EquipmentRarity::Rare, 2)],
            );
            assert_eq!(5, chosen.len());
            assert!(chosen.iter().all(|c| c.name != "a"));
        }
    }

    #[test]
    fn too_few_total_items() {
        let mut ecs = World::new();

        let equipments = EquipmentResource::init_with(&[
            EquipmentItem::init("a", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
            EquipmentItem::init("b", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
            EquipmentItem::init("c", None, EquipmentKinds::Accessory, EquipmentRarity::Common, &[EquipmentEffect::None]),
        ]);

        let progression = ProgressionComponent::init(ProgressionState::init(0, 0, &["a"], CharacterWeaponKind::Gunslinger, Equipment::init_empty()));

        ecs.insert(RandomComponent::init());
        ecs.insert(progression);
        ecs.insert(equipments);

        for _ in 0..10 {
            let chosen = get_random_items(
                &ecs,
                vec![(EquipmentRarity::Common, 12), (EquipmentRarity::Uncommon, 2), (EquipmentRarity::Rare, 2)],
            );
            assert_eq!(2, chosen.len());
            assert!(chosen.iter().all(|c| c.name != "a"));
        }
    }

    #[test]
    fn random_reward() {
        let mut ecs = World::new();
        ecs.insert(RandomComponent::init());

        let request = get_reward_request(&ecs, 3);
        assert_eq!(3, request.iter().map(|r| r.1).sum::<u32>());
    }
}
