use piston_window::{ButtonState, ImageSize, Key};

use crate::player::{Player, PlayerDirection};

pub struct Controller {
  pub left: bool,
  pub right: bool,
  crouch: bool,
  jump: bool,
  shoot: bool,
  run: bool,
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

  pub fn update<I: ImageSize>(&mut self, player: &mut Player<I>) {
    if !(self.left || self.right || self.jump) {
      player.stop();
    }
    if self.left {
      player.walk();
      player.set_dir(PlayerDirection::Left)
    }

    if self.right {
      player.walk();
      player.set_dir(PlayerDirection::Right)
    }

    if self.jump {
      player.jump();
    }
  }

  pub fn keyboard_event(&mut self, key: Key, state: ButtonState) {
    self.left = state == ButtonState::Press && [Key::A, Key::Left].iter().any(|&x| x == key);
    self.right = state == ButtonState::Press && [Key::D, Key::Right].iter().any(|&x| x == key);
    self.jump = state == ButtonState::Press && [Key::Space, Key::Up].iter().any(|&x| x == key);
    self.crouch = state == ButtonState::Press && [Key::S, Key::Down].iter().any(|&x| x == key);
    self.shoot = state == ButtonState::Press && [Key::X].iter().any(|&x| x == key);
    self.run = state == ButtonState::Press && [Key::LShift, Key::RShift].iter().any(|&x| x == key);
  }
}
