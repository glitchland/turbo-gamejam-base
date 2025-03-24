#![allow(unused_variables, dead_code)]

use turbo::prelude::*;
use crate::{BorshDeserialize, BorshSerialize};

use crate::screens::menu_screen::MenuScreen;
use crate::screens::game_screen::GameScreen;
use crate::screens::trait_def::Screen;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum ScreenVariant {
    Menu(MenuScreen),
    Game(GameScreen),
    // ...
}

impl ScreenVariant {
    pub fn init(&mut self) {
        match self {
            ScreenVariant::Menu(s) => s.init(),
            ScreenVariant::Game(s) => s.init(),
            // Add other variants to each 
        }
    }

    pub fn handle_input(&mut self) {
        match self {
            ScreenVariant::Menu(s) => s.handle_input(),
            ScreenVariant::Game(s) => s.handle_input(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self {
            ScreenVariant::Menu(s) => s.update(dt),
            ScreenVariant::Game(s) => s.update(dt),
        }
    }

    pub fn render(&self) {
        match self {
            ScreenVariant::Menu(s) => s.render(),
            ScreenVariant::Game(s) => s.render(),
        }
    }

    pub fn on_suspend(&mut self) {
        match self {
            ScreenVariant::Menu(s) => s.on_suspend(),
            ScreenVariant::Game(s) => s.on_suspend(),
        }
    }

    pub fn on_resume(&mut self) {
        match self {
            ScreenVariant::Menu(s) => s.on_resume(),
            ScreenVariant::Game(s) => s.on_resume(),
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            ScreenVariant::Menu(s) => s.is_active(),
            ScreenVariant::Game(s) => s.is_active(),
        }
    }
}

