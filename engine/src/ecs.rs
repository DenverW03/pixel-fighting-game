use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub struct Entity(u32);

pub struct ComponentStorage<T> {
    pub components: HashMap<Entity, T>,
}

impl<T> ComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert_component(&mut self, entity: Entity, component: T) {
        self.components.insert(entity, component);
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.components.get(&entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(&entity)
    }

    pub fn remove(&mut self, entity: Entity) {
        self.components.remove(&entity);
    }
}
