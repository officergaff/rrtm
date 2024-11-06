use std::{env::consts, sync::Arc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableAxisCompare, HittableList},
    interval::Interval,
    ray::Ray,
    utils::random_int,
};

#[derive(Debug)]
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
        // We create a bounding box with all the elements of the objects list
        let mut bbox = AABB::empty();
        for obj in objects[start..end].iter() {
            bbox = AABB::with_boxes(&bbox, &obj.bounding_box());
        }
        // Split the left-right nodes by the longest axis of the bounding box
        let axis = bbox.longest_axis();
        let comparator = match axis {
            0 => HittableAxisCompare::box_compare_x,
            1 => HittableAxisCompare::box_compare_y,
            _ => HittableAxisCompare::box_compare_z,
        };

        // Sort by bounding boxes' axis intervals
        let object_span = end - start;
        objects[start..end].sort_by(|a, b| comparator(a, b));
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        match object_span {
            1 => {
                // Only one object in list, give it to left and right to avoid null nodes
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                // Two objects in the list, split left and right
                left = objects[start].clone();
                right = objects[start + 1].clone();
            }
            _ => {
                // Split objects by midpoint to create a balanced tree
                let mid = start + object_span / 2;
                left = Self::construct(objects, start, mid);
                right = Self::construct(objects, mid, end);
            }
        }
        Arc::new(Self { left, right, bbox })
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let right_interval = Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max });
        let hit_right = self.right.hit(r, right_interval, rec);
        hit_left || hit_right
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
