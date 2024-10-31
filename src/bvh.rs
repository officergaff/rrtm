use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
};

pub struct BVHNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::construct(&mut list.objects, 0, len)
    }

    pub fn construct(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        // Placeholder
        let left = None;
        let right = None;
        let bbox = AABB::default();
        Self { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = match &self.left {
            Some(n) => n.hit(r, ray_t, rec),
            None => false,
        };
        let hit_right = match &self.right {
            Some(n) => n.hit(r, ray_t, rec),
            None => false,
        };
        return hit_left || hit_right;
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
