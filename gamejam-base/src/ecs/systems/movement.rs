use shipyard::{ViewMut, View, IntoIter};
use crate::ecs::components::{Position, Velocity};

/// System that updates positions based on velocities
pub fn movement_system(mut positions: ViewMut<Position>, velocities: View<Velocity>) {
    (&mut positions, &velocities).iter().for_each(|(pos, vel)| {
        pos.x += vel.x;
        pos.y += vel.y;
    });
} 