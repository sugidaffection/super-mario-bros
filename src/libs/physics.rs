use cgmath::Vector2;

pub static ACCELERATION: f64 = 0.8;
pub static DECELERATION: f64 = 0.6;
pub static MAX_SPEED: f64 = 6.0;
pub static JUMP_FORCE: f64 = 12.0;
pub static GRAVITY: f64 = 9.8;
pub static TERMINAL_VELOCITY: f64 = 8.0;
pub static JUMP_DURATION: f64 = 0.25;

#[derive(Debug)]
pub struct Physics {
    pub velocity: Vector2<f64>,
    pub speed: f64,
    pub jump_threshold: f64,
    pub jump_timer: f64,
    pub on_ground: bool,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            velocity: Vector2::new(0.0, 0.0),
            speed: 0.0,
            jump_timer: 0.0,
            jump_threshold: 0.5,
            on_ground: false,
        }
    }

    pub fn vel_x_is_almost_zero(&self, precision: f64) -> bool {
        self.velocity.x >= -precision && self.velocity.x <= precision
    }

    fn apply_gravity(&mut self, dt: f64) {
        self.velocity.y += GRAVITY * dt;
        self.velocity.y = self.velocity.y.max(-TERMINAL_VELOCITY);
    }

    fn apply_movement(&mut self, dt: f64) {
        if self.speed < 0.0 {
            self.velocity.x -= ACCELERATION * dt;
            self.velocity.x = self.velocity.x.max(-MAX_SPEED);
        } else if self.speed > 0.0 {
            self.velocity.x += ACCELERATION * dt;
            self.velocity.x = self.velocity.x.min(MAX_SPEED);
        } else {
            if self.velocity.x > 0.0 {
                self.velocity.x -= DECELERATION * dt;
                self.velocity.x = self.velocity.x.max(0.0);
            } else if self.velocity.x < 0.0 {
                self.velocity.x += DECELERATION * dt;
                self.velocity.x = self.velocity.x.min(0.0);
            }
        }
    }

    pub fn apply_jump(&mut self, dt: f64) {
        if self.on_ground {
            self.velocity.y = -JUMP_FORCE;
            self.jump_timer = JUMP_DURATION;
            self.on_ground = false;
        } else if self.jump_timer > 0.0 {
            self.velocity.y = self.velocity.y.max(JUMP_FORCE * self.jump_threshold);
            self.jump_timer -= dt;
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.apply_gravity(dt);
        self.apply_movement(dt);
    }
}
