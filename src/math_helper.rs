use std::f64::consts;

use rand;

pub fn get_point_on_unit_circle() -> (f64, f64)
{
    let theta: f64 = rand::random::<f64>() * 2.0 * consts::PI;
    (theta.cos(), theta.sin())
}
