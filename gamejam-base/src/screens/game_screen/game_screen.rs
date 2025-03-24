use turbo::prelude::*;
use crate::{BorshDeserialize, BorshSerialize};
use crate::screens::trait_def::Screen;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct GameScreen {
    is_active: bool,
}

impl GameScreen {
    pub fn new() -> Self {
        Self { is_active: true }
    }
}

impl Screen for GameScreen {
    fn init(&mut self) {
        log!("GameScreen: init");
    }

    fn handle_input(&mut self) {
        log!("GameScreen: handle_input");
    }

    fn update(&mut self, _dt: f32) {
        log!("GameScreen: update");
    }

    fn render(&self) {
        log!("GameScreen: render");
    }

    fn on_suspend(&mut self) {
        log!("GameScreen: on_suspend");
        self.is_active = false;
    }

    fn on_resume(&mut self) {
        log!("GameScreen: on_resume");
        self.is_active = true;
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}
