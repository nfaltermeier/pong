use std::f32::consts;

use rand;

pub fn get_point_on_unit_circle() -> (f32, f32) {
    let theta: f32 = rand::random::<f32>() * 2.0 * consts::PI;
    (theta.cos(), theta.sin())
}
