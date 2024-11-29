use std::sync::Arc;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{camera::Camera, hittable::Hittable};

#[wasm_bindgen]
pub struct Scene {
    camera: Camera,
    objects: Arc<dyn Hittable>,
}
