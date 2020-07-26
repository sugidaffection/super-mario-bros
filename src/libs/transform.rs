use cgmath::Vector2;
use piston_window::Size;

pub struct Transform {
  pub pos: Vector2<f64>,
  pub size: Size,
  pub scale: Vector2<f64>,
  pub rot: Vector2<f64>,
  pub flip_x: bool,
}

impl Transform {
  pub fn new() -> Self {
    Self {
      pos: Vector2::from([0.0,0.0]),
      size: Size {
        width: 16.0,
        height: 16.0,
      },
      scale: Vector2::from([1.0,1.0]),
      rot: Vector2::from([0.0,0.0]),
      flip_x: false,
    }
  }

  pub fn x(&self) -> f64 {
    self.pos.x
  }

  pub fn y(&self) -> f64 {
    self.pos.y
  }

  pub fn w(&self) -> f64 {
    self.size.width * self.scale.x
  }

  pub fn h(&self) -> f64 {
    self.size.height * self.scale.y
  }

  pub fn right(&self) -> f64 {
    self.pos.x + self.w()
  }

  pub fn center_right(&self) -> f64 {
    self.pos.x + self.w() / 2.0
  }

  pub fn bottom(&self) -> f64 {
    self.pos.y + self.h()
  }

  pub fn center_bottom(&self) -> f64 {
    self.pos.y + self.h() / 2.0
  }

  pub fn rect(&self) -> [f64;4] {
    [self.x(), self.y(), self.w(), self.h()]
  }

}
