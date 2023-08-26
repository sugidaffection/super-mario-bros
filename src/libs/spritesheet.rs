use cgmath::Vector2;
use graphics::Graphics;
use graphics::{math::Matrix2d, rectangle, Transformed};
use piston_window::{ImageSize, Size};
use sprite::Sprite;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use super::transform::{Rect, Trans, Transform};
#[derive(Debug)]
pub struct SpriteSheetConfig {
    pub grid: [usize; 2],
    pub sprite_size: Size,
    pub spacing: Vector2<f64>,
    pub offset: Vector2<f64>,
    pub scale: Vector2<f64>,
}

pub struct SpriteSheet<I: ImageSize> {
    sprite: Sprite<I>,
    grid: [usize; 2],
    sprite_size: Size,
    spacing: Vector2<f64>,
    offset: Vector2<f64>,
    scale: Vector2<f64>,
}

impl<I: ImageSize> SpriteSheet<I> {
    pub fn new(texture: Rc<I>) -> Self {
        let size = texture.get_size();
        let mut sprite = Sprite::from_texture(texture);
        sprite.set_anchor(0.0, 0.0);

        let mut sprite = Self {
            sprite,
            grid: [1, 1],
            sprite_size: Size::from(size),
            spacing: Vector2::new(0.0, 0.0),
            offset: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
        };
        sprite.set_current_tiles(0, 0);
        sprite
    }

    pub fn set_config(&mut self, config: &SpriteSheetConfig) {
        self.grid = config.grid;
        self.sprite_size = config.sprite_size;
        self.spacing = config.spacing;
        self.offset = config.offset;
        self.scale = config.scale;
        self.set_current_tiles(0, 0);
        self.sprite.set_scale(self.scale.x, self.scale.y);
        let scale = self.sprite.get_scale();
        println!("Current config {:?} {:?}", config, scale);
    }

    pub fn clone_config(&mut self) -> SpriteSheetConfig {
        SpriteSheetConfig {
            grid: self.grid,
            sprite_size: self.sprite_size,
            spacing: self.spacing,
            offset: self.offset,
            scale: self.scale,
        }
    }

    pub fn clone_sprite(&mut self) -> Sprite<I> {
        let rect = self.sprite.get_src_rect().unwrap();
        let mut sprite = Sprite::from_texture(self.sprite.get_texture().clone());
        sprite.set_src_rect(rect);
        sprite
    }

    pub fn set_current_tiles(&mut self, mut row: usize, mut col: usize) {
        row %= self.grid[0];
        col %= self.grid[1];
        let sprite_width_with_spacing = self.sprite_size.width + self.spacing.x;
        let sprite_height_with_spacing = self.sprite_size.height + self.spacing.y;
        let src_rect = From::from([
            self.offset.x + sprite_width_with_spacing * col as f64,
            self.offset.y + sprite_height_with_spacing * row as f64,
            self.sprite_size.width,
            self.sprite_size.height,
        ]);

        self.sprite.set_src_rect(src_rect);
    }

    pub fn set_flip_x(&mut self, value: bool) {
        self.sprite.set_flip_x(value);
    }

    pub fn set_src_rect(&mut self, rect: [f64; 4]) {
        self.sprite.set_src_rect(rect);
    }

    pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
        self.sprite.draw(t, b);
    }
}
