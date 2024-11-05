use rand::{distributions::Uniform, prelude::Distribution, Rng};
use std::f64;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * f64::consts::PI / 180.;
}

pub fn random_double() -> f64 {
    let between = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();
    between.sample(&mut rng)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, max as f64 + 1.) as i32
}
