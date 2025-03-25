// This is the main module file that exports our ECS functionality.
// It re-exports the components we want to make available to the rest of the codebase.

mod manager;
mod world_store;
pub mod systems;
pub mod components;

pub use manager::EcsManager;
pub use world_store::WorldStore;