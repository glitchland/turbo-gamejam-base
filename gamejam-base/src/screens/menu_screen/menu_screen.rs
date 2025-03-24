use turbo::prelude::*;
use crate::{BorshDeserialize, BorshSerialize};

use crate::screens::trait_def::Screen;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct MenuScreen {
    is_active: bool,
}

impl MenuScreen {
    pub fn new() -> Self {
        Self { is_active: true }
    }
}

impl Screen for MenuScreen {
    fn init(&mut self) {
        log!("MenuScreen: init");
    }

    fn handle_input(&mut self) {
        log!("MenuScreen: handle_input");
    }

    fn update(&mut self, _dt: f32) {
        log!("MenuScreen: update");
    }

    fn render(&self) {
        log!("MenuScreen: render");
    }

    fn on_suspend(&mut self) {
        log!("MenuScreen: on_suspend");
        self.is_active = false;
    }

    fn on_resume(&mut self) {
        log!("MenuScreen: on_resume");
        self.is_active = true;
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}
