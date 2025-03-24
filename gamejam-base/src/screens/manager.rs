#![allow(unused_variables, dead_code)]

use turbo::prelude::*;
use crate::{BorshDeserialize, BorshSerialize};

use crate::screens::variant::ScreenVariant;

// This is the manager for the screens in the game, it uses a simple stack to manage the screens
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct ScreenManager {
    screens: Vec<ScreenVariant>,
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            screens: Vec::new(),
        }
    }

    // Push a new screen on top
    pub fn push_screen(&mut self, mut new_screen: ScreenVariant) {
        if let Some(top) = self.screens.last_mut() {
            top.on_suspend();
        }
        new_screen.init();
        self.screens.push(new_screen);
    }

    pub fn pop_screen(&mut self) {
        self.screens.pop();
        if let Some(top) = self.screens.last_mut() {
            top.on_resume();
        }
    }

    // Replace top screen with a new one
    pub fn change_screen(&mut self, screen: ScreenVariant) {
        // pop the old top
        self.screens.pop();
        // then push the new one
        self.push_screen(screen);
    }

    // Clear everything and set a single screen
    pub fn set_screen(&mut self, screen: ScreenVariant) {
        self.screens.clear();
        self.push_screen(screen);
    }

    pub fn update_top(&mut self, dt: f32) {
        if let Some(top) = self.screens.last_mut() {
            top.handle_input();
            top.update(dt);
        }
    }

    pub fn render_stack(&self) {
        for screen in &self.screens {
            screen.render();
        }
    }

    pub fn get_top_screen(&self) -> Option<&ScreenVariant> {
        self.screens.last()
    }
}
