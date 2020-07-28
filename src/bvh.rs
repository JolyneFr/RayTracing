pub use crate::aabb::*;
pub use crate::object::*;
pub use rand::Rng;
pub use std::{cmp::Ordering, sync::Arc};

pub struct BvhNode {
    left: Arc<dyn Object>,
    right: Arc<dyn Object>,
    cur_box: AABB,
    if_dark: bool,
}

impl BvhNode {
    pub fn new_boxed(list: HittableList, time0: f64, time1: f64) -> Arc<dyn Object> {
        BvhNode::init(list.objects, time0, time1, list.if_dark)
    }

    pub fn init(
        mut objects: Vec<Arc<dyn Object>>,
        time0: f64,
        time1: f64,
        if_dark: bool,
    ) -> Arc<dyn Object> {
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
                let left = Self::init(left_objects, time0, time1, if_dark);
                let right = Self::init(right_objects, time0, time1, if_dark);
                let cur_box = surrounding_box(
                    left.bounding_box(time0, time1).unwrap(),
                    right.bounding_box(time0, time1).unwrap(),
                );
                Arc::new(Self {
                    left,
                    right,
                    cur_box,
                    if_dark,
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

    fn get_background(&self, t: f64) -> Color {
        if self.if_dark {
            Color::zero()
        } else {
            Color::ones() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}
