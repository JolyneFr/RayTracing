pub use crate::ray::Ray;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;
pub use std::vec;

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

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, n: Vec3, tin: f64) -> Self {
        Self {
            p: Point3 {
                x: point.x,
                y: point.y,
                z: point.z,
            },
            normal: Vec3 {
                x: n.x,
                y: n.y,
                z: n.z,
            },
            t: tin,
            front_face: true,
        }
    }

    pub fn set_face_normal(mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = (r.dir * *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

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
