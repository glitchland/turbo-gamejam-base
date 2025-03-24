#![allow(unused_variables, dead_code)]

// These are the traits for the screens in the game
pub trait Screen {
    fn init(&mut self) {}
    fn handle_input(&mut self) {}
    fn update(&mut self, dt: f32) {}
    fn render(&self) {}
    fn on_suspend(&mut self) {}
    fn on_resume(&mut self) {}
    fn is_active(&self) -> bool {
        true
    }
}