use std::borrow::BorrowMut;

use graphics::math::Matrix2d;
use graphics::{Graphics, Transformed};
use piston_window::{ButtonState, G2d, G2dTexture, Key};

use crate::Sound;

use super::collider::Side;
use super::core::{Drawable, Entity, Object2D, Updatable};
use super::{
    collider::Collision,
    controller::Controller,
    physics::Physics,
    sprites_manager::SpriteManager,
    spritesheet::{SpriteSheet, SpriteSheetConfig},
    transform::{Rect, Trans, Transform},
};

#[derive(PartialEq)]
pub enum PlayerDirection {
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
pub enum PlayerState {
    Idle,
    Walk,
    Run,
    Jump,
    Crouch,
    Fall,
    Skid,
    Push,
}

pub struct Player {
    sprites: SpriteManager,
    physics: Physics,
    state: PlayerState,
    transform: Transform,
    direction: PlayerDirection,
    input: Controller,
}

impl Player {
    pub fn new() -> Player {
        let mut transform = Transform::new();
        transform.set_position(20.0, 20.0);
        Player {
            sprites: SpriteManager::new(),
            physics: Physics::new(),
            state: PlayerState::Jump,
            transform,
            direction: PlayerDirection::Right,
            input: Controller::new(),
        }
    }

    pub fn set_sprite_sheet(&mut self, sprite_sheet: SpriteSheet, config: SpriteSheetConfig) {
        self.sprites.set_spritesheet(sprite_sheet);
        self.sprites.add_config("default", config);
        self.sprites.set_current_config("default");
    }

    pub fn add_animation(&mut self, name: &'static str, animations: Vec<[usize; 2]>) {
        self.sprites.add_animation(name, animations);
    }

    pub fn collide_with(&mut self, transform: &Transform) -> Option<Side> {
        let side = Collision::aabb(&self.transform, &transform);

        match side {
            Some(Side::RIGHT) => {
                let overlap = transform.x() - self.transform.xw();
                self.transform.translate(overlap, 0.0);
                self.physics.velocity.x = 0.0;
                if self.state != PlayerState::Jump
                    && self.input.right
                    && self.physics.on_ground
                    && self.direction == PlayerDirection::Right
                {
                    self.state = PlayerState::Push;
                } else {
                    self.state = PlayerState::Idle;
                }
            }
            Some(Side::LEFT) => {
                let overlap = self.transform.x() - transform.xw();
                self.transform.translate(-overlap, 0.0);
                self.physics.velocity.x = 0.0;
                self.state = PlayerState::Push;
                if self.state != PlayerState::Jump
                    && self.input.left
                    && self.physics.on_ground
                    && self.direction == PlayerDirection::Left
                {
                    self.state = PlayerState::Push;
                } else {
                    self.state = PlayerState::Idle;
                }
            }
            Some(Side::TOP) => {
                // Resolve collision and prevent player from going beyond the top side of the screen
                let overlap = self.transform.y() - transform.yh();
                self.transform.translate(0.0, -overlap);
                self.physics.velocity.y = 0.0;
                self.physics.can_jump = false;
            }
            Some(Side::BOTTOM) => {
                // Resolve collision and prevent player from going beyond the bottom side of the screen
                let overlap = self.transform.yh() - transform.y();
                self.transform.translate(0.0, -overlap);
                self.physics.velocity.y = 0.0;
                self.physics.on_ground = true;
            }
            None => {}
        }

        return side;
    }

    pub fn update_animation(&mut self, dt: f64) {
        self.transform
            .set_flip_x(self.direction == PlayerDirection::Left);
        match self.state {
            PlayerState::Idle => self.sprites.play_animation("idle"),
            PlayerState::Skid => self.sprites.play_animation("skid"),
            PlayerState::Walk => {
                if self.physics.on_ground {
                    self.sprites.play_animation("walk")
                }
            }
            PlayerState::Run => {
                self.sprites.play_animation("run");
            }
            PlayerState::Jump => {
                if self.input.right || self.input.left {
                    self.sprites.play_animation("jump-right")
                } else {
                    self.sprites.play_animation("jump")
                }
            }
            PlayerState::Fall => {
                self.sprites.play_animation("fall");
            }
            PlayerState::Push => {
                self.sprites.play_animation("push");
            }
            _ => {}
        }

        self.sprites.set_flip_x(self.transform.is_flip_x());

        self.sprites.update(dt);
    }

    pub fn update_input(&mut self, key: Key, state: ButtonState) {
        self.input.keyboard_event(key, state);
    }

    fn update_state(&mut self) {
        if self.physics.velocity.y > 0.0 && !self.physics.on_ground {
            self.state = PlayerState::Fall;
        }
        if self.physics.on_ground
            && (![
                PlayerState::Walk,
                PlayerState::Run,
                PlayerState::Skid,
                PlayerState::Push,
            ]
            .contains(&self.state)
                || ([PlayerState::Walk, PlayerState::Run, PlayerState::Skid].contains(&self.state)
                    && self.physics.vel_x_is_almost_zero(0.1)))
        {
            self.state = PlayerState::Idle;
        }
    }

    fn move_left(&mut self) {
        self.direction = PlayerDirection::Left;
        self.physics.set_force(-1.0, 0.0);
        if [
            PlayerState::Walk,
            PlayerState::Run,
            PlayerState::Idle,
            PlayerState::Skid,
        ]
        .contains(&self.state)
        {
            self.state = PlayerState::Walk;
            if self.physics.velocity.x > 0.0 {
                self.state = PlayerState::Skid;
            }
        }
    }

    fn move_right(&mut self) {
        self.direction = PlayerDirection::Right;
        self.physics.set_force(1.0, 0.0);
        if [
            PlayerState::Walk,
            PlayerState::Run,
            PlayerState::Idle,
            PlayerState::Skid,
        ]
        .contains(&self.state)
        {
            self.state = PlayerState::Walk;

            if self.physics.velocity.x < 0.0 {
                self.state = PlayerState::Skid;
            }
        }
    }

    fn stop(&mut self) {
        self.physics.set_force(0.0, 0.0);
    }

    fn jump(&mut self) {
        self.state = PlayerState::Jump;
        self.physics.jump();
        if self.physics.on_ground && self.physics.can_jump {
            music::play_sound(&Sound::Jump, music::Repeat::Times(0), 0.2);
        }
    }

    pub fn respawn_player_if_overflow(&mut self, max_y: f64) {
        let position = self.transform.get_position();
        if position.y > max_y {
            self.transform.set_position_y(20.0);
            self.physics.velocity.y = 0.0;
            self.physics.on_ground = false;
        }
    }

    pub fn reset_input(&mut self) {
        self.input.reset();
    }
}

impl Drawable for Player {
    fn draw(&mut self, t: Matrix2d, b: &mut G2d) {
        let transformed = t.trans(
            self.transform.get_position().x,
            self.transform.get_position().y,
        );

        self.sprites.draw(transformed, b);
    }
}

impl Updatable for Player {
    fn update(&mut self, dt: f64) {
        if self.input.left {
            self.move_left();
        }

        if self.input.right {
            self.move_right();
        }

        if self.input.run
            && ![PlayerState::Push, PlayerState::Skid].contains(&self.state)
            && self.physics.on_ground
        {
            self.physics.run();
            self.state = PlayerState::Run;
            self.sprites.set_animation_interval(0.1, "run");
        } else {
            self.physics.walk();
        }

        if !self.input.right && !self.input.left {
            self.stop();
        }

        if self.input.jump {
            self.jump();
        } else {
            self.physics.can_jump = false;
        }
        self.physics.update(dt);
        self.update_state();
        self.update_animation(dt);
        self.transform
            .translate(self.physics.velocity.x * dt, self.physics.velocity.y * dt);
    }
}

impl Object2D for Player {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_transform_mut(&mut self) -> &mut Transform {
        self.transform.borrow_mut()
    }
}

impl Entity for Player {}
