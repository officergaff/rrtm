use std::{env::consts, sync::Arc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableAxisCompare, HittableList},
    interval::Interval,
    ray::Ray,
    utils::random_int,
};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(list: &mut HittableList) -> Arc<Self> {
        let len = list.objects.len();
        Self::construct(&mut list.objects, 0, len)
    }

    pub fn construct(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Arc<Self> {
        // Placeholder
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        let axis = random_int(0, 2);
        let comparator = match axis {
            0 => HittableAxisCompare::box_compare_x,
            1 => HittableAxisCompare::box_compare_y,
            _ => HittableAxisCompare::box_compare_z,
        };
        let object_span = end - start;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            }
            _ => {
                objects[start..end].sort_by(|a, b| comparator(a, b));
                let mid = start + object_span / 2;
                left = Self::construct(objects, start, mid);
                right = Self::construct(objects, mid, end);
            }
        }
        let bbox = AABB::with_boxes(&left.bounding_box(), &right.bounding_box());
        Arc::new(Self { left, right, bbox })
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);

        let mut right_interval = Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max });
        let hit_right = self.right.hit(r, &mut right_interval, rec);

        hit_left || hit_right
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
