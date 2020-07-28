use piston_window::{ImageSize, Size, rectangle, Rectangle, DrawState};
use graphics::{Graphics, Transformed};
use graphics::math::{Matrix2d};
use sprite::{Scene, Sprite};
use cgmath::Vector2;
use crate::libs::transform::Transform;

pub struct Object<I: ImageSize> {
  color: [f32;4],
  scale: [f64;2],
  border: bool,
  transparent: bool,
  solid: bool,
  transform: Transform,
  scene: Option<Scene<I>>,
  sprite: Option<Sprite<I>>
}

impl<I> Object<I>
where I: ImageSize {

  pub fn new() -> Object<I> {
    Object {
      color: [1.0,1.0,1.0,1.0],
      scale: [1.0, 1.0],
      border: false,
      transparent: true,
      solid: true,
      transform: Transform::new(),
      scene: None,
      sprite: None
    }
  }

  pub fn set_scale(&mut self, x: f64, y: f64) {
    self.scale = [x,y];
  }

  pub fn set_transparent(&mut self, value: bool) {
    self.transparent = value;
    if value {
      self.color = [1.0,1.0,1.0,0.0];
    }else {
      self.color = [1.0,1.0,1.0,1.0];
    }
  }

  pub fn set_border(&mut self, value: bool) {
    self.border = value;
  }

  pub fn set_scene(&mut self, scene: Scene<I>) {
    self.scene = Some(scene);
  }

  pub fn set_sprite(&mut self, sprite: Sprite<I>) {
    self.sprite = Some(sprite);
  }

  pub fn run_animation(&self) {
    if let Some(scene) = &self.scene {
      scene.running();
    }
  }

  pub fn set_solid(&mut self, value: bool) {
    self.solid = value;
  }

  pub fn is_solid(&self) -> bool {
    self.solid
  }

  pub fn get_transform(&self) -> &Transform {
    &self.transform
  }

  pub fn get_position(&self) -> Vector2<f64> {
    self.transform.pos
  }

  pub fn set_position(&mut self, x: f64, y: f64) {
    self.transform.pos.x = x;
    self.transform.pos.y = y;
  }

  pub fn size(&self) -> Size {
    self.transform.size
  }

  pub fn set_size(&mut self, width: f64, height: f64) {
    self.transform.size.width = width;
    self.transform.size.height = height;
  }

  pub fn trans(&mut self, x: f64, y: f64) {
    self.transform.pos.x += x;
    self.transform.pos.y += y;
  }

  pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
    let bordered_rectangle = Rectangle::new_border(self.color, 1.0);
    if self.border {
      bordered_rectangle.draw(self.transform.rect(), &DrawState::default(), t, b);
    }else {
      rectangle(self.color, self.transform.rect(), t, b);
    }
    match &self.sprite {
      Some(sprite) => {
        sprite.draw(t
          .trans(self.transform.center_right(), self.transform.center_bottom())
          .scale(self.scale[0], self.scale[1]),
        b)
      },
      None => {}
    }

    match &self.scene {
      Some(scene) => scene.draw(t.trans(self.transform.pos.x, self.transform.pos.y), b),
      None => {}
    }

  }

}