use cgmath::Vector2;
use graphics::{rectangle, Transformed};

use super::{
    collider::{Collision, Side},
    core::{Drawable, Object2D, Updatable},
    physics::Physics,
    sprite_sheet::SpriteSheet,
    sprite_sheet_manager::SpriteSheetManager,
    transform::{Rect, Trans, Transform},
};

pub struct Enemy {
    name: &'static str,
    sprite_sheet_manager: SpriteSheetManager,
    physics: Physics,
    pub is_dead: bool,
}

impl Enemy {
    pub fn new(name: &'static str, position: Vector2<f64>) -> Self {
        let mut transform = Transform::new();
        transform.set_position(position.x, position.y);
        let mut physics = Physics::new(transform);
        physics.set_force(1.0, 0.0);
        physics.movement_speed = 50.0;
        physics.friction = 0.9;
        Self {
            name,
            sprite_sheet_manager: SpriteSheetManager::new(),
            physics,
            is_dead: false,
        }
    }

    pub fn set_sprite_sheet(&mut self, sprite_sheet: SpriteSheet) {
        self.sprite_sheet_manager.set_spritesheet(sprite_sheet);
    }

    pub fn add_animation(&mut self, name: &'static str, animations: Vec<[usize; 2]>) {
        self.sprite_sheet_manager.add_animation(name, animations);
    }

    pub fn play_animation(&mut self, name: &'static str) {
        self.sprite_sheet_manager.set_animation_interval(0.1, name);
        self.sprite_sheet_manager.play_animation(name);
    }

    pub fn dead(&mut self) {
        self.is_dead = true;
        self.sprite_sheet_manager.play_animation("dead");
        self.physics.set_force(0.0, 0.0);
    }
}

impl Drawable for Enemy {
    fn draw(&mut self, t: graphics::types::Matrix2d, b: &mut piston_window::G2d) {
        let position = self.physics.transform.get_position();
        self.sprite_sheet_manager.draw(t.trans_pos(position), b);
    }
}

impl Updatable for Enemy {
    fn update(&mut self, dt: f64) {
        self.physics.update(dt);
        self.physics
            .transform
            .translate(self.physics.velocity.x * dt, self.physics.velocity.y * dt);
        self.sprite_sheet_manager.update(dt);
    }
}

impl Object2D for Enemy {
    fn get_transform(&self) -> &Transform {
        &self.physics.transform
    }

    fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.physics.transform
    }
}

impl Collision for Enemy {
    fn collide_with(&mut self, transform: &Transform) -> Option<Side> {
        let side = self.physics.collide_with(transform);

        match side {
            Some(Side::RIGHT) => {
                self.physics.set_force(-1.0, 0.0);
                self.physics.velocity.x = 0.0;
            }
            Some(Side::LEFT) => {
                self.physics.set_force(1.0, 0.0);
                self.physics.velocity.x = 0.0;
            }
            Some(_) => {}
            None => {}
        }

        return side;
    }
}
