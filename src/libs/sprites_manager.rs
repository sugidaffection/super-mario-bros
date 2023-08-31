use super::{
    animations::SpriteSheetAnimation,
    core::{Drawable, Updatable},
    spritesheet::{SpriteSheet, SpriteSheetConfig},
};
use piston_window::{math::Matrix2d, G2d, G2dTexture};
use piston_window::{Graphics, ImageSize};

pub trait SpriteManagerFn<I: ImageSize> {
    fn set_sprite_sheet(&mut self, sprite_sheet: SpriteSheet);
}

pub struct SpriteConfig {
    pub name: &'static str,
    config: SpriteSheetConfig,
}

pub struct SpriteManager {
    sprite_sheet: Option<SpriteSheet>,
    sprite_configs: Vec<SpriteConfig>,
    current_config_name: Option<&'static str>,
    animations: Vec<SpriteSheetAnimation>,
    current_animation_name: Option<&'static str>,
}

impl SpriteManager {
    pub fn new() -> Self {
        Self {
            sprite_sheet: None,
            animations: Vec::default(),
            sprite_configs: Vec::default(),
            current_config_name: Some("default"),
            current_animation_name: None,
        }
    }

    pub fn set_spritesheet(&mut self, sprite_sheet: SpriteSheet) {
        self.sprite_sheet = Some(sprite_sheet);
    }

    pub fn set_flip_x(&mut self, value: bool) {
        self.sprite_sheet.as_mut().unwrap().set_flip_x(value);
    }

    pub fn add_animation(&mut self, name: &'static str, animations: Vec<[usize; 2]>) {
        let mut an = SpriteSheetAnimation::new(name, animations);
        an.set_animation_interval(0.2);
        self.animations.push(an);
    }

    pub fn add_config(&mut self, name: &'static str, options: SpriteSheetConfig) {
        let sprite_config = SpriteConfig {
            name: name,
            config: options,
        };
        self.sprite_configs.push(sprite_config);
    }

    pub fn set_current_config(&mut self, name: &'static str) {
        if let Some(sprite_sheet) = &mut self.sprite_sheet {
            if let Some(sprite_config) = self.sprite_configs.iter().find(|x| x.name == name) {
                self.current_config_name = Some(name);
                sprite_sheet.set_config(&sprite_config.config);
            } else {
                self.current_config_name = None
            }
        }
    }

    pub fn play_animation(&mut self, name: &'static str) {
        if let Some(animation) = self.animations.iter_mut().find(|x| x.name == name) {
            self.current_animation_name = Some(name);
            animation.play();
        }
    }

    pub fn set_animation_interval(&mut self, interval: f64, name: &'static str) {
        if let Some(animation) = self.animations.iter_mut().find(|x| x.name == name) {
            animation.set_animation_interval(interval);
        }
    }

    pub fn stop_animation(&mut self) {
        self.animations.iter_mut().for_each(|x| {
            x.stop();
        })
    }
}

impl Drawable for SpriteManager {
    fn draw(&mut self, t: Matrix2d, b: &mut G2d) {
        if let Some(sprite_sheet) = &mut self.sprite_sheet {
            sprite_sheet.draw(t, b);

            if let Some(animation_name) = self.current_animation_name {
                if let Some(animation) = self
                    .animations
                    .iter_mut()
                    .find(|x| x.name == animation_name)
                {
                    if let Some(animations) = animation.get_animation() {
                        sprite_sheet.set_current_tiles(animations[0], animations[1]);
                    }
                }
            }
        }
    }
}

impl Updatable for SpriteManager {
    fn update(&mut self, dt: f64) {
        if let Some(animation_name) = self.current_animation_name {
            if let Some(animation) = self
                .animations
                .iter_mut()
                .find(|x| x.name == animation_name)
            {
                animation.update(dt);
            }

            self.animations
                .iter_mut()
                .filter(|x| x.name != animation_name)
                .for_each(|x| {
                    x.stop();
                });
        }
    }
}
