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

pub trait EasyMutECS<T: Component> {
    fn shovel(&mut self, entity: Entity, item: T);
    fn grab_mut(&mut self, entity: Entity) -> &mut T;
}

impl<'a, T: Component> EasyMutECS<T> for WriteStorage<'a, T> {
    fn shovel(&mut self, entity: Entity, item: T) {
        self.insert(entity, item).unwrap();
    }

    fn grab_mut(&mut self, entity: Entity) -> &mut T {
        self.get_mut(entity).unwrap()
    }
}
