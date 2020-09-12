use specs::prelude::*;

use super::*;
use crate::atlas::{assert_points_equal, assert_points_not_equal, EasyMutECS, EasyMutWorld, Point, SizedPoint};

pub struct StateBuilder {
    ecs: World,
}

impl StateBuilder {
    pub fn with_timed<'a>(&'a mut self, time: i32) -> &'a mut Self {
        self.ecs.create_entity().with(TimeComponent::init(time)).build();
        self
    }

    pub fn with_character<'a>(&'a mut self, x: u32, y: u32, time: i32) -> &'a mut Self {
        make_test_character(&mut self.ecs, SizedPoint::init(x, y), time);
        self
    }

    pub fn with_sized_character<'a>(&'a mut self, position: SizedPoint, time: i32) -> &'a mut Self {
        make_test_character(&mut self.ecs, position, time);
        self
    }

    pub fn with_player<'a>(&'a mut self, x: u32, y: u32, time: i32) -> &'a mut Self {
        make_test_character(&mut self.ecs, SizedPoint::init(x, y), time);
        let player = find_at(&self.ecs, x, y);
        self.ecs.shovel(player, PlayerComponent::init());
        self
    }

    pub fn with_map<'a>(&'a mut self) -> &'a mut Self {
        self.ecs.insert(MapComponent::init(Map::init_empty()));
        self
    }

    pub fn build(&mut self) -> World {
        std::mem::replace(&mut self.ecs, World::new())
    }
}

pub fn create_test_state() -> StateBuilder {
    StateBuilder { ecs: create_world() }
}

pub fn find_at(ecs: &World, x: u32, y: u32) -> Entity {
    find_character_at_location(ecs, Point::init(x, y)).unwrap()
}

#[allow(dead_code)]
pub fn find_entity_at(ecs: &World, x: u32, y: u32) -> Entity {
    find_entity_at_location(ecs, Point::init(x, y)).unwrap()
}

pub fn find_first_entity(ecs: &World) -> Entity {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let entity = (&entities).join().next().unwrap();
    entity
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

pub fn find_all_characters(ecs: &World) -> Vec<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let char_infos = ecs.read_storage::<CharacterInfoComponent>();

    let mut all = vec![];
    for (entity, _) in (&entities, &char_infos).join() {
        all.push(entity);
    }
    all
}

pub fn make_test_character(ecs: &mut World, position: SizedPoint, time: i32) {
    ecs.create_entity()
        .with(TimeComponent::init(time))
        .with(PositionComponent::init(position))
        .with(CharacterInfoComponent::init(CharacterInfo::init(
            "TestCharacter",
            Defenses::just_health(10),
            Temperature::init(),
        )))
        .with(SkillResourceComponent::init(&[]))
        .with(SkillsComponent::init(&[]))
        .with(StatusComponent::init())
        .build();
}

pub fn set_temperature(ecs: &mut World, player: Entity, temperature: i32) {
    ecs.write_storage::<CharacterInfoComponent>()
        .grab_mut(player)
        .character
        .temperature
        .set_temperature(temperature);
}

pub fn set_health(ecs: &mut World, player: Entity, health: u32) {
    ecs.write_storage::<CharacterInfoComponent>().grab_mut(player).character.defenses = Defenses::just_health(health);
}

// This can be dangerous, if something invalidates the entity reference
// then you can crash here
pub fn assert_position(ecs: &World, entity: &Entity, expected: Point) {
    let position = ecs.get_position(entity);
    assert_points_equal(position.single_position(), expected);
}

pub fn assert_not_at_position(ecs: &World, entity: &Entity, expected: Point) {
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
    for position in (&positions).join() {
        println!("{}", position.position);
    }
    println!("");
}
