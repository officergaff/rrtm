use std::f64;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * f64::consts::PI / 180.;
}
