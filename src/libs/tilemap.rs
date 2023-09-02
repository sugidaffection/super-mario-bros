use std::rc::Rc;

use graphics::ImageSize;
use piston_window::{G2dTexture, Size};
use sprite::Sprite;

use super::core::Drawable;

pub struct TileMap {
    map: Sprite<G2dTexture>,
    size: Size,
}

impl TileMap {
    pub fn new(texture: Rc<G2dTexture>) -> Self {
        let size = texture.get_size().into();
        let mut map = Sprite::from_texture(texture);
        map.set_anchor(0.0, 0.0);
        Self { map, size }
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn set_src_rect(&mut self, src_rect: [f64; 4]) {
        self.map.set_src_rect(src_rect);
    }
}

impl Drawable for TileMap {
    fn draw(&mut self, t: graphics::types::Matrix2d, b: &mut piston_window::G2d) {
        self.map.draw(t, b)
    }
}
