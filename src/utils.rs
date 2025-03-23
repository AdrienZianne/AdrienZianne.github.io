use eframe::egui::{Context, Pos2, Rect, Vec2};

use crate::ui::constant::Constant;

// compute the distance between 2 points
pub fn distance(s1: Pos2, s2: Pos2) -> f32 {
    f32::sqrt((s2.x - s1.x).powi(2) + (s2.y - s1.y).powi(2))
}

// compute the repulsion force of the node
pub fn rep_force(s1: Pos2, s2: Pos2) -> f32 {
    f32::max(-Constant::MAX_FORCE, f32::min(Constant::MAX_FORCE, Constant::CREP / distance(s1, s2).powi(2)))
}

// compute the attraction force of the node
pub fn attract_force(s1: Pos2, s2: Pos2) -> f32 {
    Constant::CSPRING * (distance(s1, s2) / (Constant::L)).log(10.0)
}

// compute the direction between 2 points
pub fn direction(s1: Pos2, s2: Pos2) -> Vec2 {
    let normal_vec = Vec2::new(s2.x - s1.x, s2.y - s1.y).normalized();

    normal_vec
}