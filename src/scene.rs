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
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Scene {
    camera: Camera,
    world: Arc<dyn Hittable>,
}

#[wasm_bindgen]
impl Scene {
    pub fn new() -> Self {
        let lookfrom = Point3::new(13., 2., 3.);
        let lookat = Point3::new(0., 0., 0.);
        let vup = Vec3::new(0., 1., 0.);
        let camera = Camera::new(400, 16. / 9., 100, 50, 20., lookfrom, lookat, vup, 0., 12.);

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
