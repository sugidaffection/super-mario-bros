use crate::libs::transform::{Trans, Transform};
use cgmath::Vector2;

pub trait PhysicsEvent {
    fn walk(&mut self);
    fn run(&mut self);
    fn jump(&mut self);
    fn stop(&mut self);
    fn update(&mut self, dt: f64);
}
#[derive(Debug)]
pub struct Physics {
    pub acc: f64,
    pub vel: Vector2<f64>,
    pub max_vel: Vector2<f64>,
    pub speed: f64,
    pub friction: f64,
    pub gravity: f64,
    pub max_jump: f64,
    pub jump_force: f64,
    pub transform: Transform,
    pub can_move: bool,
    pub can_jump: bool,
    pub is_grounded: bool,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            acc: 0.8,
            vel: Vector2::new(0.0, 0.0),
            max_vel: Vector2::new(200.0, 800.0),
            speed: 5.0,
            friction: 0.8,
            gravity: 1.8,
            max_jump: 15.0,
            jump_force: -20.0,
            transform: Transform::new(),
            can_move: false,
            can_jump: false,
            is_grounded: false,
        }
    }

    pub fn set_can_move(&mut self, can_move: bool) {
        self.can_move = can_move;
    }

    pub fn get_can_move(&self) -> bool {
        self.can_move
    }

    pub fn vel_x_is_almost_zero(&self, precision: f64) -> bool {
        self.vel.x >= -precision && self.vel.x <= precision
    }
}

impl PhysicsEvent for Physics {
    fn walk(&mut self) {
        self.speed = if self.transform.is_flip_x() {
            -5.0
        } else {
            5.0
        };
    }

    fn run(&mut self) {
        self.speed = if self.transform.is_flip_x() { -1. } else { 1. };
    }

    fn jump(&mut self) {
        if self.is_grounded {
            self.vel.y += self.jump_force;
            self.is_grounded = false;
        }
    }

    fn stop(&mut self) {
        self.speed = 0.0;
    }

    fn update(&mut self, dt: f64) {
        self.vel.y += self.gravity * dt;
        self.vel.x *= self.friction;

        self.vel.x += self.acc * self.speed * dt;
        self.vel.x = self.vel.x.clamp(-self.max_vel.x, self.max_vel.x);
        self.vel.y = self.vel.y.clamp(-self.max_vel.y, self.max_vel.y);
        self.transform.translate(self.vel.x * dt, self.vel.y * dt);
    }
}
