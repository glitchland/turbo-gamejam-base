use shipyard::{View, ViewMut, IntoIter};
use crate::ecs::components::{Position, Enemy};
use turbo::prelude::*;

/// System that renders only enemy entities
pub fn render_enemy_system(
    positions: View<Position>,
    enemies: View<Enemy>,
) {
    // This will only iterate over entities that have BOTH Position and Enemy components
    for (pos, enemy) in (&positions, &enemies).iter() {
        log!("Drawing Enemy at position: ({}, {})", pos.x, pos.y);
        // Draw the enemy
        circ!(
            x = pos.x as i32,
            y = pos.y as i32,
            d = enemy.size as u32,
            color = enemy.color,
        );
    }
} 