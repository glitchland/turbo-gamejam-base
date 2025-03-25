# GameJam Base - Turbo + Shipyard ECS & Screen Management

This repository provides a flexible and modular template for Turbo Genesis SDK-based games, utilizing Shipyard ECS (Entity Component System) and a neat screen management system. The structure supports clean and scalable game development.

## Project Structure

- **ECS (Entity Component System)**: Implements modular entity behaviors and logic.
- **Screen Manager**: Provides a stack-based approach to manage different game screens (menus, gameplay, overlays).

## Features

- **Entity Component System** (using Shipyard)
  - Define entities with custom components.
  - Systems to update entity logic (e.g., movement, bouncing, rendering).
- **Screen Management** (custom stack based state machine)
  - Push, pop, replace screens easily.
  - Lifecycle hooks (`init`, `update`, `render`, `on_suspend`, `on_resume`).
- **Clean API**
  - Easily readable and extendable Rust code structure.

---

## Getting Started

### Setup

Clone and run:

```bash
cargo build
cargo run
```

### Example Usage

#### Creating an Entity with Components

```rust
// Create a new ECS entity
let entity = ecs_manager.create_entity();

// Add components to the entity
ecs_manager.add_components(entity, (
    Position { x: 50.0, y: 50.0 },
    Velocity { x: 1.5, y: 0.5 },
    Enemy { color: 0xff0000ff, health: 100, speed: 2, size: 5 },
));
```

#### Running Systems

Systems are modular functions executing entity logic. Example from main game loop:

```rust
state.ecs_manager.run(|world| {
    world.run(movement_system);
    world.run_with_data(bounce_system, (canvas_width, canvas_height));
    world.run(debug_system);
    world.run(render_enemy_system);
});
```

#### Screen Management

Push, pop, or replace screens during gameplay.

```rust
// Push new screen
state.screen_manager.push_screen(ScreenVariant::Menu(MenuScreen::new()));

// Pop the current screen
state.screen_manager.pop_screen();

// Replace current screen with a new one
state.screen_manager.change_screen(ScreenVariant::Game(GameScreen::new()));
```

---

## Components Overview

- `Position`: Stores x, y coordinates.
- `Velocity`: Stores movement speed along x, y axes.
- `Enemy`: Stores attributes for enemy entities (health, speed, size, color).

**Example:**

```rust
#[derive(Component, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
```

---

## Systems Overview

- `movement_system`: Updates positions based on velocities.
- `bounce_system`: Bounces entities off canvas boundaries.
- `debug_system`: Logs entity positions for debugging.
- `render_enemy_system`: Draws enemy entities.

**Example: Bounce System**

```rust
pub fn bounce_system(
    (canvas_width, canvas_height): (f32, f32),
    mut positions: ViewMut<Position>, 
    mut velocities: ViewMut<Velocity>
) {
    for (pos, vel) in (&mut positions, &mut velocities).iter() {
        if pos.x <= 0.0 || pos.x >= canvas_width {
            vel.x *= -1.0;
            pos.x = pos.x.clamp(0.0, canvas_width);
        }
        if pos.y <= 0.0 || pos.y >= canvas_height {
            vel.y *= -1.0;
            pos.y = pos.y.clamp(0.0, canvas_height);
        }
    }
}
```

---

## Screen Management Overview

Manage game states easily with screen stack.

- **Lifecycle methods**: `init`, `handle_input`, `update`, `render`, `on_suspend`, `on_resume`.
- **Screen stack management**: `push_screen`, `pop_screen`, `change_screen`, `set_screen`.

**Example: Defining a Screen**

```rust
impl Screen for GameScreen {
    fn init(&mut self) {
        log!("Game screen initialized");
    }

    fn update(&mut self, _dt: f32) {
        log!("Game screen updated");
    }

    fn render(&self) {
        log!("Rendering game screen");
    }

    fn on_suspend(&mut self) {
        self.is_active = false;
    }

    fn on_resume(&mut self) {
        self.is_active = true;
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}
```

---

## Extending the Project

You can easily extend this setup by:

- Adding new systems to handle specific game logic.
- Defining additional components and screens.
- Creating complex interactions between entities and game states.

---

## Dependencies

- **Shipyard**: ECS framework for Rust [docs](https://docs.rs/shipyard/latest/shipyard/index.html).
- **Turbo Genesis SDK**: A fantastic framework for writing games fast. [docs](https://docs.rs/turbo-genesis-sdk/2.1.0/turbo_genesis_sdk/index.html)
- **once_cell**: For thread-safe initialization. [docs](https://docs.rs/once_cell/1.18.0/once_cell/index.html)

---
## TODO

- Integrate Taffy

## License

This project template is provided under the MIT License. Feel free to use and adapt it to your project's needs.
