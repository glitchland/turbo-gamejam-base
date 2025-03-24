#![allow(unused_imports)]

// declare the submodules
pub mod manager;
pub mod variant;
pub mod trait_def;
pub mod menu_screen;
pub mod game_screen;

// re-export the submodules
pub use manager::ScreenManager;
pub use variant::ScreenVariant;
pub use trait_def::Screen;
pub use menu_screen::MenuScreen;
pub use game_screen::GameScreen;
