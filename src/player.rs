use graphics::math::Matrix2d;
use graphics::{Graphics, Transformed};
use piston_window::{ImageSize, Size};

use crate::libs::{
  collider::Collision,
  object::{Object, Object2D},
  physics::{Physics, PhysicsEvent},
  sprites_manager::SpriteManager,
  spritesheet::{SpriteSheet, SpriteSheetConfig},
  transform::{Rect, Trans, Transform},
};

#[derive(PartialEq)]
pub enum PlayerDirection {
  Left,
  Right,
}

#[derive(PartialEq, Debug)]
pub enum PlayerState {
  Grounded,
  Walk,
  Run,
  Jump,
  Crouch,
  Fall,
  Skid,
}

pub struct Player<I: ImageSize> {
  sprites: SpriteManager<I>,
  body: Physics,
  state: PlayerState,
}

impl<I> Player<I>
where
  I: ImageSize,
{
  pub fn new(player_sprite_sheet: SpriteSheet<I>) -> Player<I> {
    let mut player = Player {
      sprites: SpriteManager::new(),
      body: Physics::new(),
      state: PlayerState::Jump,
    };
    player.set_sprite_sheet(player_sprite_sheet);

    player
  }

  pub fn set_sprite_sheet(&mut self, sprite_sheet: SpriteSheet<I>) {
    self.sprites.set_spritesheet(sprite_sheet);
  }

  pub fn add_animation(&mut self, name: &'static str, animations: Vec<[usize; 2]>) {
    self.sprites.add_animation(name, animations);
  }

  pub fn add_config(&mut self, name: &'static str, config: SpriteSheetConfig) {
    self.sprites.add_config(name, config);
  }

  pub fn set_current_config(&mut self, name: &'static str) {
    self.sprites.set_current_config(name);
  }

  pub fn set_scale(&mut self, x: f64, y: f64) {
    self.body.transform.set_scale(x, y);
  }

  pub fn set_dir(&mut self, dir: PlayerDirection) {
    self.body.transform.set_flip_x(dir == PlayerDirection::Left);
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

  pub fn get_transform(&self) -> Transform {
    self.body.transform
  }

  pub fn set_position(&mut self, x: f64, y: f64) {
    self.body.transform.set_position(x, y);
  }

  pub fn update_position_x(&mut self, dt: f64) {
    self.body.transform.translate_x(self.body.vel.x);
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

  pub fn walk(&mut self) {
    self.state = PlayerState::Walk;
  }

  pub fn jump(&mut self) {
    self.state = PlayerState::Jump;
  }

  pub fn stop(&mut self) {
    self.body.stop();
  }

  pub fn collide_with(&mut self, obj: &Object<I>) {
    let collide: bool = Collision::aabb(self.body.transform, obj.get_transform());

    if collide {
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
        self.body.is_grounded = true;
        if self.state != PlayerState::Walk {
          self.state = PlayerState::Grounded;
        }
      }

      // collide top side
      if self.body.transform.y() < obj.get_transform().yh()
        && self.body.transform.center_yh() > obj.get_transform().yh()
        && self.body.transform.center_xw() > obj.get_transform().x()
        && self.body.transform.center_xw() < obj.get_transform().xw()
      {
        self.body.transform.set_position_y(obj.get_transform().yh());
        self.body.vel.y = 0.0;
        self.body.is_grounded = false;
      }
    }
  }

  pub fn update_animation(&mut self, dt: f64) {
    self.sprites.update(dt);

    match self.state {
      PlayerState::Grounded => self.sprites.play_animation("idle"),
      PlayerState::Walk => {
        if self.body.is_grounded {
          self.sprites.play_animation("walk")
        }
      }
      PlayerState::Run => self.sprites.play_animation("run"),
      PlayerState::Jump => self.sprites.play_animation("jump"),
      _ => {}
    }

    if let Some(sprite) = self.sprites.get_sprite() {
      sprite.set_flip_x(self.body.transform.is_flip_x());
    }
  }
}

impl<I> Object2D<I> for Player<I>
where
  I: ImageSize,
{
  fn draw<B: Graphics<Texture = I>>(&mut self, t: Matrix2d, b: &mut B) {
    let transformed = t.trans(
      self.body.transform.get_position().x,
      self.body.transform.get_position().y,
    );
    self.sprites.draw(transformed, b);
  }

  fn update(&mut self, dt: f64) {
    if self.state == PlayerState::Walk {
      self.body.walk();
    }

    if self.state == PlayerState::Jump {
      self.body.jump();
    }

    if self.body.vel_x_is_almost_zero(0.01) && self.state == PlayerState::Walk {
      self.state = PlayerState::Grounded;
    }

    self.body.update(dt);
    if self.state != PlayerState::Jump {
      self.body.decelerate(dt);
    }
  }
}
