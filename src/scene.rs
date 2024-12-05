use std::sync::Arc;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    hittable::{Hittable, HittableList},
    material::{Dielectric, Lambertian, Material, Metal},
    ray::Point3,
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    utils::{random_double, random_double_range},
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
    pub fn new(width: i32, aspect_ratio: f64, samples_per_pixel: i32, max_depth: i32) -> Self {
        let lookfrom = Point3::new(0., 0., 12.);
        let lookat = Point3::new(0., 0., 0.);
        let vup = Vec3::new(0., 1., 0.);
        let camera = Camera::new(
            width,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            20.,
            lookfrom,
            lookat,
            vup,
            0.,
            12.,
        );
        let mut world = HittableList::new();

        let checker = Arc::new(CheckerTexture::with_color(
            0.32,
            &Color::new(0.2, 0.3, 0.1),
            &Color::new(0.9, 0.9, 0.9),
        ));
        let ground_mat = Arc::new(Lambertian::with_texture(checker));
        world.add(Arc::new(Sphere::new(
            Point3::new(0., -1000., 0.),
            1000.,
            ground_mat,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_double();
                let center = Point3::new(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );
                if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                    let mat: Arc<dyn Material>;
                    if choose_mat < 0.8 {
                        let albedo = Color::random() * Color::random();
                        mat = Arc::new(Lambertian::new(albedo));
                    } else if choose_mat < 0.95 {
                        let albedo = Color::random_range(0.5, 1.);
                        let fuzz = random_double_range(0., 0.5);
                        mat = Arc::new(Metal::new(albedo, fuzz));
                    } else {
                        mat = Arc::new(Dielectric::new(1.5));
                    }
                    let center2 = center + Vec3::new(0., random_double_range(0., 0.5), 0.);
                    world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, mat)));
                }
            }
        }

        let mat1 = Arc::new(Dielectric::new(1.5));
        world.add(Arc::new(Sphere::new(Point3::new(4., 1., 0.), 1., mat1)));

        let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        world.add(Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1., mat2)));

        let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1., mat3)));
        let bvh = BVHNode::new(&mut world) as Arc<dyn Hittable>;

        Self {
            image: vec![255; 4 * camera.image_width() * camera.image_height()],
            camera,
            buffer: Vec::new(),
            world: bvh,
        }
    }

    pub fn to_obj(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }

    pub fn get_image(&self) -> Uint8ClampedArray {
        let img_ptr = self.image.as_ptr();
        let mem = wasm_bindgen::memory().unchecked_into::<WebAssembly::Memory>();
        Uint8ClampedArray::new(&mem.buffer())
            .slice(img_ptr as u32, (img_ptr as usize + self.image.len()) as u32)
    }

    pub fn render(&mut self) {
        let frame: Vec<[u8; 3]> = self
            .camera
            .render(&self.world)
            .iter()
            .map(|v| v.get_rgb())
            .collect();
        for (i, rgb) in frame.iter().enumerate() {
            self.image[i * 4 + 0] = rgb[0];
            self.image[i * 4 + 1] = rgb[1];
            self.image[i * 4 + 2] = rgb[2];
        }
    }

    pub fn image_width(&self) -> usize {
        self.camera.image_width()
    }
    pub fn image_height(&self) -> usize {
        self.camera.image_height()
    }
}
