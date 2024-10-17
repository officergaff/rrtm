use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        return Self {
            center,
            radius,
            material: Some(material),
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin(); // C - Q
        let a = r.direction().length_squared(); // d * d
        let h = dot(r.direction(), oc); // simplified b, b = -2h
        let c = oc.length_squared() - self.radius * self.radius; // (C-Q)*(C-Q) - radius^2
        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return false;
        }

        // Here we are computing the full quadratic equation
        // We are checking if the resulting 't' falls inside the accepted interval
        let sqrtd = f64::sqrt(discriminant);

        // Check if root falls in acceptable range. Check for both signs of the root
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        // We update the hitrecord with the 't', point of intersect
        // and the unit-length of the intersect surface normal
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.material = self.material.clone();
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        return true;
    }
}

pub fn hit_sphere_naive(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin();
    let a = dot(r.direction(), r.direction());
    let b = -2. * dot(r.direction(), oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        return -1.;
    } else {
        return (-b - f64::sqrt(discriminant)) / (2. * a);
    }
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin();
    let a = r.direction().length_squared();
    let h = dot(r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0. {
        return -1.;
    } else {
        return (h - f64::sqrt(discriminant)) / a;
    }
}
