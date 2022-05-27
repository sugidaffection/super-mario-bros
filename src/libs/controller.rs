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
    if self.left {
      player.set_dir(PlayerDirection::Left)
    } else {
      player.stop()
    };
    if self.right {
      player.set_dir(PlayerDirection::Right)
    } else {
      player.stop()
    };

    if self.left || self.right {
      player.walk();
    }

    if self.jump {
      player.jump();
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
