use cgmath::Vector2;
use graphics::math::Matrix2d;
use graphics::Graphics;
use piston_window::{G2d, G2dTexture, ImageSize, Size};
use sprite::Sprite;
use std::rc::Rc;

use super::core::Drawable;

#[derive(Debug)]
pub struct SpriteSheetConfig {
    pub grid: [usize; 2],
    pub sprite_size: Size,
    pub spacing: Vector2<f64>,
    pub offset: Vector2<f64>,
    pub scale: Vector2<f64>,
}

pub struct SpriteSheet {
    sprite: Sprite<G2dTexture>,
    config: SpriteSheetConfig,
}

impl SpriteSheet {
    pub fn new(texture: Rc<G2dTexture>, config: SpriteSheetConfig) -> Self {
        let mut sprite = Sprite::from_texture(texture);
        sprite.set_anchor(0.0, 0.0);
        sprite.set_scale(config.scale.x, config.scale.x);

        let mut sprite = Self { sprite, config };
        sprite.set_current_tiles(0, 0);
        sprite
    }

    pub fn get_sprite_src_rect_from(&self, mut row: usize, mut col: usize) -> [f64; 4] {
        row %= self.config.grid[0];
        col %= self.config.grid[1];
        let sprite_width_with_spacing = self.config.sprite_size.width + self.config.spacing.x;
        let sprite_height_with_spacing = self.config.sprite_size.height + self.config.spacing.y;
        From::from([
            self.config.offset.x + sprite_width_with_spacing * col as f64,
            self.config.offset.y + sprite_height_with_spacing * row as f64,
            self.config.sprite_size.width,
            self.config.sprite_size.height,
        ])
    }

    pub fn clone_sprite(&mut self) -> Sprite<G2dTexture> {
        let rect = self.sprite.get_src_rect().unwrap();
        let mut sprite = Sprite::from_texture(self.sprite.get_texture().clone());
        sprite.set_src_rect(rect);
        sprite
    }

    pub fn clone_sprite_from(&mut self, row: usize, col: usize) -> Sprite<G2dTexture> {
        let mut sprite = Sprite::from_texture(self.sprite.get_texture().clone());
        let src_rect = self.get_sprite_src_rect_from(row, col);
        sprite.set_src_rect(src_rect);
        sprite
    }

    pub fn set_current_tiles(&mut self, row: usize, col: usize) {
        let src_rect = self.get_sprite_src_rect_from(row, col);
        self.sprite.set_src_rect(src_rect);
    }

    pub fn set_flip_x(&mut self, value: bool) {
        self.sprite.set_flip_x(value);
    }
}

impl Drawable for SpriteSheet {
    fn draw(&mut self, t: Matrix2d, b: &mut G2d) {
        self.sprite.draw(t, b);
    }
}
