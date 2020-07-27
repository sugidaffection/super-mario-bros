use cgmath::Vector2;
use graphics::math::Matrix2d;
use graphics::{rectangle, Graphics, Transformed};
use piston_window::{ImageSize, Size};
use sprite::Sprite;
use std::collections::HashMap;

use crate::libs::{
  object::Object, 
  physics::Physics, 
  transform::Transform
};

#[derive(PartialEq)]
pub enum PlayerDirection {
  Left,
  Right,
}

pub struct Player<I: ImageSize> {
  sprite: Option<Sprite<I>>,
  animation: HashMap<&'static str, Vec<[f64; 4]>>,
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
  pub fn new() -> Player<I> {
    Player {
      sprite: Option::None,
      animation: HashMap::default(),
      current_animation: "idle",
      animation_loop: 0.0,
      transform: Transform::new(),
      physics: Physics::new(),
      is_ground: false,
    }
  }


  pub fn set_scale(&mut self, x: f64, y: f64) {
    self.transform.scale.x = x;
    self.transform.scale.y = y;
  }

  pub fn set_dir(&mut self, dir: PlayerDirection) {
    self.transform.flip_x = dir == PlayerDirection::Left;
  }

  pub fn walk(&mut self) {
    self.physics.speed = if self.transform.flip_x { -0.5 } else { 0.5 };
    self.current_animation = "walk";
  }

  pub fn set_sprite(&mut self, sprite: Sprite<I>) {
    self.sprite = Option::Some(sprite);
  }

  pub fn append_animation(&mut self, name: &'static str, mut animation: Vec<[f64; 4]>) {
    if let Some(a) = self.animation.get_mut(&name) {
      a.append(&mut animation);
    } else {
      self.animation.insert(name, animation);
    }
  }

  pub fn push_animation(&mut self, name: &'static str, rect: [f64; 4]) {
    if let Some(animation) = self.animation.get_mut(&name) {
      animation.push(rect);
    } else {
      self.animation.insert(name, vec![rect]);
    }
  }

  pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
    rectangle([1.0, 1.0, 0.5, 1.0], self.transform.rect(), t, b);
    match &self.sprite {
      Some(sprite) => sprite.draw(t.trans(self.transform.pos.x, self.transform.pos.y), b),
      None => {}
    }
  }

  pub fn update(&mut self, dt: f64) {
    self.animation_loop += dt * 0.125;

    self.physics.accelerate(dt);

    self.physics.deccelerate();
    self.physics.limit_speed();

    self.physics.acc.y += self.physics.gravity * dt;
    self.physics.vel.y += self.physics.acc.y * dt;
    self.transform.pos.y += self.physics.vel.y * dt;

    // println!("vel {}", self.physics.vel.y);
    // println!("acc {}", self.physics.acc.y);

    if !self.is_ground {
      self.current_animation = "jump";
    }

    if self.physics.acc.x == 0.0 && self.is_ground {
      self.current_animation = "idle";
    }

    // println!("Acc {:?}", self.physics.acc);
    // println!("Vel {:?}", self.physics.vel);

    self.physics.acc *= 0.0;

    match &mut self.sprite {
      Some(sprite) => {
        if let Some(animation) = self.animation.get(self.current_animation) {
          if let Some(rect) = animation.get(self.animation_loop as usize % animation.len()) {
            sprite.set_src_rect(*rect);
            sprite.set_position(self.transform.size.width, self.transform.size.height);
            sprite.set_scale(self.transform.scale.x, self.transform.scale.y);
            sprite.set_flip_x(self.transform.flip_x);
          }
        }
      }
      None => {}
    }
  }
  pub fn set_inside_window(&mut self, size: Size) {
    if self.transform.pos.x < 0.0 {
      self.transform.pos.x = 0.0;
      self.physics.acc.x = 0.0;
    }

    if self.transform.right() > size.width {
      self.transform.pos.x = size.width - self.transform.w();
      self.physics.acc.x = 0.0;
    }
  }

  pub fn update_position_x(&mut self, dt: f64) {
    self.transform.pos.x += self.physics.vel.x * dt;
  }

  pub fn is_less_center(&self, window_size: Size) -> bool {
    self.transform.right() < window_size.width / 2.0 - self.transform.w()
  }

  pub fn dir_right(&self) -> bool {
    !self.transform.flip_x
  }

  pub fn get_vel_x(&self) -> f64 {
    self.physics.vel.x
  }

  pub fn get_position(&self) -> Vector2<f64> {
    self.transform.pos
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
    if self.transform.x() < obj.get_transform().right()
      && self.transform.right() > obj.get_transform().x()
      && self.transform.y() < obj.get_transform().bottom()
      && self.transform.bottom() > obj.get_transform().y()
    {
      // Collide right side
      if self.transform.right() > obj.get_transform().x()
        && self.transform.center_right() < obj.get_transform().x()
        && self.transform.center_bottom() > obj.get_transform().y()
        && self.transform.center_bottom() < obj.get_transform().bottom()
      {
        self.transform.pos.x = obj.get_position().x - self.transform.w();
        self.physics.vel.x = 0.0;
      }

      // collide left side
      if self.transform.x() < obj.get_transform().right()
        && self.transform.center_right() > obj.get_transform().right()
        && self.transform.center_bottom() > obj.get_transform().y()
        && self.transform.center_bottom() < obj.get_transform().bottom()
      {
        self.transform.pos.x = obj.get_transform().right();
        self.physics.vel.x = 0.0;
      }

      // collide bottom side
      if self.transform.bottom() > obj.get_transform().y()
        && self.transform.center_bottom() < obj.get_transform().y()
        && self.transform.center_right() > obj.get_transform().x()
        && self.transform.center_right() < obj.get_transform().right()
      {
        self.transform.pos.y = obj.get_position().y - self.transform.h();
        self.physics.vel.y = 0.0;
        self.is_ground = true;
      }

      // collide top side
      if self.transform.y() < obj.get_transform().bottom()
        && self.transform.center_bottom() > obj.get_transform().bottom()
        && self.transform.center_right() > obj.get_transform().x()
        && self.transform.center_right() < obj.get_transform().right()
      {
        self.transform.pos.y = obj.get_transform().bottom();
        self.physics.vel.y = 0.0;
        self.is_ground = false;
      }
    }
  }
}
