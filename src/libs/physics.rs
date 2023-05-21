use cgmath::Vector2;

use super::controller::Controller;

#[derive(Debug)]
pub struct Physics {
    pub velocity: Vector2<f64>,
    pub on_ground: bool,
    pub movement_speed: f64,
    pub max_movement_speed: f64,
    pub gravity: f64,
    pub max_fall_speed: f64,
    pub friction: f64,
    pub jump_power: f64,
    pub jump_timer: f64,
    pub jump_duration: f64,
    pub jump_threshold: f64,
    pub can_hold_jump: bool,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            velocity: Vector2::new(0.0, 0.0),
            on_ground: false,
            movement_speed: 0.1,
            max_movement_speed: 1.0,
            gravity: 9.8,
            max_fall_speed: 10.0,
            friction: 0.8,
            jump_power: 180.0,
            jump_timer: 0.0,
            jump_duration: 0.1,
            jump_threshold: 0.1,
            can_hold_jump: false,
        }
    }

    pub fn set_movement_speed(&mut self, speed: f64) {
        self.movement_speed = speed;
    }

    pub fn vel_x_is_almost_zero(&self, precision: f64) -> bool {
        self.velocity.x >= -precision && self.velocity.x <= precision
    }

    pub fn update(&mut self, dt: f64, input: &Controller) {
        self.velocity.y += self.gravity * dt;

        let mut movement_force = 0.0;
        if input.left {
            movement_force = -1.0;
        }
        if input.right {
            movement_force = 1.0;
        }
        self.velocity.x += movement_force * self.movement_speed;

        self.velocity.x = self
            .velocity
            .x
            .clamp(-self.max_movement_speed, self.max_movement_speed);

        if input.jump && self.on_ground {
            self.velocity.y -= self.jump_power * dt;
            self.on_ground = false;
            self.jump_timer = 0.0;
            self.can_hold_jump = true;
        } else if input.jump && self.jump_timer < self.jump_duration && self.can_hold_jump {
            self.velocity.y -= self.jump_power * self.jump_threshold * dt;
            self.jump_timer += dt;
        } else {
            self.can_hold_jump = false;
        }

        println!("{}", self.jump_timer);

        if self.on_ground {
            if self.velocity.x.abs() <= self.friction {
                self.velocity.x = 0.0;
            } else {
                self.velocity.x -= (1.0 - self.friction) * self.velocity.x.signum();
            }
        }

        self.velocity.y = self.velocity.y.min(self.max_fall_speed);
    }
}
