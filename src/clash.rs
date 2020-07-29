// The game rules and logic for the games
mod character;
pub use character::Character;

mod skills;
use skills::invoke_skill;
pub use skills::{get_skill, SkillInfo, TargetType};

mod components;
pub use components::{create_world, CharacterInfoComponent, FieldComponent, FrameComponent, Framer, PlayerComponent, Positions, SkillsComponent};

mod map;
pub use map::{element_at_location, Map, MapComponent, MapHitTestResult, MapTile, MAX_MAP_TILES};

mod physics;
use physics::{begin_move, can_move_character, complete_move, find_character_at_location, is_area_clear, move_character, point_in_direction, wait};

#[cfg(test)]
use physics::wait_for_animations;

mod position_component;
pub use position_component::PositionComponent;

mod animation;
pub use animation::*;

mod time;
pub use time::*;

mod actions;
pub use actions::*;

mod log;
pub use log::*;

mod ai;
pub use ai::take_enemy_action;

mod combat;
pub use combat::{apply_bolt, begin_bolt, begin_melee, AttackComponent, BoltKind, WeaponKind};

mod events;
pub use events::{EventComponent, EventCoordinator, EventKind};
