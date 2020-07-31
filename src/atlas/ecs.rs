use specs::prelude::*;

pub trait EasyECS<T: Component> {
    fn grab(&self, entity: Entity) -> &T;
}

impl<'a, T: Component> EasyECS<T> for ReadStorage<'a, T> {
    fn grab(&self, entity: Entity) -> &T {
        self.get(entity).unwrap()
    }
}

impl<'a, T: Component> EasyECS<T> for WriteStorage<'a, T> {
    fn grab(&self, entity: Entity) -> &T {
        self.get(entity).unwrap()
    }
}
