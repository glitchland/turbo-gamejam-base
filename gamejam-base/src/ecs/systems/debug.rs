use shipyard::{View, IntoIter, IntoWithId};
use crate::ecs::components::Position;
use turbo::prelude::*;

/// System that logs entity positions
pub fn debug_system(positions: View<Position>) {
    for (entity, pos) in positions.iter().with_id() {
        log!("Entity {} at position: ({}, {})", entity.index(), pos.x, pos.y);
    }
} 