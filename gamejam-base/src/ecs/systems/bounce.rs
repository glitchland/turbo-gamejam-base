use shipyard::{ViewMut, View, IntoIter};
use crate::ecs::components::{Position, Velocity};

/// System that bounces entities off screen boundaries
pub fn bounce_system(
    (canvas_width, canvas_height): (f32, f32),
    mut positions: ViewMut<Position>, 
    mut velocities: ViewMut<Velocity>
) {
    for (pos, vel) in (&mut positions, &mut velocities).iter() {
        // Check x bounds
        if pos.x <= 0.0 || pos.x >= canvas_width {
            vel.x *= -1.0;
            pos.x = pos.x.clamp(0.0, canvas_width);
        }
        
        // Check y bounds
        if pos.y <= 0.0 || pos.y >= canvas_height {
            vel.y *= -1.0;
            pos.y = pos.y.clamp(0.0, canvas_height);
        }
    }
}