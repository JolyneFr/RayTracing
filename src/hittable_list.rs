pub use crate::hit_record::HitRecord;
pub use crate::object::Object;
pub use crate::ray::Ray;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;
pub use std::vec;

pub struct HittableList {
    pub objects: Vec<Box<dyn Object>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec::Vec::new(),
        }
    }

    pub fn push(&mut self, ob: Box<dyn Object>) {
        self.objects.push(ob);
    }

    pub fn ray_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut cur_rec = Option::<HitRecord>::None;

        for object in &self.objects {
            let this_rec = object.hit(ray, t_min, closest_so_far);
            if let Some(cur) = this_rec {
                closest_so_far = cur.t;
                cur_rec = this_rec;
            }
        }

        cur_rec
    }
}
