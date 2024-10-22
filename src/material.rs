use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    utils::random_double,
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
        *scattered = Ray::new_tm(rec.p, scatter_direction, r_in.time());
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
        *scattered = Ray::new_tm(rec.p, reflected, r_in.time());
        *attenuation = self.albedo;
        return dot(scattered.direction(), rec.normal) > 0.;
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Dielectric {
            refraction_index: ri,
        }
    }

    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation for reflectance
        // https://en.wikipedia.org/wiki/Schlick's_approximation
        let mut r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * f64::powf(1. - cosine, 5.);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(&r_in.direction());
        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_double() {
            // Must reflect
            direction = Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            // Must refract
            direction = Vec3::refract(&unit_direction, &rec.normal, ri);
        }
        *scattered = Ray::new_tm(rec.p, direction, r_in.time());
        true
    }
}
