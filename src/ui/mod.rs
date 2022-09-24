mod game_state;
use bevy_ecs::prelude::*;
pub use game_state::GameState;

mod utils;
pub use utils::*;

mod scenes;
pub use scenes::*;

mod draw;
pub use draw::*;

mod sprite;
pub use sprite::*;

mod animation;
pub use animation::*;

pub fn create_ui_schedule() -> SystemStage {
    SystemStage::single_threaded()
        .with_system(start_animation)
        .with_system(end_animation)
        .with_system(clear_event_buffers)
}

pub fn setup_ui_resources(world: &mut World) {
    world.insert_resource(Events::<SpriteAnimateActionEvent>::default());
    world.insert_resource(Events::<SpriteAnimateActionComplete>::default());
}

// Since we aren't using Bevy's App model, we have to clear our event buffers by hand
pub fn clear_event_buffers(mut a: ResMut<Events<SpriteAnimateActionEvent>>, mut b: ResMut<Events<SpriteAnimateActionComplete>>) {
    a.update();
    b.update();
}
