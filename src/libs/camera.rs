use cgmath::Vector2;
use piston_window::Size;

use super::{core::Entity, tilemap::TileMap, transform::Rect};

pub struct Camera {
    pub position: Vector2<f64>,
    pub window_size: Size,
    pub viewport_size: Size,
    pub map_size: Size,
    pub scale: f64,
}

impl Camera {
    pub fn new(window_size: Size, viewport_size: Size, map_size: Size) -> Self {
        Camera {
            position: Vector2::new(0.0, 0.0),
            window_size,
            viewport_size,
            map_size,
            scale: window_size.height / viewport_size.height,
        }
    }

    pub fn follow_player(&mut self, player: &dyn Entity) {
        // Calculate the camera's new position based on the player's position
        let target_x = player.get_transform().xw() - self.viewport_size.width / 2.0;
        let target_y = player.get_transform().yh() - self.viewport_size.height / 2.0;

        // Adjust the camera's position to stay within the game world boundaries
        if target_x < 0.0 {
            self.position.x = 0.0;
        } else if target_x + self.viewport_size.width > self.map_size.width {
            self.position.x = self.map_size.width - self.viewport_size.width;
        } else {
            self.position.x = target_x;
        }

        if target_y < 0.0 {
            self.position.y = 0.0;
        } else if target_y + self.viewport_size.height > self.map_size.height {
            self.position.y = self.map_size.height - self.viewport_size.height;
        } else {
            self.position.y = target_y;
        }
    }

    pub fn update_tilemap(&self, tilemap: &mut TileMap) {
        if let Some(map) = tilemap.get_current_map() {
            map.set_src_rect([
                self.position.x.max(0.0),
                0.0,
                self.viewport_size.width,
                self.viewport_size.height,
            ]);
        }
    }
}
