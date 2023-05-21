use super::{
    animations::SpriteAnimation,
    spritesheet::{SpriteSheet, SpriteSheetConfig},
};
use piston_window::math::Matrix2d;
use piston_window::{
    Filter, Flip, G2dTexture, G2dTextureContext, Graphics, ImageSize, Texture, TextureSettings,
};
use sprite::Sprite;
use std::path::PathBuf;
use std::rc::Rc;

pub trait SpriteManagerFn<I: ImageSize> {
    fn set_sprite_sheet(&mut self, sprite_sheet: SpriteSheet<I>);
}

pub struct SpriteConfig {
    pub name: &'static str,
    config: SpriteSheetConfig,
}

pub struct SpriteManager<I: ImageSize> {
    sprite_sheet: Option<SpriteSheet<I>>,
    sprite_configs: Vec<SpriteConfig>,
    current_config_name: Option<&'static str>,
    animations: Vec<SpriteAnimation>,
    current_animation_name: Option<&'static str>,
}

impl<I> SpriteManager<I>
where
    I: ImageSize,
{
    pub fn load_texture(mut context: &mut G2dTextureContext, p: &PathBuf) -> Rc<G2dTexture> {
        let mut texture_settings = TextureSettings::new();
        texture_settings.set_mag(Filter::Nearest);
        let texture = Texture::from_path(&mut context, p, Flip::None, &texture_settings).unwrap();
        Rc::new(texture)
    }

    pub fn new() -> Self {
        Self {
            sprite_sheet: None,
            animations: Vec::default(),
            sprite_configs: Vec::default(),
            current_config_name: None,
            current_animation_name: None,
        }
    }

    pub fn set_spritesheet(&mut self, sprite_sheet: SpriteSheet<I>) {
        self.sprite_sheet = Some(sprite_sheet);
    }

    pub fn get_spritesheet(&mut self) -> Option<&mut SpriteSheet<I>> {
        self.sprite_sheet.as_mut()
    }

    pub fn get_sprite(&mut self) -> Option<&mut Sprite<I>> {
        if let Some(sprite_sheet) = &mut self.sprite_sheet {
            return sprite_sheet.get_sprite();
        }
        None
    }

    pub fn add_animation(&mut self, name: &'static str, animations: Vec<[usize; 2]>) {
        let an = SpriteAnimation::new(name, animations);
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

    pub fn stop_animation(&mut self) {
        self.animations.iter_mut().for_each(|x| {
            x.stop();
        })
    }

    pub fn draw<B: Graphics<Texture = I>>(&mut self, t: Matrix2d, b: &mut B) {
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

    pub fn update(&mut self, dt: f64) {
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
