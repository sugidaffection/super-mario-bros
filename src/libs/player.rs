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
        player.set_animation();
        player.set_position(20.0, 20.0);
        player.transform.set_size(24.0, 32.0);
        player.physics.set_movement_speed(1.0);
        player
    }

    pub fn set_sprite_sheet(&mut self, sprite_sheet: SpriteSheet<I>) {
        self.sprites.set_spritesheet(sprite_sheet);
    }

    pub fn set_animation(&mut self) {
        self.add_animation("idle", vec![[1, 0]]);
        self.add_animation("jump", vec![[0, 1]]);
        self.add_animation(
            "walk",
            vec![
                [0, 0],
                [0, 1],
                [0, 2],
                [0, 3],
                [0, 4],
                [1, 0],
                [1, 1],
                [1, 2],
                [1, 3],
                [1, 4],
                [2, 0],
                [2, 1],
                [2, 2],
                [2, 3],
                [2, 4],
                [3, 0],
                [3, 1],
                [3, 2],
                [3, 3],
                [3, 4],
                [4, 0],
                [4, 1],
            ],
        );
    }

    pub fn add_animation(&mut self, name: &'static str, animations: Vec<[usize; 2]>) {
        self.sprites.add_animation(name, animations);
    }

    pub fn add_config(&mut self, name: &'static str, config: SpriteSheetConfig) {
        self.sprites.add_config(name, config);
    }

    pub fn set_current_config(&mut self, name: &'static str) {
        self.sprites.set_current_config(name);
        if let Some(spritesheet) = self.sprites.get_spritesheet() {
            let size = self.transform.get_size();
            let sprite_size = spritesheet.get_sprite_size();
            let scale_x = size.width / (sprite_size.width as f64);
            let scale_y = size.height / (sprite_size.height as f64);
            if let Some(sprite) = spritesheet.get_sprite() {
                sprite.set_scale(scale_x, scale_y);
            }
        }
    }

    pub fn set_inside_window(&mut self, size: Size) {
        if self.transform.x() < 0.0 {
            let overlap: f64 = self.transform.x() - 0.0;
            self.transform.translate_x(-overlap);
        }
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.transform.set_position(x, y);
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn collide_with(&mut self, obj: &Object<I>) {
        let (collide, side) = Collision::aabb(&self.transform, &obj.get_transform());

        if collide {
            match side {
                Some(Side::RIGHT) => {
                    // Resolve collision and prevent player from going beyond the right side of the screen
                    let overlap = obj.get_transform().x() - self.transform.xw();
                    self.transform.translate(overlap, 0.0);
                    self.physics.velocity.x = 0.0;
                }
                Some(Side::LEFT) => {
                    // Resolve collision and prevent player from going beyond the left side of the screen
                    let overlap = self.transform.x() - obj.get_transform().xw();
                    self.transform.translate(-overlap, 0.0);
                    self.physics.velocity.x = 0.0;
                }
                Some(Side::TOP) => {
                    // Resolve collision and prevent player from going beyond the top side of the screen
                    let overlap = self.transform.y() - obj.get_transform().yh();
                    self.transform.translate(0.0, -overlap);
                    self.physics.velocity.y = 0.0;
                }
                Some(Side::BOTTOM) => {
                    // Resolve collision and prevent player from going beyond the bottom side of the screen
                    let overlap = self.transform.yh() - obj.get_transform().y();
                    self.transform.translate(0.0, -overlap);
                    self.physics.velocity.y = 0.0;
                    self.physics.on_ground = true;
                }
                None => {}
            }
        }
    }

    pub fn update_animation(&mut self, dt: f64) {
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
        if state == ButtonState::Press {
            if key == Key::Left {
                self.direction = PlayerDirection::Left;
                self.state = PlayerState::Walk;
            }

            if key == Key::Right {
                self.direction = PlayerDirection::Right;
                self.state = PlayerState::Walk;
            }
        }

        self.input.keyboard_event(key, state)
    }

    fn update_state(&mut self) {
        if self.physics.on_ground && self.physics.vel_x_is_almost_zero(0.01) {
            self.state = PlayerState::Idle;
        } else if self.physics.on_ground {
            self.state = PlayerState::Walk;
        } else {
            self.state = PlayerState::Jump;
        }
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
        self.physics.update(dt, &self.input);
        self.update_state();
        self.update_animation(dt);
        self.transform
            .translate(self.physics.velocity.x, self.physics.velocity.y);
    }
}
