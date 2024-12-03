use std::sync::Arc;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    hittable::{Hittable, HittableList},
    material::Lambertian,
    ray::Point3,
    sphere::Sphere,
    texture::NoiseTexture,
    vec3::Vec3,
};
use js_sys::{Uint8ClampedArray, WebAssembly};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[wasm_bindgen]
pub struct Scene {
    camera: Camera,
    buffer: Vec<Color>,
    image: Vec<u8>,
    #[serde(skip)]
    world: Arc<dyn Hittable>,
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
    pub fn new(width: i32, aspect_ratio: f64) -> Self {
        let lookfrom = Point3::new(13., 2., 3.);
        let lookat = Point3::new(0., 0., 0.);
        let vup = Vec3::new(0., 1., 0.);
        let camera = Camera::new(
            width,
            aspect_ratio,
            10,
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
            image: vec![255; camera.image_width() * camera.image_height()],
            camera,
            buffer: Vec::new(),
            world: BVHNode::new(&mut world) as Arc<dyn Hittable>,
        }
    }

    pub fn get_image(&self) -> Uint8ClampedArray {
        let img_ptr = self.image.as_ptr();
        let mem = wasm_bindgen::memory().unchecked_into::<WebAssembly::Memory>();
        Uint8ClampedArray::new(&mem.buffer())
            .slice(img_ptr as u32, (img_ptr as usize + self.image.len()) as u32)
    }

    pub fn render(&self) -> Vec<String> {
        self.camera
            .render(&self.world)
            .iter()
            .map(|v| v.to_string())
            .collect()
    }

    pub fn image_width(&self) -> usize {
        self.camera.image_width()
    }
    pub fn image_height(&self) -> usize {
        self.camera.image_height()
    }
}
