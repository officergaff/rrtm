use std::{boxed::Box, sync::Arc};

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter outward_normal is assumed to have unit length
        self.front_face = dot(r.direction(), *outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::with_boxes(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Go through every object and check if there's a hit
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        // We keep track of the object hit that is the closest so far
        // This will be used to decrement the ray_tmax
        // This way, the only rec that we keep will end up being the one closest to the camera
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            if obj.hit(
                r,
                &mut Interval::new(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone()
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
