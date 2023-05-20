use cgmath::Vector2;
use graphics::ImageSize;

use super::player::Player;

pub struct Camera {
    pub position: Vector2<f64>,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub world_width: f64,
    pub world_height: f64,
}

impl Camera {
    pub fn new(
        viewport_width: f64,
        viewport_height: f64,
        world_width: f64,
        world_height: f64,
    ) -> Self {
        Camera {
            position: Vector2::new(0.0, 0.0),
            viewport_width,
            viewport_height,
            world_width,
            world_height,
        }
    }

    pub fn follow_player<I: ImageSize>(&mut self, player: &Player<I>) {
        // Calculate the camera's new position based on the player's position
        let target_x = player.get_position().x - self.viewport_width / 2.0;
        let target_y = player.get_position().y - self.viewport_height / 2.0;

        // Adjust the camera's position to stay within the game world boundaries
        if target_x < 0.0 {
            self.position.x = 0.0;
        } else if target_x + self.viewport_width > self.world_width {
            self.position.x = self.world_width - self.viewport_width;
        } else {
            self.position.x = target_x;
        }

        if target_y < 0.0 {
            self.position.y = 0.0;
        } else if target_y + self.viewport_height > self.world_height {
            self.position.y = self.world_height - self.viewport_height;
        } else {
            self.position.y = target_y;
        }
    }
}
