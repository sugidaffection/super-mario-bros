use crate::libs::transform::{Rect, Transform};

pub struct Collision {}
impl Collision {
    pub fn aabb(rect1: Transform, rect2: Transform) -> bool {
        f64::max(rect1.x(), rect1.xw()) >= f64::min(rect2.x(), rect2.xw())
            && f64::min(rect1.x(), rect1.xw()) <= f64::max(rect2.x(), rect2.xw())
            && f64::max(rect1.y(), rect1.yh()) >= f64::min(rect2.y(), rect2.yh())
            && f64::min(rect1.y(), rect1.yh()) <= f64::max(rect2.y(), rect2.yh())
    }
}
