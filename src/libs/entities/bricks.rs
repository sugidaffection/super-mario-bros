use std::{cell::RefCell, rc::Rc};

use graphics::{types::Matrix2d, Transformed};
use piston_window::{G2d, G2dTexture};
use sprite::Sprite;

use crate::{
    libs::{
        prelude::{Destroyable, Drawable, Object2D, Rect, Updatable},
        utils::transform::Transform,
    },
    Sound,
};

pub enum BrickType {
    Block,
    Coin,
    Mushroom,
    Ground,
}

pub struct Brick {
    brick_type: BrickType,
    sprite: Rc<RefCell<Sprite<G2dTexture>>>,
    transform: Transform,
    is_destroyed: bool,
}

impl Brick {
    pub fn new(brick_type: BrickType, sprite: Rc<RefCell<Sprite<G2dTexture>>>) -> Self {
        Self {
            brick_type,
            sprite,
            transform: Transform::new(),
            is_destroyed: false,
        }
    }
}

impl Drawable for Brick {
    fn draw(&mut self, t: Matrix2d, b: &mut G2d) {
        match self.brick_type {
            BrickType::Ground => {}
            _ => self.sprite.borrow().draw(
                t.trans(self.transform.center_xw(), self.transform.center_yh()),
                b,
            ),
        }
    }
}

impl Updatable for Brick {
    fn update(&mut self, dt: f64) {
        println!("update");
    }
}

impl Object2D for Brick {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl Destroyable for Brick {
    fn is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    fn destroy(&mut self) {
        self.is_destroyed = true;
        match self.brick_type {
            BrickType::Block => {
                music::play_sound(&Sound::Brick, music::Repeat::Times(0), 1.0);
            }
            BrickType::Coin => {
                music::play_sound(&Sound::Coin, music::Repeat::Times(0), 1.0);
            }
            _ => {}
        }
    }
}
