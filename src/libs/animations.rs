pub struct SpriteAnimation {
    pub name: &'static str,
    animations: Vec<[usize; 2]>,
    animation_lt: f64,
    animation_time: f64,
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
            animation_time: 0.0,
            animation_idx: 0,
            state: AnimationState::IDLE,
        }
    }

    pub fn set_animation_time(&mut self, t: f64) {
        self.animation_time = t;
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
                let t: f64 = if self.animation_time > 0.0 {
                    self.animation_time
                } else {
                    self.animations.len() as f64 / 60.0
                };

                if self.animation_lt >= t {
                    self.animation_idx += 1;
                    self.animation_idx %= self.animations.len();
                    self.animation_lt = 0.0;
                }
                self.animation_lt += dt;
            }
            AnimationState::IDLE => {}
        }
    }
}
