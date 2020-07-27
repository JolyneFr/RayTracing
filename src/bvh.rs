pub use crate::aabb::*;
pub use crate::object::*;
pub use rand::Rng;
pub use std::{cmp::Ordering, sync::Arc};

pub struct BvhNode {
    left: Box<dyn Object>,
    right: Box<dyn Object>,
    cur_box: AABB,
}

impl BvhNode {
    pub fn new_boxed(list: HittableList, time0: f64, time1: f64) -> Box<dyn Object> {
        BvhNode::init(list.objects, time0, time1)
    }

    pub fn init(mut objects: Vec<Box<dyn Object>>, time0: f64, time1: f64) -> Box<dyn Object> {
        let axis = rand::thread_rng().gen_range(0, 3);

        match objects.len() {
            0 => panic!("length mismatch"),
            1 => objects.remove(0),
            _ => {
                objects.sort_by(|a, b| {
                    a.bounding_box(time0, time1).unwrap().min_p[axis]
                        .partial_cmp(&b.bounding_box(time0, time1).unwrap().min_p[axis])
                        .unwrap()
                });

                let mut left_objects = objects;
                let right_objects = left_objects.split_off(left_objects.len() / 2);
                let left = Self::init(left_objects, time0, time1);
                let right = Self::init(right_objects, time0, time1);
                let cur_box = surrounding_box(
                    left.bounding_box(time0, time1).unwrap(),
                    right.bounding_box(time0, time1).unwrap(),
                );
                Box::new(Self {
                    left,
                    right,
                    cur_box,
                })
            }
        }
    }
}

impl Object for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.cur_box.hit(ray, t_min, t_max) {
            true => {
                let hit_left = self.left.hit(ray, t_min, t_max);
                let hit_right = self.right.hit(ray, t_min, t_max);
                match (hit_left, hit_right) {
                    (Some(hit_left), Some(hit_right)) => {
                        if hit_left.t < hit_right.t {
                            Some(hit_left)
                        } else {
                            Some(hit_right)
                        }
                    }
                    (Some(hit_left), None) => Some(hit_left),
                    (None, Some(hit_right)) => Some(hit_right),
                    (None, None) => None,
                }
            }
            false => None,
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.cur_box)
    }
}
