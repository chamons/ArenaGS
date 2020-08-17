// The game rules and logic for the games
mod character_info;
pub use character_info::CharacterInfo;

mod skills;
use skills::invoke_skill;
pub use skills::{can_invoke_skill, get_skill, is_good_target, AmmoKind, SkillInfo, TargetType};

mod components;
pub use components::*;

mod map;
pub use map::{element_at_location, Map, MapHitTestResult, MapTile, MAX_MAP_TILES};

mod physics;
pub use physics::*;

mod time;
pub use time::*;

mod actions;
pub use actions::*;

mod log;
pub use log::*;

mod ai;
pub use ai::{take_enemy_action, BehaviorKind};

mod combat;
pub use combat::*;

mod events;
pub use events::*;

#[cfg(test)]
mod test_helpers;
#[cfg(test)]
pub use test_helpers::*;

mod strength;
pub use strength::*;

mod defenses;
pub use defenses::*;

mod temperature;
pub use temperature::*;
