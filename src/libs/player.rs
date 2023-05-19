use graphics::math::Matrix2d;
use graphics::{Graphics, Transformed};
use piston_window::{ButtonState, ImageSize, Key, Size};

use super::collider::Side;
use super::{
    collider::Collision,
    controller::Controller,
    object::{Object, Object2D},
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
}

pub struct Player<I: ImageSize> {
    sprites: SpriteManager<I>,
    physics: Physics,
    state: PlayerState,
    transform: Transform,
    direction: PlayerDirection,
    input: Controller,
}

impl<I> Player<I>
where
    I: ImageSize,
{
    pub fn new(player_sprite_sheet: SpriteSheet<I>) -> Player<I> {
        let mut player = Player {
            sprites: SpriteManager::new(),
            physics: Physics::new(),
            state: PlayerState::Jump,
            transform: Transform::new(),
            direction: PlayerDirection::Right,
            input: Controller::new(),
        };
        player.set_sprite_sheet(player_sprite_sheet);

        player
    }

    pub fn set_sprite_sheet(&mut self, sprite_sheet: SpriteSheet<I>) {
        self.sprites.set_spritesheet(sprite_sheet);
    }

    pub fn add_animation(&mut self, name: &'static str, animations: Vec<[usize; 2]>) {
        self.sprites.add_animation(name, animations);
    }

    pub fn add_config(&mut self, name: &'static str, config: SpriteSheetConfig) {
        self.sprites.add_config(name, config);
    }

    pub fn set_current_config(&mut self, name: &'static str) {
        self.sprites.set_current_config(name);
    }

    pub fn set_inside_window(&mut self, size: Size) {
        if self.transform.x() < 0.0 {
            self.transform.set_position_x(0.0);
        }
    }

    pub fn collide_with(&mut self, obj: &Object<I>) {
        let (collide, side) = Collision::aabb(&self.transform, &obj.get_transform());

        if collide {
            match side {
                Some(Side::RIGHT) => {
                    self.transform
                        .set_position_x(obj.get_position().x - self.transform.w());
                    self.physics.velocity.x = 0.0;
                }
                Some(Side::LEFT) => {
                    self.transform.set_position_x(obj.get_transform().xw());
                    self.physics.velocity.x = 0.0;
                }
                Some(Side::TOP) => {
                    self.transform.set_position_y(obj.get_transform().yh());
                    self.physics.on_ground = false;
                }
                Some(Side::BOTTOM) => {
                    self.transform
                        .set_position_y(obj.get_position().y - self.transform.h());
                    self.physics.on_ground = true;
                    if self.state != PlayerState::Walk {
                        self.state = PlayerState::Idle;
                    }
                }
                None => {}
            }
        }
    }

    pub fn update_animation(&mut self, dt: f64) {
        if self.input.left {
            self.direction = PlayerDirection::Left;
            self.state = PlayerState::Walk;
        }

        if self.input.right {
            self.direction = PlayerDirection::Right;
            self.state = PlayerState::Walk;
        }

        self.transform
            .set_flip_x(self.direction == PlayerDirection::Left);

        match self.state {
            PlayerState::Idle => self.sprites.play_animation("idle"),
            PlayerState::Walk => {
                if self.physics.on_ground {
                    self.sprites.play_animation("walk")
                }
            }
            PlayerState::Run => self.sprites.play_animation("run"),
            PlayerState::Jump => self.sprites.play_animation("jump"),
            _ => {}
        }

        if let Some(sprite) = self.sprites.get_sprite() {
            sprite.set_flip_x(self.transform.is_flip_x());
        }

        self.sprites.update(dt);
    }

    pub fn update_input(&mut self, key: Key, state: ButtonState) {
        self.input.keyboard_event(key, state)
    }
}
impl<I> Object2D<I> for Player<I>
where
    I: ImageSize,
{
    fn draw<B: Graphics<Texture = I>>(&mut self, t: Matrix2d, b: &mut B) {
        let transformed = t.trans(
            self.transform.get_position().x,
            self.transform.get_position().y,
        );
        self.sprites.draw(transformed, b);
    }

    fn update(&mut self, dt: f64) {
        if self.physics.vel_x_is_almost_zero(0.01) && self.state == PlayerState::Walk {
            self.state = PlayerState::Idle;
        }
        self.physics.update(dt, &self.input);
        self.transform
            .translate(self.physics.velocity.x, self.physics.velocity.y);
    }
}
