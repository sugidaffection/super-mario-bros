use crate::libs::transform::{Rect, Transform};

#[derive(Debug)]
pub enum Side {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

pub struct Collision {}
impl Collision {
    pub fn aabb(rect1: &Transform, rect2: &Transform) -> Option<Side> {
        if !Collision::check_overlap(rect1, rect2) {
            return None;
        }

        let overlap_left = Collision::calculate_overlap(rect2.xw(), rect1.x());
        let overlap_right = Collision::calculate_overlap(rect1.xw(), rect2.x());
        let overlap_top = Collision::calculate_overlap(rect2.yh(), rect1.y());
        let overlap_bottom = Collision::calculate_overlap(rect1.yh(), rect2.y());

        let min_overlap = f64::min(
            f64::min(overlap_left, overlap_right),
            f64::min(overlap_top, overlap_bottom),
        );

        match min_overlap {
            overlap if overlap == overlap_left => Some(Side::LEFT),
            overlap if overlap == overlap_right => Some(Side::RIGHT),
            overlap if overlap == overlap_top => Some(Side::TOP),
            overlap if overlap == overlap_bottom => Some(Side::BOTTOM),
            _ => None,
        }
    }

    fn check_overlap(rect1: &Transform, rect2: &Transform) -> bool {
        rect1.xw() >= rect2.x()
            && rect1.x() <= rect2.xw()
            && rect1.yh() >= rect2.y()
            && rect1.y() <= rect2.yh()
    }

    fn calculate_overlap(high: f64, low: f64) -> f64 {
        high - low
    }
}
