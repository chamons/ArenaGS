use specs::prelude::*;

use super::{
    create_world, find_character_at_location, Character, CharacterInfoComponent, Map, MapComponent, PlayerComponent, PositionComponent, SkillResourceComponent,
    TimeComponent,
};
use crate::atlas::{Point, SizedPoint};

pub struct StateBuilder {
    ecs: World,
}

impl StateBuilder {
    pub fn with_timed<'a>(&'a mut self, time: i32) -> &'a mut Self {
        self.ecs.create_entity().with(TimeComponent::init(time)).build();
        self
    }

    pub fn with_character<'a>(&'a mut self, x: u32, y: u32, time: i32) -> &'a mut Self {
        self.ecs
            .create_entity()
            .with(TimeComponent::init(time))
            .with(PositionComponent::init(SizedPoint::init(x, y)))
            .with(CharacterInfoComponent::init(Character::init()))
            .with(SkillResourceComponent::init(&[]))
            .build();
        self
    }

    pub fn with_sized_character<'a>(&'a mut self, position: SizedPoint, time: i32) -> &'a mut Self {
        self.ecs
            .create_entity()
            .with(TimeComponent::init(time))
            .with(PositionComponent::init(position))
            .with(CharacterInfoComponent::init(Character::init()))
            .with(SkillResourceComponent::init(&[]))
            .build();
        self
    }

    pub fn with_player<'a>(&'a mut self, x: u32, y: u32, time: i32) -> &'a mut Self {
        self.ecs
            .create_entity()
            .with(TimeComponent::init(time))
            .with(PositionComponent::init(SizedPoint::init(x, y)))
            .with(PlayerComponent::init())
            .with(CharacterInfoComponent::init(Character::init()))
            .with(SkillResourceComponent::init(&[]))
            .build();
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
