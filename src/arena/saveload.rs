use std::fs::{read_to_string, File};
use std::path::Path;

use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker, SimpleMarkerAllocator};
use specs_derive::Component;

use super::components::*;
use super::spawner;
use crate::atlas::{get_exe_folder, BoxResult, EasyPath, ToSerialize};
use crate::clash::*;

// Lovingly borrowed from https://bfnightly.bracketproductions.com/rustbook/chapter_11.html
macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<ToSerialize>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &$data.0, // entities
            &mut $data.1, // marker
            &mut $data.2, // allocater
            &mut $de,
        )
        .unwrap();
        )*
    };
}

pub fn new_world() -> BoxResult<World> {
    let mut ecs = create_world();
    add_ui_extension(&mut ecs);
    spawner::player(&mut ecs);
    spawner::bird_monster(&mut ecs);

    let map_data_path = Path::new(&get_exe_folder()).join("maps").join("beach").join("map1.dat");
    let map_data_path = map_data_path.stringify();
    ecs.insert(MapComponent::init(Map::init(map_data_path)?));

    super::spawner::map_background(&mut ecs);

    Ok(ecs)
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: Map,
}

pub fn save(ecs: &mut World) {
    let writer = File::create("./savegame.sav").unwrap();
    let mut serializer = serde_json::Serializer::new(writer);

    {
        let map = ecs.read_resource::<MapComponent>().map.clone();
        ecs.create_entity()
            .with(SerializationHelper { map })
            .marked::<SimpleMarker<ToSerialize>>()
            .build();
    }
    let data = (ecs.entities(), ecs.read_storage::<SimpleMarker<ToSerialize>>());

    serialize_individually!(
        ecs,
        serializer,
        data,
        PositionComponent,
        RenderComponent,
        BattleSceneStateComponent,
        MousePositionComponent,
        FieldComponent,
        PlayerComponent,
        CharacterInfoComponent,
        MapComponent,
        FrameComponent,
        TimeComponent,
        LogComponent,
        SkillsComponent,
        AttackComponent,
        MovementComponent,
        SkillResourceComponent,
        BehaviorComponent,
        PlayerDeadComponent,
        SerializationHelper
    );
}

pub fn load() -> BoxResult<World> {
    let mut ecs = create_world();
    add_ui_extension(&mut ecs);

    {
        let data = read_to_string("./savegame.sav")?;
        let mut de = serde_json::Deserializer::from_str(&data);
        let mut d = (
            &mut ecs.entities(),
            &mut ecs.write_storage::<SimpleMarker<ToSerialize>>(),
            &mut ecs.write_resource::<SimpleMarkerAllocator<ToSerialize>>(),
        );

        deserialize_individually!(
            ecs,
            de,
            d,
            PositionComponent,
            RenderComponent,
            BattleSceneStateComponent,
            MousePositionComponent,
            FieldComponent,
            PlayerComponent,
            CharacterInfoComponent,
            MapComponent,
            FrameComponent,
            TimeComponent,
            LogComponent,
            SkillsComponent,
            AttackComponent,
            MovementComponent,
            SkillResourceComponent,
            BehaviorComponent,
            PlayerDeadComponent,
            SerializationHelper
        );
    }
    {
        let (map, entity) = {
            let entities = ecs.entities();
            let helper = ecs.read_storage::<SerializationHelper>();
            let (entity, helper) = (&entities, &helper).join().next().unwrap();
            (helper.map.clone(), entity)
        };
        ecs.insert(MapComponent::init(map));
        ecs.delete_entity(entity)?;
    }

    Ok(ecs)
}
