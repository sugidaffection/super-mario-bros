pub struct SpriteSheetAnimation {
    animations: Vec<[usize; 2]>,
    speed: f64,
    state: AnimationState,
    repeat: AnimationRepeat,
    current_index: usize,
    animation_lt: f64,
}

pub enum AnimationRepeat {
    FOREVER,
    ONCE,
}

#[derive(Debug, PartialEq)]
enum AnimationState {
    IDLE,
    RUNNING,
}

impl SpriteSheetAnimation {
    pub fn new(animations: Vec<[usize; 2]>, repeat: AnimationRepeat) -> Self {
        Self {
            animations: animations,
            speed: 0.2,
            state: AnimationState::IDLE,
            repeat,
            current_index: 0,
            animation_lt: 0.0,
        }
    }

    pub fn set_animation_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn play(&mut self) {
        if self.state == AnimationState::IDLE {
            self.current_index = 0;
            self.animation_lt = 0.0;
            self.state = AnimationState::RUNNING;
        }
    }

    pub fn stop(&mut self) {
        if self.state == AnimationState::RUNNING {
            self.state = AnimationState::IDLE;
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.is_playing() {
            let duration = self.animations.len() as f64;
            match self.repeat {
                AnimationRepeat::FOREVER => {
                    if self.animation_lt >= duration {
                        self.animation_lt = 0.0;
                    }
                }
                AnimationRepeat::ONCE => {
                    if self.current_index >= self.animations.len() {
                        self.stop();
                    }
                }
            }
            if self.animation_lt < duration {
                self.animation_lt += if self.speed > 0.0 { self.speed } else { dt }
            }
            self.current_index =
                ((self.animation_lt).ceil() % self.animations.len() as f64) as usize;
        }
    }

    pub fn is_playing(&mut self) -> bool {
        self.state == AnimationState::RUNNING
    }

    pub fn get_current_animation(&self) -> Option<&[usize; 2]> {
        self.animations.get(self.current_index)
    }
}
