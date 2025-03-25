# Turbo Screen Manager System

This is a flexible, modular screen management system for Turbo-Genesis-SDK games. It provides a stack-based approach to managing game screens, with support for transitions, overlays, and screen-specific behaviors.

## Features

- **Stack-based screen management**: Push, pop, and replace screens
- **Screen lifecycle events**: Initialize, activate, deactivate
- **Support for translucent screens**: Create overlay screens like pause menus
- **Type-safe screen identification**: Use downcasting to identify screen types
- **Clean separation of concerns**: Each screen manages its own state and rendering

## How to Use

### 1. Create Custom Screen Types

Create custom screens by implementing the `GameScreen` trait:

```rust
use crate::managers::GameScreen;
use std::any::Any;

pub struct MyCustomScreen {
    // Your screen-specific state here
}

impl GameScreen for MyCustomScreen {
    fn init(&mut self) {
        // Initialize resources when screen is first created
    }
    
    fn update(&mut self) {
        // Update screen logic
    }
    
    fn draw(&self) {
        // Draw screen content
    }
    
    fn activated(&mut self) {
        // Called when screen becomes active (top of stack)
    }
    
    fn deactivated(&mut self) {
        // Called when screen is no longer active
    }
    
    fn is_translucent(&self) -> bool {
        // Return true if screens behind this one should still be visible
        false
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
```

### 2. Initialize the Screen Manager

In your game's init block:

```rust
turbo::init! {
    struct GameState {
        screen_manager: ScreenManager,
    } = {
        // Create screen manager and add initial screen
        let mut screen_manager = ScreenManager::new();
        screen_manager.push_screen(Box::new(TitleScreen::new()));
        
        Self {
            screen_manager,
        }
    }
}
```

### 3. Update and Draw in the Game Loop

In your game loop:

```rust
turbo::go! {
    let mut state = GameState::load();
    
    // Handle screen-specific transitions
    if let Some(active_screen) = state.screen_manager.active_screen() {
        // Use downcast_ref to identify screen type and handle transitions
        if let Some(title_screen) = active_screen.as_any().downcast_ref::<TitleScreen>() {
            // Handle title screen transitions
        }
        else if let Some(game_screen) = active_screen.as_any().downcast_ref::<GameScreen>() {
            // Handle gameplay transitions
        }
    }
    
    // Update and draw all visible screens
    state.screen_manager.update();
    state.screen_manager.draw();
    
    state.save();
}
```

### 4. Screen Transitions

Use these methods to manage screens:

- `push_screen(screen)`: Add a new screen on top (e.g., pause menu)
- `pop_screen()`: Remove the top screen (e.g., resume game)
- `replace_screen(screen)`: Replace top screen with a new one (e.g., change levels)

## Example Screens

The codebase includes these example screens:

- **TitleScreen**: Main menu screen
- **GameScreen**: Basic gameplay screen with player movement
- **PauseScreen**: Translucent overlay with a simple menu
- **GameOverScreen**: End game screen with score display

## Extending the System

You can extend this system by:

- Creating new screen types
- Adding transitions between screens
- Implementing screen-specific input handlers
- Creating reusable UI components for screens
- Adding animations during screen transitions 