use graphics::{Graphics, Transformed};
use graphics::math::Matrix2d;
use piston_window::{ Size, ImageSize };
use sprite::Sprite;
use std::collections::HashMap;

use crate::libs::{transform, physics};
use transform::Transform;
use physics::Physics;


#[derive(PartialEq)]
pub enum PlayerDirection {
  Left,
  Right
}

pub struct Player<I: ImageSize> {
  sprite: Option<Sprite<I>>,
  animation: HashMap<&'static str, Vec<[f64; 4]>>,
  current_animation: &'static str,
  animation_loop: f64,
  transform: Transform,
  physics: Physics,
}

impl <I> Player<I>
where I: ImageSize {
  pub fn new(window_size: Size) -> Player<I> {
    Player {
      sprite: Option::None,
      animation: HashMap::default(),
      current_animation: "idle",
      animation_loop: 0.0,
      transform: Transform::new(),
      physics: Physics::new()
    }
  }

  pub fn set_dir(&mut self, dir: PlayerDirection) {
    self.transform.flip_x = dir == PlayerDirection::Left;
  }

  pub fn walk(&mut self, dt: f64) {
    self.physics.accelerate(dt);
    self.transform.pos.x += self.physics.vel.x;
  }


  pub fn set_sprite(&mut self, sprite: Sprite<I>){
    self.sprite = Option::Some(sprite);
  }

  pub fn append_animation(&mut self, name: &'static str, mut animation: Vec<[f64; 4]>) {
    if let Some(a) = self.animation.get_mut(&name) {
      a.append(&mut animation);
    }else {
      self.animation.insert(name, animation);
    }
  }

  pub fn push_animation(&mut self, name: &'static str, rect: [f64; 4]) {
    if let Some(animation) = self.animation.get_mut(&name) {
      animation.push(rect);
    }else {
      self.animation.insert(name, vec![rect]);
    }
  }

  pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
    match &self.sprite {
      Some(sprite) => sprite.draw(t.trans(self.transform.pos.x, self.transform.pos.y), b),
      None => {}
    }
  }

  pub fn update(&mut self, dt: f64) {

    self.animation_loop += dt * 10.0 * 0.125;

    match &mut self.sprite {
      Some(sprite) => {
        
        if let Some(animation) = self.animation.get(self.current_animation) {
          if let Some(rect) = animation.get(self.animation_loop as usize % animation.len()) {
            sprite.set_src_rect(*rect);
            sprite.set_position(self.transform.size.width, self.transform.size.height);
            sprite.set_scale(2.0, 2.0);
            sprite.set_flip_x(self.transform.flip_x);
          }
        }

      },
      None => {}
    }

  }

  pub fn jump(&mut self) {
  }

  pub fn stop(&mut self) {
    self.physics.acc *= 0.0;
  }

}

