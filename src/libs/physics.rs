use cgmath::Vector2;

use super::controller::Controller;

#[derive(Debug)]
pub struct Physics {
    pub velocity: Vector2<f64>,
    pub on_ground: bool,
    pub jump_timer: f64,
    pub max_jump_timer: f64,
    pub movement_speed: f64,
    pub max_movement_speed: f64,
    pub jump_power: f64,
    pub gravity: f64,
    pub max_fall_speed: f64,
    pub friction: f64,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            velocity: Vector2::new(0.0, 0.0),
            on_ground: false,
            jump_timer: 0.0,
            max_jump_timer: 0.5, // Adjust as needed
            movement_speed: 5.0,
            max_movement_speed: 5.0,
            jump_power: 10.0,
            gravity: 1.2,
            max_fall_speed: 10.0,
            friction: 0.5,
        }
    }

    pub fn vel_x_is_almost_zero(&self, precision: f64) -> bool {
        self.velocity.x >= -precision && self.velocity.x <= precision
    }

    pub fn update(&mut self, dt: f64, input: &Controller) {
        self.velocity.y += self.gravity;
        self.velocity.y = self.velocity.y.min(self.max_fall_speed);

        let movement_force = if input.left {
            -1.0
        } else if input.right {
            1.0
        } else {
            0.0
        };

        self.velocity.x += movement_force * self.movement_speed;

        self.velocity.x = self
            .velocity
            .x
            .clamp(-self.max_movement_speed, self.max_movement_speed);

        if self.on_ground {
            if self.velocity.x.abs() <= self.friction {
                self.velocity.x = 0.0;
            } else {
                self.velocity.x -= self.friction * self.velocity.x.signum();
            }
        }

        if input.jump && self.on_ground {
            self.velocity.y = -self.jump_power;
            self.on_ground = false;
        }

        if self.on_ground {
            self.velocity.y = 0.0;
        }
    }
}
