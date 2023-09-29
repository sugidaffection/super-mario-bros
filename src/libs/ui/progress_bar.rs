use cgmath::Vector2;
use graphics::{rectangle, DrawState, Rectangle};
use piston_window::Size;

use crate::libs::prelude::Drawable;

pub struct ProgressBar {
    color: [f32; 4],
    border_color: [f32; 4],
    pos: Vector2<f64>,
    size: Size,
    value: f64,
}

impl ProgressBar {
    pub fn new(color: [f32; 4], border_color: [f32; 4]) -> Self {
        Self {
            color,
            border_color,
            pos: Vector2::from([0.0, 0.0]),
            size: Size::from([48.0, 16.0]),
            value: 0.0,
        }
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        self.size.width = size[0];
        self.size.height = size[1];
    }

    pub fn set_pos(&mut self, pos: [f64; 2]) {
        self.pos.x = pos[0];
        self.pos.y = pos[1];
    }
}

impl Drawable for ProgressBar {
    fn draw(&mut self, t: graphics::types::Matrix2d, b: &mut piston_window::G2d) {
        let rect = [self.pos.x, self.pos.y, self.size.width, self.size.height];
        Rectangle::new_border(self.border_color, 1.0).draw(rect, &DrawState::default(), t, b);

        let rect = [
            self.pos.x,
            self.pos.y,
            self.size.width * self.value,
            self.size.height,
        ];
        rectangle(self.color, rect, t, b);
    }
}
