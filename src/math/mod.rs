use std::f64::EPSILON;

pub mod matrix4x4;
pub mod vec_2d;
pub mod vec_3d;
pub mod vec_4d;
pub mod plane;

fn is_near(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
