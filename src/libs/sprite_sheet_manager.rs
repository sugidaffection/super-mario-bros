use std::{borrow::BorrowMut, collections::HashMap};

use super::{
    animations::{AnimationRepeat, SpriteSheetAnimation},
    core::{Drawable, Updatable},
    sprite_sheet::SpriteSheet,
};
use piston_window::{math::Matrix2d, G2d};

pub struct SpriteSheetManager {
    sprite_sheet: Option<SpriteSheet>,
    animations: HashMap<&'static str, SpriteSheetAnimation>,
    current_animation_name: Option<&'static str>,
}

impl SpriteSheetManager {
    pub fn new() -> Self {
        Self {
            sprite_sheet: None,
            animations: HashMap::new(),
            current_animation_name: None,
        }
    }

    pub fn set_spritesheet(&mut self, sprite_sheet: SpriteSheet) {
        self.sprite_sheet = Some(sprite_sheet);
    }

    pub fn set_flip_x(&mut self, value: bool) {
        if let Some(sprite_sheet) = &mut self.sprite_sheet {
            sprite_sheet.set_flip_x(value);
        }
    }

    pub fn add_animation(&mut self, name: &'static str, animations: Vec<[usize; 2]>) {
        let animation = SpriteSheetAnimation::new(animations, AnimationRepeat::FOREVER);
        self.animations.insert(name, animation);
    }

    pub fn get_current_animation(&self) -> Option<&SpriteSheetAnimation> {
        if let Some(name) = self.current_animation_name {
            return self.animations.get(name);
        }
        None
    }

    pub fn get_current_animation_mut(&mut self) -> Option<&mut SpriteSheetAnimation> {
        if let Some(name) = self.current_animation_name {
            return self.animations.get_mut(name);
        }
        None
    }

    pub fn play_animation(&mut self, name: &'static str) {
        if let Some(current_animation_name) = self.current_animation_name {
            if current_animation_name != name {
                self.stop_current_animation();
            }
        }

        if let Some(animation) = self.animations.get_mut(name) {
            self.current_animation_name = Some(name);
            animation.play();
        }
    }

    pub fn set_animation_interval(&mut self, speed: f64, name: &'static str) {
        if let Some(animation) = self.animations.get_mut(name) {
            animation.set_animation_speed(speed);
        }
    }

    pub fn stop_current_animation(&mut self) {
        if let Some(current_animation_name) = self.current_animation_name {
            if let Some(animation) = self.animations.get_mut(current_animation_name) {
                animation.stop();
            }
        }
    }
}

impl Drawable for SpriteSheetManager {
    fn draw(&mut self, t: Matrix2d, b: &mut G2d) {
        let sprite_sheet = &mut self.sprite_sheet;
        if let Some(name) = self.current_animation_name {
            if let Some(animation) = self.animations.get(name) {
                if let Some([row, col]) = animation.get_current_animation() {
                    if let Some(sprite_sheet) = sprite_sheet {
                        sprite_sheet.set_current_tiles(*row, *col);
                        sprite_sheet.draw(t, b);
                    }
                }
            }
        }
    }
}

impl Updatable for SpriteSheetManager {
    fn update(&mut self, dt: f64) {
        if let Some(current_animation) = self.get_current_animation_mut() {
            current_animation.update(dt);
        }
    }
}
