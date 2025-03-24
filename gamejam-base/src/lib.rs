// declare the submodules here
mod screens;

// import the submodules here
use screens::*;

turbo::init! {
    struct GameState {
        screen_manager: ScreenManager
    } = {
        // Create the initial game state
        let mut screen_manager = ScreenManager::new();

        screen_manager.push_screen(ScreenVariant::Menu(MenuScreen::new()));
        screen_manager.push_screen(ScreenVariant::Game(GameScreen::new()));
        
        Self {
            screen_manager
        }
    }
}

turbo::go! {

}
