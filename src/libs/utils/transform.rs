use cgmath::Vector2;
use piston_window::Size;

use crate::libs::prelude::{Rect, Trans};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pos: Vector2<f64>,
    size: Size,
    scale: Vector2<f64>,
    rot: Vector2<f64>,
    flip_x: bool,
    flip_y: bool,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            pos: Vector2::from([0.0, 0.0]),
            size: Size {
                width: 16.0,
                height: 16.0,
            },
            scale: Vector2::from([1.0, 1.0]),
            rot: Vector2::from([0.0, 0.0]),
            flip_x: false,
            flip_y: false,
        }
    }
}

impl Rect for Transform {
    fn x(&self) -> f64 {
        self.pos.x
    }

    fn y(&self) -> f64 {
        self.pos.y
    }

    fn w(&self) -> f64 {
        self.size.width * self.scale.x
    }

    fn h(&self) -> f64 {
        self.size.height * self.scale.y
    }

    fn xw(&self) -> f64 {
        self.x() + self.w()
    }

    fn yh(&self) -> f64 {
        self.y() + self.h()
    }

    fn center_xw(&self) -> f64 {
        self.x() + self.w() / 2.0
    }

    fn center_yh(&self) -> f64 {
        self.y() + self.h() / 2.0
    }

    fn rect_right(&self) -> Vector2<f64> {
        Vector2::from([self.xw(), self.y()])
    }

    fn rect_bottom(&self) -> Vector2<f64> {
        Vector2::from([self.x(), self.yh()])
    }

    fn rect_center(&self) -> Vector2<f64> {
        Vector2::from([self.xw() / 2.0, self.yh() / 2.0])
    }

    fn rect(&self) -> [f64; 4] {
        [self.x(), self.y(), self.w(), self.h()]
    }
}

impl Trans for Transform {
    fn set_scale(&mut self, x: f64, y: f64) {
        self.scale.x = x;
        self.scale.y = y;
    }

    fn get_scale(&self) -> Vector2<f64> {
        self.scale
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.pos.x = x;
        self.pos.y = y;
    }

    fn set_position_x(&mut self, x: f64) {
        self.pos.x = x;
    }

    fn set_position_y(&mut self, y: f64) {
        self.pos.y = y
    }

    fn get_position(&self) -> Vector2<f64> {
        self.pos
    }

    fn set_size(&mut self, w: f64, h: f64) {
        self.size.width = w;
        self.size.height = h;
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.pos.x += x;
        self.pos.y += y;
    }

    fn translate_x(&mut self, x: f64) {
        self.pos.x += x;
    }

    fn translate_y(&mut self, y: f64) {
        self.pos.y += y;
    }

    fn set_flip_x(&mut self, value: bool) {
        self.flip_x = value;
    }

    fn is_flip_x(&self) -> bool {
        self.flip_x
    }

    fn set_flip_y(&mut self, value: bool) {
        self.flip_y = value;
    }

    fn is_flip_y(&self) -> bool {
        self.flip_y
    }

    fn rotate(&mut self, x: f64, y: f64) {
        self.rot.x = x;
        self.rot.y = y;
    }
}
