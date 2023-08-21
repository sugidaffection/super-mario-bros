use std::cell::RefCell;
use std::rc::Rc;

use crate::libs::transform::{Rect, Trans, Transform};
use cgmath::Vector2;
use graphics::math::Matrix2d;
use graphics::{Graphics, Transformed};
use piston_window::{rectangle, DrawState, G2dTexture, ImageSize, Rectangle, Size};
use sprite::{Scene, Sprite};

pub trait Object2D<I: ImageSize> {
    fn draw<B: Graphics<Texture = I>>(&mut self, t: Matrix2d, b: &mut B);
    fn update(&mut self, dt: f64);
}

pub struct Object<I: ImageSize> {
    pub name: String,
    color: [f32; 4],
    border: bool,
    transparent: bool,
    solid: bool,
    transform: Transform,
    scene: Option<Scene<I>>,
    sprite: Option<Rc<RefCell<Sprite<I>>>>,
}

impl<I> Object<I>
where
    I: ImageSize,
{
    pub fn new(name: String) -> Object<I> {
        Object {
            name,
            color: [1.0, 1.0, 1.0, 1.0],
            border: false,
            transparent: true,
            solid: true,
            transform: Transform::new(),
            scene: None,
            sprite: None,
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

    pub fn get_transform(&self) -> Transform {
        self.transform
    }

    pub fn set_border(&mut self, value: bool) {
        self.border = value;
    }

    pub fn set_scene(&mut self, scene: Scene<I>) {
        self.scene = Some(scene);
    }

    pub fn set_sprite(&mut self, sprite: Rc<RefCell<Sprite<I>>>) {
        self.sprite = Some(sprite);
    }

    pub fn run_animation(&self) {
        if let Some(scene) = &self.scene {
            scene.running();
        }
    }
}

impl<I: ImageSize> Default for Object<I> {
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
        }
    }
}

impl<I> Object2D<I> for Object<I>
where
    I: ImageSize,
{
    fn draw<B: Graphics<Texture = I>>(&mut self, t: Matrix2d, b: &mut B) {
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
        if let Some(sprite) = self.sprite.as_mut() {
            sprite.borrow_mut().set_scale(scale.x, scale.y);
            sprite.borrow().draw(
                t.trans(self.transform.center_xw(), self.transform.center_yh()),
                b,
            )
        }
    }

    fn update(&mut self, dt: f64) {}
}

impl<I: ImageSize> Trans for Object<I> {
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
