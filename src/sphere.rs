use crate::{
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
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
