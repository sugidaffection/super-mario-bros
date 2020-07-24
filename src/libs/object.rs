use piston_window::{ImageSize};
use graphics::{Graphics, Transformed};
use graphics::math::{Matrix2d};
use sprite::{Scene, Sprite};

use crate::libs::transform::Transform;

pub struct Object<I: ImageSize> {
  solid: bool,
  pub transform: Transform,
  scene: Option<Scene<I>>,
  sprite: Option<Sprite<I>>
}

impl<I> Object<I>
where I: ImageSize {

  pub fn new() -> Self {
    Self {
      solid: true,
      transform: Transform::new(),
      scene: None,
      sprite: None
    }
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

  pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
    match &self.sprite {
      Some(sprite) => sprite.draw(t.trans(self.transform.pos.x, self.transform.pos.y), b),
      None => {}
    }

    match &self.scene {
      Some(scene) => scene.draw(t.trans(self.transform.pos.x, self.transform.pos.y), b),
      None => {}
    }

  }

}