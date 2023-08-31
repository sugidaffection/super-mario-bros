use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use crate::libs::transform::{Rect, Trans, Transform};
use crate::Sound;
use cgmath::Vector2;
use graphics::math::Matrix2d;
use graphics::{Graphics, Transformed};
use piston_window::{rectangle, DrawState, G2d, G2dTexture, Rectangle, Size};
use sprite::{Scene, Sprite};

use super::core::{Drawable, Entity, Object2D, Updatable};

pub struct Object {
    pub name: String,
    color: [f32; 4],
    border: bool,
    transparent: bool,
    solid: bool,
    transform: Transform,
    scene: Option<Scene<G2dTexture>>,
    sprite: Option<Rc<RefCell<Sprite<G2dTexture>>>>,
    destroyed: bool,
}

impl Object {
    pub fn new(name: String) -> Object {
        Object {
            name,
            color: [1.0, 1.0, 1.0, 1.0],
            border: false,
            transparent: true,
            solid: true,
            transform: Transform::new(),
            scene: None,
            sprite: None,
            destroyed: false,
        }
    }

    pub fn is_solid(&self) -> bool {
        self.solid
    }

    pub fn set_solid(&mut self, value: bool) {
        self.solid = value;
    }

    pub fn set_transparent(&mut self, value: bool) {
        self.transparent = value;
    }

    pub fn set_border(&mut self, value: bool) {
        self.border = value;
    }

    pub fn set_scene(&mut self, scene: Scene<G2dTexture>) {
        self.scene = Some(scene);
    }

    pub fn set_sprite(&mut self, sprite: Rc<RefCell<Sprite<G2dTexture>>>) {
        self.sprite = Some(sprite);
    }

    pub fn run_animation(&self) {
        if let Some(scene) = &self.scene {
            scene.running();
        }
    }

    pub fn destroy(&mut self) {
        if self.name == "brick" {
            self.destroyed = true;
            music::play_sound(&Sound::Brick, music::Repeat::Times(0), 1.0);
        } else {
            self.coin();
        }
    }

    pub fn coin(&mut self) {
        if self.name == "coin" {
            music::play_sound(&Sound::Coin, music::Repeat::Times(0), 1.0);
        }
    }

    pub fn is_destroyed(&self) -> bool {
        self.destroyed
    }
}

impl Default for Object {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            border: false,
            transparent: true,
            solid: true,
            transform: Transform::new(),
            scene: None,
            sprite: None,
            destroyed: false,
        }
    }
}

impl Drawable for Object {
    fn draw(&mut self, t: Matrix2d, b: &mut G2d) {
        if !self.transparent {
            if self.border {
                Rectangle::new_border(self.color, 1.0).draw(
                    self.transform.rect(),
                    &DrawState::default(),
                    t,
                    b,
                );
            } else {
                rectangle(self.color, self.transform.rect(), t, b);
            }
        }
        let scale = self.get_scale();
        if !self.destroyed {
            if let Some(sprite) = self.sprite.as_mut() {
                sprite.borrow().draw(
                    t.scale(scale.x, scale.y)
                        .trans(self.transform.center_xw(), self.transform.center_yh()),
                    b,
                )
            }
        }
    }
}

impl Trans for Object {
    fn set_scale(&mut self, x: f64, y: f64) {
        self.transform.set_scale(x, y);
    }

    fn get_scale(&self) -> Vector2<f64> {
        self.transform.get_scale()
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.transform.set_position(x, y);
    }

    fn set_position_x(&mut self, x: f64) {
        self.transform.set_position_x(x);
    }

    fn set_position_y(&mut self, y: f64) {
        self.transform.set_position_y(y);
    }

    fn get_position(&self) -> Vector2<f64> {
        self.transform.get_position()
    }

    fn set_size(&mut self, w: f64, h: f64) {
        self.transform.set_size(w, h);
    }

    fn get_size(&self) -> Size {
        self.transform.get_size()
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.transform.translate(x, y)
    }

    fn translate_x(&mut self, x: f64) {
        self.transform.translate_x(x)
    }

    fn translate_y(&mut self, y: f64) {
        self.transform.translate_y(y)
    }

    fn set_flip_x(&mut self, value: bool) {
        self.transform.set_flip_x(value);
    }

    fn is_flip_x(&self) -> bool {
        self.transform.is_flip_x()
    }

    fn set_flip_y(&mut self, value: bool) {
        self.transform.set_flip_y(value);
    }

    fn is_flip_y(&self) -> bool {
        self.transform.is_flip_y()
    }

    fn rotate(&mut self, x: f64, y: f64) {
        self.transform.rotate(x, y);
    }
}

impl Object2D for Object {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_transform_mut(&mut self) -> &mut Transform {
        self.transform.borrow_mut()
    }
}

impl Updatable for Object {
    fn update(&mut self, dt: f64) {}
}

impl Entity for Object {}
