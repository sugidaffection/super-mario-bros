use crate::libs::transform::{Rect, Transform};

pub enum Side {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

pub struct Collision {}
impl Collision {
    pub fn aabb(rect1: &Transform, rect2: &Transform) -> (bool, Option<Side>) {
        let collide = f64::max(rect1.x(), rect1.xw()) >= f64::min(rect2.x(), rect2.xw())
            && f64::min(rect1.x(), rect1.xw()) <= f64::max(rect2.x(), rect2.xw())
            && f64::max(rect1.y(), rect1.yh()) >= f64::min(rect2.y(), rect2.yh())
            && f64::min(rect1.y(), rect1.yh()) <= f64::max(rect2.y(), rect2.yh());

        if collide {
            if rect1.xw() > rect2.x()
                && rect1.center_xw() < rect2.x()
                && rect1.center_yh() > rect2.y()
                && rect1.center_yh() < rect2.yh()
            {
                return (true, Some(Side::RIGHT));
            }

            if rect1.x() < rect2.xw()
                && rect1.center_xw() > rect2.xw()
                && rect1.center_yh() > rect2.y()
                && rect1.center_yh() < rect2.yh()
            {
                return (true, Some(Side::LEFT));
            }

            if rect1.yh() > rect2.y()
                && rect1.center_yh() < rect2.y()
                && rect1.center_xw() > rect2.x()
                && rect1.center_xw() < rect2.xw()
            {
                return (true, Some(Side::BOTTOM));
            }

            if rect1.y() < rect2.yh()
                && rect1.center_yh() > rect2.yh()
                && rect1.center_xw() > rect2.x()
                && rect1.center_xw() < rect2.xw()
            {
                return (true, Some(Side::TOP));
            }
        }

        (false, None)
    }
}
