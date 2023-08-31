use std::collections::HashMap;

use piston_window::{ButtonState, Key};

pub enum KeyAction {
    MoveLeft,
    MoveRight,
    Jump,
    Run,
}

pub struct Controller {
    pub left: bool,
    pub right: bool,
    pub crouch: bool,
    pub jump: bool,
    pub shoot: bool,
    pub run: bool,
    inputs: HashMap<Key, KeyAction>,
}

impl Controller {
    pub fn new() -> Self {
        let mut inputs = HashMap::new();
        inputs.insert(Key::A, KeyAction::MoveLeft);
        inputs.insert(Key::Left, KeyAction::MoveLeft);
        inputs.insert(Key::D, KeyAction::MoveRight);
        inputs.insert(Key::Right, KeyAction::MoveRight);
        inputs.insert(Key::Up, KeyAction::Jump);
        inputs.insert(Key::Space, KeyAction::Jump);
        inputs.insert(Key::LShift, KeyAction::Run);
        Self {
            left: false,
            right: false,
            crouch: false,
            jump: false,
            shoot: false,
            run: false,
            inputs,
        }
    }

    pub fn keyboard_event(&mut self, key: Key, state: ButtonState) {
        if let Some(input) = self.inputs.get(&key) {
            match input {
                KeyAction::MoveLeft => self.left = state == ButtonState::Press,
                KeyAction::MoveRight => self.right = state == ButtonState::Press,
                KeyAction::Jump => self.jump = state == ButtonState::Press,
                KeyAction::Run => self.run = state == ButtonState::Press,
            }
        }
    }

    pub fn reset(&mut self) {
        self.left = false;
        self.right = false;
        self.jump = false;
        self.run = false;
    }
}
