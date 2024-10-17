use std::{boxed::Box, rc::Rc};

use crate::{
    interval::Interval,
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};
#[derive(Clone, Default, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter outward_normal is assumed to have unit length
        let is_front_face = dot(r.direction(), *outward_normal) < 0.;
        self.normal = if is_front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Go through every object and check if there's a hit
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        // We keep track of the object hit that is the closest so far
        // This will be used to decrement the ray_tmax
        // This way, the only rec that we keep will end up being the one closest to the camera
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            if obj.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec
            }
        }
        return hit_anything;
    }
}
