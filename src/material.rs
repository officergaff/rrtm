use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, unit_vector, Vec3},
};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction where the random_unit_vector is the exact
            // opposite of the normal, thus producing a 0-vector scatter and can lead to undefined
            // behaviors
            scatter_direction = rec.normal
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        reflected = unit_vector(&reflected) + Vec3::random_unit_vector() * self.fuzz;
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return dot(scattered.direction(), rec.normal) > 0.;
    }
}
