use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

pub struct World {
    next_id: u32,
    storages: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            storages: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        Entity(id)
    }

    fn get_storage<T: 'static>(&mut self) -> &mut ComponentStorage<T> {
        let type_id = TypeId::of::<T>();

        if !self.storages.contains_key(&type_id) {
            self.storages
                .insert(type_id, Box::new(ComponentStorage::<T>::new()));
        }

        self.storages
            .get_mut(&type_id)
            .unwrap()
            .downcast_mut::<ComponentStorage<T>>()
            .unwrap()
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let component_storage = self.get_storage::<T>();
        component_storage.insert_component(entity, component)
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get(&type_id)
            .and_then(|s| s.downcast_ref::<ComponentStorage<T>>())
            .and_then(|st| st.components.get(&entity))
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get_mut(&type_id)
            .and_then(|s| s.downcast_mut::<ComponentStorage<T>>())
            .and_then(|st| st.components.get_mut(&entity))
    }
}
