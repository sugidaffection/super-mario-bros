use cgmath::Vector2;
use piston_window::Size;

pub struct Transform {
  pub pos: Vector2<f64>,
  pub size: Size,
  pub rot: Vector2<f64>,
  pub flip_x: bool
}

impl Transform {

  pub fn new() -> Self {
    Self {
      pos: Vector2 { x: 0.0, y: 0.0 },
      size: Size { width: 0.0, height: 0.0 },
      rot: Vector2 { x: 0.0, y: 0.0 },
      flip_x: false
    }
  }

  pub fn get_right_side(&mut self) -> f64 {
    self.pos.x + self.size.width
  }

  pub fn get_bottom_side(&mut self) -> f64 {
    self.pos.y + self.size.width
  }

  pub fn scale(&mut self, x: f64, y: f64) {
    self.size.width *= x;
    self.size.height *= y;
  }

}