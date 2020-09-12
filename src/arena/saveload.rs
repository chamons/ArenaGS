use std::fs::{read_to_string, File};
#[cfg(test)]
use std::io::Read;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker, SimpleMarkerAllocator};
use specs_derive::Component;

use super::components::*;
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

pub fn map_background(ecs: &mut World) {
    ecs.create_entity()
        .with(RenderComponent::init(RenderInfo::init_with_order(
            SpriteKinds::BeachBackground,
            RenderOrder::Background,
        )))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();
}

pub fn new_world() -> BoxResult<World> {
    let mut ecs = create_world();
    add_ui_extension(&mut ecs);
    crate::clash::content::spawner::player(&mut ecs);
    crate::clash::content::spawner::bird_monster(&mut ecs);

    let map_data_path = Path::new(&get_exe_folder()).join("maps").join("beach").join("map1.dat");
    let map_data_path = map_data_path.stringify();
    ecs.insert(MapComponent::init(Map::init(map_data_path)?));

    map_background(&mut ecs);

    Ok(ecs)
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: Map,
}

#[cfg(test)]
pub fn save_to_string(ecs: &mut World) -> String {
    let mut writer = vec![];
    save(ecs, &mut writer);

    let mut out = Vec::new();
    let mut c = writer.as_slice();
    c.read_to_end(&mut out).unwrap();
    String::from_utf8(out).unwrap()
}

pub fn save_to_disk(ecs: &mut World) {
    let mut writer = File::create("./savegame.sav").unwrap();
    save(ecs, &mut writer);
}

fn save<T: Write>(ecs: &mut World, writer: &mut T) {
    let mut serializer = serde_json::Serializer::new(&mut *writer);

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
        StatusComponent,
        OrbComponent,
        FlightComponent,
        SkipRenderComponent,
        FieldCastComponent,
        RenderComponent,
        BattleSceneStateComponent,
        MousePositionComponent,
        SerializationHelper
    );
}

pub fn load_from_disk() -> BoxResult<World> {
    let data = read_to_string("./savegame.sav")?;
    load(data)
}

#[cfg(test)]
pub fn load_from_string(data: String) -> BoxResult<World> {
    load(data)
}

fn load(data: String) -> BoxResult<World> {
    let mut ecs = create_world();
    add_ui_extension(&mut ecs);

    {
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
            StatusComponent,
            OrbComponent,
            FlightComponent,
            SkipRenderComponent,
            FieldCastComponent,
            RenderComponent,
            BattleSceneStateComponent,
            MousePositionComponent,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::Point;

    #[test]
    fn save_load_smoke() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let save = save_to_string(&mut ecs);
        ecs = load_from_string(save).unwrap();
        assert_eq!(2, find_all_characters(&ecs).len());
    }

    #[test]
    fn save_load_with_field() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_field(&mut ecs, &player, Point::init(2, 6), FieldEffect::Damage(Damage::init(1)), FieldKind::Fire, 1);
        wait_for_animations(&mut ecs);

        assert_field_exists(&ecs, 2, 6);

        let save = save_to_string(&mut ecs);
        ecs = load_from_string(save).unwrap();

        assert_field_exists(&ecs, 2, 6);
    }

    #[test]
    fn save_load_with_orbs() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);

        assert_field_exists(&ecs, 2, 4);
        assert_field_count(&ecs, 5);

        let save = save_to_string(&mut ecs);
        ecs = load_from_string(save).unwrap();

        assert_field_exists(&ecs, 2, 4);
        assert_field_count(&ecs, 5);
    }
}
