// The game rules and logic for the games
mod character;
pub use character::Character;

mod skills;
use skills::invoke_skill;
pub use skills::{can_invoke_skill, get_skill, is_good_target, AmmoKind, SkillInfo, SkillResourceComponent, TargetType};

mod components;
pub use components::{create_world, CharacterInfoComponent, FieldComponent, FrameComponent, Framer, PlayerComponent, Positions, SkillsComponent};

mod map;
pub use map::{element_at_location, Map, MapComponent, MapHitTestResult, MapTile, MAX_MAP_TILES};

mod physics;
pub use physics::*;

mod position_component;
pub use position_component::PositionComponent;

mod time;
pub use time::*;

mod actions;
pub use actions::*;

mod log;
pub use log::*;

mod ai;
pub use ai::{take_enemy_action, BehaviorComponent, BehaviorKind};

mod combat;
pub use combat::{bolt, explode, field, melee, start_bolt, AttackComponent, AttackKind, BoltKind, FieldKind, WeaponKind};

mod events;
pub use events::{EventComponent, EventCoordinator, EventKind};

mod delayed_effect;
pub use delayed_effect::{tick_delayed_effects, DelayedEffect, DelayedEffectComponent, DelayedEffectKind};

#[cfg(test)]
mod test_helpers;
#[cfg(test)]
pub use test_helpers::*;
