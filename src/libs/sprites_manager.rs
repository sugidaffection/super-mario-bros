use sprite::Sprite;
use std::collections::HashMap;
use std::rc::Rc;
use piston_window::ImageSize;

pub struct SpriteManager<I: ImageSize> {
  sprites: HashMap<String, Vec<Sprite<I>>>,
  current_sprite_name: &'static str,
  animations: HashMap<String, Vec<usize>>,
  current_animation_name: &'static str,
  current_animation_loop: f64,
  current_animation_index: usize
}

impl<I> SpriteManager<I>
where
  I: ImageSize,
{
  pub fn new() -> Self {
      Self {
          sprites: HashMap::default(),
          current_sprite_name: "default",
          animations: HashMap::default(),
          current_animation_name: "default",
          current_animation_loop: 0.0,
          current_animation_index: 0
      }
  }

  pub fn load(&mut self, name: &'static str, rc: &Rc<I>, rect: [f64; 4], scale: [f64; 2]) {
      let mut sprite: Sprite<I> = Sprite::from_texture_rect(rc.clone(), rect);
      sprite.set_scale(scale[0], scale[0]);
      sprite.set_position(rect[2] * scale[0] / 2.0, rect[3] * scale[1] / 2.0);

      if let Some(sprites) = self.sprites.get_mut(name) {
          sprites.push(sprite);
      } else {
          self.sprites.insert(name.to_owned(), vec![sprite]);
      }
  }

  pub fn loads(&mut self, name: &'static str, rc: &Rc<I>, rects: Vec<[f64; 4]>, scale: [f64; 2]) {
      for rect in rects {
          self.load(name, rc, rect, scale);
      }
  }

  pub fn get(&self, name: &'static str, index: usize) -> Option<&Sprite<I>> {
      self.sprites.get(name).map(|x| &x[index])
  }

  pub fn get_mut(&mut self, name: &'static str, index: usize) -> Option<&mut Sprite<I>> {
      self.sprites.get_mut(name).map(|x| &mut x[index])
  }

  pub fn get_first(&self, name: &'static str) -> Option<&Sprite<I>> {
      self.sprites.get(name).map(|x| x.first().unwrap())
  }

  pub fn append_animation(&mut self, name: &'static str, mut animations: Vec<usize>){
    if let Some(a) = self.animations.get_mut(name) {
        a.append(&mut animations);
    } else {
        self.animations.insert(name.to_owned(), animations);
    }
  }

  pub fn push_animation(&mut self, name: &'static str, rect: usize) {
    if let Some(animation) = self.animations.get_mut(name) {
      animation.push(rect);
    } else {
      self.animations.insert(name.to_owned(), vec![rect]);
    }
  }
  
  pub fn set_animation_name(&mut self, animation: &'static str) {
      self.current_animation_name = animation;
  }

  pub fn get_sprite_animation(&mut self) -> Option<&mut Sprite<I>> {
    if let Some(animation) = self.animations.get(self.current_animation_name) {
        if let Some(idx) = animation.get(self.current_animation_loop as usize % animation.len()) {
            return self.get_mut(self.current_sprite_name, *idx);
        }
    }
    
    None
  }

  pub fn play(&mut self, dt: f64) {
    self.current_animation_loop += dt;
  }
}