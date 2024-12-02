use rand::{distributions::Uniform, prelude::Distribution, Rng};
use std::f64;

use js_sys::Promise;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn initialize(num_threads: usize, set_panic_hook: bool) -> Promise {
    if set_panic_hook {
        console_error_panic_hook::set_once();
    }
    wasm_bindgen_rayon::init_thread_pool(num_threads)
}
