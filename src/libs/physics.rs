use cgmath::Vector2;

use super::{
    collider::{Collider, Collision, Side},
    transform::{Rect, Trans, Transform},
};

#[derive(Debug)]
pub struct Physics {
    pub velocity: Vector2<f64>,
    max_velocity: Vector2<f64>,
    pub on_ground: bool,
    pub movement_speed: f64,
    pub gravity: f64,
    pub friction: f64,
    pub jump_power: f64,
    pub jump_duration: f64,
    pub jump_max_duration: f64,
    pub can_jump: bool,
    pub force: Vector2<f64>,
    pub mass: f64,
    pub deceleration: f64,
    pub skid_factor: f64,
    pub transform: Transform,
}

impl Physics {
    pub fn new(transform: Transform) -> Self {
        Self {
            velocity: Vector2::new(0.0, 0.0),
            max_velocity: Vector2::new(100.0, 400.0),
            on_ground: false,
            movement_speed: 500.0,
            gravity: 1.2,
            friction: 0.6,
            jump_power: 200.0,
            jump_duration: 0.0,
            jump_max_duration: 0.7,
            can_jump: false,
            force: Vector2::new(0.0, 0.0),
            mass: 450.0,
            deceleration: 120.0,
            skid_factor: 1.08,
            transform,
        }
    }

    pub fn vel_x_is_almost_zero(&self, precision: f64) -> bool {
        self.velocity.x >= -precision && self.velocity.x <= precision
    }

    pub fn apply_gravity(&mut self, dt: f64) {
        let fall_speed = self.gravity * self.mass;
        self.velocity.y += fall_speed * dt;
        self.velocity.y = self
            .velocity
            .y
            .clamp(-self.max_velocity.y, self.max_velocity.y);
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.jump_duration = 0.0;
            self.can_jump = true;
        }

        if self.velocity.y > 0.0 || self.jump_duration >= self.jump_max_duration {
            self.can_jump = false;
        }
    }

    pub fn run(&mut self) {
        self.max_velocity.x = 120.0;
    }

    pub fn walk(&mut self) {
        self.max_velocity.x = 100.0;
    }

    pub fn apply_jump(&mut self, dt: f64) {
        if self.can_jump && self.jump_duration < self.jump_max_duration {
            let jump_strength = if self.jump_duration == 0.0 {
                1.0
            } else {
                1.0 - self.jump_duration / self.jump_max_duration
            };
            self.velocity.y = (-self.jump_power * jump_strength).max(-500.0);
            self.jump_duration += dt;
            self.on_ground = false;
        }
    }

    pub fn apply_horizontal_movement(&mut self, dt: f64) {
        self.velocity.x += self.force.x * self.movement_speed * dt;

        if self.force.x == 0.0 {
            let deceleration = self.deceleration * dt;
            if self.velocity.x > 0.0 {
                self.velocity.x = (self.velocity.x - deceleration).max(0.0);
            } else if self.velocity.x < 0.0 {
                self.velocity.x = (self.velocity.x + deceleration).min(0.0);
            }
        } else if self.force.x > 0.0 {
            let friction = self.friction * self.velocity.x.abs() * dt;
            self.velocity.x -= friction * dt;
        } else if self.force.x < 0.0 {
            let friction = self.friction * self.velocity.x.abs() * dt;
            self.velocity.x += friction * dt;
        }

        self.velocity.x = self
            .velocity
            .x
            .clamp(-self.max_velocity.x, self.max_velocity.x);

        if self.force.x != 0.0 && self.force.x.signum() != self.velocity.x.signum() {
            self.velocity.x *= self.skid_factor;

            self.velocity.x += self.force.x * self.movement_speed * dt;
        }
    }

    pub fn set_force(&mut self, x: f64, y: f64) {
        self.force.x = x;
        self.force.y = y;
    }

    pub fn update(&mut self, dt: f64) {
        self.apply_gravity(dt);
        self.apply_horizontal_movement(dt);
        self.apply_jump(dt);
    }
}

impl Collision for Physics {
    fn collide_with(&mut self, transform: &Transform) -> Option<Side> {
        let side = Collider::aabb(&self.transform, &transform);

        match side {
            Some(Side::RIGHT) => {
                let overlap = transform.x() - self.transform.xw();
                self.transform.translate(overlap, 0.0);
                self.velocity.x = 0.0;
            }
            Some(Side::LEFT) => {
                let overlap = self.transform.x() - transform.xw();
                self.transform.translate(-overlap, 0.0);
                self.velocity.x = 0.0;
            }
            Some(Side::TOP) => {
                let overlap = self.transform.y() - transform.yh();
                self.transform.translate(0.0, -overlap);
                self.velocity.y = 0.0;
                self.can_jump = false;
            }
            Some(Side::BOTTOM) => {
                let overlap = self.transform.yh() - transform.y();
                self.transform.translate(0.0, -overlap);
                self.velocity.y = 0.0;
                self.on_ground = true;
            }
            None => {}
        }

        return side;
    }
}
