use image::{DynamicImage, GenericImageView, Rgb};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{color::Color, perlin::Perlin, ray::Point3};

pub trait Texture: Send + Sync + Debug {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Debug)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn with_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.albedo
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1. / scale,
            even,
            odd,
        }
    }

    pub fn with_color(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(*c1)),
            Arc::new(SolidColor::new(*c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x = f64::floor(self.inv_scale * p.x()) as i32;
        let y = f64::floor(self.inv_scale * p.y()) as i32;
        let z = f64::floor(self.inv_scale * p.z()) as i32;
        let is_even = (x + y + z) % 2 == 0;
        match is_even {
            true => self.even.value(u, v, p),
            false => self.odd.value(u, v, p),
        }
    }
}

#[derive(Debug)]
pub struct RTImage {
    image: Option<DynamicImage>,
}

impl RTImage {
    pub fn new(filename: &str) -> Self {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("textures")
            .join(filename);
        let image = match image::open(path) {
            Ok(img) => Some(img),
            Err(e) => {
                eprintln!("ERROR: Could not load image file '{}': {}", filename, e);
                None
            }
        };
        Self { image }
    }

    pub fn width(&self) -> u32 {
        self.image.as_ref().map_or(0, |img| img.width())
    }
    pub fn height(&self) -> u32 {
        self.image.as_ref().map_or(0, |img| img.height())
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> Rgb<u8> {
        match &self.image {
            Some(img) => {
                let x = x.min(self.width() - 1);
                let y = y.min(self.height() - 1);

                // Convert pixels to RGB
                let pixel = img.get_pixel(x, y);
                Rgb([pixel[0], pixel[1], pixel[2]])
            }
            None => Rgb([255, 0, 255]), // Magenta for error
        }
    }

    pub fn get_linear_pixel(&self, x: u32, y: u32) -> [f64; 3] {
        let pixel = self.pixel_data(x, y);
        [
            pixel[0] as f64 / 255.,
            pixel[1] as f64 / 255.,
            pixel[2] as f64 / 255.,
        ]
    }
}

#[derive(Debug)]
pub struct ImageTexture {
    image: RTImage, // Using the TextureImage we created earlier
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        ImageTexture {
            image: RTImage::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        // If we have no texture data, return solid cyan as debugging aid
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        // Convert to integer pixel coordinates
        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        // Get pixel data and convert to color
        let pixel = self.image.pixel_data(i, j);
        const COLOR_SCALE: f64 = 1.0 / 255.0;

        Color::new(
            pixel[0] as f64 * COLOR_SCALE,
            pixel[1] as f64 * COLOR_SCALE,
            pixel[2] as f64 * COLOR_SCALE,
        )
    }
}

#[derive(Debug)]
struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        return Color::new(1., 1., 1.) * self.noise.noise(p);
    }
}
