use graphics::math::Matrix2d;
use graphics::{Graphics, Transformed};
use piston_window::{ButtonState, ImageSize, Key, Size};

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
    Grounded,
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

    pub fn set_dir(&mut self, dir: PlayerDirection) {
        self.transform.set_flip_x(dir == PlayerDirection::Left);
        self.direction = dir;
    }

    pub fn set_inside_window(&mut self, size: Size) {
        if self.transform.x() < 0.0 {
            self.transform.set_position_x(0.0);
        }
    }

    pub fn walk(&mut self) {
        self.state = PlayerState::Walk;
    }

    pub fn jump(&mut self) {
        self.state = PlayerState::Jump;
    }

    pub fn centered(&mut self, window_size: Size) -> bool {
        self.transform.center_xw() >= window_size.width / 2.0
    }

    pub fn collide_with(&mut self, obj: &Object<I>) {
        let collide: bool = Collision::aabb(self.transform, obj.get_transform());

        if collide {
            // Collide right side
            if self.transform.xw() > obj.get_transform().x()
                && self.transform.center_xw() < obj.get_transform().x()
                && self.transform.center_yh() > obj.get_transform().y()
                && self.transform.center_yh() < obj.get_transform().yh()
            {
                self.transform
                    .set_position_x(obj.get_position().x - self.transform.w());
                self.physics.velocity.x = 0.0;
            }

            // collide left side
            if self.transform.x() < obj.get_transform().xw()
                && self.transform.center_xw() > obj.get_transform().xw()
                && self.transform.center_yh() > obj.get_transform().y()
                && self.transform.center_yh() < obj.get_transform().yh()
            {
                self.transform.set_position_x(obj.get_transform().xw());
                self.physics.velocity.x = 0.0;
            }

            // collide bottom side
            if self.transform.yh() > obj.get_transform().y()
                && self.transform.center_yh() < obj.get_transform().y()
                && self.transform.center_xw() > obj.get_transform().x()
                && self.transform.center_xw() < obj.get_transform().xw()
            {
                self.transform
                    .set_position_y(obj.get_position().y - self.transform.h());
                self.physics.on_ground = true;
                if self.state != PlayerState::Walk {
                    self.state = PlayerState::Grounded;
                }
            }

            // collide top side
            if self.transform.y() < obj.get_transform().yh()
                && self.transform.center_yh() > obj.get_transform().yh()
                && self.transform.center_xw() > obj.get_transform().x()
                && self.transform.center_xw() < obj.get_transform().xw()
            {
                self.transform.set_position_y(obj.get_transform().yh());
                self.physics.on_ground = false;
            }
        }
    }

    pub fn update_animation(&mut self, dt: f64) {
        self.sprites.update(dt);

        match self.state {
            PlayerState::Grounded => self.sprites.play_animation("idle"),
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
        if self.input.left {
            self.physics.speed = -1.0;
            self.walk();
            self.set_dir(PlayerDirection::Left)
        }

        if self.input.right {
            self.physics.speed = 1.0;
            self.walk();
            self.set_dir(PlayerDirection::Right)
        }

        if self.physics.vel_x_is_almost_zero(0.01) && self.state == PlayerState::Walk {
            self.state = PlayerState::Grounded;
            self.physics.speed = 0.0;
        }

        self.physics.update(dt);

        if self.input.jump {
            self.jump();
            self.physics.apply_jump(dt);
        } else {
            self.physics.jump_timer = 0.0;
        }

        self.transform
            .translate(self.physics.velocity.x * dt, self.physics.velocity.y * dt);
    }
}
