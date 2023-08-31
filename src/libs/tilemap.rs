use piston_window::G2dTexture;
use sprite::Sprite;

use super::core::Drawable;

pub struct TileMap {
    maps: Vec<Sprite<G2dTexture>>,
    current_level: usize,
}

impl TileMap {
    pub fn new() -> Self {
        Self {
            maps: Vec::new(),
            current_level: 0,
        }
    }

    pub fn register_level(&mut self, level: usize, map: Sprite<G2dTexture>) {
        self.maps.insert(level - 1, map);
    }

    pub fn set_current_level(&mut self, level: usize) {
        self.current_level = level;
    }

    pub fn get_current_map(&mut self) -> Option<&mut Sprite<G2dTexture>> {
        self.maps.get_mut(self.current_level)
    }
}

impl Drawable for TileMap {
    fn draw(&mut self, t: graphics::types::Matrix2d, b: &mut piston_window::G2d) {
        if let Some(map) = self.maps.get(self.current_level) {
            map.draw(t, b)
        }
    }
}
