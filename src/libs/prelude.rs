use cgmath::Vector2;
use graphics::types::Matrix2d;
use piston_window::{G2d, Size};

use super::utils::transform::Transform;

pub trait Rect {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn w(&self) -> f64;
    fn h(&self) -> f64;
    fn xw(&self) -> f64;
    fn yh(&self) -> f64;
    fn center_xw(&self) -> f64;
    fn center_yh(&self) -> f64;
    fn rect_right(&self) -> Vector2<f64>;
    fn rect_bottom(&self) -> Vector2<f64>;
    fn rect_center(&self) -> Vector2<f64>;
    fn rect(&self) -> [f64; 4];
}

pub trait Trans {
    fn set_scale(&mut self, x: f64, y: f64);
    fn get_scale(&self) -> Vector2<f64>;
    fn set_position(&mut self, x: f64, y: f64);
    fn set_position_x(&mut self, x: f64);
    fn set_position_y(&mut self, y: f64);
    fn get_position(&self) -> Vector2<f64>;
    fn set_size(&mut self, w: f64, y: f64);
    fn get_size(&self) -> Size;
    fn translate(&mut self, x: f64, y: f64);
    fn translate_x(&mut self, x: f64);
    fn translate_y(&mut self, y: f64);
    fn set_flip_x(&mut self, value: bool);
    fn is_flip_x(&self) -> bool;
    fn is_flip_y(&self) -> bool;
    fn set_flip_y(&mut self, value: bool);
    fn rotate(&mut self, x: f64, y: f64);
}

pub trait Drawable {
    fn draw(&mut self, t: Matrix2d, b: &mut G2d);
}

pub trait Updatable {
    fn update(&mut self, dt: f64);
}

pub trait Object2D {
    fn get_transform(&self) -> &Transform;
    fn get_transform_mut(&mut self) -> &mut Transform;
}
pub trait Entity: Object2D + Drawable {}

pub trait Destroyable {
    fn is_destroyed(&self) -> bool;
    fn destroy(&mut self);
}

pub trait GameBuilder {
    fn new() -> Self;
    fn build(self) -> Result<(), String>;
}
