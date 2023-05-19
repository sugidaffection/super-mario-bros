pub struct SpriteAnimation {
    pub name: &'static str,
    animations: Vec<[usize; 2]>,
    animation_lt: f64,
    animation_interval: f64,
    animation_idx: usize,
    state: AnimationState,
}

#[derive(Debug)]
enum AnimationState {
    IDLE,
    RUNNING,
}

impl SpriteAnimation {
    pub fn new(name: &'static str, animations: Vec<[usize; 2]>) -> Self {
        Self {
            name: name,
            animations: animations,
            animation_lt: 0.0,
            animation_interval: 0.0,
            animation_idx: 0,
            state: AnimationState::IDLE,
        }
    }

    pub fn set_animation_interval(&mut self, t: f64) {
        self.animation_interval = t;
    }

    pub fn get_animation(&self) -> Option<&[usize; 2]> {
        self.animations.get(self.animation_idx)
    }

    pub fn play(&mut self) {
        self.state = AnimationState::RUNNING;
    }

    pub fn stop(&mut self) {
        self.state = AnimationState::IDLE;
        self.animation_idx = 0;
    }

    pub fn update(&mut self, dt: f64) {
        match self.state {
            AnimationState::RUNNING => {
                let t: f64 = if self.animation_interval > 0.0 {
                    self.animation_interval
                } else {
                    1.0 / self.animations.len() as f64
                };

                self.animation_lt += dt;

                if self.animation_lt >= t {
                    self.animation_idx = (self.animation_idx + 1) % self.animations.len();
                    self.animation_lt -= t;
                }
            }
            AnimationState::IDLE => {}
        }
    }
}
