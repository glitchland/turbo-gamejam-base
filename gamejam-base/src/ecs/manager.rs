use shipyard::{
    World,
    Component, 
    EntityId,
    EntitiesView,
    View,
    Get,
    Unique,
    UniqueView
};
use turbo::prelude::*;
use crate::{BorshDeserialize, BorshSerialize};
use super::world_store::WorldStore;

/// A simple manager for handling game entities and their components
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct EcsManager {
    world_id: u64,
}

impl EcsManager {
    /// Creates a new ECS manager
    pub fn new() -> Self {
        let id = WorldStore::generate_id();
        let world = World::new();
        WorldStore::store_world(id, world);
        
        Self { world_id: id }
    }

    /// Helper to access the World
    pub fn with_world<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut World) -> R,
    {
        WorldStore::with_world(self.world_id, f)
    }

    /// Creates a new entity
    pub fn create_entity(&self) -> EntityId {
        self.with_world(|world| world.add_entity(()))
    }

    /// Adds components to an entity
    pub fn add_components<C>(&self, entity: EntityId, components: C)
    where
        C: shipyard::TupleAddComponent,
    {
        self.with_world(|world| {
            world.add_component(entity, components);
        });
    }

    /// Deletes an entity
    pub fn delete_entity(&self, entity: EntityId) {
        self.with_world(|world| {
            world.delete_entity(entity);
        });
    }

    /// Runs a system function
    pub fn run<F>(&self, f: F)
    where
        F: FnOnce(&mut World),
    {
        self.with_world(|world| f(world));
    }

    /// Gets a component from an entity
    pub fn get_component<T>(&self, entity: EntityId) -> Option<T> 
    where
        T: Component + Clone,
    {
        self.with_world(|world| {
            let storage = world.borrow::<View<T>>().ok()?;
            storage.get(entity).ok().map(|component| component.clone())
        })
    }

    /// Gets a unique component
    pub fn get_unique<T>(&self) -> Option<T>
    where
        T: Component + Send + Sync + Unique + Clone,
    {
        self.with_world(|world| {
            world.borrow::<UniqueView<T>>()
                .ok()
                .map(|view| (*view).clone())
        })
    }

    pub fn entity_exists(&self, entity: EntityId) -> bool {
        self.with_world(|world| {
            let mut all_storages = world
                .all_storages_mut()
                .expect("Failed to borrow all_storages mut");
            all_storages.is_entity_alive(entity)
        })
    }
    
    pub fn count_entities(&self) -> usize {
        self.with_world(|world| {
            let all_storages = world
                .all_storages()
                .expect("Failed to borrow AllStorages");
            // Borrow `EntitiesView` instead of `Entities`
            let entities_view = all_storages
                .borrow::<EntitiesView>()
                .expect("Failed to borrow EntitiesView");
            
            entities_view.iter().count()
        })
    }

    /// Alias for count_entities
    pub fn entity_count(&self) -> usize {
        self.count_entities()
    }
}