use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}
