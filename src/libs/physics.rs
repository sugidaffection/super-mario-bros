use crate::libs::transform::{Trans, Transform};
use cgmath::Vector2;

pub trait PhysicsEvent {
  fn walk(&mut self);
  fn run(&mut self);
  fn jump(&mut self);
  fn stop(&mut self);
  fn fall(&mut self);
  fn update(&mut self, dt: f64);
  fn accelerate(&mut self, dt: f64);
  fn decelerate(&mut self, dt: f64);
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
  pub is_moving: bool,
}

impl Physics {
  pub fn new() -> Self {
    Self {
      acc: Vector2 { x: 0.0, y: 0.0 },
      vel: Vector2 { x: 0.0, y: 0.0 },
      max_vel: Vector2 { x: 5.0, y: 5.0 },
      speed: 0.0,
      move_speed: 100.0,
      friction: 0.6,
      gravity: 450.0,
      max_jump: 5.0,
      jump_force: 3.0,
      transform: Transform::new(),
      can_move: false,
      can_jump: false,
      is_grounded: false,
      is_moving: false,
    }
  }

  pub fn set_can_move(&mut self, can_move: bool) {
    self.can_move = can_move;
  }

  pub fn get_can_move(&self) -> bool {
    self.can_move
  }

  pub fn acc_x_is_almost_zero(&self, precision: f64) -> bool {
    self.acc.x >= -precision && self.acc.x <= precision
  }

  pub fn vel_x_is_almost_zero(&self, precision: f64) -> bool {
    self.vel.x >= -precision && self.vel.x <= precision
  }
}

impl PhysicsEvent for Physics {
  fn walk(&mut self) {
    self.speed = if self.transform.is_flip_x() {
      -self.move_speed
    } else {
      self.move_speed
    };

    self.is_moving = true;
  }

  fn run(&mut self) {
    self.speed = if self.transform.is_flip_x() {
      -self.move_speed
    } else {
      self.move_speed
    };
    self.is_moving = true;
  }

  fn jump(&mut self) {
    if self.is_grounded {
      self.acc.y -= self.jump_force;
      self.vel.y -= self.acc.y;
      if self.vel.y < -self.max_jump {
        self.vel.y = -self.max_jump;
      }
      self.is_grounded = false;
    }
  }

  fn fall(&mut self) {
    self.transform.translate_y(self.vel.y);
  }

  fn stop(&mut self) {
    self.speed = 0.0;
    self.is_moving = false;
  }

  fn accelerate(&mut self, dt: f64) {
    self.acc.x += self.speed * dt;
    self.acc.y += self.gravity * dt;
    self.vel += self.acc * dt;
  }

  fn decelerate(&mut self, dt: f64) {
    if !self.vel_x_is_almost_zero(0.01) {
      self.vel.x -= self.friction * self.vel.x * dt;
    } else {
      self.vel.x = 0.0;
    }
  }

  fn update(&mut self, dt: f64) {
    self.acc *= 0.0;

    self.accelerate(dt);

    if self.vel.y > self.max_vel.y {
      self.vel.y = self.max_vel.y;
    }

    self.fall();
  }
}
