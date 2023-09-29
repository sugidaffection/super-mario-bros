use cgmath::Vector2;
use piston_window::Size;

use crate::libs::prelude::{Entity, Rect, Updatable};

use super::stages::StageMap;

pub struct Camera {
    pub position: Vector2<f64>,
    pub window_size: Size,
    pub viewport_size: Size,
    pub scale: f64,
    pub target_position: Vector2<f64>,
}

impl Camera {
    pub fn new(window_size: Size, viewport_size: Size) -> Self {
        Camera {
            position: Vector2::new(0.0, 0.0),
            window_size,
            viewport_size,
            scale: window_size.height / viewport_size.height,
            target_position: Vector2::new(0.0, 0.0),
        }
    }

    pub fn follow_player(&mut self, player: &dyn Entity) {
        self.target_position.x = player.get_transform().xw() - self.viewport_size.width / 2.0;
        self.target_position.y = player.get_transform().yh() - self.viewport_size.height / 2.0;
    }

    pub fn update_camera_view(&mut self, map: &mut StageMap) {
        let target_x = self.target_position.x;
        let target_y = self.target_position.y;
        let map_size = map.get_size();

        if target_x < 0.0 {
            self.target_position.x = 0.0;
        } else if target_x + self.viewport_size.width > map_size.width {
            self.target_position.x = map_size.width - self.viewport_size.width;
        }

        if target_y < 0.0 {
            self.target_position.y = 0.0;
        } else if target_y + self.viewport_size.height > map_size.height {
            self.target_position.y = map_size.height - self.viewport_size.height;
        }

        map.set_src_rect([
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
