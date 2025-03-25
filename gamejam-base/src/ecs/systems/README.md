# ECS Systems

This directory contains the Entity Component System (ECS) systems for the game. Systems are functions that operate on components to implement game logic.

## Adding New Systems

To add a new system:

1. Create a new function in `mod.rs` that takes the required component views as parameters
2. Use Shipyard's view types (`View<T>`, `ViewMut<T>`) to access components
3. Implement your system logic using the provided views
4. Export the function with `pub`

Example:
```rust
pub fn my_system(mut positions: ViewMut<Position>, velocities: View<Velocity>) {
    (&mut positions, &velocities).iter().for_each(|(pos, vel)| {
        // Your system logic here
    });
}
```

## Using Systems

Systems can be called in the main game loop using the `EcsManager`:

```rust
ecs_manager.run(|world| {
    world.run(my_system);
});
```

## Available Systems

- `movement_system`: Updates entity positions based on their velocities
- `bounce_system`: Makes entities bounce off screen boundaries
- `debug_system`: Logs entity positions for debugging

## Best Practices

1. Keep systems focused on a single responsibility
2. Use appropriate view types (View/ViewMut) based on whether you need to modify components
3. Use Shipyard's iterator methods for efficient component access
4. Add documentation comments to explain system behavior
5. Consider system dependencies and execution order
