use cgmath::Vector2;
use graphics::math::Matrix2d;
use graphics::Graphics;
use piston_window::{ImageSize, Size};
use sprite::Sprite;
use std::rc::Rc;

pub struct SpriteSheetConfig {
    pub grid: [usize; 2],
    pub sprite_size: Size,
    pub spacing: Vector2<f64>,
    pub offset: Vector2<f64>,
    pub flip_x: bool,
    pub flip_y: bool,
}

pub struct SpriteSheet<I: ImageSize> {
    sprite: Sprite<I>,
    grid: [usize; 2],
    sprite_size: Size,
    spacing: Vector2<f64>,
    offset: Vector2<f64>,
    flip_x: bool,
    flip_y: bool,
}

impl<I: ImageSize> SpriteSheet<I> {
    pub fn new(texture: Rc<I>) -> Self {
        let size = texture.get_size();
        let mut sprite = Sprite::from_texture(texture);
        sprite.set_anchor(0.0, 0.0);

        let mut sprite = Self {
            sprite: sprite,
            grid: [1, 1],
            sprite_size: Size::from(size),
            spacing: Vector2::from([0.0, 0.0]),
            offset: Vector2::from([0.0, 0.0]),
            flip_x: false,
            flip_y: false,
        };
        sprite.set_current_tiles(0, 0);
        sprite
    }

    pub fn set_config(&mut self, config: &SpriteSheetConfig) {
        self.grid = config.grid;
        self.sprite_size = config.sprite_size;
        self.spacing = config.spacing;
        self.offset = config.offset;
        let scale_x = 16.0 / self.sprite_size.width;
        let scale_y = 16.0 / self.sprite_size.height;
        self.sprite.set_scale(scale_x, scale_y);
        self.sprite.set_flip_x(config.flip_x);
        self.sprite.set_flip_y(config.flip_y);

        self.set_current_tiles(0, 0);
    }

    pub fn clone_config(&mut self) -> SpriteSheetConfig {
        SpriteSheetConfig {
            grid: self.grid,
            sprite_size: self.sprite_size,
            spacing: self.spacing,
            offset: self.offset,
            flip_x: false,
            flip_y: false,
        }
    }

    pub fn get_sprite_size(&self) -> &Size {
        &self.sprite_size
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

    pub fn get_sprite(&mut self) -> Option<&mut Sprite<I>> {
        Some(&mut self.sprite)
    }

    pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
        self.sprite.draw(t, b);
    }
}
