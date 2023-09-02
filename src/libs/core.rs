use graphics::{types::Matrix2d, Graphics};
use piston_window::{G2d, G2dTexture};

use super::transform::{Rect, Transform};

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
    fn destroy(&mut self) {}
}
