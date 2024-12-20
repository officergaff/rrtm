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
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct CameraUpdate {
    width: Option<i32>,
    aspect_ratio: Option<f64>,
    samples_per_pixel: Option<u32>,
    max_depth: Option<i32>,
    vfov: Option<f64>,
    lookfrom: Option<[f64; 3]>,
    lookat: Option<[f64; 3]>,
    vup: Option<[f64; 3]>,
    defocus_angle: Option<f64>,
    focus_dist: Option<f64>,
}

#[derive(Serialize)]
#[wasm_bindgen]
pub struct Scene {
    camera: Camera,
    buffer: Vec<Color>, // Aggregate of ray samples per pixel
    image: Vec<u8>,     // Resulting image
    current_sample_count: u32,
    samples_per_pixel: u32,
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
        let lookfrom = Point3::new(13., 2., 3.);
        let lookat = Point3::new(0., 0., 0.);
        let vup = Vec3::new(0., 1., 0.);
        let camera = Camera::new(
            width,
            aspect_ratio,
            1, // Modification to do progressive rendering
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
                        let fuzz = 0.;
                        //let fuzz = random_double_range(0., 0.5);
                        mat = Arc::new(Metal::new(albedo, fuzz));
                    } else {
                        mat = Arc::new(Dielectric::new(1.5));
                    }
                    let center2 = center + Vec3::new(0., random_double_range(0., 0.5), 0.);
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
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
            buffer: vec![Color::default(); camera.image_width() * camera.image_height()],
            camera,
            current_sample_count: 0,
            samples_per_pixel: samples_per_pixel as u32,
            world: bvh,
        }
    }

    // Helps with debugging values
    pub fn to_obj(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }

    pub fn get_camera_obj(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.camera).unwrap()
    }

    // Get the current image
    pub fn get_image(&self) -> Uint8ClampedArray {
        let img_ptr = self.image.as_ptr();
        let mem = wasm_bindgen::memory().unchecked_into::<WebAssembly::Memory>();
        Uint8ClampedArray::new(&mem.buffer())
            .slice(img_ptr as u32, (img_ptr as usize + self.image.len()) as u32)
    }

    // Basically captures one new ray sample per pixel
    pub fn render(&mut self) {
        self.current_sample_count += 1;
        let frame_sample = self.camera.render(&self.world);
        for (i, s) in frame_sample.into_iter().enumerate() {
            self.buffer[i] += s;
            let rgb = (self.buffer[i] / self.current_sample_count as f64).get_rgb();
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
    pub fn current_samples(&self) -> u32 {
        self.current_sample_count
    }
    pub fn clear(&mut self) {
        self.buffer = vec![Color::default(); self.image_width() * self.image_height()];
        self.image = vec![255; 4 * self.image_width() * self.image_height()];
    }

    pub fn update_camera(&mut self, js_camera: JsValue) -> Result<(), JsValue> {
        let camera_update: CameraUpdate = serde_wasm_bindgen::from_value(js_camera)?;

        let lookfrom = camera_update
            .lookfrom
            .map(|arr| Point3::new(arr[0], arr[1], arr[2]))
            .unwrap_or_else(|| self.camera.lookfrom);

        let lookat = camera_update
            .lookat
            .map(|arr| Point3::new(arr[0], arr[1], arr[2]))
            .unwrap_or_else(|| self.camera.lookat);

        let vup = camera_update
            .vup
            .map(|arr| Vec3::new(arr[0], arr[1], arr[2]))
            .unwrap_or_else(|| self.camera.vup);

        self.camera = Camera::new(
            camera_update
                .width
                .unwrap_or(self.camera.image_width() as i32),
            camera_update.aspect_ratio.unwrap_or(16. / 9.),
            1, // Keep progressive rendering
            camera_update.max_depth.unwrap_or(self.camera.max_depth),
            camera_update.vfov.unwrap_or(self.camera.vfov),
            lookfrom,
            lookat,
            vup,
            camera_update
                .defocus_angle
                .unwrap_or(self.camera.defocus_angle),
            camera_update.focus_dist.unwrap_or(self.camera.focus_dist),
        );

        self.clear();
        self.current_sample_count = 0;

        Ok(())
    }
}
