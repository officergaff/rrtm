use crate::{
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = center - r.origin();
    let a = dot(r.direction(), r.direction());
    let b = -2. * dot(r.direction(), oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    return discriminant >= 0.;
}
