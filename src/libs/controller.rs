use piston_window::{Key, ButtonState, ImageSize};

use crate::player::{Player, PlayerDirection};

pub struct Controller {
  left: bool,
  right: bool,
  crouch: bool,
  jump: bool,
  shoot: bool,
  run: bool
}

impl Controller {

  pub fn new() -> Self {
    Self {
      left: false,
      right: false,
      crouch: false,
      jump: false,
      shoot: false,
      run: false
    }
  }

  pub fn update<I: ImageSize>(&mut self, player: &mut Player<I>, dt: f64) {
    if self.left || self.right {
      player.walk(dt);
    }

    if self.left { player.set_dir(PlayerDirection::Left) };
    if self.right { player.set_dir(PlayerDirection::Right) };

    if self.jump {
      player.jump();
    }

  }

  pub fn keyboard_event(&mut self, key: Key, state: ButtonState) {
    match key {
      Key::A |
      Key::Left => self.left = state == ButtonState::Press,
      Key::D |
      Key::Right => self.right = state == ButtonState::Press,
      Key::Space |
      Key::Up => self.jump = state == ButtonState::Press,
      Key::S |
      Key::Down => self.crouch = state == ButtonState::Press,
      Key::X => self.shoot = state == ButtonState::Press,
      Key::LShift |
      Key::RShift => self.run = state == ButtonState::Press,
      _ => {}
    }
  }

}