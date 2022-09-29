mod game_state;
use bevy_ecs::prelude::*;
pub use game_state::GameState;

mod utils;
pub use utils::*;

mod battle;
pub use battle::*;

mod draw;
pub use draw::*;

mod sprite;
pub use sprite::*;

mod animation;
pub use animation::*;

mod scenes;
pub use scenes::*;

mod schedule;
pub use schedule::*;

mod events;
pub use events::*;

pub const GAME_WIDTH: f32 = 1280.0;
pub const GAME_HEIGHT: f32 = 960.0;

pub trait BackingImage {
    fn filename(&self) -> &str;
}

pub fn setup_ui_resources(world: &mut World) {
    world.insert_resource(Events::<SpriteAnimateActionEvent>::default());
    world.insert_resource(Events::<SpriteAnimateActionCompleteEvent>::default());
    world.insert_resource(Events::<MovementAnimationEvent>::default());
    world.insert_resource(Events::<MovementAnimationComplete>::default());
}

// Since we aren't using Bevy's App model, we have to clear our event buffers by hand
pub fn clear_event_buffers(
    mut a: ResMut<Events<SpriteAnimateActionEvent>>,
    mut b: ResMut<Events<SpriteAnimateActionCompleteEvent>>,
    mut c: ResMut<Events<MovementAnimationEvent>>,
    mut d: ResMut<Events<MovementAnimationComplete>>,
) {
    a.update();
    b.update();
    c.update();
    d.update();
}
