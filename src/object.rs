pub use crate::hit_record::HitRecord;
pub use crate::ray::Ray;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(c: Point3, r: f64) -> Self {
        Self {
            center: Point3 {
                x: c.x,
                y: c.y,
                z: c.z,
            },
            radius: r,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.squared_length();
        let half_b = oc * ray.dir;
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let n = (ray.at(temp) - self.center).unit();
                let outward_normal = (ray.at(temp) - self.center) / self.radius;
                let rec = HitRecord::new(ray.at(temp), n, temp);
                rec.set_face_normal(ray, &outward_normal);
                return Some(rec);
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let n = (ray.at(temp) - self.center).unit();
                let outward_normal = (ray.at(temp) - self.center) / self.radius;
                let rec = HitRecord::new(ray.at(temp), n, temp);
                rec.set_face_normal(ray, &outward_normal);
                return Some(rec);
            }
        }
        Option::None
    }
}
