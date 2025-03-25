use shipyard::World;
use std::collections::HashMap;

thread_local! {
    // Store Worlds in a thread-local HashMap
    static WORLD_STORE: std::cell::RefCell<HashMap<u64, World>> = std::cell::RefCell::new(HashMap::new());
    static NEXT_ID: std::cell::RefCell<u64> = std::cell::RefCell::new(1);
}

/// A helper struct for managing ECS World instances.
pub struct WorldStore;

impl WorldStore {
    /// Creates a new unique ID for tracking World instances.
    pub fn generate_id() -> u64 {
        NEXT_ID.with(|next_id| {
            let mut next = next_id.borrow_mut();
            let id = *next;
            *next += 1;
            id
        })
    }

    /// Stores a World instance with the given ID.
    pub fn store_world(id: u64, world: World) {
        WORLD_STORE.with(|store| {
            let mut store = store.borrow_mut();
            store.insert(id, world);
        });
    }

    /// Performs an operation on a World by its ID.
    /// If the World doesn't exist yet, it creates a new one.
    pub fn with_world<F, R>(id: u64, f: F) -> R
    where
        F: FnOnce(&mut World) -> R,
    {
        WORLD_STORE.with(|store| {
            let mut store = store.borrow_mut();
            let world = store.entry(id).or_insert_with(World::new);
            f(world)
        })
    }

    /// Removes a World by its ID.
    pub fn remove_world(id: u64) {
        WORLD_STORE.with(|store| {
            let mut store = store.borrow_mut();
            store.remove(&id);
        });
    }
}