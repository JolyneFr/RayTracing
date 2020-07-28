pub use crate::object::*;
pub use std::sync::Arc;

pub struct XYrect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYrect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Object for XYrect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let tin = (self.k - ray.orig.z) / ray.dir.z;
        if tin < t_min || tin > t_max {
            return None;
        }

        let x = ray.orig.x + tin * ray.dir.x;
        let y = ray.orig.y + tin * ray.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let mut cur_rec =
            HitRecord::new(ray.at(tin), Vec3::new(0.0, 0.0, 1.0), tin, self.mp.clone());
        cur_rec.set_face_normal(ray, &outward_normal);
        cur_rec.set_uv((u, v));
        Some(cur_rec)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let output_box = AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        Some(output_box)
    }

    fn get_background(&self, _t: f64) -> Color {
        Color::zero()
    }
}
