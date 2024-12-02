use std::sync::Arc;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    hittable::{Hittable, HittableList},
    material::Lambertian,
    ray::Point3,
    sphere::Sphere,
    texture::NoiseTexture,
    vec3::Vec3,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[wasm_bindgen]
pub struct Scene {
    camera: Camera,
    #[serde(skip)]
    world: Arc<dyn Hittable>,
}

#[derive(Serialize)]
pub struct Data {
    name: String,
    value: i32,
}

#[wasm_bindgen]
pub fn hello() -> JsValue {
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(44444, 1., 100, 50, 20., lookfrom, lookat, vup, 0., 12.);
    serde_wasm_bindgen::to_value(&camera).unwrap()
}

#[wasm_bindgen]
impl Scene {
    pub fn new(width: i32, aspect_ratio: f64, samples_per_pixel: i32) -> Self {
        let lookfrom = Point3::new(13., 2., 3.);
        let lookat = Point3::new(0., 0., 0.);
        let vup = Vec3::new(0., 1., 0.);
        let camera = Camera::new(
            width,
            aspect_ratio,
            samples_per_pixel,
            50,
            20.,
            lookfrom,
            lookat,
            vup,
            0.,
            12.,
        );

        let mut world = HittableList::new();
        let pertext = Arc::new(NoiseTexture::new());
        let ground = Arc::new(Sphere::new(
            Point3::new(0., -1000., 0.),
            1000.,
            Arc::new(Lambertian::with_texture(pertext.clone())),
        ));
        let sphere = Arc::new(Sphere::new(
            Point3::new(0., 2., 0.),
            2.,
            Arc::new(Lambertian::with_texture(pertext.clone())),
        ));
        world.add(ground);
        world.add(sphere);
        Self {
            camera,
            world: BVHNode::new(&mut world) as Arc<dyn Hittable>,
        }
    }
}