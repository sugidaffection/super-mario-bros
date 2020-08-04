use crate::SpriteManager;
use graphics::math::Matrix2d;
use graphics::{Graphics, Transformed};
use piston_window::{ImageSize, Size};
use std::collections::HashMap;

use crate::libs::{
  object::{Object, Object2D}, 
  physics::Physics, 
  transform::{Transform, Trans, Rect}
};

#[derive(PartialEq)]
pub enum PlayerDirection {
  Left,
  Right,
}

pub struct Player<I: ImageSize> {
  sm: SpriteManager<I>,
  animation: HashMap<&'static str, Vec<usize>>,
  current_animation: &'static str,
  animation_loop: f64,
  transform: Transform,
  physics: Physics,
  is_ground: bool,
}

impl<I> Player<I>
where
  I: ImageSize,
{
  pub fn new(sm: SpriteManager<I>) -> Player<I> {
    Player {
      sm: sm,
      animation: HashMap::default(),
      current_animation: "idle",
      animation_loop: 0.0,
      transform: Transform::new(),
      physics: Physics::new(),
      is_ground: false,
    }
  }

  pub fn set_scale(&mut self, x: f64, y: f64) {
    self.transform.set_scale(x, y);
  }

  pub fn set_dir(&mut self, dir: PlayerDirection) {
    self.transform.set_flip_x(dir == PlayerDirection::Left);
  }

  pub fn walk(&mut self) {
    self.physics.speed = if self.transform.is_flip_x() { -0.5 } else { 0.5 };
    self.current_animation = "walk";
  }

  pub fn append_animation(&mut self, name: &'static str, mut animation: Vec<usize>) {
    if let Some(a) = self.animation.get_mut(&name) {
      a.append(&mut animation);
    } else {
      self.animation.insert(name, animation);
    }
  }

  pub fn push_animation(&mut self, name: &'static str, rect: usize) {
    if let Some(animation) = self.animation.get_mut(&name) {
      animation.push(rect);
    } else {
      self.animation.insert(name, vec![rect]);
    }
  }

  pub fn draw<B: Graphics<Texture = I>>(&mut self, t: Matrix2d, b: &mut B) {
    if let Some(animation) = self.animation.get(self.current_animation) {
      if let Some(idx) = animation.get(self.animation_loop as usize % animation.len()) {
        if let Some(sprite) = self.sm.get_mut("mario_small_red", *idx) {
          sprite.draw(t.trans(self.transform.get_position().x, self.transform.get_position().y), b)
        }
      }
    }
  }

  pub fn update(&mut self, dt: f64) {
    self.animation_loop += dt * 0.125;

    self.physics.accelerate(dt);

    self.physics.deccelerate();
    self.physics.limit_speed();

    self.physics.acc.y += self.physics.gravity * dt;
    self.physics.vel.y += self.physics.acc.y * dt;
    self.transform.translate_y(self.physics.vel.y * dt);

    if !self.is_ground {
      self.current_animation = "jump";
    }

    if self.physics.acc.x == 0.0 && self.is_ground {
      self.current_animation = "idle";
    }

    self.physics.acc *= 0.0;

    if let Some(animation) = self.animation.get(self.current_animation) {
      if let Some(idx) = animation.get(self.animation_loop as usize % animation.len()) {
        if let Some(sprite) = self.sm.get_mut("mario_small_red", *idx) {
          sprite.set_flip_x(self.transform.is_flip_x());
        }
      }
    }
  }

  pub fn set_inside_window(&mut self, size: Size) {
    if self.transform.x() < 0.0 {
      self.transform.set_position_x(0.0);
      self.physics.acc.x = 0.0;
    }

    if self.transform.xw() > size.width {
      self.transform.set_position_x(size.width - self.transform.w());
      self.physics.acc.x = 0.0;
    }
  }

  pub fn update_position_x(&mut self, dt: f64) {
    self.transform.translate_x(self.physics.vel.x * dt);
  }

  pub fn is_less_center(&self, window_size: Size) -> bool {
    self.transform.xw() < window_size.width / 2.0 - self.transform.w()
  }

  pub fn dir_right(&self) -> bool {
    !self.transform.is_flip_x()
  }

  pub fn get_vel_x(&self) -> f64 {
    self.physics.vel.x
  }

  pub fn jump(&mut self, dt: f64) {
    if self.is_ground {
      self.is_ground = false;
      self.physics.acc.y -= 25.0 * dt;
    }
  }

  pub fn stop(&mut self) {
    self.physics.speed = 0.0;
  }

  pub fn collide_with(&mut self, obj: &Object<I>) {
    if self.transform.x() < obj.get_transform().xw()
      && self.transform.xw() > obj.get_transform().x()
      && self.transform.y() < obj.get_transform().yh()
      && self.transform.yh() > obj.get_transform().y()
    {
      // Collide right side
      if self.transform.xw() > obj.get_transform().x()
        && self.transform.center_xw() < obj.get_transform().x()
        && self.transform.center_yh() > obj.get_transform().y()
        && self.transform.center_yh() < obj.get_transform().yh()
      {
        self.transform.set_position_x(obj.get_position().x - self.transform.w());
        self.physics.vel.x = 0.0;
      }

      // collide left side
      if self.transform.x() < obj.get_transform().xw()
        && self.transform.center_xw() > obj.get_transform().xw()
        && self.transform.center_yh() > obj.get_transform().y()
        && self.transform.center_yh() < obj.get_transform().yh()
      {
        self.transform.set_position_x(obj.get_transform().xw());
        self.physics.vel.x = 0.0;
      }

      // collide bottom side
      if self.transform.yh() > obj.get_transform().y()
        && self.transform.center_yh() < obj.get_transform().y()
        && self.transform.center_xw() > obj.get_transform().x()
        && self.transform.center_xw() < obj.get_transform().xw()
      {
        self.transform.set_position_y(obj.get_position().y - self.transform.h());
        self.physics.vel.y = 0.0;
        self.is_ground = true;
      }

      // collide top side
      if self.transform.y() < obj.get_transform().yh()
        && self.transform.center_yh() > obj.get_transform().yh()
        && self.transform.center_xw() > obj.get_transform().x()
        && self.transform.center_xw() < obj.get_transform().xw()
      {
        self.transform.set_position_y(obj.get_transform().yh());
        self.physics.vel.y = 0.0;
        self.is_ground = false;
      }
    }
  }
}
