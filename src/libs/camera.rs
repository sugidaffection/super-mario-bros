use cgmath::Vector2;
use piston_window::Size;

use super::{
    core::{Entity, Updatable},
    tilemap::TileMap,
    transform::Rect,
};

pub struct Camera {
    pub position: Vector2<f64>,
    pub window_size: Size,
    pub viewport_size: Size,
    pub map_size: Size,
    pub scale: f64,
    pub target_position: Vector2<f64>,
}

impl Camera {
    pub fn new(window_size: Size, viewport_size: Size, map_size: Size) -> Self {
        Camera {
            position: Vector2::new(0.0, 0.0),
            window_size,
            viewport_size,
            map_size,
            scale: window_size.height / viewport_size.height,
            target_position: Vector2::new(0.0, 0.0),
        }
    }

    pub fn follow_player(&mut self, player: &dyn Entity) {
        let target_x = player.get_transform().xw() - self.viewport_size.width / 2.0;
        let target_y = player.get_transform().yh() - self.viewport_size.height / 2.0;

        let mut position = self.position.clone();

        if target_x < 0.0 {
            position.x = 0.0;
        } else if target_x + self.viewport_size.width > self.map_size.width {
            position.x = self.map_size.width - self.viewport_size.width;
        } else {
            position.x = target_x;
        }

        if target_y < 0.0 {
            position.y = 0.0;
        } else if target_y + self.viewport_size.height > self.map_size.height {
            position.y = self.map_size.height - self.viewport_size.height;
        } else {
            position.y = target_y;
        }

        self.target_position.x = position.x;
        self.target_position.y = position.y;
    }

    pub fn update_tilemap(&self, tilemap: &mut TileMap) {
        tilemap.set_src_rect([
            self.position.x,
            self.position.y,
            self.viewport_size.width,
            self.viewport_size.height,
        ]);
    }

    pub fn lerp(from: f64, to: f64, time: f64) -> f64 {
        (1.0 - time) * from + time * to
    }
}

impl Updatable for Camera {
    fn update(&mut self, dt: f64) {
        self.position.x = Self::lerp(self.position.x, self.target_position.x, 2.0 * dt);
        self.position.y = Self::lerp(self.position.y, self.target_position.y, 2.0 * dt);
    }
}
