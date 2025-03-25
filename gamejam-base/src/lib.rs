// declare the submodules here
mod screens;
mod ecs;

// import the submodules here
use screens::*;
use ecs::*;
use ecs::systems::*;
use ecs::components::*;
use turbo::prelude::*;
use shipyard::{Component, IntoIter, ViewMut, View, UniqueView};

init! {
    struct GameState {
        screen_manager: ScreenManager,
        ecs_manager: EcsManager,
        canvas_width: f32,
        canvas_height: f32,
    } = {

        // Get turbo canvas dimensions
        let canvas_size = resolution();
        let canvas_width = canvas_size.0 as f32;
        let canvas_height = canvas_size.1 as f32;

        // Create the initial game state
        let mut screen_manager = ScreenManager::new();

        screen_manager.push_screen(ScreenVariant::Menu(MenuScreen::new()));
        screen_manager.push_screen(ScreenVariant::Game(GameScreen::new()));

        // Create the ECS manager
        let ecs_manager = EcsManager::new();

        // Create ECS entities
        let enemy = ecs_manager.create_entity();
        ecs_manager.add_components(enemy, (
            Position { x: 200.0, y: 150.0 },
            Velocity { x: -1.0, y: 0.0 },
            Enemy { color: 0xffffffff, health: 100, speed: 2, size: 2 },
        ));

        Self {
            screen_manager,
            ecs_manager,
            canvas_width,
            canvas_height,
        }
    }
}

// This is the main loop
go! {
    // Load the game state
    let mut state = GameState::load();

    // Example: Get the number of entities
    let count = state.ecs_manager.entity_count();
    text!("Entity count: {}", count; font = "large", x = 64); 
    
    // Update the active screen
    if let Some(top_screen) = state.screen_manager.get_top_screen() {
        state.screen_manager.update_top(1.0/60.0);
    }
    
    // Run the ECS systems
    state.ecs_manager.run(|world| {
        // Run systems in sequence
        world.run(movement_system);
        world.run_with_data(bounce_system, (state.canvas_width, state.canvas_height));
        world.run(debug_system);
        world.run(render_enemy_system);
    });

    // Render the screens
    state.screen_manager.render_stack();
    
    state.save();
}
