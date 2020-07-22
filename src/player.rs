use cgmath::Vector2;
use graphics::{Graphics, Transformed};
use graphics::math::Matrix2d;
use piston_window::{ Size, ImageSize };
use sprite::Sprite;
use std::collections::HashMap;

struct Controller {
  left: bool,
  right: bool,
  jump: bool
}

impl Controller {
  
  fn new() -> Self {
    Self {
      left: false,
      right: false,
      jump: false
    }
  }
}

pub struct Player<I: ImageSize> {
  sprite: Option<Sprite<I>>,
  animation: HashMap<&'static str, Vec<[f64; 4]>>,
  acc: Vector2<f64>,
  vel: Vector2<f64>,
  pos: Vector2<f64>,
  size: Size,
  scale: Vector2<f64>,
  gravity: Vector2<f64>,
  controller: Controller,
  speed: f64,
  max_speed: f64,
  flip: bool,
  window_size: Size,
  current_animation: &'static str,
  animation_loop: f64,
  can_jump: bool,
  pub can_move: bool
}

impl <I> Player<I>
where I: ImageSize {
  pub fn new(window_size: Size) -> Player<I> {
    Player {
      sprite: Option::None,
      animation: HashMap::default(),
      acc: Vector2 { x: 0.0, y: 0.0 },
      vel: Vector2 { x: 0.0, y: 0.0 },
      pos: Vector2 { x: 0.0, y: 0.0 },
      size: Size { width: 16.0, height: 16.0 },
      scale: Vector2 { x: 2.0, y: 2.0 },
      gravity: Vector2 { x: 0.0, y: 2.0 },
      controller: Controller::new(),
      speed: 4.0,
      max_speed: 20.0,
      flip: false,
      window_size: window_size,
      current_animation: "idle",
      animation_loop: 0.0,
      can_jump: false,
      can_move: true
    }
  }

  pub fn get_vel_x(&self) -> f64 {
    self.vel.x
  }

  pub fn set_sprite(&mut self, sprite: Sprite<I>){
    self.sprite = Option::Some(sprite);
  }

  pub fn append_animation(&mut self, name: &'static str, mut animation: Vec<[f64; 4]>) {
    if let Some(a) = self.animation.get_mut(&name) {
      a.append(&mut animation);
    }else {
      self.animation.insert(name, animation);
    }
  }

  pub fn push_animation(&mut self, name: &'static str, rect: [f64; 4]) {
    if let Some(animation) = self.animation.get_mut(&name) {
      animation.push(rect);
    }else {
      self.animation.insert(name, vec![rect]);
    }
  }

  pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
    match &self.sprite {
      Some(sprite) => sprite.draw(t.trans(self.pos.x, self.pos.y), b),
      None => {}
    }
  }

  pub fn update(&mut self, dt: f64) {

    self.acc *= 0.0;

    self.animation_loop += dt * 10.0 * 0.125;

    match &mut self.sprite {
      Some(sprite) => {
        
        if let Some(animation) = self.animation.get(self.current_animation) {
          if let Some(rect) = animation.get(self.animation_loop as usize % animation.len()) {
            sprite.set_src_rect(*rect);
            sprite.set_position(self.size.width, self.scale.y);
            sprite.set_scale(self.scale.x, self.scale.y);
            sprite.set_flip_x(self.flip);
          }
        }

      },
      None => {}
    }


    if self.controller.left {
      self.acc.x = -self.speed;
    }

    if self.controller.right {
      self.acc.x = self.speed;
    }

    if self.controller.left && !self.controller.right && self.can_jump ||
      !self.controller.left && self.controller.right && self.can_jump
     {
      self.current_animation = "walk";
    }

    if !self.controller.left && !self.controller.right && self.can_jump {
      self.current_animation = "idle";
    }

    

    self.vel += self.acc * dt * self.max_speed;
    self.vel += self.gravity * dt;
    self.vel.x *= 0.9;

    self.pos += self.vel * dt;
    if self.pos.x + (self.size.width * self.scale.x) > self.window_size.width / 2.0 {
      self.pos.x = self.window_size.width / 2.0 - (self.size.width * self.scale.x);
      self.can_move = false;
    }else {
      self.can_move = true;
    }
    


    if self.pos.y + (self.size.height * self.scale.y) > self.window_size.height - (self.size.height * self.scale.y) {
      self.pos.y = self.window_size.height - (self.size.height * self.scale.y * 2.0);
      self.acc.y = 0.0;
      self.vel.y = 0.0;
      self.can_jump = true;
    }

    if self.vel.x > self.max_speed {
      self.vel.x = self.max_speed;
    }

    if self.vel.x < -self.max_speed {
      self.vel.x = -self.max_speed;
    }

    if self.pos.x < 0.0 {
      self.pos.x = 0.0;
    } else if self.pos.x + self.size.width > self.window_size.width - self.size.width {
      self.pos.x = self.window_size.width - (2.0 * self.size.width);
    }

    println!("Acceleration x,y : {:.2}, {:.2}", self.acc.x, self.acc.y);
    println!("Velocity x,y: {:.2}, {:.2}", self.vel.x, self.vel.y);

  }

  pub fn walk_left(&mut self) {
    self.controller.left = true;
    self.flip = true;
  }

  pub fn walk_right(&mut self) {
    self.controller.right = true;
    self.flip = false;
  }

  pub fn jump(&mut self) {
    if self.can_jump {
      self.vel.y = -20.0;
      self.can_jump = false;
      self.current_animation = "jump";
    }
  }

  pub fn stop_left(&mut self) {
    self.controller.left = false;
  }

  pub fn stop_right(&mut self) {
    self.controller.right = false;
  }
}

