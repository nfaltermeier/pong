use std::f32::consts;

use rand;

pub fn get_point_on_unit_circle(theta: f32) -> (f32, f32) {
    (theta.cos(), theta.sin())
}

pub fn get_random_point_on_unit_circle() -> (f32, f32) {
    let theta: f32 = rand::random::<f32>() * 2.0 * consts::PI;
    get_point_on_unit_circle(theta)
}
