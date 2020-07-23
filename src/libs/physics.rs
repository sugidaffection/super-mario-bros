use cgmath::Vector2;

pub struct Physics {
  pub acc: Vector2<f64>,
  pub vel: Vector2<f64>,
  pub max_vel: Vector2<f64>,
  pub speed: f64,
  pub friction: f64,
  pub gravity: f64
}

impl Physics {

  pub fn new() -> Self {
    Self {
      acc: Vector2 { x: 0.0, y: 0.0 },
      vel: Vector2 { x: 0.0, y: 0.0 },
      max_vel: Vector2 { x: 0.0, y: 0.0 },
      speed: 1.0,
      friction: 0.1,
      gravity: 0.7,
    }
  }

  pub fn accelerate(&mut self, dt: f64) {
    self.acc.x += self.speed * dt;
    self.vel += self.acc * dt;

    if self.vel.x > self.max_vel.x {
      self.vel = self.max_vel;
    }

    if self.vel.x < -self.max_vel.x {
      self.vel = -self.max_vel;
    }
  }

}