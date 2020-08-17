use specs::prelude::*;

use super::{EventStatus, Scene};

pub trait Storyteller {
    fn initial_scene(&self) -> Box<dyn Scene>;
    fn check_place(&self, ecs: &World) -> EventStatus;
}
