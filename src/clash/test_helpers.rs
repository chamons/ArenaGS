use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

use super::*;
use crate::atlas::prelude::*;

pub struct StateBuilder {
    ecs: World,
}

impl StateBuilder {
    pub fn with_timed(&mut self, time: i32) -> &mut Self {
        self.ecs.create_entity().with(TimeComponent::init(time)).build();
        self
    }

    pub fn with_character(&mut self, x: u32, y: u32, time: i32) -> &mut Self {
        make_test_character(&mut self.ecs, SizedPoint::init(x, y), time);
        self
    }

    pub fn with_sized_character(&mut self, position: SizedPoint, time: i32) -> &mut Self {
        make_test_character(&mut self.ecs, position, time);
        self
    }

    pub fn with_player(&mut self, x: u32, y: u32, time: i32) -> &mut Self {
        make_test_character(&mut self.ecs, SizedPoint::init(x, y), time);
        let player = find_at(&self.ecs, x, y);
        self.ecs.shovel(player, PlayerComponent::init());
        self
    }

    pub fn with_gunslinger(&mut self, x: u32, y: u32, equip: &[(EquipmentKinds, EquipmentItem, usize)]) -> &mut Self {
        let mut state = ProgressionState::init_gunslinger();
        for (kind, item, index) in equip {
            state.equipment.add(*kind, item.clone(), *index);
        }
        self.ecs.insert(ProgressionComponent::init(state));

        let mut resources = (*self.ecs.read_resource::<SkillsResource>()).clone();
        super::embattle::create_player(&mut self.ecs, &mut resources, Point::init(x, y));
        // Overwrite existing
        self.ecs.insert(resources);

        self
    }

    pub fn with_map(&mut self) -> &mut Self {
        self.ecs.insert(MapComponent::init(Map::init_empty()));
        self
    }

    pub fn with_progression(&mut self) -> &mut Self {
        self.ecs.insert(ProgressionComponent::init(ProgressionState::init_gunslinger()));
        self
    }

    pub fn build(&mut self) -> World {
        std::mem::replace(&mut self.ecs, World::new())
    }
}

pub fn test_eq(name: &str, kind: EquipmentKinds, effect: &[EquipmentEffect], index: usize) -> (EquipmentKinds, EquipmentItem, usize) {
    (kind, EquipmentItem::init(name, None, kind, EquipmentRarity::Common, effect), index)
}

pub fn create_test_state() -> StateBuilder {
    let mut ecs = create_world();
    ecs.insert(super::content::test::get_test_skills());

    StateBuilder { ecs }
}

pub fn find_at(ecs: &World, x: u32, y: u32) -> Entity {
    find_character_at_location(ecs, Point::init(x, y)).unwrap()
}

pub fn assert_character_at(ecs: &World, x: u32, y: u32) {
    assert!(find_character_at_location(ecs, Point::init(x, y)).is_some());
}

#[allow(dead_code)]
pub fn find_entity_at(ecs: &World, x: u32, y: u32) -> Entity {
    find_entity_at_location(ecs, Point::init(x, y)).unwrap()
}

pub fn find_first_entity(ecs: &World) -> Entity {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    (&entities).join().next().unwrap()
}

pub fn find_at_time(ecs: &World, desired_time: i32) -> Entity {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let times = ecs.read_storage::<TimeComponent>();

    for (entity, time) in (&entities, &times).join() {
        if desired_time == time.ticks {
            return entity;
        }
    }
    panic!();
}

pub fn find_all_entities(ecs: &World) -> Vec<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();

    let mut all = vec![];
    for entity in (&entities).join() {
        all.push(entity);
    }
    all
}

pub fn make_test_character(ecs: &mut World, position: SizedPoint, time: i32) -> Entity {
    ecs.create_entity()
        .with(TimeComponent::init(time))
        .with(PositionComponent::init(position))
        .with(IsCharacterComponent::init())
        .with(NamedComponent::init("TestCharacter"))
        .with(DefenseComponent::init(Defenses::just_health(10)))
        .with(SkillPowerComponent::init(0))
        .with(TemperatureComponent::init(Temperature::init()))
        .with(SkillResourceComponent::init(&[]))
        .with(SkillsComponent::init(&[]))
        .with(StatusComponent::init())
        .marked::<SimpleMarker<ToSerialize>>()
        .build()
}

pub fn set_temperature(ecs: &mut World, player: Entity, temperature: i32) {
    ecs.write_storage::<TemperatureComponent>()
        .grab_mut(player)
        .temperature
        .set_temperature(temperature);
}

pub fn set_health(ecs: &mut World, player: Entity, health: u32) {
    ecs.write_storage::<DefenseComponent>().grab_mut(player).defenses = Defenses::just_health(health);
}

// This can be dangerous, if something invalidates the entity reference
// then you can crash here
pub fn assert_position(ecs: &World, entity: Entity, expected: Point) {
    let position = ecs.get_position(entity);
    assert_points_equal(position.single_position(), expected);
}

pub fn assert_not_at_position(ecs: &World, entity: Entity, expected: Point) {
    let position = ecs.get_position(entity);
    assert_points_not_equal(position.single_position(), expected);
}

pub fn new_turn_wait_characters(ecs: &mut World) {
    add_ticks(ecs, 100);
    for c in find_all_characters(ecs) {
        wait(ecs, c);
    }

    tick_next_action(ecs);
    wait_for_animations(ecs);
}

#[allow(dead_code)]
pub fn dump_all_position(ecs: &World) {
    let positions = ecs.read_storage::<PositionComponent>();
    let is_characters = ecs.read_storage::<IsCharacterComponent>();
    let orb_components = ecs.read_storage::<OrbComponent>();
    let attack_components = ecs.read_storage::<AttackComponent>();
    let fields = ecs.read_storage::<FieldComponent>();
    let times = ecs.read_storage::<TimeComponent>();
    for (position, is_character, orb, attack, field, time) in (
        &positions,
        (&is_characters).maybe(),
        (&orb_components).maybe(),
        (&attack_components).maybe(),
        (&fields).maybe(),
        (&times).maybe(),
    )
        .join()
    {
        let mut description = format!("{}", position.position);
        if is_character.is_some() {
            description.push_str(" (Char)");
        }
        if orb.is_some() {
            description.push_str(" (Orb)");
        }
        if attack.is_some() {
            description.push_str(" (Attack)");
        }
        if field.is_some() {
            description.push_str(" (Field)");
        }
        if time.is_some() {
            description.push_str(format!(" ({})", time.unwrap().ticks).as_str());
        }
        println!("{}", description);
    }
    println!();
}

pub fn assert_field_exists(ecs: &World, x: u32, y: u32) {
    let fields = ecs.read_storage::<FieldComponent>();
    let positions = ecs.read_storage::<PositionComponent>();
    let found_field = (&fields, (&positions).maybe()).join().any(|(f, p)| {
        f.fields.iter().any(|(field_position, _)| {
            if let Some(field_position) = field_position {
                field_position.x == x && field_position.y == y
            } else {
                p.unwrap().position.contains_point(&Point::init(x, y))
            }
        })
    });
    assert!(found_field);
}

pub fn assert_field_count(ecs: &World, expected: usize) {
    let fields = ecs.read_storage::<FieldComponent>();
    let count: usize = (&fields).join().map(|f| f.fields.len()).sum();
    assert_eq!(expected, count);
}
