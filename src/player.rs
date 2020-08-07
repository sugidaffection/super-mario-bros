use graphics::math::Matrix2d;
use graphics::{Graphics, Transformed};
use piston_window::{ImageSize, Size};

use crate::libs::{
  object::{Object, Object2D},
  physics::{Physics, PhysicsEvent},
  sprites_manager::SpriteManager,
  transform::{Rect, Trans},
};

#[derive(PartialEq)]
pub enum PlayerDirection {
  Left,
  Right,
}

pub struct Player<I: ImageSize> {
  sprites: SpriteManager<I>,
  body: Physics,
  is_ground: bool,
}

impl<I> Player<I>
where
  I: ImageSize,
{
  pub fn set_sprites(&mut self, sprites: SpriteManager<I>) {
    self.sprites = sprites;
  }

  pub fn set_scale(&mut self, x: f64, y: f64) {
    self.body.transform.set_scale(x, y);
  }

  pub fn set_dir(&mut self, dir: PlayerDirection) {
    self.body.transform.set_flip_x(dir == PlayerDirection::Left);
  }

  pub fn walk(&mut self) {
    self.body.walk();
    self.sprites.set_animation_name("walk");
  }

  pub fn append_animation(&mut self, name: &'static str, animation: Vec<usize>) {
    self.sprites.append_animation(name, animation);
  }

  pub fn push_animation(&mut self, name: &'static str, rect: usize) {
    self.sprites.push_animation(name, rect);
  }

  pub fn set_inside_window(&mut self, size: Size) {
    if self.body.transform.x() < 0.0 {
      self.body.transform.set_position_x(0.0);
    }

    if self.body.transform.xw() > size.width {
      self
        .body
        .transform
        .set_position_x(size.width - self.body.transform.w());
    }
  }

  pub fn update_position_x(&mut self, dt: f64) {
    self.body.transform.translate_x(self.body.vel.x * dt);
  }

  pub fn limit_move_size(&mut self, size: Size) {
    self
      .body
      .set_can_move(self.body.transform.xw() < size.width / 2.0 - self.body.transform.w());
  }

  pub fn is_can_move(&self) -> bool {
    self.body.get_can_move()
  }

  pub fn dir_right(&self) -> bool {
    !self.body.transform.is_flip_x()
  }

  pub fn get_vel_x(&self) -> f64 {
    self.body.vel.x
  }

  pub fn jump(&mut self, _: f64) {
    if self.is_ground {
      self.is_ground = false;
      self.body.jump();
    }
  }

  pub fn stop(&mut self) {
    self.body.stop();
  }

  pub fn collide_with(&mut self, obj: &Object<I>) {
    if self.body.transform.x() < obj.get_transform().xw()
      && self.body.transform.xw() > obj.get_transform().x()
      && self.body.transform.y() < obj.get_transform().yh()
      && self.body.transform.yh() > obj.get_transform().y()
    {
      // Collide right side
      if self.body.transform.xw() > obj.get_transform().x()
        && self.body.transform.center_xw() < obj.get_transform().x()
        && self.body.transform.center_yh() > obj.get_transform().y()
        && self.body.transform.center_yh() < obj.get_transform().yh()
      {
        self
          .body
          .transform
          .set_position_x(obj.get_position().x - self.body.transform.w());
        self.body.vel.x = 0.0;
      }

      // collide left side
      if self.body.transform.x() < obj.get_transform().xw()
        && self.body.transform.center_xw() > obj.get_transform().xw()
        && self.body.transform.center_yh() > obj.get_transform().y()
        && self.body.transform.center_yh() < obj.get_transform().yh()
      {
        self.body.transform.set_position_x(obj.get_transform().xw());
        self.body.vel.x = 0.0;
      }

      // collide bottom side
      if self.body.transform.yh() > obj.get_transform().y()
        && self.body.transform.center_yh() < obj.get_transform().y()
        && self.body.transform.center_xw() > obj.get_transform().x()
        && self.body.transform.center_xw() < obj.get_transform().xw()
      {
        self
          .body
          .transform
          .set_position_y(obj.get_position().y - self.body.transform.h());
        self.body.vel.y = 0.0;
        self.is_ground = true;
      }

      // collide top side
      if self.body.transform.y() < obj.get_transform().yh()
        && self.body.transform.center_yh() > obj.get_transform().yh()
        && self.body.transform.center_xw() > obj.get_transform().x()
        && self.body.transform.center_xw() < obj.get_transform().xw()
      {
        self.body.transform.set_position_y(obj.get_transform().yh());
        self.body.vel.y = 0.0;
        self.is_ground = false;
      }
    }
  }
}

impl<I> Object2D<I> for Player<I>
where
  I: ImageSize,
{
  fn new() -> Player<I> {
    Player {
      sprites: SpriteManager::new(),
      body: Physics::new(),
      is_ground: false,
    }
  }

  fn draw<B: Graphics<Texture = I>>(&mut self, t: Matrix2d, b: &mut B) {
    if let Some(sprite) = self.sprites.get_sprite_animation() {
      sprite.draw(
        t.trans(
          self.body.transform.get_position().x,
          self.body.transform.get_position().y,
        ),
        b,
      )
    }
  }

  fn update(&mut self, dt: f64) {
    self.sprites.play(dt * 0.125);

    if !self.is_ground {
      self.sprites.set_animation_name("jump");
    }

    if self.body.acc_x_is_nearest_zero(0.01) && self.is_ground {
      self.sprites.set_animation_name("idle");
    }

    if let Some(sprite) = self.sprites.get_sprite_animation() {
      sprite.set_flip_x(self.body.transform.is_flip_x());
    }

    self.body.update(dt);
  }
}
