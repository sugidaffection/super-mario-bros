use piston_window::{ButtonState, ImageSize, Key};

use crate::libs::player::{Player, PlayerDirection};
#[derive(Debug)]
pub struct Controller {
    pub left: bool,
    pub right: bool,
    pub crouch: bool,
    pub jump: bool,
    pub shoot: bool,
    pub run: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            crouch: false,
            jump: false,
            shoot: false,
            run: false,
        }
    }

    pub fn keyboard_event(&mut self, key: Key, state: ButtonState) {
        match key {
            Key::A | Key::Left => self.left = state == ButtonState::Press,
            Key::D | Key::Right => self.right = state == ButtonState::Press,
            Key::Space | Key::Up => self.jump = state == ButtonState::Press,
            Key::S | Key::Down => self.crouch = state == ButtonState::Press,
            Key::X => self.shoot = state == ButtonState::Press,
            Key::LShift | Key::RShift => self.run = state == ButtonState::Press,
            _ => {}
        }
    }
}
