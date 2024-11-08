use std::{fmt::Debug, sync::Arc};

use crate::{color::Color, ray::Point3};

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
