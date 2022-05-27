use crate::libs::transform::{Trans, Transform};
use cgmath::Vector2;

pub trait PhysicsEvent {
  fn walk(&mut self);
  fn run(&mut self);
  fn jump(&mut self);
  fn stop(&mut self);
  fn update(&mut self, dt: f64);
}

pub struct Physics {
  pub acc: Vector2<f64>,
  pub vel: Vector2<f64>,
  pub max_vel: Vector2<f64>,
  pub speed: f64,
  pub move_speed: f64,
  pub friction: f64,
  pub gravity: f64,
  pub max_jump: f64,
  pub jump_force: f64,
  pub transform: Transform,
  pub can_move: bool,
  pub can_jump: bool,
  pub is_grounded: bool,
}

impl Physics {
  pub fn new() -> Self {
    Self {
      acc: Vector2 { x: 0.0, y: 0.0 },
      vel: Vector2 { x: 0.0, y: 0.0 },
      max_vel: Vector2 { x: 10.0, y: 5.0 },
      speed: 0.0,
      move_speed: 4.0,
      friction: 2.4,
      gravity: 9.31,
      max_jump: 10.0,
      jump_force: -3.5,
      transform: Transform::new(),
      can_move: false,
      can_jump: false,
      is_grounded: false,
    }
  }

  pub fn set_can_move(&mut self, can_move: bool) {
    self.can_move = can_move;
  }

  pub fn get_can_move(&self) -> bool {
    self.can_move
  }

  pub fn accelerate(&mut self, dt: f64) {
    self.acc.x += self.speed;
    self.acc.y += self.gravity * dt;
    self.vel += self.acc;
  }

  pub fn acc_x_is_almost_zero(&self, precision: f64) -> bool {
    self.acc.x >= -precision && self.acc.x <= precision
  }

  pub fn vel_x_is_almost_zero(&self, precision: f64) -> bool {
    self.vel.x >= -precision && self.vel.x <= precision
  }

  pub fn deccelerate(&mut self, dt: f64) {
    self.vel.x += self.vel.x * -self.friction * dt;
  }
}

impl PhysicsEvent for Physics {
  fn walk(&mut self) {
    self.speed = if self.transform.is_flip_x() {
      -self.move_speed
    } else {
      self.move_speed
    };
  }

  fn run(&mut self) {
    self.speed = if self.transform.is_flip_x() {
      -self.move_speed
    } else {
      self.move_speed
    };
  }

  fn jump(&mut self) {
    if self.is_grounded {
      self.acc.y = 0.0;
      self.vel.y = -self.vel.y + self.jump_force;
      self.is_grounded = false;
    }
  }

  fn stop(&mut self) {
    self.speed = 0.0;
  }

  fn update(&mut self, dt: f64) {
    self.accelerate(dt);
    self.deccelerate(dt);

    if self.vel.y > self.max_vel.y {
      self.vel.y = self.max_vel.y;
    }
    self.transform.translate_y(self.vel.y);
    self.acc *= 0.0;
  }
}
